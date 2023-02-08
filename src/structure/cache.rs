use serde::{Deserialize, Serialize};

use crate::structure::post::{Author, Page, Tag};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCache {
	cached_images: Vec<CachedImage>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CachedImage {
	pub post_id: i64,
	pub has_sample_url: bool,
	pub has_file_url: bool,
	pub author: Author,
	pub tags: Vec<CachedTag>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CachedTag {
	pub id: i64,
	pub name: String,
}

impl ImageCache {
	pub async fn new(page: &Page) -> Self {
		let mut cache: ImageCache = ImageCache {
			cached_images: Vec::new(),
		};

		for post in page.posts.clone() {
			let post_id = post.id.as_i64().unwrap();
			let author = post.author;
			let tags = Self::init_tags(post.tags);
			let has_sample_url = post.sample_url.is_some();
			let has_file_url = post.file_url.is_some();

			let image: CachedImage = CachedImage {
				post_id,
				has_sample_url,
				has_file_url,
				author,
				tags,
			};
			cache.cached_images.push(image)
		}
		cache
	}

	fn init_tags(tags: Vec<Tag>) -> Vec<CachedTag> {
		let mut tags_c: Vec<CachedTag> = Vec::new();
		for tag in tags {
			let id = tag.id;
			let name = tag.name_en;
			let tag_c: CachedTag = CachedTag { id, name };
			tags_c.push(tag_c)
		}
		tags_c
	}
}
