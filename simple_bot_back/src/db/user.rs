use sqlx::PgPool;

// Асинхронная функция, которая принимает ссылку на пул соединений,
// ID пользователя и его сообщение.
pub async fn save_message(pool: &PgPool, user_id: i64, message: &str) -> Result<(), sqlx::Error> {
    // Макрос query! проверяет синтаксис SQL прямо во время компиляции.
    // Названия колонок (user_id, us_mess) должны строго совпадать с твоей миграцией.
    sqlx::query!(
        "INSERT INTO user_message (user_id, us_mess) VALUES ($1, $2)",
        user_id,
        message
    )
    .execute(pool) // Выполняем запрос в нашей базе
    .await?; // Ждем окончания асинхронной операции

    Ok(())
}
