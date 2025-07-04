use actix_web::{web, App, HttpServer};
use actix_web::{get, HttpResponse};
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;
use actix_files::{Files, NamedFile};

mod db;
mod models;
mod routes;
mod auth;
mod state;

use crate::state::AppState;
use crate::models::Project;
use crate::routes::{login::*, bugs::*, projects::*, assign::*};
use tera::Tera;
// use routes::ui::*;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("✅ Bug Tracker is running!")
}

#[get("/login")]
async fn login_page() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/login.html")?)
}

#[get("/dashboard")]
async fn dashboard_page() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/dashboard.html")?)
}

#[get("/bugs")]
async fn bugs_page() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/bugs.html")?)
}

#[get("/assign")]
async fn assign_page() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/assign.html")?)
}

#[get("/projects")]
async fn projects_page() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("static/projects.html")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = sqlx::SqlitePool::connect(&db_url).await.expect("Failed to connect to DB");
    db::init_db(&pool).await.expect("DB init failed");

    let projects: Arc<Mutex<Vec<Project>>> = Arc::new(Mutex::new(Vec::new()));
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");
    let app_state = web::Data::new(AppState {
        db: pool.clone(),
        projects: projects.clone(),
    });

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(tera.clone()))
            .service(index)
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
            // New HTML page routes
            .service(login_page)
            .service(dashboard_page)
            .service(bugs_page)
            .service(assign_page)
            .service(projects_page)

            // Fallback: serve remaining static files (e.g., CSS)
            .service(Files::new("/", "./static").index_file("login.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
