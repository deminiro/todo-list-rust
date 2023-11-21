use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBaseTask {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
