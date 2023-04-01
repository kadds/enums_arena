pub trait Index: Copy {
    fn to_usize(self) -> usize;
    fn from_usize(s: usize) -> Self;
}

pub trait Generation: PartialEq + Eq + Copy {
    fn add(&mut self);
}

macro_rules! define_index {
    ($ty: ty) => {
        impl Index for $ty {
            fn to_usize(self) -> usize {
                self as usize
            }
            fn from_usize(s: usize) -> Self {
                s as Self
            }
        }
    };
}

macro_rules! define_generation_number {
    ($ty: ty) => {
        impl Generation for $ty {
            fn add(&mut self) {
                *self += 1;
            }
        }
    };
}

define_index!(u8);
define_index!(u16);
define_index!(u32);
define_index!(u64);

define_generation_number!(u8);
define_generation_number!(u16);
define_generation_number!(u32);
define_generation_number!(u64);

impl Generation for () {
    fn add(&mut self) {}
}
