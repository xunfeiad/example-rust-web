// author: xunfei
// Project: untitled4
// File: ttmp
// Date: 2023/12/26 21:25

use crate::service::ttmp::{retrieve_channel_id, vehicle2service};
use actix_web::{get, HttpResponse, Responder};

#[get("/vehicle_service")]
pub async fn get_vehicle_service() -> impl Responder {
    let res = vehicle2service().await;
    match res {
        Ok(uses) => HttpResponse::Ok().json(uses),
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}

#[get("/get_channel_id")]
pub async fn get_channel() -> impl Responder {
    let res = retrieve_channel_id().await;
    match res {
        Ok(channel) => HttpResponse::Ok().json(channel),
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}
