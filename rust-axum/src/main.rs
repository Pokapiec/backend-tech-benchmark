// use std::sync::Arc;
// use tokio::sync::Mutex;

// use axum::http::{
//     header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
//     HeaderValue, Method,
// };

use axum::{
    extract::{Multipart, Query, State},
    // http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};
// use std::env;

use serde::{Deserialize, Serialize};
// use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub param1: Option<String>,
    pub param2: Option<String>,
}

#[tokio::main]
async fn main() {
    // let cors = CorsLayer::new()
    //     .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_credentials(true)
    //     .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    dotenv::dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "postgres://postgres:postgres@postgres:5432/postgres".to_string();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let app = Router::new()
        .route("/string/", get(get_string_response))
        .route("/simple-json/", get(get_simple_json_response))
        .route("/query-params/", get(get_merged_query_params_response))
        .route("/sql-select/", get(get_sql_response))
        .route("/file-upload/", post(upload_file_and_return_read_content))
        .with_state(pool);
    // .layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_string_response() -> impl IntoResponse {
    "Hello World!"
}

pub async fn get_simple_json_response() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
        "key_nest": {"kn1": "value_nest_1", "knn2": {"key": "value"}},
    });

    Json(json_response)
}

pub async fn get_merged_query_params_response(
    Query(QueryOptions { param1, param2 }): Query<QueryOptions>,
) -> impl IntoResponse {
    let json_response = serde_json::json!({
        "param1": param1,
        "param2": param2,
    });

    Json(json_response)
}

#[allow(non_snake_case)]
#[derive(FromRow, Serialize)]
pub struct SQLRecord {
    pub id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub age: Option<i32>,
    pub salary: Option<String>,
    pub birthdate: Option<String>,
    pub isactive: Option<bool>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub address: Option<String>,
}

pub async fn get_sql_response(State(pool): State<sqlx::PgPool>) -> impl IntoResponse {
    let data = sqlx::query_as::<_, SQLRecord>(
        "SELECT id, firstname, lastname, age, salary::varchar, birthdate::varchar, isactive, email, phonenumber, address FROM public.exampletable ORDER BY id ASC",
    ) // .bind(1)
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch data");

    Json(data)
}

// upload_file_and_return_read_content handle reads file from multipart/form-data content and return it as response
pub async fn upload_file_and_return_read_content(mut multipart: Multipart) -> impl IntoResponse {
    let mut content: String = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        content = field.text().await.unwrap();
    }
    Json(content)
}
