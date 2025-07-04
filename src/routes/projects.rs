use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use chrono::Utc;

use crate::models::{Project, NewProject, UpdateProjectAdmin, AssignMemberToProjectRequest};

#[get("/projects")]
async fn list_projects(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => HttpResponse::InternalServerError().body(format!("DB error: {}", err)),
    }
}

#[post("/projects")]
async fn create_project(
    pool: web::Data<SqlitePool>,
    json: web::Json<NewProject>,
) -> impl Responder {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, description, created_by, created_at)
         VALUES (?, ?, ?, ?)
         RETURNING *",
    )
    .bind(&json.name)
    .bind(&json.description)
    .bind(&json.created_by)
    .bind(now)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(err) => HttpResponse::InternalServerError().body(format!("Create failed: {}", err)),
    }
}

#[patch("/projects/{id}")]
async fn update_project(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    json: web::Json<UpdateProjectAdmin>,
) -> impl Responder {
    let project_id = path.into_inner();

    let existing = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_one(pool.get_ref())
        .await;

    let existing = match existing {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };

    let updated_name = json.name.clone().unwrap_or(existing.name);
    let updated_desc = json.description.clone().unwrap_or(existing.description);

    let result = sqlx::query_as::<_, Project>(
        "UPDATE projects
         SET name = ?, description = ?
         WHERE project_id = ?
         RETURNING *",
    )
    .bind(updated_name)
    .bind(updated_desc)
    .bind(project_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::InternalServerError().body(format!("Update failed: {}", err)),
    }
}

#[delete("/projects/{id}")]
pub async fn delete_project(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let project_id = path.into_inner();

    let res = sqlx::query("DELETE FROM projects WHERE project_id = ?")
        .bind(project_id)
        .execute(pool.get_ref())
        .await;

    match res {
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

#[post("/projects/{id}/assign")]
pub async fn assign_member_to_project(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    json: web::Json<AssignMemberToProjectRequest>,
) -> impl Responder {
    let project_id = path.into_inner();
    let AssignMemberToProjectRequest { username } = json.into_inner();

    // Check project exists
    let project_check = sqlx::query("SELECT 1 FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(None) = project_check {
        return HttpResponse::NotFound().body("Project not found");
    }

    // Get user_id by username
    let user_row = sqlx::query!("SELECT user_id FROM users WHERE username = ?", username)
        .fetch_optional(pool.get_ref())
        .await;

    let user_id = match user_row {
        Ok(Some(record)) => record.user_id,
        Ok(None) => return HttpResponse::NotFound().body("User not found"),
        Err(err) => return HttpResponse::InternalServerError().body(format!("DB error: {}", err)),
    };

    // Insert into project_members
    let res = sqlx::query(
        "INSERT OR REPLACE INTO project_members (project_id, user_id) VALUES (?, ?)",
    )
    .bind(project_id)
    .bind(user_id)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().body("Member assigned to project successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Assignment failed: {}", err)),
    }
}
