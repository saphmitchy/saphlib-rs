use magma::*;

pub trait Monoid
where
    Self: Magma,
{
    fn identity() -> Self::S;
}

macro_rules! impl_max_min {
    ($t:ty) => {
        impl Monoid for MaxMagma<$t> {
            fn identity() -> Self::S {
                <$t>::MIN
            }
        }

        impl Monoid for MinMagma<$t> {
            fn identity() -> Self::S {
                <$t>::MAX
            }
        }
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
        impl Monoid for AddMagma<$t> {
            fn identity() -> Self::S {
                $zero
            }
        }
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
