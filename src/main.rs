mod tasks;
mod postgres;

use tasks::tasks_controller;
use actix_web::{web, App, HttpServer};
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/tasks").configure(tasks_controller::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
