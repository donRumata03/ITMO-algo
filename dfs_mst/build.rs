//! Bundle A.rs and the crate libraries into A_bundled.rs

use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut bundler: Bundler = Bundler::new(Path::new("src/bin/A.rs"),
	                                        Path::new("src/bin/A_bundled.rs"));
	bundler.crate_name("dfs_mst");
	bundler.run();
	Ok(())
}