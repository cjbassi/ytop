use sha2::{Digest, Sha256};
use std::fs;
use std::process::Command;

const VERSION: &str = "0.4.3";

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

#[tokio::main]
async fn main() {
	let aur_dir = "/home/cjbassi/playground/packages/ytop";
	let aur_bin_dir = "/home/cjbassi/playground/packages/ytop-bin";
	let homebrew_dir = "/home/cjbassi/playground/packages/homebrew-ytop";

	let aur_template = include_str!("../templates/aur");
	let aur_bin_template = include_str!("../templates/aur-bin");
	let homebrew_template = include_str!("../templates/homebrew");

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

	std::env::set_current_dir(homebrew_dir).unwrap();
	fs::write(
		"ytop.rb",
		homebrew_template
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ MACOS_SHA256 }}", &macos_hash)
			.replace("{{ LINUX_SHA256 }}", &linux_hash),
	)
	.unwrap();
	Command::new("git")
		.args(&["add", "."])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["commit", "-m", VERSION])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["push"])
		.status()
		.expect("failed to execute process");

	std::env::set_current_dir(aur_dir).unwrap();
	fs::write(
		"PKGBUILD",
		aur_template
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ SHA256 }}", &repo_hash),
	)
	.unwrap();
	let output = Command::new("makepkg")
		.args(&["--printsrcinfo"])
		.output()
		.expect("failed to execute process")
		.stdout;
	fs::write(".SRCINFO", output).unwrap();
	Command::new("git")
		.args(&["add", "."])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["commit", "-m", VERSION])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["push"])
		.status()
		.expect("failed to execute process");

	std::env::set_current_dir(aur_bin_dir).unwrap();
	fs::write(
		"PKGBUILD",
		aur_bin_template
			.replace("{{ VERSION }}", VERSION)
			.replace("{{ SHA256 }}", &linux_hash),
	)
	.unwrap();
	let output = Command::new("makepkg")
		.args(&["--printsrcinfo"])
		.output()
		.expect("failed to execute process")
		.stdout;
	fs::write(".SRCINFO", output).unwrap();
	Command::new("git")
		.args(&["add", "."])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["commit", "-m", VERSION])
		.status()
		.expect("failed to execute process");
	Command::new("git")
		.args(&["push"])
		.status()
		.expect("failed to execute process");
}
