use serde::{Deserialize, Serialize};
use serde_json::{Number};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    meta: Option<Meta>,
    #[serde(rename = "data")]
    pub(crate) post: Vec<Post>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    next: String,
    prev: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub(crate) id: Number,
    rating: String,
    status: String,
    author: Author,
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
    tags: Vec<Tag>,
    video_duration: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    id: i64,
    name_en: String,
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
#[derive(Serialize, Deserialize, Debug)]
struct CreatedAt {
    json_class: String,
    s: i64,
    n: i64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Author {
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