use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{downloader::download_file, post::Page, utils::parse_file_extension};
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCollection {
	images: Vec<Image>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
	pub file_name_url: String,
	pub file_name: String,
	pub post_id: u64,
	pub file_url: String,
}

impl ImageCollection {
	pub fn new(page: Page) -> Self  {
		let mut collection: ImageCollection = ImageCollection { images: Vec::new() };
		for post in page.post {
			let post_id = post.id.as_u64().unwrap();
			if let Ok(..) = Self::verify_file_url(post.file_url.clone()) {
				let file_url = post.file_url.unwrap();
				let (file_name_url, file_name) = Self::parse_file_url(&file_url, post_id).expect("Failed to parse file url");
				let image: Image = Image {
					post_id,
					file_url,
					file_name,
					file_name_url,
				};
				collection.images.push(image);
			} else {
				println!("Image doesn't have a file url. Trying next image...");

			}
		}
		collection
	}

	fn parse_file_url(file_url: &str, post_id: u64) -> Result<(String, String), anyhow::Error> {
		let parser = Url::parse(file_url).ok().expect("Failed to parse the url.");
		let url_segments = parser.path_segments().unwrap();
		let file_name_url = url_segments.last();
		if file_name_url.is_some() {
			let cfile_name_url = file_name_url.clone().unwrap();
			let file_extension = parse_file_extension(cfile_name_url).unwrap();
			let file_name = format!("Post_{}.{}", post_id, file_extension);
			Ok((cfile_name_url.to_string(), file_name))
		} else {
			Err(anyhow!("Failed to parse file url."))
		}
	}

	pub fn print_debug(&self) {
		for image in &self.images {
			println!("Image's full url: {:?}", image.file_url);
		}
	}

	pub fn download_all_image(&self) {
		for image in &self.images {
			download_file(image).expect("Failed to download file.");
		}
	}

	pub fn verify_file_url(post_file_url: Option<String>) -> Result<(), anyhow::Error> {
		if post_file_url.is_some() {
			Ok(())
		} else {
			Err(anyhow!("Image has no file url."))
		}
	}
}
