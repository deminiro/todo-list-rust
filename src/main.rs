mod tasks;
mod postgres;

// use postgres::config::establish_connection;
// use clap::Parser;
use tasks::tasks_controller;
use actix_web::{web, App, HttpServer};
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // establish_connection();
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/tasks").configure(tasks_controller::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
//     let args: Args = Args::parse();
//     make_tasks_actions(args);
// }