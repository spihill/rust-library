use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

pub trait Integer : Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign + Ord {
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

impl_integer!(usize, isize);

impl_sized_integer!(u8, 8; u16, 16; u32, 32; u64, 64; u128, 128;
                    i8, 8; i16, 16; i32, 32; i64, 64; i128, 128);

impl_unsigned_integer!(u8, u16, u32, u64, u128, usize);
impl_signed_integer!(i8, i16, i32, i64, i128, isize);
