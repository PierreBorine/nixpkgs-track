use std::{env, fs, io::Write};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{Shell, generate_to};

include!("src/cli.rs");

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();

	fs::create_dir_all(&out_dir).unwrap();

	let mut cmd = Cli::command();
	for &shell in Shell::value_variants() {
		let path = generate_to(shell, &mut cmd, "nixpkgs-track", &out_dir).unwrap();

		// Remove default completions for positional arguments in Fish.
		if shell == Shell::Fish {
			let mut file = fs::OpenOptions::new()
				.append(true)
				.open(path)
				.unwrap();
			writeln!(file, "complete -c nixpkgs-track -f").unwrap();
		}
	}
}
