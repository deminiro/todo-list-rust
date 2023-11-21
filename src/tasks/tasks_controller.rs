use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use super::tasks_types::{ApiRequestTask, ApiUpdateTask};
use super::tasks_service;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(get_tasks))
      .route("/", web::post().to(add_task))
      .route("/", web::put().to(update_task))
      .route("/{task_id}", web::delete().to(remove_task));
      // Add other task routes here
}

async fn get_tasks() -> impl Responder {
  match tasks_service::get_tasks() {
    Ok(tasks) => HttpResponse::Ok().json(tasks),
    Err(e) => {
      eprintln!("Error getting tasks: {}", e);
      HttpResponse::InternalServerError().body("Error loading tasks")
    }
  }
}

async fn add_task(payload: web::Json<ApiRequestTask>) -> impl Responder {
  match tasks_service::add_task(payload.into_inner()) {
    Ok(task) => HttpResponse::Ok().json(task),
    Err(e) => {
      eprintln!("Error getting task: {}", e);
      HttpResponse::InternalServerError().body("Error loading task")
    }
  }
}

async fn remove_task(path: web::Path<Uuid>) -> impl Responder {
  match tasks_service::remove_task(path.into_inner()) {
    Ok(status) => HttpResponse::Ok().json(status),
    Err(e) => {
      eprintln!("Error getting task: {}", e);
      HttpResponse::InternalServerError().body("Error loading task")
    }
  }
}

async fn update_task(payload: web::Json<ApiUpdateTask>) -> impl Responder {
  print!("here");
  print!("here 2 {:?}", &payload);
  match tasks_service::update_task(payload.into_inner()) {
    Ok(task) => HttpResponse::Ok().json(task),
    Err(e) => {
      eprintln!("Error getting task: {}", e);
      HttpResponse::InternalServerError().body("Error loading task")
    }
  }
}