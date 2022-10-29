// //! Bundle task files and the crate libraries into {}_bundled.rs
//
// use std::path::Path;
// extern crate rustsourcebundler;
// use rustsourcebundler::Bundler;
//
// fn change_filename_to_bundled(p: &Path) -> Box<Path> {
// 	let mut new_path = p.to_path_buf();
// 	new_path.set_file_name(format!("{}_bundled.rs", p.file_stem().unwrap().to_str().unwrap()));
// 	new_path.into()
// }
//
// fn bundle(src_path: &str) {
// 	let src_path = Path::new(src_path);
// 	let input_path = Path::new("src").join(src_path);
// 	let output_path = Path::new("src").join(change_filename_to_bundled(src_path));
// 	let mut bundler: Bundler = Bundler::new(&input_path, &output_path);
// 	bundler.crate_name("dfs_mst");
// 	bundler.run();
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// 	// bundle("main.rs");
// 	bundle("bin/A.rs");
// 	bundle("bin/B.rs");
// 	Ok(())
// }

use rustsourcebundler::Bundler;
use std::fs;
use std::path::Path;

fn main() {
	let entries = fs::read_dir("src/bin")
		.unwrap()
		.map(|e| e.unwrap().path())
		.map(|e| (e.clone(), e.file_stem().unwrap().to_owned().into_string()))
		.filter(|(_e, stem)| stem.is_ok())
		.map(|(e, stem)| (e, stem.unwrap()))
		.filter(|(_e, stem)| !stem.starts_with('_'))
		.map(|(e, stem)| (e, "src/bin/_bundled_".to_string() + stem.as_str() + ".rs"));
	entries.for_each(|(from, to)| {
		let mut bundler: Bundler = Bundler::new(Path::new(&from), Path::new(&to));
		bundler.crate_name("dfs_mst");
		// bundler.header("// DO NOT EDIT: This file is generated.\n#![allow(warnings, unused)]");
		bundler.run();
	});
}