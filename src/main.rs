mod types;
mod tasks;

// use clap::Parser;
// use tasks::tasks_service::make_tasks_actions;
// use types::{Task, Args};
// Assume Task and TaskListWrapper are defined in types.rs
use types::{TaskListWrapper};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// Define a shared state
struct AppState {
    tasks: Mutex<TaskListWrapper>,
}

// Define your API endpoints
async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

// Add more API endpoints as needed...

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// fn main() {
//     let args: Args = Args::parse();
//     make_tasks_actions(args);
// }