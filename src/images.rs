use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub image_url: String,
    pub post_id: i64,
}