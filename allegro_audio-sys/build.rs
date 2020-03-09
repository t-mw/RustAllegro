// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::env::var;

fn main() {
	if var("CARGO_FEATURE_LINK_NONE").is_ok() {
		return;
	}

	let debug = match var("CARGO_FEATURE_LINK_DEBUG") {
		Err(_) => "",
		Ok(_) => "-debug",
	};

	let static_ = match var("CARGO_FEATURE_LINK_STATIC") {
		Err(_) => "",
		Ok(_) => "-static",
	};

	if !static_.is_empty() {
		println!(
			"cargo:rustc-link-lib=static=allegro_audio{}{}",
			debug, static_
		);
	} else {
		println!("cargo:rustc-link-lib=dylib=allegro_audio{}", debug);
	}

	if let Ok(link_dir) = var("ALLEGRO_LINK_DIR") {
		println!("cargo:rustc-link-search=native={}", link_dir);
	}
}
