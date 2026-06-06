use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod db;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Переменная DATABASE_URL не найдена в .env");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    println!("✅ Успешное подключение к PostgreSQL!");

    let app = Router::new()
        .route(
            "/message",
            axum::routing::post(handlers::user::accept_bot_data),
        )
        .with_state(pool);

    // 3. Определяем адрес и порт (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Запуск сервера на http://{}", addr);

    // 4. Создаем слушатель TCP-протокола от Tokio
    let listener = TcpListener::bind(addr).await.unwrap();

    // 5. Запускаем сервер Axum, передавая ему слушателя и наши маршруты
    axum::serve(listener, app).await.unwrap();
}
