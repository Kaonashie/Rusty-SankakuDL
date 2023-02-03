use std::{fs::File, io::Write, path::Path, str};

use actix_rt::System;
use anyhow::Error;
use awc::Client;

use crate::images::Image;

pub const DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

fn request_file_bytes(image: &Image) -> Result<bytes::Bytes, Error> {
	System::new().block_on(async {
		let client = Client::default();
		let mut res = client
			.get(&image.file_url)
			.insert_header(("authority", "s.sankakucomplex.com"))
			.insert_header((
				"user-agent",
				"Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36",
			))
			.send()
			.await
			.unwrap();
		let body = res.body().limit(bytesize::ByteSize::gb(1).as_u64() as usize).await?;
		// println!("Response: {:?}", res); Debug
		Ok(body)
	})
}

pub fn save_file_to_disk(image: &Image) -> Result<(), Error> {
	let file_path = format!("./{}/{}", DEFAULT_DOWNLOAD_DIRECTORY, image.file_name);

	if !Path::new(&file_path).exists() {
		let mut file = File::create(file_path).expect("Failed to create file.");
		let data = request_file_bytes(image).expect("Failed to download image data.");
		file.write_all(&*data).expect("Failed to write image bytes into file.");
	} else {
		println!("File already exists on disk.");
	}
	Ok(())
}
