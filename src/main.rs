use actix_web::{web, App, HttpServer};
use actix_web::{get, HttpResponse};
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod routes;
mod auth;

use crate::models::Project;
use crate::routes::{login::*, bugs::*, projects::*, assign::*};
use tera::Tera;
use routes::ui::*;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("✅ Bug Tracker is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Connect to SQLite and wrap with Mutex
    let pool = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    db::init_db(&pool).await.expect("DB init failed");

    let shared_pool = web::Data::new(Mutex::new(pool));
    let projects: Arc<Mutex<Vec<Project>>> = Arc::new(Mutex::new(Vec::new()));
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(shared_pool.clone())
            .app_data(web::Data::new(projects.clone()))
            .app_data(web::Data::new(tera.clone()))
            .service(index)
            .service(login)
            .service(create_new_user)
            .service(list_projects)
            .service(create_project)
            .service(update_project)
            .service(delete_project)
            // Add your bug and assign routes here when ready
            // .service(create_bug)
            // .service(get_all_bugs)
            // .service(get_bug)
            // .service(update_bug)
            // .service(delete_bug)
            // .service(assign_form)
            // .service(assign_submit)
            // .service(login_form)
            // .service(login_submit)
            // .service(bug_form)
            // .service(bug_submit)
            // .service(bug_list)
            // .service(project_page)
            // .service(project_submit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
