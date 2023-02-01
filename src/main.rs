use crate::{
	downloader::{download_file, single_file_request_to_vec},
	post::Page,
	utils::{create_dl_directory, get_chosen_url},
};

mod downloader;
mod images;
mod post;
mod utils;

fn main() {
	create_dl_directory();
	let res = single_file_request_to_vec().expect("TODO: panic message");
	let page: Page = serde_json::from_str(&res).unwrap();
	let urls = get_chosen_url(page).unwrap();
	for image in urls {
		download_file(image.1.as_str(), image.0).expect("TODO: panic message");
	}
}
