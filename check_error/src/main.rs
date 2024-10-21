use actix_web::{error::Error, web, App, HttpResponse, HttpServer};
use std::fs::File;
use std::io::Read;

async fn hello() -> Result<HttpResponse, Error> {
    let _ = File::open("fictionalfile.txt")?;
    Ok(HttpResponse::Ok().body("file read successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
