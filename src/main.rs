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
    let pool = sqlx::SqlitePool::connect(&db_url).await.expect("Failed to connect to DB");
    db::init_db(&pool).await.expect("DB init failed");

    let projects: Arc<Mutex<Vec<Project>>> = Arc::new(Mutex::new(Vec::new()));
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(projects.clone()))
            .app_data(web::Data::new(tera.clone()))
            // .service(assign_form)
            // .service(assign_submit)
            .service(login)
            .service(create_new_user)
            .service(list_projects)
            .service(create_project)
            .service(update_project)
            .service(delete_project)
            .service(create_bug)
            .service(get_all_bugs)
            .service(get_bug)
            .service(update_bug)
            .service(delete_bug)
            .service(show_assign_form)
            .service(assign_bug)
            .service(assign_member_to_project)
            // .service(list_projects)
            // .service(add_project)
            // .service(index)
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
