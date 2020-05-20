use library::graph::diameter::diameter;
use library::graph::template::{WeightedGraph, WeightedGraphType};
use proconio::{input, fastout};


#[fastout]
fn main() {
	input! {
		n: usize
	}
	let mut g = WeightedGraph::<i64>::new(n);
	for _ in 0..n-1 {
		input! {
			a: usize,
			b: usize,
			c: i64,
		}
		g.add_edge_bidirectional(a, b, &c);
	}
	let (d, path) = diameter(&g);
	println!("{} {}", d, path.len());
	for (i, e) in path.iter().enumerate() {
		if i != 0 {print!(" ");}
		print!("{}", *e);
	}
	println!();
}