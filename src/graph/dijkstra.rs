use crate::graph::template::WeightedGraphType;
use crate::datastructure::radix_heap::RadixHeap;

pub fn dijkstra<G: WeightedGraphType<usize, u64>>(graph: &G, start: usize, max_edge_cost: u64) -> (Vec<Option<u64>>, Vec<usize>){
	let n = graph.len();
	let mut prev = vec![n; n];
	let mut dist = vec![None; n];
	let mut used = vec![false; n];
	let mut heap = RadixHeap::<usize>::new(max_edge_cost);
	heap.push(0, start);
	dist[start] = Some(0);
	while let Some((k, u)) = heap.pop() {
		if used[u] {continue;}
		used[u] = true;
		for (v, c) in graph.e(u).iter() {
			let nc = k + c;
			if dist[*v].unwrap_or(std::u64::MAX) <= nc {continue;}
			dist[*v] = Some(nc);
			prev[*v] = u;
			heap.push(nc, *v);
		}
	}
	(dist, prev)
}