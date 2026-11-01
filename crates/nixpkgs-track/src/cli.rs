use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[command(version, about, subcommand_negates_reqs = true)]
pub struct Cli {
	/// Numerical ID of the pull request to check (e.g. 1234) or a GitHub URL to a pull request (e.g. <https://github.com/nixos/nixpkgs/pull/1234>).
	#[clap(required(true), value_parser = parse_pull_request_id)]
	pub pull_request: Option<u64>,

	#[command(subcommand)]
	pub command: Option<Commands>,

	/// GitHub token
	#[clap(long, short, env = "GITHUB_TOKEN")]
	pub token: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Add pull request(s) to track list
	Add {
		#[clap(required(true), value_parser = parse_pull_request_id)]
		pull_requests: Vec<u64>,
	},
	/// Remove pull request(s) from track list
	Remove {
		#[clap(required_unless_present("all"), value_parser = parse_pull_request_id)]
		pull_requests: Vec<u64>,

		#[clap(long, conflicts_with = "pull_requests")]
		all: bool,

		#[clap(short, long, exclusive = true)]
		interactive: bool,
	},
	/// List tracked pull requests
	List {
		#[clap(long)]
		json: bool,
	},
	/// Check tracked pull requests
	Check {},
}

fn parse_pull_request_id(input: &str) -> Result<u64, String> {
	if let Ok(id) = input.parse::<u64>() {
		return Ok(id);
	}

	// Downcase URL since GitHub URLs are case-insensitive (e.g. "nixos" vs "NixOS").
	if let Ok(url) = Url::parse(input.to_lowercase().as_str())
		&& url.domain() == Some("github.com")
	{
		let segments: Vec<_> = url
			.path_segments()
			.map(Iterator::collect)
			.unwrap_or_default();
		if let ["nixos", "nixpkgs", "pull", id, ..] = segments.as_slice()
			&& let Ok(id) = id.parse::<u64>()
		{
			return Ok(id);
		}
	}

	Err(format!(
		"malformed pull request ID or URL: {input}. expected format: <u64> or https://github.com/nixos/nixpkgs/pull/<u64>"
	))
}

#[cfg(test)]
mod parse_pull_request_id_tests {
	use super::*;

	#[test]
	#[should_panic(expected = "malformed pull request ID or URL")]
	fn test_parse_invalid_pull_request_id() {
		parse_pull_request_id("invalid").unwrap();
	}

	#[test]
	fn test_parse_valid_pull_request_id() {
		assert_eq!(parse_pull_request_id("370713").unwrap(), 370_713);
		assert_eq!(parse_pull_request_id("https://github.com/nixos/nixpkgs/pull/370713").unwrap(), 370_713);
		assert_eq!(
			parse_pull_request_id("https://github.com/NixOS/nixpkgs/pull/392611/files#diff-7b2093bb8f2a6170ffb914e178c91654f6d14c00ab17e9b4d8092fa943deb7b0").unwrap(),
			392_611
		);
	}
}
