use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
struct GitVersion {
	pub project_path: PathBuf,
	pub version_hash: [u8; 160],
	pub has_issue_cache: Option<bool>
}

impl GitVersion {
	pub fn get_current_version(path: Path) -> Self {
		let out = Command::new("git")
			.args(&["rev-parse", "HEAD"])
			.output()
			.expect("git fails");

	}
}

impl State for GitVersion {
	type Error = String
	fn bisect(&self, end: &Self) -> Result<Self, Self::Error> {

	}
}

struct RustIssue66617 {}
