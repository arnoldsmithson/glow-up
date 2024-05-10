#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer, NamedFile, TempFile};
use rocket::form::Form;
use uuid::Uuid;

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[get("/")]
async fn serve_home_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(relative!("/www/static/html/index.html")).await
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload(mut form: Form<Upload<'_>>) -> std::io::Result<String> {

    println!("form.file = {:?}", form.file);

    let f_2 = match form.file.content_type().unwrap() {
        x if x.is_gif() => ".gif",
        x if x.is_jpeg() => ".jpeg",
        x if x.is_png() => ".png",
        x if x.is_svg() => ".svg",
        x if x.is_mp4() => ".mp4",
        x if x.is_mov() => ".mov",
        x if x.is_webm() => ".webm",
        x if x.is_webp() => ".webp",
        _ => "invalid",
    };

    let file_id: String = Uuid::new_v4().hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_owned();
    let file_name = String::from(relative!("/www/static/")) + &file_id + f_2;

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    form.file.persist_to(file_name).await?;

    let image_url = format!("/public/{}", file_id + f_2);

    Ok(format!(r#"<img id="image" src="{0}" hx-get="{0}">"#, image_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![serve_home_page, upload]).mount("/public", FileServer::from(relative!("/www/static")))
}