//! Bundle A.rs and the crate libraries into A_bundled.rs

use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn change_filename_to_bundled(p: &Path) -> Box<Path> {
	let mut new_path = p.to_path_buf();
	new_path.set_file_name(format!("{}_bundled.rs", p.file_stem().unwrap().to_str().unwrap()));
	new_path.into()
}

fn bundle(src_path: &str) {
	let src_path = Path::new(src_path);
	let input_path = Path::new("src").join(src_path);
	let output_path = Path::new("src").join(change_filename_to_bundled(src_path));
	let mut bundler: Bundler = Bundler::new(&input_path, &output_path);
	bundler.crate_name("dfs_mst");
	bundler.run();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// bundle("main.rs");
	bundle("bin/A.rs");
	bundle("bin/B.rs");
	Ok(())
}