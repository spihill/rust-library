use crate::graph::template::{IndexType, WeightedGraphType};
use crate::math::monoid::Zero;
use std::ops::Add;
use std::cmp::PartialOrd;
use std::clone::Clone;

fn dfs<I: IndexType, T: Add<Output=T> + Clone + Zero + PartialOrd, G: WeightedGraphType<I, T>>(graph: &G, u: usize, p: usize) -> (usize, T) {
	let mut dist = <T>::zero();
	let mut g = u;
	for (v, c) in graph.e(u) {
		if *v == p {continue;}
		let (g2, c2) = dfs(graph, *v, u);
		let c = c.clone() + c2;
		if dist < c {
			g = g2;
			dist = c;
		}
	}
	(g, dist)
}
fn dfs2<I: IndexType, T: Add<Output=T> + Clone + Zero + PartialOrd, G: WeightedGraphType<I, T>>(graph: &G, u: usize, p: usize, next: &mut Vec<usize>) -> (usize, T) {
	let mut dist = <T>::zero();
	let mut g = u;
	for (v, c) in graph.e(u) {
		if *v == p {continue;}
		let (g2, c2) = dfs2(graph, *v, u, next);
		let c = c.clone() + c2;
		if dist < c {
			g = g2;
			dist = c;
			next[u] = *v;
		}
	}
	(g, dist)
}

pub fn diameter<I: IndexType, T: Add<Output=T> + Clone + Zero + PartialOrd, G: WeightedGraphType<I, T>>(graph: &G) -> (T, Vec<I>){
	let (g, _) = dfs(graph, 0, std::usize::MAX);
	let mut next = vec![std::usize::MAX; graph.size().size()];
	let (_, d) = dfs2(graph, g, std::usize::MAX, &mut next);
	let mut res = Vec::new();
	let mut v = g;
	while {res.push(<I>::from_id(v, graph.size())); next[v] != std::usize::MAX} {
		v = next[v];
	}
	(d, res)
}