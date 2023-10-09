use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
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

#[derive(Serialize)]
struct ExampleTable {
    ID: i32,
    FirstName: String,
    LastName: String,
    Age: i32,
    Salary: f64,
    BirthDate: String,
    IsActive: bool,
    Email: String,
    PhoneNumber: String,
    Address: String,
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
    let query_string = "SELECT * FROM public.exampletable ORDER BY id ASC";
    let query = sqlx::query_as::<_, ExampleTable>(query_string)
        .fetch_all(&data.db)
        .await
        .unwrap();
    // let query =
    //     sqlx::query_as::<_, ExampleTable>("SELECT * FROM public.exampletable ORDER BY id ASC")
    //         .fetch_all(&data.db)
    //         .await
    //         .unwrap();

    HttpResponse::Ok().json(query)
}

// #[post("/file-upload/")]
// async fn file_upload_endpoint(req: HttpRequest, payload: web::Payload) -> impl Responder {
//     let mut data = web::BytesMut::new();

//     while let Some(chunk) = payload.into_inner().read_to_end(&mut data).await {
//         let chunk = chunk.unwrap();
//         data.extend_from_slice(&chunk);
//     }

//     let uploaded_data = String::from_utf8(data.to_vec()).unwrap();

//     HttpResponse::Ok().body(format!("Uploaded file content:\n{}", uploaded_data))
// }

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgresql://username:password@localhost/database_name";
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(string_response)
            .service(json_response)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
