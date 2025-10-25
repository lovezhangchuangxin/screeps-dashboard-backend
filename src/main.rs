use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use screeps_rust_api::screeps_api_from_env;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::io::ReaderStream;

use crate::res::draw_res_image;

mod constants;
mod res;
mod utils;

// 定义查询参数结构体
#[derive(Deserialize)]
struct ResQueryParams {
    username: String,
    shard: String,
}

// 定义响应结构体
#[derive(Serialize)]
struct ResResponse {
    success: bool,
    data: Option<std::collections::HashMap<String, std::collections::HashMap<String, i32>>>,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    utils::create_data_dir().expect("create data dir failed");
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    println!("Starting server on port {}", port);

    // 初始化API客户端
    let api = Arc::new(screeps_api_from_env!().unwrap());

    // 构建应用路由
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route(
            "/res",
            get({
                let api = api.clone();
                move |query: Query<ResQueryParams>| get_res_handler(api.clone(), query)
            }),
        )
        .route(
            "/res/image",
            get({
                let api = api.clone();
                move |query: Query<ResQueryParams>| get_res_image_handler(api.clone(), query)
            }),
        );

    // 运行应用，监听3000端口
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 基础处理函数，返回静态字符串
async fn root() -> &'static str {
    "Hello, World!"
}

// 获取玩家资源信息的处理函数
async fn get_res_handler(
    api: Arc<screeps_rust_api::ScreepsApi>,
    Query(params): Query<ResQueryParams>,
) -> (StatusCode, Json<ResResponse>) {
    let result = res::query_res(&api, &params.username, &params.shard).await;

    match result {
        Ok(data) => (
            StatusCode::OK,
            Json(ResResponse {
                success: true,
                data: Some(data),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            }),
        ),
    }
}

// 获取玩家资源信息图片的处理函数
async fn get_res_image_handler(
    api: Arc<screeps_rust_api::ScreepsApi>,
    Query(params): Query<ResQueryParams>,
) -> impl IntoResponse {
    let path = draw_res_image(&api, &params.username, &params.shard)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("Error: {}", e)))?;

    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(file);

    // 构建响应
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(Body::from_stream(stream))
        .unwrap();
    Ok(response)
}
