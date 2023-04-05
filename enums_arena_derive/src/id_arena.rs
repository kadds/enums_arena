use convert_case::Casing;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated,
    token::{Gt, Lt},
    Data, DeriveInput, GenericParam, TypeParam,
};

pub fn enums_id_arena_to(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Err(syn::Error::new(
                Span::call_site().into(),
                "This macro only supports enums.",
            ))
        }
    };
    let vis_control = &ast.vis;

    let id_ident = format_ident!("{}Id", name);
    let generics = &ast.generics;
    let mut generics2 = generics.clone();
    generics2.params.clear();

    let mut generic_res = Vec::new();

    for p in &generics.params {
        match p {
            GenericParam::Lifetime(_) => continue,
            GenericParam::Type(t) => {
                generics2.params.push(p.clone());
                let i = t.ident.clone();
                generic_res.push(quote! {
                    #i: Clone
                })
            }
            GenericParam::Const(_c) => {}
        }
    }
    let user_generics = if generics2.params.is_empty() {
        quote! {}
    } else {
        quote! {
            ::#generics2
        }
    };
    let user_bound = quote! {
        #(#generic_res),*
    };

    let mut extend_enum_fields = Vec::new();
    let mut vec_defines = Vec::new();
    let mut get_cloned_match_body = Vec::new();
    let mut alloc_match_body = Vec::new();
    let mut clear_vecs = Vec::new();
    let mut update_match_body = Vec::new();

    let mut field_fn = Vec::new();

    let enum_name_ident = format_ident!("{}ExtendEnum", name);

    let n = variants.len() as u64;
    let repr = if n <= u8::MAX as u64 {
        quote! {
            #[repr(u8)]
        }
    } else if n <= u16::MAX as u64 {
        quote! {
            #[repr(u16)]
        }
    } else if n <= u32::MAX as u64 {
        quote! {
            #[repr(u32)]
        }
    } else {
        quote! {
            #[repr(u64)]
        }
    };

    for variant in variants {
        let ident = &variant.ident;
        let ident_case = format!("{}", ident).to_case(convert_case::Case::Snake);
        let fields = &variant.fields;

        let is_empty = fields.is_empty();
        let ret_ty = match fields.len() {
            0 => {
                quote! {
                    ()
                }
            }
            1 => {
                let field = fields.iter().next().unwrap();
                quote! {
                    #field
                }
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site().into(),
                    "This macro doesn't support multi-fields in enums",
                ))
            }
        };
        let alloc_ident = format_ident!("alloc_{}", ident_case);
        let vec_ident = format_ident!("{}_vec", ident_case);
        let get_ident = format_ident!("get_{}", ident_case);
        let get_mut_ident = format_ident!("get_{}_mut", ident_case);

        extend_enum_fields.push(quote! {#ident});
        if !is_empty {
            vec_defines.push(quote! {
                #vec_ident: Vec<#ret_ty>
            });
            clear_vecs.push(quote! {
                self.#vec_ident.clear()
            });
            alloc_match_body.push(quote! {
                #name::#ident(val) => self.#alloc_ident(val)
            });
            get_cloned_match_body.push(quote! {
                #enum_name_ident::#ident => {
                    Some(#name::#ident #user_generics(
                        self.#vec_ident.get(ty_index.to_usize()).cloned()?
                    ))
                }
            });
            update_match_body.push(quote! {
                #name::#ident(val) => {
                    if ty != #enum_name_ident::#ident {
                        return None;
                    }
                    self.#vec_ident[real_index] = val;
                }
            });
            field_fn.push(quote!{
                #[allow(unused)]
                pub fn #alloc_ident(&mut self, val: #ret_ty) -> #id_ident<HIDE_I, HIDE_G> {
                    let index = HIDE_I::from_usize(self.enums_vec_id_offset_of.len());
                    let real_index = HIDE_I::from_usize(self.#vec_ident.len());
                    self.#vec_ident.push(val);
                    self.enums_vec_id_offset_of.push(real_index);
                    (#enum_name_ident::#ident, index, self.g)
                }

                #[allow(unused)]
                pub fn #get_ident(&self, id: #id_ident<HIDE_I, HIDE_G>) -> Option<& #ret_ty> {
                    let (ty, index, g) = id;
                    if g != self.g {
                        return None;
                    }
                    let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
                    if let #enum_name_ident::#ident = ty {
                        return Some(&self.#vec_ident[real_index]);
                    }
                    None
                }

                #[allow(unused)]
                pub fn #get_mut_ident(&mut self, id: #id_ident<HIDE_I, HIDE_G>) -> Option<&mut #ret_ty> {
                    let (ty, index, g) = id;
                    if g != self.g {
                        return None;
                    }
                    let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
                    if let #enum_name_ident::#ident = ty {
                        return Some(&mut self.#vec_ident[real_index]);
                    }
                    None
                }
            })
        } else {
            alloc_match_body.push(quote! {
                #name::#ident => self.#alloc_ident()
            });
            get_cloned_match_body.push(quote! {
                #enum_name_ident::#ident => {
                    match ty_index.to_usize() {
                        0 => Some(#name::#ident #user_generics),
                        _ => None,
                    }
                }
            });
            update_match_body.push(quote! {
                #name::#ident => {
                    if ty != #enum_name_ident::#ident {
                        return None;
                    }
                }
            });
            field_fn.push(quote! {
                #[allow(unused)]
                pub fn #alloc_ident(&mut self) -> #id_ident<HIDE_I, HIDE_G> {
                    let index = HIDE_I::from_usize(self.enums_vec_id_offset_of.len());
                    self.enums_vec_id_offset_of.push(HIDE_I::from_usize(0));
                    (#enum_name_ident::#ident, index, self.g)
                }
            });
        }
    }

    let mut new_generics = generics.clone();

    if new_generics.lt_token.is_some() {
    } else {
        new_generics.lt_token = Some(Lt::default());
        new_generics.gt_token = Some(Gt::default())
    }

    new_generics.params.push(syn::GenericParam::Type(TypeParam {
        ident: format_ident!("HIDE_I"),
        attrs: Vec::new(),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    }));
    new_generics.params.push(syn::GenericParam::Type(TypeParam {
        ident: format_ident!("HIDE_G"),
        attrs: Vec::new(),
        colon_token: None,
        bounds: Punctuated::new(),
        eq_token: None,
        default: None,
    }));

    let arena_name_ident = format_ident!("{}IdArena", name);
    let impl_part = quote! {

        #[allow(unused)]
        pub fn ty(&self, id: #id_ident<HIDE_I, HIDE_G>) -> #enum_name_ident {
            id.0
        }

        #[allow(unused)]
        pub fn len(&self) -> usize {
            self.enums_vec_id_offset_of.len()
        }
        #[allow(unused)]
        pub fn clear(&mut self) {
            self.g.add();
            self.enums_vec_id_offset_of.clear();
            #(#clear_vecs);*
        }

        #[allow(unused)]
        pub fn alloc(&mut self, val: #name #generics) -> #id_ident<HIDE_I, HIDE_G> {
            match val {
                #(#alloc_match_body),*
            }
        }

        #[allow(unused)]
        pub fn update(&mut self, id: #id_ident<HIDE_I, HIDE_G>, val: #name #generics) -> Option<()> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();

            match val {
                #(#update_match_body),*
            };
            Some(())
        }

        #(#field_fn)*
    };

    let res = quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, ::std::hash::Hash)]
        #repr
        #vis_control enum #enum_name_ident {
            #(#extend_enum_fields),*
        }

        type #id_ident<I, G> = (#enum_name_ident, I, G);

        #[derive(Default)]
        #vis_control struct #arena_name_ident #new_generics {
            g: HIDE_G,

            enums_vec_id_offset_of: Vec<HIDE_I>,
            #(#vec_defines),*
        }

        impl #new_generics #arena_name_ident #new_generics
        where HIDE_I: ::enums_arena_defines::Index,
            HIDE_G: ::enums_arena_defines::Generation,
        {
            #impl_part
        }

        impl #new_generics #arena_name_ident #new_generics
        where HIDE_I: ::enums_arena_defines::Index,
            HIDE_G: ::enums_arena_defines::Generation,
            #user_bound
        {
            #[allow(unused)]
            pub fn get(&self, id: #id_ident<HIDE_I, HIDE_G>) -> Option<#name #user_generics> {
                let (ty, index, g) = id;
                if g != self.g {
                    return None;
                }
                let ty_index = self.enums_vec_id_offset_of.get(index.to_usize())?.clone();
                match ty {
                    #(#get_cloned_match_body),*
                }
            }
        }

    };

    Ok(res.into())
}
