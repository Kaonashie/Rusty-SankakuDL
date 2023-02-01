use std::{fs::File, io::Write, str};

use curl::easy::{Easy, Easy2, Handler, List, WriteError};
use serde_json::Result;
use url::Url;

use crate::utils::parse_file_extension;

struct Collector(Vec<u8>);
impl Handler for Collector {
	fn write(&mut self, data: &[u8]) -> anyhow::Result<usize, WriteError> {
		self.0.extend_from_slice(data);
		Ok(data.len())
	}
}

pub static DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

pub fn download_file(file_url: &str, post_id: i64) -> Result<()> {
	let mut easy_dl = Easy::new();
	let mut list_dl = List::new();
	let parser = Url::parse(file_url).unwrap();
	let url_segments = parser.path_segments().unwrap();
	let file_name_url = url_segments.last().unwrap();
	let file_extension = parse_file_extension(file_name_url).unwrap();
	let file_name = format!("Post_{}.{}", post_id, file_extension);
	let file_path = format!("./{}/{}", DEFAULT_DOWNLOAD_DIRECTORY, file_name);
	let mut file = File::create(&file_path).unwrap();
	list_dl.append("authority: s.sankakucomplex.com").ok();
	list_dl.append("user-agent: Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36").ok();

	println!("Attempting to download file: {:?}", file_name_url);

	easy_dl.url(file_url).unwrap();
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
		file_name_url, &file_name
	);
	Ok(())
}

pub fn single_file_request_to_vec() -> std::result::Result<String, anyhow::Error> {
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
		.url("https://capi-v2.sankakucomplex.com/posts/keyset?limit=10")
		.expect("TODO: panic message");
	easy2.http_headers(list).unwrap();
	easy2.perform().unwrap();

	let content = easy2.get_ref();
	let content_string = String::from_utf8_lossy(&content.0);
	
    
	Ok(content_string.into_owned())
}
