use axum::{Router, extract::Query, http::StatusCode, response::Json, routing::get};
use screeps_rust_api::screeps_api_from_env;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod res;

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
        );

    // 运行应用，监听3000端口
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
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
