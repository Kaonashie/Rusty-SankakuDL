use utils::create_dl_directory;

use crate::structure::{images::ImageCollection, post::Page};

mod downloader;
mod server;
mod structure;
mod terminal;
mod utils;

async fn dl_init(num_of_images: u32) -> std::io::Result<()> {
	create_dl_directory();
	let page = Page::new(num_of_images).await;
	let collection = ImageCollection::new(page).await;
	// collection.print_debug(); Debug(Duh)
	collection.save_all_images().await;
	Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let selection: bool = terminal::server_or_dl();
	if selection {
		server::init().await.unwrap();
	} else {
		let num_of_images = terminal::term_init();
		dl_init(num_of_images)
			.await
			.expect("Failed to initialize download mode.");
	}
	Ok(())
}
