use actix_web::{get, post, patch, delete, web, HttpResponse, Responder, Error};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use uuid::Uuid;
use sqlx::SqlitePool;

use crate::models::{User,LoginRequest, NewUser};
use crate::auth;
use crate::auth::Authenticated;

const SALT: &str = "bugtrack2025";


#[post("/login")]
async fn login(pool: web::Data<SqlitePool>, body: web::Json<LoginRequest>) -> Result<impl Responder, Error>  {
    let username = &body.username;
    let password = &body.password;
    let salted = format!("{}{}", SALT, password);

    let user = sqlx::query_as::<_, User>("SELECT user_id, name, role, username, hashed_password FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match user{
        Some(db_user) => {
            let stored_hash = db_user.hashed_password;
            if verify(&salted, &stored_hash).unwrap_or(false) {
                let user_id = Uuid::parse_str(&db_user.user_id)
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid UUID"))?;
                let token = auth::create_token(user_id, &db_user.role);
                Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "success", "token": token })))
            }else{
                Ok(HttpResponse::Unauthorized().json(serde_json::json!({ "status": "failure" })))
            }
        }
        None => Ok(HttpResponse::Unauthorized().body("User not found")),
    }
}

#[post("/create_user")]
async fn create_new_user(pool: web::Data<SqlitePool>, form: web::Json<NewUser>) -> Result<impl Responder, Error>  {
    let user_id = Uuid::new_v4().to_string();
    let salted_password = format!("{}{}", SALT, form.password);
    let hashed = hash(&salted_password, DEFAULT_COST)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    sqlx::query(
        "INSERT INTO users (user_id, name, role, username, hashed_password)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&user_id)
    .bind(&form.name)
    .bind(&form.role)
    .bind(&form.username)
    .bind(&hashed)
    .execute(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let user = User {
        user_id: user_id,
        name: form.name.clone(),
        role: form.role.clone(),
        username: form.username.clone(),
        hashed_password: hashed,
    };

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "message": "User created successfully",
        "user": user
    })))
    
}