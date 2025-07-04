use std::sync::{Arc, Mutex};
use sqlx::SqlitePool;
use crate::models::Project;

pub struct AppState {
    pub db: SqlitePool,
    pub projects: Arc<Mutex<Vec<Project>>>,
}
