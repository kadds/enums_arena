//! # enums_arena
//!
//! `enums_arena` is an arena that enums can be stored efficiently
//!
//! usage:
//! ```rust
//! #[derive(PartialEq, Debug, Clone, Default)]
//! struct Click {
//!     x: i32,
//!     y: i32,
//! }
//! #[derive(enums_arena_derive::EnumsIdArena, PartialEq, Debug)]
//! enum Event {
//!     Click(Click),
//!     Tick(f32),
//!     Close,
//! }
//!
//! let mut arena = EventIdArena::<u32, ()>::default();
//! let id = arena.alloc_click(Click::default());
//! arena.alloc_tick(1f32);
//! arena.alloc_close();
//!
//! assert_eq!(arena.get_cloned(id), Some(Event::Click(Click::default())));
//! arena.clear();
//! assert_eq!(arena.get_cloned(id), None);
//! ```
pub use enums_arena_derive::*;

pub mod mock {

    #[derive(PartialEq, Debug)]
    /// Example of a user-defined structure.
    ///
    /// It implements derive trait [`enums_arena_derive::EnumsIdArena`]
    /// to generate [`MockIdArena`] [`MockId`] [`MockExtendEnum`]
    pub enum Mock<'a, T> {
        Mock1,
        Mock2(T),
        Mock3((i8, u64, &'a str)),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(u8)]
    /// Auto generated from [`Mock<T>`].
    pub enum MockExtendEnum {
        Mock1,
        Mock2,
        Mock3,
    }

    /// Auto generated
    pub type MockId<I, G> = (MockExtendEnum, I, G);

    #[derive(Default)]
    /// Auto generated from [`Mock<T>`].
    ///
    /// Type parameter T comes from Mock<**T**>.
    ///
    /// Type parameter I is index data type, including [`u8, u16, u32, u64`].
    ///
    /// Type parameter G is arena generation data type, including [`u8, 16, u32, u64, ()`].
    pub struct MockIdArena<'a, T, I, G> {
        g: G,
        enums_vec_id_offset_of: Vec<I>,

        mock2_vec: Vec<T>,
        mock3_vec: Vec<(i8, u64, &'a str)>
    }

    /// Auto generated from [`Mock<T>`].
    impl<'a, T, I, G> MockIdArena<'a, T, I, G>
    where
        I: enums_arena_defines::Index,
        G: enums_arena_defines::Generation,
    {
        /// Returns alloc times.
        pub fn len(&self) -> usize {
            self.enums_vec_id_offset_of.len()
        }

        /// Clears the arena, removing all values.
        ///
        /// Create a new generation and
        /// all ids allocated in the previous generation are invalid.
        pub fn clear(&mut self) {
            self.g.add();
            self.enums_vec_id_offset_of.clear();
            self.mock2_vec.clear();
        }

        /// Auto generated from `Mock<T>::Mock1`.
        pub fn alloc_mock1(&mut self) -> MockId<I, G> {
            let index = I::from_usize(self.enums_vec_id_offset_of.len());
            self.enums_vec_id_offset_of.push(I::from_usize(0));
            (MockExtendEnum::Mock1, index, self.g)
        }

        /// Auto generated from `Mock<T>::Mock2<T>`.
        pub fn alloc_mock2(&mut self, val: T) -> MockId<I, G> {
            let index = I::from_usize(self.enums_vec_id_offset_of.len());
            let real_index = I::from_usize(self.mock2_vec.len());
            self.mock2_vec.push(val);
            self.enums_vec_id_offset_of.push(real_index);
            (MockExtendEnum::Mock2, index, self.g)
        }

        /// Auto generated from `Mock<T>::Mock3`.
        pub fn alloc_mock3(&mut self, val: (i8, u64, &'a str)) -> MockId<I, G> {
            let index = I::from_usize(self.enums_vec_id_offset_of.len());
            let real_index = I::from_usize(self.mock3_vec.len());
            self.mock3_vec.push(val);
            self.enums_vec_id_offset_of.push(real_index);
            (MockExtendEnum::Mock3, index, self.g)
        }
    }

    /// Auto generated from [`Mock<T>`].
    impl<'a, T, I, G> MockIdArena<'a, T, I, G>
    where
        I: enums_arena_defines::Index,
        G: enums_arena_defines::Generation,
        T: Clone,
    {
        pub fn get_cloned(&self, id: MockId<I, G>) -> Option<Mock<T>> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let ty_index = self.enums_vec_id_offset_of.get(index.to_usize())?.clone();
            match ty {
                MockExtendEnum::Mock1 => match ty_index.to_usize() {
                    0 => Some(Mock::Mock1),
                    _ => None,
                },
                MockExtendEnum::Mock2 => Some(Mock::Mock2::<T>(
                    self.mock2_vec.get(ty_index.to_usize()).cloned()?,
                )),
                MockExtendEnum::Mock3 => Some(Mock::Mock3::<T>(
                    self.mock3_vec.get(ty_index.to_usize()).cloned()?,
                ))
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(PartialEq, Eq, Debug, Clone)]
    struct Detail {
        a: i32,
        b: i32,
    }

    #[derive(EnumsIdArena, PartialEq, Eq, Debug)]
    enum Enum {
        Value(i32),
        None,
        ListAB((i32, u32)),
        Detail(Detail),
        // ListCD(i32, u32),
        // Place{x: u32, z: i8}
    }

    #[test]
    pub fn test_enum() {
        let mut vec = EnumIdArena::<u8, ()>::default();
        let id = vec.alloc_value(5);
        assert_eq!(vec.get_cloned(id), Some(Enum::Value(5)));

        let id = vec.alloc_none();
        assert_eq!(vec.get_cloned(id), Some(Enum::None));

        let id = vec.alloc_list_ab((0, 1));
        assert_eq!(vec.get_cloned(id), Some(Enum::ListAB((0, 1))));

        let id = vec.alloc_detail(Detail { a: 1, b: 0 });
        assert_eq!(
            vec.get_cloned(id),
            Some(Enum::Detail(Detail { a: 1, b: 0 }))
        );

        assert_eq!(vec.len(), 4);
    }

    #[derive(EnumsIdArena, PartialEq, Eq, Debug)]
    enum Node<'a, 'b> {
        Name(&'a str),
        Parent((&'a str, &'b str)),
        None,
    }

    #[test]
    pub fn test_lifetime() {
        let mut vec = NodeIdArena::<u32, u8>::default();
        let id = vec.alloc_name("s");
        assert_eq!(vec.get_cloned(id), Some(Node::Name("s")));
        vec.clear();
        assert_eq!(vec.get_cloned(id), None)
    }

    #[derive(EnumsIdArena, PartialEq, Eq, Debug)]
    enum NodeV2<'a, T> {
        Name(&'a str),
        Parent((&'a str, T)),
        None,
    }

    #[test]
    pub fn test_type() {
        let mut vec = NodeV2IdArena::<i8, u32, u8>::default();
        let id = vec.alloc_parent(("s", 1));
        assert_eq!(vec.get_cloned(id), Some(NodeV2::Parent(("s", 1))));
    }
}
