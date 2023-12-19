use diesel::{RunQueryDsl, insert_into, QueryDsl};
use tasks_types::{DataBaseTask, ApiRequestTask};
use crate::{schema::tasks::dsl::*, postgres::config::establish_connection};
use crate::schema::tasks;

use super::tasks_types::{self, UpdateTask, NewTask, ApiUpdateTask};

pub fn get_tasks_database() -> Result<Vec<DataBaseTask>, diesel::result::Error> {
  let mut conn = establish_connection();
  tasks.load::<DataBaseTask>(&mut conn)
}

pub fn create_task_database(new_task: NewTask) -> Result<usize, diesel::result::Error> {
  let mut conn = establish_connection();

  insert_into(tasks::table)
      .values(new_task)
      .execute(&mut conn)
}

pub fn update_task_database(task_id: i32, task_data: &UpdateTask) -> Result<usize, diesel::result::Error> {
  let mut conn = establish_connection();
  use crate::schema::tasks::dsl::*;

  diesel::update(tasks.find(task_id))
      .set(task_data)
      .execute(&mut conn)
}

pub fn remove_task_database(task_id: i32) -> Result<usize, diesel::result::Error> {
  let mut conn = establish_connection();

  diesel::delete(tasks.find(task_id))
    .execute(&mut conn)
}

pub fn get_tasks() -> Result<Vec<DataBaseTask>, diesel::result::Error> {
  let data = get_tasks_database();

  data
}

pub fn add_task(body: ApiRequestTask) -> Result<usize, diesel::result::Error> {
  let new_task = NewTask {
    title: body.title, // If it's already Option<String>, use it directly
    description: Some(body.description.unwrap_or_else(|| "".to_string())),
    completed: Some(body.completed.unwrap_or(false)),
    priority: Some(body.priority.unwrap_or_else(|| "".to_string())),
  };

  create_task_database(new_task)
}

pub fn update_task(body: ApiUpdateTask) ->Result<usize, diesel::result::Error> {
  let mut task_updates = UpdateTask::default();

  task_updates.title = body.title;
  task_updates.description = body.description;
  task_updates.completed = body.completed;
  task_updates.priority = body.priority;

  // Always set the updated_at to current time
  task_updates.updated_at = Some(chrono::Local::now().naive_local());

  Ok(update_task_database(body.id, &task_updates)?)
}

pub fn remove_task(task_id: i32) -> Result<usize, diesel::result::Error> {
  remove_task_database(task_id)
}