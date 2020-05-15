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
use alga::general::{Operator, Identity, AbstractMagma, AbstractSemigroup, AbstractMonoid};
use num::integer;
use crate::math::modint::{Mod107, Mod998, RuntimeMod, ModInt, ToModInt};

macro_rules! impl_marker(
	// Finds the generic parameters of the type and implements the trait for it
	(@para_rec
		[$tra1t:ty, ($($clause:tt)+), ($($type_constr:tt)*)]
		(< $($params:tt)*)
	) => {
		impl< $($params)* $tra1t for $($type_constr)*< $($params)*
			where $($clause)+
		{}
	};
	// Munches some token trees for searching generic parameters of the type
	(@para_rec
		[$tra1t:ty, ($($clause:tt)+), ($($prev:tt)*)]
		($cur:tt $($rest:tt)*)
	) => {
		impl_marker!(@para_rec
			[$tra1t, ($($clause)+), ($($prev)* $cur)]
			($($rest)*)
		);
	};
	// Handles the trailing separator after where clause
	(@where_rec
		[$tra1t:ty, ($($typ3:tt)+), ($($clause:tt)+)]
		($(;)*)
	) => {
		impl_marker!(@para_rec
			[$tra1t, ($($clause)+), ()]
			($($typ3)+)
		);
	};
	// Implements the trait for the generic type and continues searching other types
	(@where_rec
		[$tra1t:ty, ($($typ3:tt)+), ($($clause:tt)+)]
		(; $($rest:tt)+)
	) => {
		impl_marker!(@para_rec
			[$tra1t, ($($clause)+), ()]
			($($typ3)+)
		);
		impl_marker!(@rec
			[$tra1t, ()]
			($($rest)+)
		);
	};
	// Munches some token trees for searching the end of the where clause
	(@where_rec
		[$tra1t:ty, ($($typ3:tt)+), ($($prev:tt)*)]
		($cur:tt $($rest:tt)*)
	) => {
		impl_marker!(@where_rec
			[$tra1t, ($($typ3)+), ($($prev)* $cur)]
			($($rest)*)
		);
	};
	// Handles the trailing separator for non-generic type and implements the trait
	(@rec
		[$tra1t:ty, ($($typ3:tt)*)]
		($(;)*)
	) => {
		impl $tra1t for $($typ3)* { }
	};
	// Implements the trait for the non-generic type and continues searching other types
	(@rec
		[$tra1t:ty, ($($typ3:tt)*)]
		(; $($rest:tt)+)
	) => {
		impl $tra1t for $($typ3)* { }
		impl_marker!(@rec
			[$tra1t, ()]
			($($rest)+)
		);
	};
	// Detects that there is indeed a where clause for the type and tries to find where it ends.
	(@rec
		[$tra1t:ty, ($($prev:tt)+)]
		(where $($rest:tt)+)
	) => {
		impl_marker!(@where_rec
			[$tra1t, ($($prev)+), ()]
			($($rest)+)
		);
	};
	// Munches some token trees for detecting if we have where clause or not
	(@rec
		[$tra1t:ty, ($($prev:tt)*)]
		($cur:tt $($rest:tt)*)
	) => {
		impl_marker!(@rec
			[$tra1t, ($($prev)* $cur)]
			($($rest)*)
		);
	};
	// Entry point to the macro
	($tra1t:ty; $($rest:tt)+) => {
		impl_marker!(@rec
			[$tra1t, ()]
			($($rest)+)
		);
	};
);
macro_rules! impl_magma(
	($M:ty; $op: ident; $($T:ty),* $(,)*) => {
		$(impl AbstractMagma<$M> for $T {
			#[inline]
			fn operate(&self, lhs: &Self) -> Self {
				self.$op(lhs)
			}
		})*
	}
);
macro_rules! impl_semigroup(
	(<$M:ty> for $($T:tt)+) => {
		impl_marker!(AbstractSemigroup<$M>; $($T)+);
	}
);
macro_rules! impl_monoid(
	(<$M:ty> for $($T:tt)+) => {
		impl_semigroup!(<$M> for $($T)+);
		impl_marker!(AbstractMonoid<$M>; $($T)+);
	}
);
macro_rules! impl_ident {
	($M:ty; $V:expr; $($T:ty),* $(,)*) => {
		$(impl Identity<$M> for $T { #[inline] fn identity() -> $T {$V} })+
	}
}

macro_rules! impl_new_monoid (
	($op:tt,$tr:ident,$f:ident;$e:expr;$($($t:ty),* => $id:expr);*) => {
		#[derive(Clone, Copy)]
		pub struct $op;
		impl Operator for $op {
			fn operator_token() -> Self {
				$op
			}
		}
		trait $tr<T> {
			fn $f(&self, lhs: &Self) -> Self;
		}
		$(
			$(
				impl_ident!($op; $id; $t);
				impl $tr<$t> for $t {
					#[inline]
					fn $f(&self, rhs: &Self) -> Self {
						($e)(self, rhs)
					}
				}
				impl_magma!($op; $f; $t);
				impl_monoid!(<$op> for $t;);
			)*
		)*
	};
	($op:tt,$tr:ident,$f:ident;$e:expr;$($($t:ty),* => TYPE $id:ident());*) => {
		#[derive(Clone, Copy)]
		pub struct $op;
		impl Operator for $op {
			fn operator_token() -> Self {
				$op
			}
		}
		trait $tr<T> {
			fn $f(&self, lhs: &Self) -> Self;
		}
		$(
			$(
				impl_ident!($op; <$t>::$id(); $t);
				impl $tr<$t> for $t {
					#[inline]
					fn $f(&self, rhs: &Self) -> Self {
						($e)(self, rhs)
					}
				}
				impl_magma!($op; $f; $t);
				impl_monoid!(<$op> for $t;);
			)*
		)*
	};
	($op:tt,$tr:ident,$f:ident;$e:expr;$($($t:ty),* => TYPE $id:ident($val:expr));*) => {
		#[derive(Clone, Copy)]
		pub struct $op;
		impl Operator for $op {
			fn operator_token() -> Self {
				$op
			}
		}
		trait $tr<T> {
			fn $f(&self, lhs: &Self) -> Self;
		}
		$(
			$(
				impl_ident!($op; <$t>::$id($val); $t);
				impl $tr<$t> for $t {
					#[inline]
					fn $f(&self, rhs: &Self) -> Self {
						($e)(self, rhs)
					}
				}
				impl_magma!($op; $f; $t);
				impl_monoid!(<$op> for $t;);
			)*
		)*
	}
);
impl_new_monoid!(Affine, AffineTrait, affine_f;
                |lhs: &(_, _), rhs: &(_, _)| (lhs.0 * rhs.0, lhs.1 * rhs.0 + rhs.1);
                (u8, u8), (u16, u16), (u32, u32), (u64, u64), (usize, usize),(i8, i8), (i16, i16), (i32, i32), (i64, i64), (isize, isize) => (1, 0);
                (f32, f32), (f64, f64) => (1.0, 0.0);
                (ModInt<Mod107>, ModInt<Mod107>), (ModInt<Mod998>, ModInt<Mod998>), (ModInt<RuntimeMod>, ModInt<RuntimeMod>) => (1.to_modint(), 0.to_modint()) );

impl_new_monoid!(GCD, GCDTrait, gcd_f;
                |lhs: &_, rhs: &_| integer::gcd(*lhs, *rhs);
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 0);

impl_new_monoid!(AddMonoid, AddMonoidTrait, plus_f;
                |lhs: &_, rhs: &_| *lhs + *rhs;
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 0;
                ModInt<Mod107>, ModInt<Mod998>, ModInt<RuntimeMod> => 0.to_modint());

impl_new_monoid!(MulMonoid, MulMonoidTrait, product_f;
                |lhs: &_, rhs: &_| *lhs * *rhs;
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => 1;
                ModInt<Mod107>, ModInt<Mod998>, ModInt<RuntimeMod> => 1.to_modint());

impl_new_monoid!(MinMonoid, MinMonoidTrait, min_f;
                |lhs: &_, rhs: &_| std::cmp::min(*lhs, *rhs);
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => TYPE max_value());

impl_new_monoid!(MaxMonoid, MaxMonoidTrait, max_m;
                |lhs: &_, rhs: &_| std::cmp::max(*lhs, *rhs);
                u8, u16, u32, u64, usize, i8, i16, i32, i64, isize => TYPE min_value());