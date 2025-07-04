// use actix_web::{get, post, web, HttpResponse, Responder};
// use sqlx::SqlitePool;
// use tera::{Tera, Context};
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct AssignForm {
//     pub bug_id: i64,
//     pub developer_id: String,
// }

// #[get("/bugs/assign")]
// async fn assign_form(tmpl: web::Data<Tera>) -> impl Responder {
//     let ctx = Context::new();
//     let html = tmpl.render("assign_form.html", &ctx).unwrap();
//     HttpResponse::Ok().body(html)
// }

// #[post("/bugs/assign")]
// async fn assign_submit(
//     tmpl: web::Data<Tera>,
//     pool: web::Data<SqlitePool>,
//     form: web::Form<AssignForm>,
// ) -> impl Responder {
//     let mut ctx = Context::new();
//     let result = sqlx::query("UPDATE bugs SET developer_id = ? WHERE bug_id = ?")
//         .bind(&form.developer_id)
//         .bind(form.bug_id)
//         .execute(pool.get_ref())
//         .await;

//     if let Ok(res) = result {
//         if res.rows_affected() > 0 {
//             ctx.insert("success", &true);
//             ctx.insert("bug_id", &form.bug_id);
//             ctx.insert("developer_id", &form.developer_id);
//         } else {
//             ctx.insert("success", &false);
//             ctx.insert("error", "Bug not found.");
//         }
//     } else {
//         ctx.insert("success", &false);
//         ctx.insert("error", "Assignment failed.");
//     }

//     let html = tmpl.render("assign_result.html", &ctx).unwrap();
//     HttpResponse::Ok().body(html)
// }
