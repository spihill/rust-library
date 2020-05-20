#![allow(clippy::unused_io_amount)]
#![allow(clippy::float_cmp)]
#![allow(clippy::approx_constant)]
#![allow(clippy::useless_format)]

use crate::math::monoid::{Semigroup, Operator};
use std::marker::PhantomData;
use std::ops::Range;

pub struct DisjointSparseTable<T, O>
where
	T: Semigroup<O>,
	O: Operator
{
	size: usize,
	semi: Vec<Vec<T>>,
	msb: Vec<usize>,
	_phantom: PhantomData<fn() -> O>,
}

impl<T, O> DisjointSparseTable <T, O>
where
	T: Semigroup<O> + std::fmt::Display,
	O: Operator
{
	pub fn build(s: &[T]) -> Self {
		let size = s.len();
		let mut semi = Vec::new();
		let mut size_pow2 = 1;
		while size_pow2 < size {
			size_pow2 *= 2;
		}
		let mut len = std::cmp::max(size_pow2 / 2, 1);
		while len != 0 {
			let mut v = Vec::<T>::new();
			v.extend_from_slice(s);
			for i in (len..size).step_by(len * 2) {
				for j in 1..len {
					if i + j == size {break;}
					v[i+j] = v[i+j-1].operate(&v[i+j]);
				}
				for j in 2..len+1 {
					v[i-j] = v[i-j].operate(&v[i-j+1]);
				}
			}
			semi.push(v);
			len /= 2;
		}
		let mut msb = Vec::new();
		msb.push(0);
		let mut pow2 = 2;
		let mut pos = 0;
		for i in 1..size_pow2+1 {
			if pow2 == i {
				pow2 *= 2;
				pos += 1;
			}
			msb.push(pos);
		}
		DisjointSparseTable{size, semi, msb, _phantom: PhantomData}
	}
	pub fn get(&self, range: Range<usize>) -> T {
		let l = range.start;
		let r = range.end - 1;
		if l == r {
			self.semi.last().unwrap()[l].clone()
		} else {
			let msb = self.semi.len() - self.msb[l ^ r] - 1;
			self.semi[msb][l].operate(&self.semi[msb][r])
		}
	}
	pub fn len(&self) -> usize {
		self.size
	}
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}
	pub fn debug_print(&self) {
		for i in 0..self.msb.len() {
			println!("{}, msb {}", i, self.msb[i]);
		}
		for s in &self.semi {
			for x in s {
				print!("{}, ", x);
			}
			println!();
		}
	}
}