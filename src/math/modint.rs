#![allow(clippy::suspicious_arithmetic_impl)]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::cmp::PartialEq;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};

#[macro_use]
mod macros {
macro_rules! impl_to_modint {
	($($t:ty)*) => ($(
		impl <MOD: ModNumber> ToModInt<MOD> for $t {
			#[inline]
			fn to_modint(self) -> ModInt<MOD> {
				if <$t>::min_value() == 0 {
					<MOD>::from_bu(self as u64)
				} else {
					<MOD>::from_bi(self as i64)
				}
			}
		}
	)*)
}
macro_rules! impl_modint {
	($($im:ty),*) => ($(
		impl ModNumber for $im {
 			type ModType = u32;
			#[inline]
			from_bi_prim_to_modint!($im);
			from_bu_prim_to_modint!($im);
			#[inline]
			fn direct(x: u32) -> ModInt<$im> {
				ModInt::<$im>(x)
			}
		}
	)*)
}

macro_rules! from_bu_prim_to_modint {
	($($im:ty),*) => ($(
		#[inline]
		fn from_bu(x: u64) -> ModInt<$im> {
			let ret_type = ModInt::<$im>(0 as u32);
			let m = ret_type.modv() as u64;
			ModInt::<$im>((x % m) as u32)
		}
	)*)
}
macro_rules! from_bi_prim_to_modint {
	($($im:ty),*) => ($(
		#[inline]
		fn from_bi(i: i64) -> ModInt<$im> {
			let ret_type = ModInt::<$im>(0 as u32);
			let m = ret_type.modv() as i64;
			let mut i = i % m;
			if i < 0 {i += m;}
			ModInt::<$im>((i % m) as u32)
		}
	)*)
}

macro_rules! impl_modint_operators {
	($($m:ty)*) => ($(
		impl Add<ModInt<$m>> for ModInt<$m> {
			type Output = ModInt<$m>;
			fn add(self, rhs: ModInt<$m>) -> ModInt<$m> {
				ModInt::<$m>((self.0 + rhs.0) % self.modv())
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
				if x < 0 {x += self.modv() as i32}
				ModInt::<$m>(x as u32 % self.modv())
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
				ModInt::<$m>((x % self.modv() as u64) as u32)
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
				self * rhs.pow((self.modv() - 2) as u64)
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
					Ok(<$m>::from_bi(s.parse::<i64>()?))
				} else {
					Ok(<$m>::from_bu(s.parse::<u64>()?))
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

pub trait ToModInt<MOD: ModNumber> {
	fn to_modint(self) -> ModInt<MOD>;
}
impl_to_modint!(i8 i16 i32 i64 u8 u16 u32 u64 usize isize);

pub trait ModNumber : Clone + Copy {
	type ModType: Clone + Copy;
	fn from_bi(x: i64) -> ModInt<Self>;
	fn from_bu(x: u64) -> ModInt<Self>;
	fn direct(x: u32) -> ModInt<Self>;
}

impl_modint!(RuntimeMod, Mod107, Mod998);


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

pub trait ModIntTrait<MOD: ModNumber> {
	fn modv(&self) -> MOD::ModType;
}
impl ModIntTrait<Mod107> for ModInt<Mod107> {fn modv(&self) -> <Mod107 as ModNumber>::ModType {1_000_000_007}}
impl ModIntTrait<Mod998> for ModInt<Mod998> {fn modv(&self) -> <Mod998 as ModNumber>::ModType {1_000_000_007}}
impl ModIntTrait<RuntimeMod> for ModInt<RuntimeMod> {fn modv(&self) -> <RuntimeMod as ModNumber>::ModType {unsafe {RUNTIME_MOD}}}

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