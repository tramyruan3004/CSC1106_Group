// use actix_web::{get, post, web, HttpResponse, Responder};
// use tera::{Tera, Context};
// use sqlx::SqlitePool;
// use std::sync::{Arc, Mutex};
// use crate::models::{NewBug, Project};

// type SharedProjects = Arc<Mutex<Vec<Project>>>;

// #[get("/")]
// async fn index(tmpl: web::Data<Tera>) -> impl Responder {
//     let html = tmpl.render("index.html", &Context::new()).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[get("/login")]
// async fn login_form(tmpl: web::Data<Tera>) -> impl Responder {
//     let html = tmpl.render("login.html", &Context::new()).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[post("/login")]
// async fn login_submit(form: web::Form<crate::models::LoginRequest>) -> impl Responder {
//     if form.username == "admin" && form.password == "admin123" {
//         HttpResponse::Ok().body("Login successful!")
//     } else {
//         HttpResponse::Unauthorized().body("Invalid credentials")
//     }
// }

// #[get("/bugs/new")]
// async fn bug_form(tmpl: web::Data<Tera>) -> impl Responder {
//     let html = tmpl.render("bug_form.html", &Context::new()).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[post("/bugs/new")]
// async fn bug_submit(
//     tmpl: web::Data<Tera>,
//     pool: web::Data<SqlitePool>,
//     form: web::Form<NewBug>,
// ) -> impl Responder {
//     let _ = sqlx::query("INSERT INTO bugs (title, description, reported_by, severity, status)
//                          VALUES (?, ?, ?, ?, 'Open')")
//         .bind(&form.title)
//         .bind(&form.description)
//         .bind(&form.reported_by)
//         .bind(&form.severity)
//         .execute(pool.get_ref())
//         .await;

//     HttpResponse::Ok().body("Bug submitted successfully.")
// }

// #[get("/bugs")]
// async fn bug_list(pool: web::Data<SqlitePool>, tmpl: web::Data<Tera>) -> impl Responder {
//     let bugs = sqlx::query_as::<_, crate::models::Bug>("SELECT * FROM bugs")
//         .fetch_all(pool.get_ref())
//         .await
//         .unwrap_or_else(|_| vec![]);

//     let mut ctx = Context::new();
//     ctx.insert("bugs", &bugs);
//     let html = tmpl.render("bug_list.html", &ctx).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[get("/projects")]
// async fn project_page(projects: web::Data<SharedProjects>, tmpl: web::Data<Tera>) -> impl Responder {
//     let project_list = projects.lock().unwrap();
//     let mut ctx = Context::new();
//     ctx.insert("projects", &*project_list);
//     let html = tmpl.render("project_page.html", &ctx).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[post("/projects")]
// async fn project_submit(
//     projects: web::Data<SharedProjects>,
//     form: web::Form<Project>,
// ) -> impl Responder {
//     let mut list = projects.lock().unwrap();
//     list.push(form.into_inner());
//     HttpResponse::SeeOther()
//         .append_header(("Location", "/projects"))
//         .finish()
// }
