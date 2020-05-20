use library::datastructure::disjoint_sparse_table::DisjointSparseTable;
use library::math::monoid::AddMonoid;
use proconio::{input, fastout};

type RangeSum<T> = DisjointSparseTable<T, AddMonoid>;

#[fastout]
fn main() {
	input!{
		n: usize,
		q: usize,
		a: [i32; n],
		query: [(usize, usize); q]
	}
	let rsq = RangeSum::build(&a);
	for (l, r) in query {
		println!("{}", rsq.get(l..r));
	}
}