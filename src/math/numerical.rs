use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Shl, ShlAssign, Shr, ShrAssign};
use std::convert::From;

pub trait Integer : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self>
				  + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
				  + Shl<Output=Self> + Shr<Output=Self>
				  + ShlAssign + ShrAssign
				  + Ord + Copy {
	fn zero() -> Self;
	fn one() -> Self;
	fn leading_zeros(self) -> u32;
	fn max_value() -> Self;
	fn min_value() -> Self;
}

pub trait SizedInteger : Integer {
	fn bit_size() -> u32;
}

pub trait UnSigned : Integer {}

pub trait Signed : Integer {}

pub trait UnSigned32 : UnSigned + From<u32> {}
pub trait Signed32 : Signed + From<i32> {}

macro_rules! impl_integer {
	($($t:ty),*) => {
		$(
			impl Integer for $t {
				fn zero() -> $t {0}
				fn one() -> $t {1}
				fn leading_zeros(self) -> u32 {self.leading_zeros()}
				fn max_value() -> Self {<$t>::max_value()}
				fn min_value() -> Self {<$t>::min_value()}
			}
		)*
	};
}

macro_rules! impl_sized_integer {
	($($t:ty,$bs:expr);*) => {
		$(
			impl_integer!($t);
			impl SizedInteger for $t {
				fn bit_size() -> u32 {$bs}
			}
		)*
	};
}

macro_rules! impl_unsigned_integer {
	($($t:ty),*) => {
		$(
			impl UnSigned for $t {}
		)*
	};
}

macro_rules! impl_signed_integer {
	($($t:ty),*) => {
		$(
			impl Signed for $t {}
		)*
	};
}

macro_rules! impl_unsigned_32_integer {
	($($t:ty),*) => {
		$(
			impl UnSigned32 for $t {}
		)*
	};
}

macro_rules! impl_signed_32_integer {
	($($t:ty),*) => {
		$(
			impl Signed32 for $t {}
		)*
	};
}

impl_integer!(usize, isize);

impl_sized_integer!(u8, 8; u16, 16; u32, 32; u64, 64; u128, 128;
                    i8, 8; i16, 16; i32, 32; i64, 64; i128, 128);

impl_unsigned_integer!(u8, u16, u32, u64, u128, usize);
impl_unsigned_32_integer!(u32, u64, u128);
impl_signed_integer!(i8, i16, i32, i64, i128, isize);
impl_signed_32_integer!(i32, i64, i128);
