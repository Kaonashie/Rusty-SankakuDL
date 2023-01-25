use curl::easy::{Easy, List, Easy2, Handler, WriteError};
use serde_json::{Result};
use std::fs::File;
use std::io::{Write};
use std::str;
use url::Url;
use crate::post::Page;
use crate::utils::{create_dl_directory, parse_file_extension};

mod post;
mod utils;

struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> anyhow::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub static DEFAULT_DOWNLOAD_DIRECTORY: &str = "./sankaku-downloads";

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