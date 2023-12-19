use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::schema::tasks;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct DataBaseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
}

#[derive(AsChangeset, Debug, Default)]
#[table_name="tasks"]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct ApiUpdateTask {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequestTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListWrapper {
    pub tasks: Vec<DataBaseTask>,
}
