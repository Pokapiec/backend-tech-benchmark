use actix_multipart::Multipart;
use actix_web::dev::Payload;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;
use std::collections::HashMap;

pub struct AppState {
    db: PgPool,
}

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

#[derive(Serialize, FromRow)]
struct ExampleTable {
    id: i32,
    firstname: String,
    lastname: String,
    age: i32,
    salary: String,
    birthdate: String,
    isactive: bool,
    email: String,
    phonenumber: String,
    address: String,
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

#[get("/query-params/")]
async fn query_params_endpoint(
    query_params: web::Query<HashMap<String, String>>,
) -> impl Responder {
    HttpResponse::Ok().json(query_params.into_inner())
}

#[get("/sql-select/")]
async fn sql_select_endpoint(data: web::Data<AppState>) -> impl Responder {
    let query_string = "SELECT ID,FirstName,LastName,Age,Salary::varchar,BirthDate::varchar,IsActive,Email,PhoneNumber,Address FROM public.exampletable ORDER BY id ASC";
    let result = sqlx::query_as::<_, ExampleTable>(query_string)
        .fetch_all(&data.db)
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}

// #[post("/file-upload/")]
// async fn file_upload_endpoint(payload: Multipart) -> impl Responder {
//     let mut file_content = Bytes::new();

//     while let Some(chunk) = payload.next().await {
//         let data = chunk
//             .map_err(|_e| actix_web::error::ErrorInternalServerError("Failed to read a chunk"))?;

//         file_content.extend_from_slice(&data);
//     }

//     // Convert the content to a string
//     let file_content_str = String::from_utf8(file_content.to_vec())
//         .map_err(|e| actix_web::error::ErrorBadRequest(format!("Invalid UTF-8: {}", e)))?;

//     Ok(HttpResponse::Ok().body(file_content_str))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgresql://postgres:postgres@localhost/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(string_response)
            .service(query_params_endpoint)
            .service(json_response)
            .service(sql_select_endpoint)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
