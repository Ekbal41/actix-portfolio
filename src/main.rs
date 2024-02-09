use actix_web::{web, App, HttpServer};
mod handlers;
use handlers::main::add_project;
use handlers::main::delete_project;
use handlers::main::index;
use handlers::main::version;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/add", web::post().to(add_project))
            .route("/delete/{id}", web::delete().to(delete_project))
            .route("/version", web::get().to(version))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
