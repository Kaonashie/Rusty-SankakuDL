use actix_rt::System;
use anyhow::{anyhow, Error};
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{downloader::save_file_to_disk, post::Page, utils::parse_file_extension};

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
	pub fn new(num_of_image: u32) -> Self {
		let mut collection: ImageCollection = ImageCollection { images: Vec::new() };
		let res = Self::request_page(num_of_image).expect("Could not get page data.");
		let page: Page = serde_json::from_value(res).unwrap();
		for post in page.post {
			let post_id = post.id.as_u64().unwrap();
			if let Ok(..) = Self::verify_file_url(post.file_url.clone()) {
				let file_url = post.file_url.unwrap();
				let (file_name_url, file_name) =
					Self::parse_file_url(&file_url, post_id).expect("Failed to parse file url");
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

	fn parse_file_url(file_url: &str, post_id: u64) -> Result<(String, String), Error> {
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

	pub fn save_all_images(&self) {
		for image in &self.images {
			save_file_to_disk(image).expect("TODO: panic message");
		}
	}

	pub fn verify_file_url(post_file_url: Option<String>) -> Result<(), Error> {
		if post_file_url.is_some() {
			Ok(())
		} else {
			Err(anyhow!("Image has no file url."))
		}
	}

	fn request_page(num_of_image: u32) -> Result<Value, Error> {
		System::new().block_on(async {
			let client = Client::default();
			let mut res = client
				.get(format!("https://capi-v2.sankakucomplex.com/posts/keyset?limit={}", num_of_image).as_str())
				.insert_header(("authority", "capi-v2.sankakucomplex.com"))
				.insert_header(("access-control-request-headers", "client-type, platform"))
				.insert_header(("access-control-request-method", "GET"))
				.insert_header(("origin", "https://beta.sankakucomplex.com"))
				.insert_header(("referer", "https://beta.sankakucomplex.com"))
				.insert_header((
					"user-agent",
					"Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36",
				))
				.send()
				.await
				.unwrap();
			let content = res.json::<Value>().await?;
			Ok(content)
		})
	}
}
