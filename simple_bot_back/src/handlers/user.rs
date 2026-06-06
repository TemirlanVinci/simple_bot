use axum::{Json, extract::State};
use sqlx::PgPool;

// Импортируем структуру из models.rs и функцию вставки из db.rs
use crate::db::user;
use crate::models::user::CreateUsMessage;

pub async fn accept_bot_data(
    State(pool): State<PgPool>,           // Автоматически достаем пул из Axum
    Json(payload): Json<CreateUsMessage>, // Автоматически парсим JSON в структуру
) -> &'static str {
    // Вызываем функцию из db.rs, передавая туда данные из структуры.
    // Так как payload.user_message — это String, мы передаем её как ссылку (&payload.user_message)
    match user::save_message(&pool, payload.user_id, &payload.user_message).await {
        Ok(_) => {
            println!(
                "Сообщение от пользователя {} успешно сохранено!",
                payload.user_id
            );
            "Данные успешно записаны в БД" // Этот текст уйдет другу в качестве ответа
        }
        Err(e) => {
            eprintln!("Ошибка при записи в БД: {}", e);
            "Ошибка сервера при сохранении данных"
        }
    }
}
