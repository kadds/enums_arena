use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod id_arena;

#[proc_macro_derive(EnumsIdArena)]
pub fn enums_id_arena(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    id_arena::enums_id_arena_to(&ast).unwrap_or_else(|err| err.to_compile_error().into())
}
