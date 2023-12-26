use std::thread::current;
// author: xunfei
// Project: untitled4
// File: user.rs
// Date: 2023/12/24 23:33
use crate::model::user::{delete_user, list_users, retrieve_user, update_user, User};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Serialize, Deserialize)]
struct Cursor {
    limit: i32,
    offset: i32,
}

#[get("/")]
pub async fn get_users(cursor: web::Query<Cursor>) -> impl Responder {
    let res = list_users(cursor.limit, cursor.offset).await;
    match res {
        Ok(uses) => HttpResponse::Ok().json(uses),
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}

#[put("/{user_id}")]
pub async fn update(user_obj: web::Json<User>, user_id: web::Path<i32>) -> impl Responder {
    let user = retrieve_user(user_id.into_inner()).await;
    // match user {  }
    let res = update_user(user_obj.into_inner()).await;
    match res {
        Ok(()) => HttpResponse::Ok().json(()),
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}

// #[delete("/")]
