use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use tera::{Tera, Context};

use crate::db::update_bug_inner;
use crate::state::AppState;
use crate::models::{UpdateBugAdmin, Bug, AssignBugForm};
use crate::auth;
use crate::auth::Authenticated;

#[get("/bugs/assign")]
pub async fn show_assign_form(auth: Authenticated, tmpl: web::Data<Tera>) -> impl Responder {
    if !(auth.is_admin() || auth.is_developer()) {
        return HttpResponse::Forbidden().body("UNAUTHORISED ACCESS");
    }
    let ctx = tera::Context::new();
    let rendered = tmpl.render("assign_bug.html", &ctx);
    match rendered {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

#[post("/bugs/assign")]
pub async fn assign_bug(auth: Authenticated, data: web::Data<AppState>,tmpl: web::Data<Tera>, form: web::Form<AssignBugForm>) -> impl Responder {
    if !auth.is_admin() {
        return HttpResponse::Forbidden().body("ADMIN ONLY");
    }
    let db = &data.db;  
    let bug_id = form.bug_id;
    let dev_id = form.developer_id.clone();

    // Try updating the bug with new developer_id
    let update = UpdateBugAdmin {
        description: None,
        severity: None,
        bug_type: None,
        progress_status: None,
        developer_id: Some(dev_id.clone()),
    };

    match update_bug_inner(db, bug_id, update).await {
        Ok(bug) => {
            let mut ctx = tera::Context::new();
            ctx.insert("bug", &bug);
            ctx.insert("developer_id", &dev_id);
            let rendered = tmpl.render("assign_success.html", &ctx);
            match rendered {
                Ok(html) => HttpResponse::Ok().body(html),
                Err(e) => HttpResponse::InternalServerError().body(format!("Render error: {}", e)),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid bug ID or developer ID"),
    }
}