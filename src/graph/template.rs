pub trait Vertex<W, I> {
	fn new(size: I, val: W) -> Self;
	fn at(&self, i: I) -> W;
}

pub struct LineVertex<W: Clone> {
	v: Vec<W>,
}

impl<W: Clone> Vertex<W, usize> for LineVertex<W> {
	fn new(size: usize, val: W) -> Self {
		LineVertex{v: vec![val; size]}
	}
	fn at(&self, i: usize) -> W {
		self.v[i].clone()
	}
}

pub struct PlaneVertex<W: Clone> {
	v: Vec<Vec<W>>,
}

impl<W: Clone> Vertex<W, (usize, usize)> for PlaneVertex<W> {
	fn new(size: (usize, usize), val: W) -> Self {
		PlaneVertex{v: vec![vec![val; size.1]; size.0]}
	}
	fn at(&self, i: (usize, usize)) -> W {
		self.v[i.0][i.1].clone()
	}
}

pub trait IndexType : Copy {
	fn to_id(&self, size: Self) -> usize;
	fn from_id(id: usize, size: Self) -> Self;
	fn size(&self) -> usize;
}

impl IndexType for usize {
	fn to_id(&self, _: Self) -> usize {
		*self
	}
	fn from_id(id: usize, _: Self) -> usize {
		id
	}
	fn size(&self) -> usize {
		*self
	}
}

impl IndexType for (usize, usize) {
	fn to_id(&self, size: Self) -> usize {
		self.0 * size.1 + self.1
	}
	fn from_id(id: usize, size: Self) -> Self {
		(id / size.1, id % size.1)
	}
	fn size(&self) -> usize {
		self.0 * self.1
	}
}

pub trait WeightedGraphType<I: IndexType, T>
{
	fn new(n: I) -> Self;
	fn add_edge(&mut self, from: usize, to: usize, v: &T);
	fn add_edge_bidirectional(&mut self, from: usize, to: usize, v: &T) {
		self.add_edge(from, to, v);
		self.add_edge(to, from, v);
	}
	fn size(&self) -> I;
	fn e_mut(&mut self, v: usize) -> &mut [(usize, T)];
	fn e(&self, v: usize) -> & [(usize, T)];
	fn index_to_id(&self, index: I) -> usize {
		index.to_id(self.size())
	}
	fn id_to_index(&self, id: usize) -> I {
		<I>::from_id(id, self.size())
	}
	fn len(&self) -> usize {
		self.size().size()
	}
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub struct WeightedGraph<T: Clone> {
	size: usize,
	edge: Vec<Vec<(usize, T)>>
}

impl<T: Clone> WeightedGraphType<usize, T> for WeightedGraph<T> {
	fn new(size: usize) -> Self {
		let edge = vec![Vec::new(); size.size()];
		Self{size, edge}
	}
	fn add_edge(&mut self, from: usize, to: usize, v: &T) {
		self.edge[from].push((to, v.clone()))
	}
	fn size(&self) -> usize {
		self.size
	}
	fn e(&self, v: usize) -> &[(usize, T)] {
		&self.edge[v]
	}
	fn e_mut(&mut self, v: usize) -> &mut [(usize, T)] {
		&mut self.edge[v]
	}
}