use std::path::PathBuf;
use std::fs::read_to_string;

#[derive(Hash)]
struct ModulePath {
	data: String
}

fn remove_needless_whitespace(line: &str) -> String {
		let mut res = String::new();
		let mut indent = true;
		let mut len = 0;
		for c in line.chars() {
			if ! char::is_whitespace(c) {
				res.push(c);
				indent = false;
				len = 0;
			} else {
				len += 1;
				if !indent || len == 1 {
					res.push(' ');
				}
			}
		}
		if len > 0 {
			res.pop();
		}
		res
}

fn add_bundle_module(line: &str, package_name: &str, prefix: &str) -> String {
	let mut pat = String::from(" ");
	pat.push_str(package_name);
	pat.push_str("::");
	let mut to = String::from(" ");
	to.push_str(prefix);
	to.push_str("bundle::");
	to.push_str(package_name);
	to.push_str("::");
	line.replace(&pat, &to)
}

impl ModulePath {
	fn new_from_line(line: &str, package_name: &str) -> Option<Self> {
		if let Some(s) = Self::_new_from_line(line, package_name, package_name) {
			Some(s)
		} else if let Some(s) = Self::_new_from_line(line, package_name, "crate") {
			Some(s)
		} else {
			None
		}
	}
	fn _new_from_line(line: &str, package_name: &str, top_module: &str) -> Option<Self> {
		let line = remove_needless_whitespace(line);
		let mut pat = String::from("use ");
		pat.push_str(top_module);
		pat.push_str("::");

		let mut to = package_name.to_string();
		to.push_str("::");
		if line.starts_with(&pat) {
			let mut line = line.replace(&pat, &to);
			line.pop();
			Some(ModulePath{data: line})
		} else {
			None
		}
	}
	fn split(&self) -> Vec<String> {
		let mut res: Vec<String> = self.data.split("::").map(|s| s.to_string()).collect();
		res.pop();
		res
	} 
	fn get_path(&self, top: &ModuleNode, project_root: &PathBuf) -> PathBuf {
		let mut mps = self.split();
		if let ModuleNodeType::Node(package_name) = &top.node {
			if &mps[0] != package_name && mps[0] != "crate" {
				panic!("{}, {}", mps[0], package_name);
			}
			if !project_root.is_dir() {
				panic!();
			}
		} else {
			panic!();
		}
		*(mps.last_mut().unwrap()) += ".rs";
		let mut path = project_root.clone().join("src");
		for e in mps.iter().skip(1) {
			path = path.join(e);
		}
		if !path.is_file() {
			panic!("{:?}", path);
		}
		path
	}
}

enum ModuleNodeType {
	Node(String),
	Leaf(Vec<String>)
}

struct ModuleNode {
	node: ModuleNodeType,
	nodes: Vec<ModuleNode>
}

impl ModuleNode {
	pub fn new(package_name: &str) -> Self {
		let node = ModuleNodeType::Node(package_name.into());
		let nodes = Vec::new();
		ModuleNode{node, nodes}
	}
	fn push_module(&mut self, mut m: Vec<String>, src: Vec<String>) {
		if let ModuleNodeType::Node(_) = &self.node {
			m.remove(0);
			if m.is_empty() {
				return self._push_leaf(src)
			}
			for mt in self.nodes.iter_mut() {
				if let ModuleNodeType::Node(mt_e) = &mt.node {
					if *mt_e == m[0] {
						mt.push_module(m, src);
						return;
					}
				}
			}
			let node = ModuleNodeType::Node(m[0].clone());
			let nodes = Vec::new();
			self.nodes.push(ModuleNode{node, nodes});
			self.nodes.last_mut().unwrap().push_module(m, src);
		} else {
			panic!();
		}
	}
	fn _push_leaf(&mut self, src: Vec<String>) {
		if ! self.nodes.is_empty() {
			return;
		}
		let node = ModuleNodeType::Leaf(src);
		let nodes = Vec::new();
		self.nodes.push(ModuleNode{node, nodes});
	}
	fn print(&self, depth: usize, package_name: &str) {
		let indent = String::from("	").repeat(depth);
		if let ModuleNodeType::Node(n) = &self.node {
			println!("{}pub mod {} {{", indent, n);
			for e in &self.nodes {
				e.print(depth+1, package_name);
			}
			println!("{}}} // mod {}", indent, n);
		} else if let ModuleNodeType::Leaf(src) = &self.node {
			for line in src.iter() {
				println!("{}{}", indent, add_bundle_module(line, package_name, "crate::"));
			}
		} else {
			panic!();
		}
	}
}

pub struct ModuleTree {
	project_root: PathBuf,
	package_name: String,
	main_src: Vec<String>,
	top: ModuleNode
}

impl ModuleTree {
	pub fn new(project_root: &PathBuf, package_name: &str, main_path: &PathBuf) -> Self {
		let top = ModuleNode::new(package_name);
		let package_name = package_name.to_string();
		let main_src = Self::get_file_contents(&package_name, &main_path);
		let project_root = project_root.clone();
		ModuleTree{project_root, package_name, main_src, top}
	}
	pub fn print(&mut self) {
		let mut module_path = Vec::new();
		for line in &self.main_src {
			println!("{}", add_bundle_module(line, &self.package_name, ""));
			let mpath = ModulePath::new_from_line(line, &self.package_name);
			if let Some(mpath) = mpath {
				module_path.push(mpath);
			}
		}
		while ! module_path.is_empty() {
			let ms = self.push_module(module_path.last().unwrap());
			module_path.pop();
			for ems in ms {
				module_path.push(ems);
			}
		}
		println!("pub mod bundle {{");
		self.top.print(0, &self.package_name);
		println!("}} // mod bundle");
	}
	fn _get_module_path_from_crate_from_src(src: &[String], package_name: &str) -> Vec<ModulePath> {
		let mut res = Vec::new();
		for line in src {
			if let Some(l) = ModulePath::new_from_line(line, package_name) {
				res.push(l);
			}
		}
		res
	}
	fn push_module(&mut self, module_path: &ModulePath) -> Vec<ModulePath> {
		let mut m = Vec::new();
		for e in &module_path.split() {
			m.push(e.to_string());
		}
		let src_path = module_path.get_path(&self.top, &self.project_root);
		let mut res = Vec::new();
		let src = Self::get_file_contents(&self.package_name, &src_path);
		for line in &src {
			if let Some(s) = ModulePath::new_from_line(line, &self.package_name) {
				res.push(s);
			}
		}
		self.top.push_module(m, src);
		res
	}
	fn get_file_contents(package_name: &str, src_path: &PathBuf) -> Vec<String> {
		let src = read_to_string(src_path.clone());
		let mut src: Vec<String> = if let Ok(s) = &src {
			s.split('\n').map(|s| s.to_string()).collect()
		} else {
			panic!("Unknown Error");
		};
		for e in &mut src {
			let mut pat = String::from(" ");
			pat.push_str(package_name);
			pat.push_str("::");
			*e = e.replace(" crate::", &pat);
		}
		src
	}
}