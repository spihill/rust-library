use std::marker::PhantomData;
use std::ops::Range;
use crate::math::monoid::{Operator, Monoid};

pub enum Node<T, O>
where
	T: Monoid<O>,
	O: Operator
{
	Nil(std::marker::PhantomData<O>),
	Element {
		val: T,
		left: Box<Node<T, O>>,
		right: Box<Node<T, O>>
	}
}

impl<T, O> Node<T, O> 
where
	T: Monoid<O>,
	O: Operator
{
	fn _set(&mut self, p: i64, v: &T, l: i64, r: i64) -> T {
		if let Self::Element{val: vv, left: lnode, right: rnode} = self {
			if p < l || r <= p {
				vv.clone()
			} else if r - l == 1 {
				*vv = v.clone();
				v.clone()
			} else {
				let lv = &lnode._set(p, v, l, (l+r)/2);
				let rv = &rnode._set(p, v, (l+r)/2, r);
				*vv = lv.operate(rv);
				vv.clone()
			}
		} else if r - l == 1 && p == l {
			let lnode = Box::new(Self::Nil(PhantomData));
			let rnode = Box::new(Self::Nil(PhantomData));
			*self = Self::Element{val: v.clone(), left: lnode, right: rnode};
			v.clone()
		} else if l <= p && p < r {
			let mut lnode = Box::new(Self::Nil(PhantomData));
			let mut rnode = Box::new(Self::Nil(PhantomData));
			let lv = &lnode._set(p, v, l, (l+r)/2);
			let rv = &rnode._set(p, v, (l+r)/2, r);
			let vv = lv.operate(rv);
			*self = Self::Element{val: vv.clone(), left: lnode, right: rnode};
			vv
		} else {
			T::identity()
		}
	}
	fn _get(&self, a: i64, b: i64, l: i64, r: i64) -> T {
		if let Self::Element{val: v, left: lnode, right: rnode} = self {
			if b <= l  || r <= a {
				T::identity()
			} else if a <= l && r <= b {
				v.clone()
			} else {
				let lv = &(lnode._get(a, b, l, (l+r)/2));
				let rv = &(rnode._get(a, b, (l+r)/2, r));
				lv.operate(rv)
			}
		} else {
			T::identity()
		}
	}
}

pub struct DynamicSegmentTree<T, O>
where
	T: Monoid<O>,
	O: Operator
{
	range: Range<i64>,
	root: Node<T, O>
}

impl<T, O> DynamicSegmentTree<T, O>
where
	T: Monoid<O>,
	O: Operator
{
	pub fn new(range: Range<i64>) -> Self {
		Self{range, root: Node::Nil(PhantomData)}
	}
	pub fn set(&mut self, p: i64, v: T) {
		self.root._set(p, &v, self.range.start, self.range.end);
	}
	pub fn get(&self, range: Range<i64>) -> T {
		self.root._get(range.start, range.end, self.range.start, self.range.end)
	}
}