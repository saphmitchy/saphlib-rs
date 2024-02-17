use std::ops::Add;

pub trait Zero: Add<Self> + Sized {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> Self {
                $v
            }
        }
    };
}

impl_zero!(usize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);
