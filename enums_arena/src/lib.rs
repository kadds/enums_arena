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
//! assert_eq!(arena.get(id), Some(Event::Click(Click::default())));
//! arena.clear();
//! assert_eq!(arena.get(id), None);
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

        /// Alloc value and return id
        pub fn alloc(&mut self, val: Mock<'a, T>) -> MockId<I, G> {
            match val {
                Mock::Mock1 => self.alloc_mock1(),
                Mock::Mock2(val) => self.alloc_mock2(val),
                Mock::Mock3(val) => self.alloc_mock3(val),
            }
        }

        /// Update value for then given id
        pub fn update(&mut self, id: MockId<I, G>, val: Mock<'a, T>) -> Option<()> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();

            match val {
                Mock::Mock1 => {
                    if ty != MockExtendEnum::Mock1 {
                        return None;
                    }
                },
                Mock::Mock2(val) => {
                    if ty != MockExtendEnum::Mock2 {
                        return None;
                    }
                    self.mock2_vec[real_index] = val;
                },
                Mock::Mock3(val) => {
                    if ty != MockExtendEnum::Mock3 {
                        return None;
                    }
                    self.mock3_vec[real_index] = val;
                },
            }
            Some(())
        }

        /// Get enum type from the id
        pub fn ty(&self, id: MockId<I, G>) -> MockExtendEnum {
            id.0
        }

        /// Auto generated from `Mock<T>::Mock2`.
        pub fn get_mock2(&self, id: MockId<I, G>) -> Option<&T> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
            if let MockExtendEnum::Mock2 = ty {
                return Some(&self.mock2_vec[real_index]);
            }
            None
        }

        /// Auto generated from `Mock<T>::Mock3`.
        pub fn get_mock3(&self, id: MockId<I, G>) -> Option<&(i8, u64, &'a str)> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
            if let MockExtendEnum::Mock3 = ty {
                return Some(&self.mock3_vec[real_index]);
            }
            None
        }

        /// Auto generated from `Mock<T>::Mock2`.
        pub fn get_mock2_mut(&mut self, id: MockId<I, G>) -> Option<&mut T> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
            if let MockExtendEnum::Mock2 = ty {
                return Some(&mut self.mock2_vec[real_index]);
            }
            None
        }

        /// Auto generated from `Mock<T>::Mock3`.
        pub fn get_mock3_mut(&mut self, id: MockId<I, G>) -> Option<&mut (i8, u64, &'a str)> {
            let (ty, index, g) = id;
            if g != self.g {
                return None;
            }
            let real_index = self.enums_vec_id_offset_of[index.to_usize()].to_usize();
            if let MockExtendEnum::Mock3 = ty {
                return Some(&mut self.mock3_vec[real_index]);
            }
            None
        }
    }

    /// Auto generated from [`Mock<T>`].
    impl<'a, T, I, G> MockIdArena<'a, T, I, G>
    where
        I: enums_arena_defines::Index,
        G: enums_arena_defines::Generation,
        T: Clone,
    {
        pub fn get(&self, id: MockId<I, G>) -> Option<Mock<T>> {
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
        let mut arena = EnumIdArena::<u8, ()>::default();

        let id = arena.alloc_none();
        assert_eq!(arena.get(id), Some(Enum::None));

        let id = arena.alloc_list_ab((0, 1));
        assert_eq!(arena.get(id), Some(Enum::ListAB((0, 1))));

        let id = arena.alloc_detail(Detail { a: 1, b: 0 });
        assert_eq!(
            arena.get(id),
            Some(Enum::Detail(Detail { a: 1, b: 0 }))
        );

        assert_eq!(arena.len(), 3);


        let id = arena.alloc_value(5);
        assert_eq!(arena.get(id), Some(Enum::Value(5)));

        assert_eq!(arena.update(id, Enum::Value(4)), Some(()));

        assert_eq!(arena.get_value_mut(id), Some(&mut 4));

        *arena.get_value_mut(id).unwrap() = 3;

        assert_eq!(arena.get_value(id), Some(&3));
    }

    #[derive(EnumsIdArena, PartialEq, Eq, Debug)]
    pub enum Node<'a, 'b> {
        Name(&'a str),
        Parent((&'a str, &'b str)),
        None,
    }

    #[test]
    pub fn test_lifetime() {
        let mut arena = NodeIdArena::<u32, u8>::default();
        let id = arena.alloc_name("s");
        assert_eq!(arena.get(id), Some(Node::Name("s")));
        arena.clear();
        assert_eq!(arena.get(id), None);
        assert_eq!(arena.update(id, Node::Name("name1")), None);
    }

    #[derive(EnumsIdArena, PartialEq, Eq, Debug)]
    pub(crate) enum NodeV2<'a, T> {
        Name(&'a str),
        Parent((&'a str, T)),
        None,
    }

    #[test]
    pub fn test_type() {
        let mut arena = NodeV2IdArena::<i8, u32, u8>::default();
        let id = arena.alloc_parent(("s", 1));
        assert_eq!(arena.get(id), Some(NodeV2::Parent(("s", 1))));
    }
}
