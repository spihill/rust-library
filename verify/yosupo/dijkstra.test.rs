// "https://judge.yosupo.jp/problem/shortest_path"

use library::graph::dijkstra::dijkstra;
use library::graph::template::*;
use proconio::{input, fastout};

type Graph = WeightedGraph<u64>;

#[fastout]
fn main() {
	input! {
		N: usize,
		M: usize,
		s: usize,
		t: usize,
	}
	let mut g = Graph::new(N);
	let mut max_cost = 0;
	for _ in 0..M {
		input! {
			a: usize,
			b: usize,
			c: u64,
		}
		g.add_edge(a, b, &c);
		max_cost = std::cmp::max(max_cost, c);
	}
	let (dist, prev) = dijkstra(&g, s, max_cost);
	if let Some(c) = dist[t] {
		print!("{} ", c);
		let mut res = Vec::new();
		let mut t = t;
		while t != N {
			res.push(t);
			t = prev[t];
		}
		println!("{}", res.len() - 1);
		res.reverse();
		for i in 0..res.len()-1 {
			println!("{} {}", res[i], res[i+1]);
		}
	} else {
		println!("-1");
	}
}