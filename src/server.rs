use std::{fs::File, io::Read};

use crate::{downloader::DEFAULT_CACHE_DIRECTORY, structure::post::Post, utils::init_cache_directory};
use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use awc::Client;

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
	Ok(HttpResponse::Ok().content_type("image").body(data))
}

fn get_cache_files() -> Vec<String> {
	let mut files_in_dir: Vec<String> = Vec::new();
	for file in std::fs::read_dir(DEFAULT_CACHE_DIRECTORY).unwrap() {
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
