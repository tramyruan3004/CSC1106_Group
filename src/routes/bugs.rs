// use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
// use crate::models::{NewBug, UpdateBug};
// use crate::db;
// use sqlx::SqlitePool;

// #[post("/bugs/new")]
// async fn create_bug(pool: web::Data<SqlitePool>, json: web::Json<NewBug>) -> impl Responder {
//     match db::insert_bug(&pool, json.0).await {
//         Ok(bug) => HttpResponse::Ok().json(bug),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Failed: {}", e)),
//     }
// }

// #[get("/bugs")]
// async fn get_all_bugs(pool: web::Data<SqlitePool>) -> impl Responder {
//     match db::get_all_bugs(&pool).await {
//         Ok(bugs) => HttpResponse::Ok().json(bugs),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
//     }
// }

// #[get("/bugs/{id}")]
// async fn get_bug(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
//     match db::get_bug_by_id(&pool, path.into_inner()).await {
//         Ok(bug) => HttpResponse::Ok().json(bug),
//         Err(_) => HttpResponse::NotFound().body("Bug not found"),
//     }
// }

// #[patch("/bugs/{id}")]
// async fn update_bug(
//     pool: web::Data<SqlitePool>,
//     path: web::Path<i64>,
//     json: web::Json<UpdateBug>,
// ) -> impl Responder {
//     match db::update_bug(&pool, path.into_inner(), json.0).await {
//         Ok(updated) => HttpResponse::Ok().json(updated),
//         Err(_) => HttpResponse::NotFound().body("Update failed"),
//     }
// }

// #[delete("/bugs/{id}")]
// async fn delete_bug(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
//     match db::delete_bug(&pool, path.into_inner()).await {
//         Ok(res) => {
//             if res.rows_affected() == 0 {
//                 HttpResponse::NotFound().body("Bug not found")
//             } else {
//                 HttpResponse::Ok().body("Bug deleted")
//             }
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
//     }
// }
