use images::ImageCollection;

use crate::{terminal::term_init, utils::create_dl_directory};

mod downloader;
mod images;
mod post;
mod terminal;
mod utils;

fn dl_init(num_of_images: u32) {
	create_dl_directory();
	let collection = ImageCollection::new(num_of_images);
	// collection.print_debug(); Debug(Duh)
	collection.save_all_images();
}

fn main() {
	let num_of_images = term_init();
	dl_init(num_of_images);
}
