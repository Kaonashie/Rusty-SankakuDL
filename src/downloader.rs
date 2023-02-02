use std::{fs::File, io::Write, str, path::Path};

use curl::easy::{Easy, Easy2, Handler, List, WriteError};
use serde_json::Result;

use crate::images::Image;

struct Collector(Vec<u8>);
impl Handler for Collector {
	fn write(&mut self, data: &[u8]) -> anyhow::Result<usize, WriteError> {
		self.0.extend_from_slice(data);
		Ok(data.len())
	}
}

pub const DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

pub fn download_file(image: &Image) -> Result<()> {
	let mut easy_dl = Easy::new();
	let mut list_dl = List::new();
	list_dl.append("authority: s.sankakucomplex.com").ok();
	list_dl.append("user-agent: Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36").ok();
	let file_path = format!("./{}/{}", DEFAULT_DOWNLOAD_DIRECTORY, image.file_name);

	if Path::new(&file_path).exists() {
		let mut file = File::create(file_path).unwrap();
		easy_dl.url(image.file_url.as_str()).unwrap();
		easy_dl
			.write_function(move |data| {
				file.write_all(data).unwrap();
				Ok(data.len())
			})
			.unwrap();
		easy_dl.http_headers(list_dl).unwrap();
		easy_dl.perform().unwrap();
		println!(
			"Successfully downloaded file: {} \nRenamed to {}.",
			image.file_name_url, &image.file_name
		);
	} else {
		println!("File already exists.");
	}
	Ok(())
}

pub fn single_file_request_to_vec(num_of_image: u32) -> std::result::Result<String, anyhow::Error> {
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
