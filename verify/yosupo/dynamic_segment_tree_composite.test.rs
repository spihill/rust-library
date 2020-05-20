use library::datastructure::dynamic_segment_tree::DynamicSegmentTree;
use library::math::monoid::Affine;
use library::math::modint::{ModInt, Mod998};

use proconio::{input, fastout};

type Modint = ModInt<Mod998>;
type RangeAffine<T> = DynamicSegmentTree<(T, T), Affine>;
// type RangeAffine<T> = SegmentTree<(T, T), Affine>;

#[fastout]
fn main() {
	let k = 100;
	input!{
		n: i64,
		q: usize,
		ab: [(Modint, Modint); n],
	}
	let mut raq = RangeAffine::new(0..n*k);
	for (i, e) in ab.iter().enumerate() {
		raq.set(i as i64 * k, *e);
	}
	for _ in 0..q {
		input! {
			x: i32
		}
		if x == 0 {
			input! {
				p: i64,
				c: Modint,
				d: Modint,
			}
			raq.set(p * k, (c, d))
		} else {
			input! {
				l: i64,
				r: i64,
				x: Modint,
			}
			let res = raq.get(l * k..r * k);
			println!("{}", res.0 * x + res.1);
		}
	}
}