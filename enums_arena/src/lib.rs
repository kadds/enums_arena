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
//! let mut arena = EventIdArena::default();
//! let id = arena.alloc_click(Click::default());
//! arena.alloc_tick(1f32);
//! arena.alloc_close();
//!
//! assert_eq!(arena.get_cloned(id), Some(Event::Click(Click::default())))
//! ```
pub use enums_arena_derive::*;

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
        let mut vec = EnumIdArena::default();
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
    }
}
