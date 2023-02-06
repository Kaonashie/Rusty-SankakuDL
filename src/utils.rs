use std::{
	ffi::OsStr,
	io::{Read, Write},
	path::Path,
};

use crate::{downloader::DEFAULT_CACHE_DIRECTORY, structure::post::Page};
use crate::{downloader::DEFAULT_DOWNLOAD_DIRECTORY, structure::images::ImageCollection};

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
		println!("No downloads folder found.\nCreating default one.");
		std::fs::create_dir_all(DEFAULT_DOWNLOAD_DIRECTORY).expect("Failed to create local downloads directory.");
		cmd_pause();
	}
}

pub fn create_cache_directory() -> Result<String, anyhow::Error> {
	let is_created: bool = Path::new(DEFAULT_CACHE_DIRECTORY).is_dir();

	if !is_created {
		println!("No cache folder found.\nCreating default one.");
		std::fs::create_dir_all(DEFAULT_CACHE_DIRECTORY).expect("Failed to created cache directory.");
		Ok(("Created cache directory.").to_string())
	} else {
		Ok(("Cache directory already present.").to_string())
	}
}

pub async fn init_cache_directory() {
	let _err = create_cache_directory();
	let file_path = format!("{}/{}", DEFAULT_CACHE_DIRECTORY, "test.json");
	let is_created = Path::new(&file_path).is_file();

	if !is_created {
		let page: Page = Page::new(2).await;
		let collection: ImageCollection = ImageCollection::new(page).await;
		std::fs::write(file_path, serde_json::to_string_pretty(&collection).unwrap()).unwrap();
	} else {
		println!("Cache file already created.");
	}
}

pub fn parse_file_extension(file_name: &str) -> Option<&str> {
	Path::new(file_name).extension().and_then(OsStr::to_str)
}
