use magma::AddMagma;
use monoid::Monoid;

pub trait Group
where
    Self: Monoid,
{
    fn inverse(x: Self::S) -> Self::S;
}

macro_rules! impl_add {
    ($t: ty) => {
        impl Group for AddMagma<$t> {
            fn inverse(x: Self::S) -> Self::S {
                -x
            }
        }
    };
}

impl_add!(i8);
impl_add!(i16);
impl_add!(i32);
impl_add!(i64);
impl_add!(i128);
impl_add!(f32);
impl_add!(f64);
