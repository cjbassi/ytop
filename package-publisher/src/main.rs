use std::fs;
use std::process::Command;
use std::env;

use sha2::{Digest, Sha256};
use hex;
use tokio;
use reqwest;

const VERSION: &str = "0.4.3";

const AUR_DIR: &str = "/home/cjbassi/playground/packages/ytop";
const AUR_BIN_DIR: &str = "/home/cjbassi/playground/packages/ytop-bin";
const HOMEBREW_DIR: &str = "/home/cjbassi/playground/packages/homebrew-ytop";

const AUR_TEMPLATE: &str = include_str!("../templates/aur");
const AUR_BIN_TEMPLATE: &str = include_str!("../templates/aur-bin");
const HOMEBREW_TEMPLATE: &str = include_str!("../templates/homebrew");

const AUR_FILE: &str = "PKGBUILD";
const AUR_BIN_FILE: &str = "PKGBUILD";
const HOMEBREW_FILE: &str = "ytop.rb";


async fn fetch_archive(url: &str) -> Vec<u8> {
	reqwest::get(url)
		.await
		.unwrap()
		.bytes()
		.await
		.unwrap()
		.to_vec()
}

fn hash_archive(archive: &[u8]) -> String {
	let mut hasher = Sha256::new();
	hasher.input(archive);
	hex::encode(&hasher.result()[..])
}

fn srcinfo() {
	let output = Command::new("makepkg")
		.args(&["--printsrcinfo"])
		.output()
		.unwrap()
		.stdout;
	fs::write(".SRCINFO", output).unwrap();
}

fn git_add_commit_push() {
	Command::new("git")
		.args(&["add", "."])
		.status()
		.unwrap();
	Command::new("git")
		.args(&["commit", "-m", VERSION])
		.status()
		.unwrap();
	Command::new("git")
		.args(&["push"])
		.status()
		.unwrap();
}

#[tokio::main]
async fn main() {
	let macos_url = format!(
		"https://github.com/cjbassi/ytop/releases/download/{}/ytop-{}-x86_64-apple-darwin.tar.gz",
		VERSION, VERSION
	);
	let linux_url = format!("https://github.com/cjbassi/ytop/releases/download/{}/ytop-{}-x86_64-unknown-linux-gnu.tar.gz", VERSION, VERSION);
	let repo_url = format!("https://github.com/cjbassi/ytop/archive/{}.tar.gz", VERSION);

	let macos_archive = fetch_archive(&macos_url).await;
	let linux_archive = fetch_archive(&linux_url).await;
	let repo_archive = fetch_archive(&repo_url).await;

	let macos_hash = hash_archive(&macos_archive);
	let linux_hash = hash_archive(&linux_archive);
	let repo_hash = hash_archive(&repo_archive);

	env::set_current_dir(HOMEBREW_DIR).unwrap();
	fs::write(
		HOMEBREW_FILE,
		HOMEBREW_TEMPLATE
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ MACOS_SHA256 }}", &macos_hash)
			.replace("{{ LINUX_SHA256 }}", &linux_hash),
	)
	.unwrap();
	git_add_commit_push();

	env::set_current_dir(AUR_DIR).unwrap();
	fs::write(
		AUR_FILE,
		AUR_TEMPLATE
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ REPO_SHA256 }}", &repo_hash),
	)
	.unwrap();
	srcinfo();
	git_add_commit_push();

	env::set_current_dir(AUR_BIN_DIR).unwrap();
	fs::write(
		AUR_BIN_FILE,
		AUR_BIN_TEMPLATE
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ LINUX_SHA256 }}", &linux_hash),
	)
	.unwrap();
	srcinfo();
	git_add_commit_push();
}
