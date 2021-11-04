use core::ops::{AddAssign, Shl};

pub trait ValType:
    AddAssign + Default + Copy + Shl<u32, Output = Self>
{
    const BITS: u32;
}

macro_rules! valtype_impl {
    ($type: ty) => {
        impl ValType for $type {
            const BITS: u32 = <$type>::BITS;
        }
    };
}

valtype_impl!(u8);
valtype_impl!(u16);
valtype_impl!(u32);
valtype_impl!(u64);
valtype_impl!(u128);
