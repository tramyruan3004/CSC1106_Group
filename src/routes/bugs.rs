use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use crate::models::{NewBug, UpdateBugAdmin};
use crate::db::{
    create_bug_inner,
    get_all_bugs_inner,
    update_bug_inner,
    delete_bug_inner,
    get_bug_by_id_inner
};

use sqlx::SqlitePool;

#[post("/bugs/new")]
async fn create_bug(
    pool: web::Data<SqlitePool>,
    json: web::Json<NewBug>,
) -> impl Responder {
    let bug_data = json.into_inner();

    if bug_data.title.trim().is_empty() {
        return HttpResponse::BadRequest().body("Title is required.");
    }
    if bug_data.description.trim().is_empty() {
        return HttpResponse::BadRequest().body("Description is required.");
    }
    if bug_data.reported_by.trim().is_empty() {
        return HttpResponse::BadRequest().body("Reporter email is required.");
    }
    if bug_data.severity.trim().is_empty() {
        return HttpResponse::BadRequest().body("Severity is required.");
    }

    match create_bug_inner(pool.get_ref(), bug_data).await {
        Ok(bug) => HttpResponse::Ok().json(bug),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed: {}", e)),
    }
}



#[get("/bugs")]
async fn get_all_bugs(pool: web::Data<SqlitePool>) -> impl Responder {
    match get_all_bugs_inner(pool.get_ref()).await {
        Ok(bugs) => HttpResponse::Ok().json(bugs),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


#[get("/bugs/{id}")]
async fn get_bug(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let bug_id = path.into_inner();

    match get_bug_by_id_inner(pool.get_ref(), bug_id).await {
        Ok(bug) => HttpResponse::Ok().json(bug),
        Err(_) => HttpResponse::NotFound().body("Bug not found"),
    }
}


#[patch("/bugs/{id}")]
async fn update_bug(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    json: web::Json<UpdateBugAdmin>,
) -> impl Responder {
    let id = path.into_inner();
    let update_data = json.into_inner();

    match update_bug_inner(pool.get_ref(), id, update_data).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => {
            println!("Update failed: {}", e);
            HttpResponse::InternalServerError().body("Update failed")
        }
    }    
}



#[delete("/bugs/{id}")]
async fn delete_bug(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    match delete_bug_inner(pool.get_ref(), id).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                HttpResponse::NotFound().body("Bug not found")
            } else {
                HttpResponse::Ok().body("Bug deleted")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}