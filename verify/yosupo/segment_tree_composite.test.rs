use library::datastructure::segment_tree::*;
use library::math::monoid::Affine;
use library::math::modint::{ModInt, Mod998};

use proconio::{input, fastout};

type Modint = ModInt<Mod998>;
type RangeAffine<T> = SegmentTree<(T, T), Affine>;
// type RangeAffine<T> = SegmentTree<(T, T), Affine>;

#[fastout]
fn main() {
	input!{
		n: usize,
		q: usize,
		ab: [(Modint, Modint); n],
	}
	let mut raq = RangeAffine::build(&ab);
	for _ in 0..q {
		input! {
			x: i32
		}
		if x == 0 {
			input! {
				p: usize,
				c: Modint,
				d: Modint,
			}
			raq.set(p, (c, d))
		} else {
			input! {
				l: usize,
				r: usize,
				x: Modint,
			}
			let res = raq.get(l..r);
			println!("{}", res.0 * x + res.1);
		}
	}
}