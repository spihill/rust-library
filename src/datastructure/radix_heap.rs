#![allow(clippy::type_complexity)]
use std::cmp::min;

pub struct RadixHeap<V: Clone> {
	bucket: Vec<(u64, Vec<Box<(u64, V)>>)>,
	size: usize
}

impl<V: Clone> RadixHeap<V> {
	pub fn new(key: u64) -> Self {
		let s = 64 - key.leading_zeros() as usize + 2;
		RadixHeap{bucket: vec![(0, Vec::new()); s], size: usize::max_value()}
	}
	pub fn push(&mut self, key: u64, value: V) {
		if self.size == usize::max_value() {
			self.size = 1;
			for (i, b) in self.bucket.iter_mut().enumerate() {
				b.0 = key + Self::shift_add(i);
			}
			self.bucket[0].1.push(Box::new((key, value)));
		} else {
			self.size += 1;
			let b = self.bucket.iter_mut().rev().find(|x| x.0 <= key).unwrap();
			b.1.push(Box::new((key, value)));
		}
	}
	pub fn pop(&mut self) -> Option<(u64, V)> {
		if self.size == 0 {
			return None;
		}
		self.size -= 1;
		let left = self.bucket.iter().position(|t| !t.1.is_empty()).unwrap();
		if left != 0 {
			let bucket = std::mem::replace(&mut self.bucket[left], (0, Vec::new()));
			let result = bucket.1.iter().map(|e| e.0).min().unwrap();
			let max_key = self.bucket.get(left + 1).unwrap_or(&(u64::max_value(), Vec::new())).0;
			for i in 0..=left {
				self.bucket[i].0 = min(result + Self::shift_add(i), max_key);
			}
			for b in bucket.1 {
				let x = self.bucket[..left].iter_mut().rev().find(|x| x.0 <= b.0).unwrap();
				x.1.push(b);
			}
		}
		Some(*self.bucket[0].1.pop().unwrap())
	}
	fn shift_add(i: usize) -> u64 {
		if i == 0 {
			0
		} else {
			1 << (i - 1)
		}
	}
}