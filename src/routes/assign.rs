use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use tera::{Tera, Context};

use crate::db::update_bug_inner;
use crate::models::{UpdateBugAdmin, Bug, AssignBugForm};


#[get("/bugs/assign")]
pub async fn show_assign_form(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = tmpl.render("assign_bug.html", &ctx);
    match rendered {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

#[post("/bugs/assign")]
pub async fn assign_bug(pool: web::Data<SqlitePool>,tmpl: web::Data<Tera>, form: web::Form<AssignBugForm>) -> impl Responder {
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

    match update_bug_inner(pool.get_ref(), bug_id, update).await {
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