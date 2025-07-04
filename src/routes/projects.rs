use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use chrono::Utc;
use crate::state::AppState;
use crate::auth;
use crate::auth::Authenticated;

use crate::models::{Project, NewProject, UpdateProjectAdmin, AssignMemberToProjectRequest};

#[get("/projects")]
async fn list_projects(auth: Authenticated, data: web::Data<AppState>) -> impl Responder {
    let projects = data.projects.lock().unwrap();
    HttpResponse::Ok().json(&*projects)
}

#[post("/projects")]
async fn create_project(
    auth: Authenticated,
    data: web::Data<AppState>,
    json: web::Json<NewProject>,
) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().body("Admin only");
    }
    let now = Utc::now().to_rfc3339();
    let db = &data.db;

    let result = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, description, created_by, created_at)
         VALUES (?, ?, ?, ?)
         RETURNING *",
    )
    .bind(&json.name)
    .bind(&json.description)
    .bind(&json.created_by)
    .bind(now)
    .fetch_one(db)
    .await;

    match result {
        Ok(project) => {
            let mut projects = data.projects.lock().unwrap();
            projects.push(project.clone());
            HttpResponse::Ok().json(project)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Create failed: {}", err)),
    }
}

#[patch("/projects/{id}")]
async fn update_project(
    auth: Authenticated,
    data: web::Data<AppState>,
    path: web::Path<i64>,
    json: web::Json<UpdateProjectAdmin>,
) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().body("Admin only");
    }
    let project_id = path.into_inner();
    let db = &data.db;

    let existing = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_one(db)
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
    .fetch_one(db)
    .await;

    match result {
        Ok(updated) => {
            let mut projects = data.projects.lock().unwrap();
            if let Some(p) = projects.iter_mut().find(|p| p.project_id == project_id) {
                *p = updated.clone();
            }
            HttpResponse::Ok().json(updated)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Update failed: {}", err)),
    }
}

#[delete("/projects/{id}")]
pub async fn delete_project(
    auth: Authenticated,
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().body("Admin only");
    }
    let project_id = path.into_inner();
    let db = &data.db;

    let res = sqlx::query("DELETE FROM projects WHERE project_id = ?")
        .bind(project_id)
        .execute(db)
        .await;

    match res {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body("Project not found")
            } else {
                let mut projects = data.projects.lock().unwrap();
                projects.retain(|p| p.project_id != project_id);
                HttpResponse::Ok().body("Project deleted")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Delete failed: {}", err)),
    }
}

#[post("/projects/{id}/assign")]
pub async fn assign_member_to_project(
    auth: Authenticated,
    data: web::Data<AppState>,
    path: web::Path<i64>,
    json: web::Json<AssignMemberToProjectRequest>,
) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().body("Admin only");
    }
    let project_id = path.into_inner();
    let db = &data.db;
    let AssignMemberToProjectRequest { username } = json.into_inner();

    // Check project exists
    let project_check = sqlx::query("SELECT 1 FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_optional(db)
        .await;

    if let Ok(None) = project_check {
        return HttpResponse::NotFound().body("Project not found");
    }

    // Get user_id by username
    let user_row = sqlx::query!("SELECT user_id FROM users WHERE username = ?", username)
        .fetch_optional(db)
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
    .execute(db)
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().body("Member assigned to project successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Assignment failed: {}", err)),
    }
}
