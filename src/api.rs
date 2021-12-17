use crate::model::{pool as ModelPool, user as ModelUser};
use crate::service::{pool as ServicePool, user as ServiceUser};
use actix_web::{web, HttpResponse, Responder};

pub async fn login(req: web::Json<ModelUser::LoginRequest>) -> impl Responder {
    println!("zhTian req : {:?}", req);

    match ServiceUser::login(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("Login Failed"),
    }
}

pub async fn search(req: web::Json<ModelPool::SearchRequest>) -> impl Responder {
    // call contract.
    match ServicePool::search(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("seach pool Failed"),
    }
}
