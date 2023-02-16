use std::{
	ffi::OsStr,
	io::{Read, Write},
	path::Path,
};

use anyhow::{anyhow, Error};
use url::Url;

use crate::{
	downloader::{DEFAULT_CACHE_DIRECTORY, DEFAULT_DOWNLOAD_DIRECTORY},
	structure::{cache::ImageCache, post::Page},
};

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
	let path = format!("{}/json", DEFAULT_CACHE_DIRECTORY);
	let is_created: bool = Path::new(&path).is_dir();
	std::fs::create_dir_all(format!("{}/samples", DEFAULT_CACHE_DIRECTORY))
		.expect("Failed to create samples directory.");

	if !is_created {
		println!("No cache folder found.\nCreating default one.");
		std::fs::create_dir_all(path).expect("Failed to created cache directory.");
		Ok(("Created cache directory.").to_string())
	} else {
		Ok(("Cache directory already present.").to_string())
	}
}

pub async fn init_cache_directory() {
	let _err = create_cache_directory();
	let file_path = format!("{}/json/{}", DEFAULT_CACHE_DIRECTORY, "test.json");
	let is_created = Path::new(&file_path).is_file();

	if !is_created {
		let page: Page = Page::new(10).await;
		// let collection: ImageCollection = ImageCollection::new(page).await;
		let cache: ImageCache = ImageCache::new(&page).await;
		std::fs::write(file_path, serde_json::to_string_pretty(&cache).unwrap()).unwrap();
	} else {
		println!("Cache file already created.");
	}
}

pub fn parse_file_url(file_url: &str, post_id: u64) -> Result<String, Error> {
	let parser = Url::parse(file_url).ok().expect("Failed to parse the url.");
	let url_segments = parser.path_segments().unwrap();
	let file_name_url = url_segments.last();
	if file_name_url.is_some() {
		let cfile_name_url = file_name_url.clone().unwrap();
		let file_extension = parse_file_extension(cfile_name_url).unwrap();
		let file_name = format!("Post_{}.{}", post_id, file_extension);
		Ok(file_name)
	} else {
		Err(anyhow!("Failed to parse file url."))
	}
}

pub fn parse_file_extension(file_name: &str) -> Option<&str> {
	Path::new(file_name).extension().and_then(OsStr::to_str)
}
