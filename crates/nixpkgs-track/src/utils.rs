use miette::{Context, IntoDiagnostic};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

fn format_unit(value: i64, unit: &str) -> Option<String> {
	if value > 0 {
		Some(format!("{} {}{}", value, unit, if value > 1 { "s" } else { "" }))
	} else {
		None
	}
}

pub fn format_seconds_to_time_ago(seconds: i64) -> String {
	let minutes = seconds / 60;
	let hours = minutes / 60;
	let days = hours / 24;

	let remaining_seconds = seconds % 60;
	let remaining_minutes = minutes % 60;
	let remaining_hours = hours % 24;

	let result: Vec<String> = [
		format_unit(days, "day"),
		format_unit(remaining_hours, "hour"),
		format_unit(remaining_minutes, "minute"),
		format_unit(remaining_seconds, "second"),
	]
	.into_iter()
	.flatten()
	.collect();

	match result.as_slice() {
		[] => "0 seconds".to_string(),
		[single] => single.clone(),
		[.., last] => format!("{} and {}", result[..result.len() - 1].join(" "), last),
	}
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
	pub pull_requests: Vec<u64>,
}

impl Cache {
	pub fn new() -> Self {
		Cache { pull_requests: vec![] }
	}
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("An error occurred while reading or writing the cache file.")]
pub struct CacheFsError(#[from] pub std::io::Error);

pub fn cache_dir() -> miette::Result<PathBuf> {
	let cache_dir = user_dirs::cache_dir()
		.into_diagnostic()?
		.join("nixpkgs-track");
	if !cache_dir.exists() {
		fs::create_dir_all(&cache_dir).map_err(CacheFsError)?;
	}

	Ok(cache_dir.join("cache.json"))
}

pub fn read_cache(cache: &PathBuf) -> miette::Result<Cache> {
	serde_json::from_str(&fs::read_to_string(cache).map_err(CacheFsError)?)
		.into_diagnostic()
		.context("Failed to parse cache file")
}
