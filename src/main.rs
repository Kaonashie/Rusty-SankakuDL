use curl::easy::{Easy, List, Easy2, Handler, WriteError};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Result};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::str;
use url::Url;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    meta: Option<Meta>,
    #[serde(rename = "data")]
    post: Vec<Post>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    next: String,
    prev: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: Number,
    rating: String,
    status: String,
    author: Author,
    sample_url: Option<String>,
    sample_width: Option<i64>,
    sample_height: Option<i64>,
    preview_url: Option<String>,
    preview_width: Option<i64>,
    preview_height: Option<i64>,
    file_url: Option<String>,
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
struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> anyhow::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

static DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

fn cmd_pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "Press any key to continue...").ok();
    stdout.flush().ok();
    let _ = stdin.read(&mut [0u8]).ok();
}

fn create_dl_directory() {
    let is_created: bool = Path::new(DEFAULT_DOWNLOAD_DIRECTORY).is_dir();

    if !is_created {
        println!("No downloads folder found.\nCreating the default one...");
        std::fs::create_dir(DEFAULT_DOWNLOAD_DIRECTORY).expect("Failed to created local downloads directory.");
        cmd_pause();
    }
}

fn parse_file_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name).extension().and_then(OsStr::to_str)
}

fn download_file(file_url: &str, post_id: i64) -> Result<()> {
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

fn parse_response(data_input: &str) -> Result<(String, i64)> {
    let mut p: Page = serde_json::from_str(&data_input)?;
    let post_1 = &mut p.post[0];
    let post_id = post_1.id.as_i64().unwrap();

    let chosen_url: &str;

    if post_1.file_url  != None{
        println!("File url present.\nWill attempt to download file in 'original' quality.");
        let file_url = post_1.file_url.as_mut().unwrap();
        // download_file(file_url, post_id).ok();
        chosen_url = file_url.as_str();
    } else if post_1.preview_url != None {
        println!("File url not present.\nWill attempt to download file in 'preview' quality.");
        let preview_url = post_1.preview_url.as_mut().unwrap();
        // download_file(preview_url, post_id).ok();
        chosen_url = preview_url.as_str();
    } else if post_1.sample_url != None {
        println!("File and preview urls missing.\nWill attempt to download file in 'sample' quality.");
        let sample_url = post_1.sample_url.as_mut().unwrap();
        // download_file(sample_url, post_id).ok();
        chosen_url = sample_url.as_str();
    } else {
        println!("No urls found. Try again later.");
        chosen_url = "NULL";
    }
    Ok((chosen_url.to_owned(), post_id.to_owned()))
}

fn single_file_request_to_vec() -> std::result::Result<String, anyhow::Error> {
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
    easy2.url("https://capi-v2.sankakucomplex.com/posts/keyset?limit=1").expect("TODO: panic message");
    easy2.http_headers(list).unwrap();
    easy2.perform().unwrap();

    let content = easy2.get_ref();
    let content_string = String::from_utf8_lossy(&content.0);

    Ok(content_string.into_owned())
}

fn main() {
    create_dl_directory();
    let res = single_file_request_to_vec().expect("TODO: panic message");
    let parsed_res = parse_response(res.as_str()).expect("TODO: panic message");
    let chosen_url = parsed_res.0.as_str();
    let post_id = parsed_res.1;
    download_file(chosen_url, post_id).expect("TODO: panic message");
}
