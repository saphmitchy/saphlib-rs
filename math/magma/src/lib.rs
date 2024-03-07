use std::marker::PhantomData;

pub trait Magma {
    type S;
    fn binary_operation(lhs: Self::S, rhs: Self::S) -> Self::S;
}

pub trait Commutative {}

pub struct MaxMagma<T>(PhantomData<T>);
pub struct MinMagma<T>(PhantomData<T>);
pub struct AddMagma<T>(PhantomData<T>);

macro_rules! impl_max_min {
    ($t:ty) => {
        impl Magma for MaxMagma<$t> {
            type S = $t;
            fn binary_operation(lhs: Self::S, rhs: Self::S) -> Self::S {
                lhs.max(rhs)
            }
        }

        impl Magma for MinMagma<$t> {
            type S = $t;
            fn binary_operation(lhs: Self::S, rhs: Self::S) -> Self::S {
                lhs.min(rhs)
            }
        }
        impl Commutative for MaxMagma<$t> {}
        impl Commutative for MinMagma<$t> {}
    };
}

impl_max_min!(usize);
impl_max_min!(i8);
impl_max_min!(i16);
impl_max_min!(i32);
impl_max_min!(i64);
impl_max_min!(i128);
impl_max_min!(u8);
impl_max_min!(u16);
impl_max_min!(u32);
impl_max_min!(u64);
impl_max_min!(u128);

macro_rules! impl_add {
    ($t: ty, $zero:tt) => {
        impl Magma for AddMagma<$t> {
            type S = $t;
            fn binary_operation(lhs: Self::S, rhs: Self::S) -> Self::S {
                lhs + rhs
            }
        }
        impl Commutative for AddMagma<$t> {}
    };
}

impl_add!(usize, 0);
impl_add!(i8, 0);
impl_add!(i16, 0);
impl_add!(i32, 0);
impl_add!(i64, 0);
impl_add!(i128, 0);
impl_add!(u8, 0);
impl_add!(u16, 0);
impl_add!(u32, 0);
impl_add!(u64, 0);
impl_add!(u128, 0);
impl_add!(f32, 0.0);
impl_add!(f64, 0.0);
