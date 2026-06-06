use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUsMessage {
    pub user_id: i64,
    pub user_message: String,
}
