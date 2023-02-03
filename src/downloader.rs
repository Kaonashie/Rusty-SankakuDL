use std::{fs::File, io::Write, str, path::Path};

use actix_rt::System;
use anyhow::Error;
use awc::Client;

use curl::easy::{Easy2, Handler, List, WriteError};


use crate::images::Image;

struct Collector(Vec<u8>);
impl Handler for Collector {
	fn write(&mut self, data: &[u8]) -> anyhow::Result<usize, WriteError> {
		self.0.extend_from_slice(data);
		Ok(data.len())
	}
}

pub const DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

fn download_file(image: &Image) -> Result<bytes::Bytes, Error> {
	System::new().block_on(async {
		let client = Client::default();
		let mut res = client
			.get(&image.file_url)
			.insert_header(("authority", "s.sankakucomplex.com"))
			.insert_header(("user-agent", "Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36"))
			.send()
			.await
			.unwrap();
		let body = res.body().await?;
		// println!("Response: {:?}", res); Debug
		Ok(body)
	})
}

pub fn save_file_to_disk(image: &Image) -> Result<(), Error> {
		let file_path = format!("./{}/{}", DEFAULT_DOWNLOAD_DIRECTORY, image.file_name);

		if !Path::new(&file_path).exists() {
			let mut file = File::create(file_path).expect("Failed to create file.");
			let data = download_file(image).expect("Failed to download file.");
			file.write_all(&*data).expect("Failed to write image bytes into file.");
		} else {
			println!("File already exists on disk.");
		}
		Ok(())
	}


pub fn single_file_request_to_vec(num_of_image: u32) -> Result<String, Error> {
	let mut list = List::new();
	let mut easy2 = Easy2::new(Collector(Vec::new()));

	list.append("authority: capi-v2.sankakucomplex.com").ok();
	list.append("access-control-request-headers: client-type,platform")
		.ok();
	list.append("access-control-request-method: GET").ok();
	list.append("origin: https://beta.sankakucomplex.com").ok();
	list.append("referer: https://beta.sankakucomplex.com").ok();
	list.append("user-agent: Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36").ok();

	easy2.get(true).unwrap();
	easy2
		.url(
			format!(
				"https://capi-v2.sankakucomplex.com/posts/keyset?limit={}",
				num_of_image
			)
			.as_str(),
		)
		.expect("Failed to get response from Sankaku.");
	easy2.http_headers(list).unwrap();
	easy2.perform().unwrap();

	let content = easy2.get_ref();
	let content_string = String::from_utf8_lossy(&content.0);

	Ok(content_string.into_owned())
}
