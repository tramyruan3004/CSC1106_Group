use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bug {
    pub bug_id: i64,
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub reported_at: String,
    pub bug_type: String,
    pub severity: String,
    pub progress_status: String,
    pub project_id: String,
    pub developer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewBug {
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub bug_type: String, 
    pub severity: String,
    pub project_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBugAdmin {
    pub description: Option<String>,
    pub severity: Option<String>,
    pub bug_type: Option<String>, 
    pub progress_status: Option<String>,
    pub developer_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct UpdateBugDeveloper {
    pub severity: Option<String>,
    pub progress_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Project {
    pub project_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub created_by: String,
}

#[derive(Debug, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub description: String,
    pub created_by: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectAdmin {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub role: String,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub role: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub hashed_password: Option<String>,
}

#[derive(Deserialize)]
pub struct AssignBugForm {
    pub bug_id: i64,
    pub developer_id: String,
}

#[derive(serde::Deserialize)]
pub struct AssignMemberToProjectRequest {
    pub username: String,
}
