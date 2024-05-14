#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer, NamedFile, TempFile};
use rocket::form::Form;
use uuid::Uuid;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self};

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[derive(Database)]
#[database("test")]
struct Logs(sqlx::SqlitePool);

#[get("/")]
async fn serve_home_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(relative!("/www/static/html/index.html")).await
}

#[get("/contact")]
async fn serve_contact_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(relative!("/www/static/html/contact.html")).await
}

#[get("/about")]
async fn serve_about_page() -> Result<NamedFile, std::io::Error> {
    NamedFile::open(relative!("/www/static/html/about.html")).await
}


#[post("/upload", format = "multipart/form-data", data = "<form>")]
async fn upload(mut db: Connection<Logs>, mut form: Form<Upload<'_>>) -> std::io::Result<String> {

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
    let file_name = String::from(relative!("/www/static/uploads/")) + &file_id + f_2;

    println!("destination = {}", file_name);
    println!("length = {} bytes", form.file.len());

    form.file.persist_to(file_name).await?;

    let image_url = format!("/public/uploads/{}", file_id.clone() + f_2);

    sqlx::query("INSERT INTO files (file_name, user_id) VALUES (?, 1)")
    .bind(file_id+f_2)
    .execute(&mut *db.as_mut()).await.expect("Uh oh!! Wrong stuff!!");

    Ok(format!(r#"<img id="image" src="{0}" hx-get="{0}">"#, image_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![serve_home_page, upload, serve_about_page, serve_contact_page])
        .mount("/public", FileServer::from(relative!("/www/static")))
}