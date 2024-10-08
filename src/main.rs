use axum::{
    routing::get,
    http::StatusCode,
    extract::Query,
    Json, Router,
};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    // ルートハンドラを設定
    let app = Router::new()
        // `GET /` は `root` ハンドラに送信される
        .route("/", get(root));

    // ポート3000でサーバーを起動
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// クエリパラメータをパースするための構造体
#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// レスポンス用のデータ構造
#[derive(Serialize)]
struct HelloWorldResponse {
    message: String,
}

// ルートハンドラ - ステータスコードとJSONレスポンスを返す
async fn root(Query(params): Query<HelloParams>) -> Result<(StatusCode, Json<HelloWorldResponse>), (StatusCode, &'static str)> {
    // `name` パラメータがない場合、400 Bad Requestを返す
    let name = match params.name {
        Some(name) if !name.is_empty() => name, // 空文字列チェックも含める
        _ => return Err((StatusCode::BAD_REQUEST, "Missing 'name' parameter")),
    };

    let response = HelloWorldResponse {
        message: format!("Hello, {}!", name),
    };
    
    // ステータスコード200 OKと共にJSON形式でレスポンスを返す
    Ok((StatusCode::OK, Json(response)))
}
