// "https://judge.yosupo.jp/problem/staticrmq"

use library::datastructure::disjoint_sparse_table::DisjointSparseTable;
use library::math::monoid::MinMonoid;
use proconio::{input, fastout};

type RangeMin<T> = DisjointSparseTable<T, MinMonoid>;

#[fastout]
fn main() {
	input!{
		n: usize,
		q: usize,
		a: [i32; n],
		query: [(usize, usize); q]
	}
	let rmq = RangeMin::build(&a);
	for (l, r) in query {
		println!("{}", rmq.get(l..r));
	}
}