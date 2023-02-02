use images::ImageCollection;

use crate::{
	downloader::single_file_request_to_vec, post::Page, terminal::term_init,
	utils::create_dl_directory,
};

mod downloader;
mod images;
mod post;
mod terminal;
mod utils;

fn dl_init(num_of_images: u32) {
	create_dl_directory();
	let res = single_file_request_to_vec(num_of_images).expect("TODO: panic message");
	let page: Page = serde_json::from_str(&res).unwrap();
	let collection = ImageCollection::new(page);
	collection.print_debug();
	collection.download_all_image();
}

fn main() {
	let num_of_images = term_init();
	dl_init(num_of_images);
}
