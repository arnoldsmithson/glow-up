#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};
use rocket::fs::{relative, FileServer, NamedFile, TempFile};
use rocket::form::Form;
use uuid::Uuid;

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api")]
async fn serve_home_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(relative!("/www/static/index.html")).await
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[post("/clicked")]
fn clicked() -> String {
    format!("Clicked button")
}

#[get("/replaced")]
fn replace() -> String {
    format!("Replaced!")
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload(mut form: Form<Upload<'_>>) -> std::io::Result<()> {

    println!("form.file = {:?}", form.file);

    let f_2 = String::from(if form.file.content_type().unwrap().is_gif() {".gif"} else {
        if form.file.content_type().unwrap().is_jpeg() {".jpeg"} else {
            if form.file.content_type().unwrap().is_png() {".png"} else {
                if form.file.content_type().unwrap().is_svg() {".svg"} else {
                    if form.file.content_type().unwrap().is_mp4() {".mp4"} else {
                        if form.file.content_type().unwrap().is_mov() {".mov"} else {
                            if form.file.content_type().unwrap().is_webm() {".webm"} else {
                                if form.file.content_type().unwrap().is_webp() {".webp"} else {"invalid"}
                            }
                        }
                    }
                }
            }
        }
    });

    let file_id: String = Uuid::new_v4().hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_owned();
    let file_name = String::from(relative!("/www/static/")) + &file_id + &f_2;

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    form.file.persist_to(file_name).await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, delay, serve_home_page, clicked, replace, upload]).mount("/public", FileServer::from(relative!("/www/static")))
}