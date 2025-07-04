use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use chrono::Utc;
use std::sync::Mutex;

use crate::models::{Project, NewProject, UpdateProjectAdmin};

#[get("/projects")]
async fn list_projects(pool: web::Data<Mutex<SqlitePool>>) -> impl Responder {
    let pool = pool.lock().unwrap();
    match sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(&*pool)
        .await
    {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => HttpResponse::InternalServerError().body(format!("DB error: {}", err)),
    }
}

#[post("/projects")]
async fn create_project(
    pool: web::Data<Mutex<SqlitePool>>,
    json: web::Json<NewProject>,
) -> impl Responder {
    let now = Utc::now().to_rfc3339();
    let pool = pool.lock().unwrap();

    match sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, description, created_by, created_at, users_list)
         VALUES (?, ?, ?, ?, '')
         RETURNING *",
    )
    .bind(&json.name)
    .bind(&json.description)
    .bind(&json.created_by)
    .bind(now)
    .fetch_one(&*pool)
    .await
    {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(err) => HttpResponse::InternalServerError().body(format!("Create failed: {}", err)),
    }
}

#[patch("/projects/{id}")]
async fn update_project(
    pool: web::Data<Mutex<SqlitePool>>,
    path: web::Path<i64>,
    json: web::Json<UpdateProjectAdmin>,
) -> impl Responder {
    let pool = pool.lock().unwrap();
    let project_id = path.into_inner();

    let existing = match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_one(&*pool)
        .await
    {
        Ok(project) => project,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };

    let updated_name = json.name.clone().unwrap_or(existing.name);
    let updated_desc = json.description.clone().unwrap_or(existing.description);
    let updated_users = json.users_list.clone().unwrap_or(existing.users_list);

    match sqlx::query_as::<_, Project>(
        "UPDATE projects
         SET name = ?, description = ?, users_list = ?
         WHERE project_id = ?
         RETURNING *",
    )
    .bind(updated_name)
    .bind(updated_desc)
    .bind(updated_users)
    .bind(project_id)
    .fetch_one(&*pool)
    .await
    {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::InternalServerError().body(format!("Update failed: {}", err)),
    }
}

#[delete("/projects/{id}")]
pub async fn delete_project(
    pool: web::Data<Mutex<SqlitePool>>,
    path: web::Path<i64>,
) -> impl Responder {
    let pool = pool.lock().unwrap();
    let project_id = path.into_inner();

    match sqlx::query("DELETE FROM projects WHERE project_id = ?")
        .bind(project_id)
        .execute(&*pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body("Project not found")
            } else {
                HttpResponse::Ok().body("Project deleted")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Delete failed: {}", err)),
    }
}
