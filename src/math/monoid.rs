// Copyright 2013-2014 The Algebra Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(clippy::redundant_closure_call)]
use crate::math::modint::{Mod107, Mod998, RuntimeMod, ModInt};
use std::ops::Rem;

pub trait Zero {fn zero() -> Self;}

macro_rules! impl_zero {
	($e:expr;$($t:ty),*) => {
		$(
			impl Zero  for $t {
				fn zero() -> $t {
					$e
				}
			}
		)*
	};
}

impl_zero!(0; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_zero!(0.0; f32, f64);

pub trait Operator : Copy {}

pub trait Identity<T: Operator> : Clone {
	fn identity() -> Self;
}

pub trait Magma<T: Operator> : Clone {
	fn operate(&self, rhs: &Self) -> Self;
}

pub trait Semigroup<T: Operator> : Magma<T> {}

pub trait Monoid<T: Operator> : Semigroup<T> + Identity<T> {}

macro_rules! impl_identity {
	($op:ty;$t:ty => $e:expr) => {
		impl Identity<$op> for $t {
			fn identity() -> Self {
				$e
			}
		}
	};
	($op:ty;$t:ty => TYPE $f:ident()) => {
		impl Identity<$op> for $t {
			fn identity() -> Self {
				<$t>::$f()
			}
		}
	};
}

macro_rules! define_operator {
	($op:ident, $tr:ident, $f:ident) => {
		#[derive(Clone, Copy)]
		pub struct $op {}
		impl Operator for $op {}
		pub trait $tr<T: Clone> {
			fn $f(&self, rhs: &Self) -> Self;
		}
	}
}

macro_rules! impl_magma {
	($op:ident, $tr:ident, $f:ident; $e:expr; $($t:ty),*) => {
		define_operator!($op, $tr, $f);
		$(
			impl Magma<$op> for $t {
				fn operate(&self, rhs: &Self) -> Self {
					($e)(self, rhs)
				}
			}
		)*
	};
}

macro_rules! impl_semigroup {
	($op:ident, $tr:ident, $f:ident; $e:expr; $($t:ty),*) => {
		impl_magma!($op, $tr, $f; $e; $($t),*);
		$(impl Semigroup<$op> for $t {})*
	};
}
macro_rules! impl_monoid {
	($op:ident, $tr:ident, $f:ident; $e:expr; $($($t:ty),* => $id:expr);*) => {
		impl_semigroup!($op, $tr, $f; $e; $($($t),*),*);
		$($(
			impl_identity!($op; $t => $id);
		)*)*
	};
	($op:ident, $tr:ident, $f:ident; $e:expr; $($($t:ty),* => TYPE $id:ident());*) => {
		impl_semigroup!($op, $tr, $f; $e; $($($t),*),*);
		$($(
			impl_identity!($op; $t => TYPE $id());
		)*)*
	};
}

fn gcd<T: PartialEq>(a: T, b: T) -> T
where
	T: Zero + Copy + Rem<Output=T>
{
	if b == <T>::zero() {
		a
	} else {
		gcd(b, a % b)
	}
}
impl_monoid!(Affine, AffineTrait, affine_f;
                |lhs: &(_, _), rhs: &(_, _)| (lhs.0 * rhs.0, lhs.1 * rhs.0 + rhs.1);
                (u8, u8), (u16, u16), (u32, u32), (u64, u64), (usize, usize),(i8, i8), (i16, i16), (i32, i32), (i64, i64), (isize, isize) => (1, 0);
                (f32, f32), (f64, f64) => (1.0, 0.0);
                (ModInt<Mod107>, ModInt<Mod107>), (ModInt<Mod998>, ModInt<Mod998>), (ModInt<RuntimeMod>, ModInt<RuntimeMod>) => (1.into(), 0.into()) );

impl_monoid!(GCD, GCDTrait, gcd_f;
                |lhs: &_, rhs: &_| gcd(*lhs, *rhs);
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 0);

impl_monoid!(AddMonoid, AddMonoidTrait, plus_f;
                |lhs: &_, rhs: &_| *lhs + *rhs;
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 0;
                ModInt<Mod107>, ModInt<Mod998>, ModInt<RuntimeMod> => 0.into() );

impl_monoid!(MulMonoid, MulMonoidTrait, product_f;
                |lhs: &_, rhs: &_| *lhs * *rhs;
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 1;
                ModInt<Mod107>, ModInt<Mod998>, ModInt<RuntimeMod> => 1.into());

impl_monoid!(MinMonoid, MinMonoidTrait, min_f;
                |lhs: &_, rhs: &_| std::cmp::min(*lhs, *rhs);
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => TYPE max_value());

impl_monoid!(MaxMonoid, MaxMonoidTrait, max_m;
                |lhs: &_, rhs: &_| std::cmp::max(*lhs, *rhs);
				u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => TYPE min_value());