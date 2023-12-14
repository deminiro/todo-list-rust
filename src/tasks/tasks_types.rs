use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct DataBaseTask {
    pub id: i32, // Corresponds to Int4
    pub title: String, // Corresponds to Varchar
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequestTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUpdateTask {
    pub task: ApiRequestTask,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListWrapper {
    pub tasks: Vec<DataBaseTask>,
}
