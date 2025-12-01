use std::fmt::{Display, Formatter, Result };
use Solution::*;

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I8(v) => v.fmt(f),
            I16(v) => v.fmt(f),
            I32(v) => v.fmt(f),
            I64(v) => v.fmt(f),
            I128(v) => v.fmt(f),
            Isize(v) => v.fmt(f),
            U8(v) => v.fmt(f),
            U16(v) => v.fmt(f),
            U32(v) => v.fmt(f),
            U64(v) => v.fmt(f),
            U128(v) => v.fmt(f),
            Usize(v) => v.fmt(f),
            Str(v) => v.fmt(f),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}
