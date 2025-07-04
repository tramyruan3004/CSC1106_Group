use sqlx::{SqlitePool, sqlite::SqliteQueryResult};
use chrono::Utc;
use crate::models::*;

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS bugs (
            bug_id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            reported_by TEXT NOT NULL,
            reported_at DATETIME NOT NULL,
            bug_type TEXT,
            severity TEXT NOT NULL,
            progress_status TEXT NOT NULL DEFAULT 'Open',
            project_id TEXT,
            developer_id TEXT
        )"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_by TEXT NOT NULL,
            created_at DATETIME NOT NULL
        );"
    ).execute(pool).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            user_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            role TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            hashed_password TEXT NOT NULL
        );"
    ).execute(pool).await?;
        sqlx::query(
        "CREATE TABLE IF NOT EXISTS project_members (
            project_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            PRIMARY KEY (project_id, user_id),
            FOREIGN KEY (project_id) REFERENCES projects(project_id),
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );"
    ).execute(pool).await?;
    Ok(())
}

pub async fn create_bug_inner(pool: &SqlitePool, bug: NewBug) -> Result<Bug, sqlx::Error> {
    let reported_at = Utc::now().naive_utc().to_string();
    let progress_status = "new";
    let developer_id: Option<String> = None;

    // Insert without returning
    let result = sqlx::query(
        "INSERT INTO bugs (
            title, description, reported_by, reported_at, 
            bug_type, severity, progress_status, project_id, developer_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&bug.title)
    .bind(&bug.description)
    .bind(&bug.reported_by)
    .bind(&reported_at)
    .bind(&bug.bug_type)
    .bind(&bug.severity)
    .bind(progress_status)
    .bind(&bug.project_id)
    .bind(&developer_id)
    .execute(pool)
    .await?;

    let inserted_id = result.last_insert_rowid();

    // Now fetch the inserted row
    let inserted_bug = sqlx::query_as::<_, Bug>(
        "SELECT * FROM bugs WHERE bug_id = ?"
    )
    .bind(inserted_id)
    .fetch_one(pool)
    .await?;

    Ok(inserted_bug)
}

pub async fn get_all_bugs_inner(pool: &SqlitePool) -> Result<Vec<Bug>, sqlx::Error> {
    sqlx::query_as::<_, Bug>("SELECT * FROM bugs").fetch_all(pool).await
}

pub async fn get_bug_by_id_inner(pool: &SqlitePool, id: i64) -> Result<Bug, sqlx::Error> {
    sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE bug_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_bug_inner(pool: &SqlitePool, id: i64, update: UpdateBugAdmin) -> Result<Bug, sqlx::Error> {
    let current = get_bug_by_id_inner(pool, id).await?;

    let new_desc = update.description.unwrap_or(current.description);
    let new_severity = update.severity.unwrap_or(current.severity);
    let new_type = update.bug_type.unwrap_or(current.bug_type);
    let new_status = update.progress_status.unwrap_or(current.progress_status);
    let new_dev = update.developer_id.or(current.developer_id);

    // Do the update
    let result = sqlx::query(
        "UPDATE bugs 
         SET description = ?, severity = ?, bug_type = ?, progress_status = ?, developer_id = ?
         WHERE bug_id = ?"
    )
    .bind(new_desc)
    .bind(new_severity)
    .bind(new_type)
    .bind(new_status)
    .bind(new_dev)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    // Fetch the updated row
    get_bug_by_id_inner(pool, id).await
}


pub async fn delete_bug_inner(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM bugs WHERE bug_id = ?")
        .bind(id)
        .execute(pool)
        .await
}

