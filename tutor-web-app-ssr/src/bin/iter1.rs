use std::env;
use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| 
        "127.0.0.1:8080".to_string());
    println!("Listening on: {}, open browser and visit have a try!", addr);

    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter1/**/*"
        ))
        .unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .service(fs::Files::new(
                "/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(addr)?
    .run()
    .await
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

