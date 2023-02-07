use anyhow::Error;
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
	meta: Option<Meta>,
	#[serde(rename = "data")]
	pub(crate) posts: Vec<Post>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Meta {
	next: String,
	prev: Option<serde_json::Value>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Post {
	pub(crate) id: Number,
	rating: String,
	status: String,
	pub author: Author,
	pub(crate) sample_url: Option<String>,
	sample_width: Option<i64>,
	sample_height: Option<i64>,
	pub(crate) preview_url: Option<String>,
	preview_width: Option<i64>,
	preview_height: Option<i64>,
	pub(crate) file_url: Option<String>,
	width: i64,
	height: i64,
	file_size: i64,
	file_type: String,
	created_at: CreatedAt,
	has_children: bool,
	has_comments: bool,
	has_notes: bool,
	is_favorited: bool,
	user_vote: Option<serde_json::Value>,
	md5: String,
	parent_id: Option<serde_json::Value>,
	change: i64,
	fav_count: i64,
	recommended_posts: i64,
	recommended_score: i64,
	vote_count: i64,
	total_score: i64,
	comment_count: Option<serde_json::Value>,
	source: Option<serde_json::Value>,
	in_visible_pool: bool,
	is_premium: bool,
	is_rating_locked: bool,
	is_note_locked: bool,
	is_status_locked: bool,
	redirect_to_signup: bool,
	sequence: Option<serde_json::Value>,
	pub(crate) tags: Vec<Tag>,
	video_duration: Option<serde_json::Value>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
	pub id: i64,
	pub name_en: String,
	name_ja: Option<String>,
	#[serde(rename = "type")]
	tag_type: i64,
	count: i64,
	post_count: i64,
	pool_count: i64,
	locale: Option<String>,
	rating: Option<String>,
	version: Option<i64>,
	#[serde(rename = "tagName")]
	tag_name: String,
	total_post_count: i64,
	total_pool_count: i64,
	name: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
struct CreatedAt {
	json_class: String,
	s: i64,
	n: i64,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Author {
	id: i64,
	name: String,
	avatar: String,
	avatar_rating: String,
}
impl AsRef<Post> for Post {
	fn as_ref(&self) -> &Post {
		self
	}
}

impl Page {
	pub(crate) async fn new(num_of_images: u32) -> Self {
		let res = Self::request_page(num_of_images).await.unwrap();
		let page: Page = serde_json::from_value(res).unwrap();

		page
	}
	pub async fn request_page(num_of_image: u32) -> Result<Value, Error> {
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
	}
}

impl Post {
	pub async fn new(post_id: u32) -> Self {
		let res = Self::request_post(post_id).await.expect("Failed to request post.");
		let json: Vec<Post> = serde_json::from_value(res).unwrap();
		let post: Post = json[0].clone();
		post
	}

	async fn request_post(post_id: u32) -> Result<Value, Error> {
		let client = Client::default();
		let mut res = client
			.get(
				format!(
					"https://capi-v2.sankakucomplex.com/posts?lang=en&page=1&limit=1&tags=id_range:{}",
					post_id
				)
				.as_str(),
			)
			.insert_header(("authority", "capi-v2.sankakucomplex.com"))
			.insert_header(("origin", "https://beta.sankakucomplex.com"))
			.insert_header(("referer", "https://beta.sankakucomplex.com"))
			.insert_header((
				"user-agent",
				"Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36",
			))
			.send()
			.await
			.unwrap();
		// println!("{:?}", res.headers());

		let contents = res.json::<Value>().await?;

		Ok(contents)
	}
}
