use std::{
	ffi::OsStr,
	io::{Read, Write},
	path::Path,
};

use crate::{downloader::DEFAULT_DOWNLOAD_DIRECTORY, post::Page};

pub fn cmd_pause() {
	let mut stdin = std::io::stdin();
	let mut stdout = std::io::stdout();

	write!(stdout, "Press any key to continue...").ok();
	stdout.flush().ok();
	let _ = stdin.read(&mut [0u8]).ok();
}

pub fn create_dl_directory() {
	let is_created: bool = Path::new(DEFAULT_DOWNLOAD_DIRECTORY).is_dir();

	if !is_created {
		println!("No downloads folder found.\nCreating the default one...");
		std::fs::create_dir(DEFAULT_DOWNLOAD_DIRECTORY)
			.expect("Failed to created local downloads directory.");
		cmd_pause();
	}
}

pub fn parse_file_extension(file_name: &str) -> Option<&str> {
	Path::new(file_name).extension().and_then(OsStr::to_str)
}

pub fn get_chosen_url(page: Page) -> Result<Vec<(i64, String)>, anyhow::Error> {
	let mut images: Vec<(i64, String)> = Vec::new();
	let mut chosen_url: String;

	for post in page.post {
		let post_id = post.id.as_i64().unwrap();
		let file_url = post.file_url;
		let preview_url = post.preview_url;
		let sample_url = post.sample_url;

		if file_url != None {
			println!("File url present.\nWill attempt to download file in 'original' quality.");
			// download_file(file_url, post_id).ok();
			chosen_url = file_url.unwrap();
		} else if preview_url != None {
			println!("File url not present.\nWill attempt to download file in 'preview' quality.");
			// download_file(preview_url, post_id).ok();
			chosen_url = preview_url.unwrap();
		} else if sample_url != None {
			println!(
				"File and preview urls missing.\nWill attempt to download file in 'sample' quality."
			);
			// download_file(sample_url, post_id).ok();
			chosen_url = sample_url.unwrap();
		} else {
			println!("No urls found. Try again later.");
			chosen_url = "NULL".parse()?;
		}
		images.push((post_id, chosen_url));
	}
	Ok(images)
}
