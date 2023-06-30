// //! Bundle task files and the crate libraries into {}_bundled.rs

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
		bundler.crate_name("nt");
		// bundler.header("// DO NOT EDIT: This file is generated.\n#![allow(warnings, unused)]");
		bundler.run();
	});
}