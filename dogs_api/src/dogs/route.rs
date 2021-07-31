use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::error_handler::CustomError;
use crate::dogs::{Dog, Dogs};

#[get("/dogs")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let dogs = Dogs::find_all()?;
    Ok(HttpResponse::Ok().json(dogs))
}

#[get("/dogs/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let dog = Dogs::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(dog))
}

#[post("/dogs")]
async fn create(dog: web::Json<Dog>) -> Result<HttpResponse, CustomError> {
    let dog = Dogs::create(dog.into_inner())?;
    Ok(HttpResponse::Ok().json(dog))
}

#[put("/dogs/{id}")]
async fn update(
    id: web::Path<i32>,
    dog: web::Json<Dog>,
) -> Result<HttpResponse, CustomError> {
    let dog = Dogs::update(id.into_inner(), dog.into_inner())?;
    Ok(HttpResponse::Ok().json(dog))
}

#[delete("/dogs/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_dog = Dogs::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_dog })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}