use std::env;
use std::path::PathBuf;
use std::ffi::OsStr;
use library::bundle::module_tree::ModuleTree;

fn parent_path(path: &PathBuf) -> PathBuf {
	path.parent().unwrap().to_path_buf()
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let package_name = args[1].clone();
	let cargo_toml = PathBuf::from(&args[2]);
	let src_file = PathBuf::from(&args[3]);
	if args.len() != 4 {panic!("args.len() != 4");}
	if ! cargo_toml.is_file() || cargo_toml.file_name() != Some(OsStr::new("Cargo.toml")) {
		panic!("Wrong file(Cargo.toml)");
	}
	if ! src_file.is_file() || src_file.extension() != Some(OsStr::new("rs")) {
		panic!("Wrong file(Source file)");
	}
	let project_root = parent_path(&cargo_toml);
	let mut mt = ModuleTree::new(&project_root, &package_name, &src_file);
	mt.print();
}
