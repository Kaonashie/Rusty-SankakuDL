use std::{
	fs::File,
	io::{Read, Write},
};

use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use awc::Client;

use crate::{
	downloader::DEFAULT_CACHE_DIRECTORY,
	structure::post::Post,
	utils::{init_cache_directory, parse_file_url},
};

#[get("/image")]
async fn serve_image(_req: HttpRequest) -> Result<impl Responder> {
	let image_content = web::block(|| std::fs::read("./sankaku-downloads/Post_32685749.png").unwrap()).await?;

	Ok(HttpResponse::Ok().content_type("image/png").body(image_content))
}

#[get("/api/image-info")]
async fn get_image_info(_req: HttpRequest) -> Result<impl Responder> {
	let files = get_cache_files();
	let mut file = File::open(&files[0]).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	Ok(HttpResponse::Ok().body(contents))
}

#[get("/api/image-data/{image_url:.*}")]
async fn get_image_data(path: web::Path<u32>) -> Result<impl Responder> {
	let post_id = path.into_inner();
	// Post::test_url(post_id);

	let post = Post::new(post_id).await;
	let client = Client::default();
	let image_url = post.file_url.unwrap();
	let mut res = client
		.get(&image_url)
		.insert_header(("authority", "s.sankakucomplex.com"))
		.insert_header((
			"user-agent",
			"Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36",
		))
		.send()
		.await
		.unwrap();
	let data = res.body().limit(bytesize::ByteSize::gb(1).as_u64() as usize).await?;
	Ok(HttpResponse::Ok().content_type("image").body(data))
}

#[get("/api/image-sample/{post_id:.*}")]
async fn get_sample_data(path: web::Path<u32>) -> Result<impl Responder> {
	let post_id = path.into_inner();

	let post = Post::new(post_id).await;
	let client = Client::default();
	let image_url = post.sample_url.unwrap();
	let mut res = client
		.get(&image_url)
		.insert_header(("authority", "s.sankakucomplex.com"))
		.insert_header((
			"user-agent",
			"Mozilla/5.0 (Linux) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36",
		))
		.send()
		.await
		.unwrap();
	let data = res.body().limit(bytesize::ByteSize::gb(1).as_u64() as usize).await?;

	let preview_url = post.preview_url.unwrap();
	let file_ext = parse_file_url(preview_url.as_str(), post_id.into()).unwrap();
	let file_path = format!("{}/samples/{}.", DEFAULT_CACHE_DIRECTORY, file_ext);

	if !std::path::Path::new(&file_path).exists() {
		let mut file = std::fs::File::create(file_path).unwrap();
		file.write_all(&data).expect("Failed to write image data to file.");
		// println!("File fetched from sankaku server."); Debug print to see if cache works
		Ok(HttpResponse::Ok().content_type("image").body(data))
	} else {
		let mut bytes = Vec::new();
		let mut file = File::open(file_path).unwrap();
		file.read_to_end(&mut bytes).expect("Failed to read file.");
		// println!("File fetched from local storage."); Debug print so see if cache works
		Ok(HttpResponse::Ok().content_type("image").body(bytes))
	}
}

fn get_cache_files() -> Vec<String> {
	let path = format!("{}/json", DEFAULT_CACHE_DIRECTORY);
	let mut files_in_dir: Vec<String> = Vec::new();
	for file in std::fs::read_dir(&path).unwrap() {
		let file = file.unwrap();
		let path = file.path();
		files_in_dir.push(path.to_str().unwrap().to_string());
	}
	return files_in_dir;
}

// #[actix_web::main]
pub async fn init() -> std::io::Result<()> {
	init_cache_directory().await;
	HttpServer::new(|| {
		let cors = Cors::permissive();

		App::new()
			.wrap(cors)
			.service(serve_image)
			.service(get_image_info)
			.service(get_image_data)
			.service(get_sample_data)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
