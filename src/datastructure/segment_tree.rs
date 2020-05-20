use std::marker::PhantomData;
use std::ops::Range;
use crate::math::monoid::{MaxMonoid, MinMonoid, AddMonoid, MulMonoid, Affine, GCD, Monoid, Operator};

pub struct SegmentTree<T, O> 
where
	T: Monoid<O>,
	O: Operator,
{
	size: usize,
	monoid: Vec<T>,
	_phantom: PhantomData<fn() -> O>,
}
impl<T, O> SegmentTree<T, O>
where
	T: Monoid<O>,
	O: Operator
{
	pub fn new(sz: usize) -> Self {
		let mut n = 1usize;
		while n < sz {
			n <<= 1;
		}
		SegmentTree{size: n, monoid: vec![T::identity(); n*2-1], _phantom: PhantomData}
	}
	pub fn build(v: &[T]) -> Self {
		let mut seg = Self::new(v.len());
		for (i, e) in v.iter().enumerate() {
			seg.monoid[i+seg.size-1] = e.clone();
		}
		for i in (0..seg.size-1).rev() {
			seg.monoid[i] = seg.monoid[i*2+1].operate(&seg.monoid[i*2+2]);
		}
		seg
	}
	pub fn set(&mut self, i: usize, v: T) {
		let mut i = i + self.size - 1;
		self.monoid[i] = v;
		while i != 0 {
			i = (i-1) / 2;
			self.monoid[i] = self.monoid[i*2+1].operate(&self.monoid[i*2+2]);
		}
	}
	pub fn get(&self, range: Range<usize>) -> T {
		let mut lm = T::identity();
		let mut rm = T::identity();
		let mut l = range.start + self.size - 1;
		let mut r = range.end + self.size - 1;
		while l < r {
			if l & 1 == 0 {lm = lm.operate(&self.monoid[l]);}
			if r & 1 == 0 {rm = self.monoid[r-1].operate(&rm);}
			l /= 2;
			r = (r - 1) / 2;
		}
		lm.operate(&rm)
	}
	pub fn at(&self, p: usize) -> T {
		self.monoid[p + self.size-1].clone()
	}
}

pub type RangeAffine<T> = SegmentTree<(T, T), Affine>;
pub type RangeGCD<T> = SegmentTree<T, GCD>;
pub type RangeSum<T> = SegmentTree<T, AddMonoid>;
pub type RangeProduct<T> = SegmentTree<T, MulMonoid>;
pub type RangeMin<T> = SegmentTree<T, MinMonoid>;
pub type RangeMax<T> = SegmentTree<T, MaxMonoid>;