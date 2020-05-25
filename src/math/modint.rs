#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::redundant_closure_call)]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::cmp::PartialEq;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};
use std::convert::From;

macro_rules! from_int_to_modint_sub {
	($m:ty;$t:ty;$e:expr) => {
		impl From<$t> for ModInt<$m> {
			fn from(x: $t) -> Self {
				$e(x)
			}
		}
	};
}

macro_rules! impl_from_big_uint_to_modint {
	($m1:ty,$($m2:ty),*;$($t:ty),*) => {
		impl_from_big_uint_to_modint!($m1;$($t),*);
		impl_from_big_uint_to_modint!($($m2),*;$($t),*);
	};
	($m:ty;$($t:ty),*) => {
		$(
			from_int_to_modint_sub!($m; $t; |x: $t| Self((x % <$m>::modv() as $t) as u32));
		)*
	};
}

macro_rules! impl_from_big_int_to_modint {
	($m1:ty,$($m2:ty),*;$($t:ty),*) => {
		impl_from_big_int_to_modint!($m1;$($t),*);
		impl_from_big_int_to_modint!($($m2),*;$($t),*);
	};
	($m:ty;$($t:ty),*) => {
		$(
			from_int_to_modint_sub!($m; $t; |x: $t| {
				let m = <$m>::modv() as $t;
				let mut x = x % m;
				if x < 0 {x += m;}
				Self(x as u32)
			});
		)*
	};
}

macro_rules! impl_from_small_int_to_modint {
	($m1:ty,$($m2:ty),*;$($t:ty),*) => {
		impl_from_small_int_to_modint!($m1;$($t),*);
		impl_from_small_int_to_modint!($($m2),*;$($t),*);
	};
	($m:ty;$($t:ty),*) => {
		$(
			from_int_to_modint_sub!($m; $t; |x: $t| {
				let x = x as i32;
				let m = <$m>::modv() as i32;
				let mut x = x % m;
				if x < 0 {x += m;}
				Self(x as u32)
			});
		)*
	};
}

macro_rules! impl_from_small_uint_to_modint {
	($m1:ty,$($m2:ty),*;$($t:ty),*) => {
		impl_from_small_uint_to_modint!($m1;$($t),*);
		impl_from_small_uint_to_modint!($($m2),*;$($t),*);
	};
	($m:ty;$($t:ty),*) => {
		$(
			from_int_to_modint_sub!($m; $t; |x: $t| {
				let x = x as i32;
				let m = <$m>::modv() as i32;
				let mut x = x % m;
				if x < 0 {x += m;}
				Self(x as u32)
			});
		)*
	};
}

impl_from_big_uint_to_modint!(Mod998, Mod107, RuntimeMod; u32, u64, u128, usize);
impl_from_big_int_to_modint!(Mod998, Mod107, RuntimeMod; i32, i64, i128, isize);
impl_from_small_int_to_modint!(Mod998, Mod107, RuntimeMod; i8, i16);
impl_from_small_uint_to_modint!(Mod998, Mod107, RuntimeMod; u8, u16);

#[macro_use]
mod macros {
macro_rules! impl_modint {
	($($im:ty,$b:block);*) => ($(
		impl ModNumber for $im {
			type ModType = u32;
			fn modv() -> Self::ModType $b
			#[inline]
			fn direct(x: u32) -> ModInt<$im> {
				ModInt::<$im>(x)
			}
		}
	)*)
}

macro_rules! impl_modint_operators {
	($($m:ty)*) => ($(
		impl Add<ModInt<$m>> for ModInt<$m> {
			type Output = ModInt<$m>;
			fn add(self, rhs: ModInt<$m>) -> ModInt<$m> {
				ModInt::<$m>((self.0 + rhs.0) % <$m>::modv())
			}
		}
		impl AddAssign<ModInt<$m>> for ModInt<$m> {
			fn add_assign(&mut self, rhs: ModInt<$m>) {
				*self = *self + rhs;
			}
		}
		impl Sub<ModInt<$m>> for ModInt<$m> {
			type Output = ModInt<$m>;
			fn sub(self, rhs: ModInt<$m>) -> ModInt<$m> {
				let mut x = self.0 as i32 - rhs.0 as i32;
				if x < 0 {x += <$m>::modv() as i32}
				ModInt::<$m>(x as u32 % <$m>::modv())
			}
		}
		impl SubAssign<ModInt<$m>> for ModInt<$m> {
			fn sub_assign(&mut self, rhs: ModInt<$m>) {
				*self = *self - rhs;
			}
		}
		impl Mul<ModInt<$m>> for ModInt<$m> {
			type Output = ModInt<$m>;
			fn mul(self, rhs: ModInt<$m>) -> ModInt<$m> {
				let x = self.0 as u64 * rhs.0 as u64;
				ModInt::<$m>((x % <$m>::modv() as u64) as u32)
			}
		}
		impl MulAssign<ModInt<$m>> for ModInt<$m> {
			fn mul_assign(&mut self, rhs: ModInt<$m>) {
				*self = *self * rhs;
			}
		}
		impl Div<ModInt<$m>> for ModInt<$m> {
			type Output = ModInt<$m>;
			fn div(self, rhs: ModInt<$m>) -> ModInt<$m> {
				self * rhs.pow((<$m>::modv() - 2) as u64)
			}
		}
		impl DivAssign<ModInt<$m>> for ModInt<$m> {
			fn div_assign(&mut self, rhs: ModInt<$m>) {
				*self = *self / rhs;
			}
		}
		impl PartialEq<ModInt<$m>> for ModInt<$m> {
			fn eq(&self, rhs: &ModInt<$m>) -> bool {
				self.0 == rhs.0
			}
		}
	)*)
}
macro_rules! impl_modint_fromstr {
	($($m:ty),*) => ($(
		impl FromStr for ModInt<$m> {
			type Err = ParseIntError;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				if s.bytes().next() == Some(b'-') {
					Ok(<ModInt<$m>>::from(s.parse::<i64>()?))
				} else {
					Ok(<ModInt<$m>>::from(s.parse::<u64>()?))
				}
			}
		}
	)*)
}
macro_rules! impl_modint_display {
	($($m:ty),*) => ($(
		impl Display for ModInt<$m> {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}
	)*)
}
} // mod macros

pub trait ModNumber : Clone + Copy {
	type ModType: Clone + Copy;
	fn modv() -> Self::ModType;
	fn direct(x: u32) -> ModInt<Self>;
}

impl_modint!(RuntimeMod, {unsafe{RUNTIME_MOD}};
             Mod107, {1_000_000_007};
             Mod998, {998_244_353});


static mut RUNTIME_MOD: u32 = 0;
#[derive(Clone, Copy)]
pub struct RuntimeMod {}
impl RuntimeMod {
	pub fn set(x: u32) {
		unsafe {
			RUNTIME_MOD = x;
		}
	}
}
#[derive(Clone, Copy)]
pub struct Mod107 {}
#[derive(Clone, Copy)]
pub struct Mod998 {}
#[derive(Clone, Copy)]
pub struct ModInt<MOD: ModNumber> (pub MOD::ModType);

impl_modint_operators!(Mod107 Mod998 RuntimeMod);

impl<MOD: ModNumber> ModInt<MOD> 
where 
	ModInt<MOD>: MulAssign
{
	pub fn pow(self, mut p: u64) -> Self {
		let mut res = <MOD>::direct(1);
		let mut m = self;
		while p != 0 {
			if p & 1 == 1 {
				res *= m;
			}
			p >>= 1;
			m *= m;
		}
		res
	}
}

impl_modint_fromstr!(RuntimeMod, Mod107, Mod998);

impl_modint_display!(RuntimeMod, Mod107, Mod998);