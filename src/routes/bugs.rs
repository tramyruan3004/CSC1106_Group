use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use crate::models::{NewBug, UpdateBugAdmin};
use crate::db::{
    create_bug_inner,
    get_all_bugs_inner,
    update_bug_inner,
    delete_bug_inner,
    get_bug_by_id_inner
};
use crate::state::AppState;


#[post("/bugs/new")]
async fn create_bug(data: web::Data<AppState>, json: web::Json<NewBug>) -> impl Responder {
    let db = &data.db;
    let bug_data = json.into_inner(); 

    match create_bug_inner(db, bug_data).await {
        Ok(bug) => HttpResponse::Ok().json(bug),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed: {}", e)),
    }
}


#[get("/bugs")]
async fn get_all_bugs(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;
    match get_all_bugs_inner(db).await {
        Ok(bugs) => HttpResponse::Ok().json(bugs),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


#[get("/bugs/{id}")]
async fn get_bug(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let db = &data.db;
    let bug_id = path.into_inner();

    match get_bug_by_id_inner(db, bug_id).await {
        Ok(bug) => HttpResponse::Ok().json(bug),
        Err(_) => HttpResponse::NotFound().body("Bug not found"),
    }
}


#[patch("/bugs/{id}")]
async fn update_bug(
    data: web::Data<AppState>,
    path: web::Path<i64>,
    json: web::Json<UpdateBugAdmin>,
) -> impl Responder {
    let db = &data.db;
    let id = path.into_inner();
    let update_data = json.into_inner();

    match update_bug_inner(db, id, update_data).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => {
            println!("Update failed: {}", e);
            HttpResponse::InternalServerError().body("Update failed")
        }
    }    
}



#[delete("/bugs/{id}")]
async fn delete_bug(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let db = &data.db;
    let id = path.into_inner();

    match delete_bug_inner(db, id).await {
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