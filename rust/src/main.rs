use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

#[derive(Serialize)]
struct ResponseJSON {
    key1: String,
    key2: String,
    key3: String,
    key_nest: NestedData,
}

#[derive(Serialize)]
struct NestedData {
    kn1: String,
    knn2: NestedNestedData,
}

#[derive(Serialize)]
struct NestedNestedData {
    key: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/string/")]
async fn string_response() -> impl Responder {
    HttpResponse::Ok().body("Everything is working fine")
}

#[get("/simple-json/")]
async fn json_response() -> impl Responder {
    let response = ResponseJSON {
        key1: "value1".to_string(),
        key2: "value2".to_string(),
        key3: "value3".to_string(),
        key_nest: NestedData {
            kn1: "value_nest_1".to_string(),
            knn2: NestedNestedData {
                key: "value".to_string(),
            },
        },
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(string_response)
        .service(json_response)
        .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
