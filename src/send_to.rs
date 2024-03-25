use serde_json::Value;
use crate::variable::{TOKEN, CHAT_ID, CHAT_ID_OWNER};


pub async fn request_to_telegram_api(method: &str, data: &Value) -> Result<(String), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/{}", TOKEN, method);
    let response = reqwest::Client::new()
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await?;
    let status = response.status().to_string();
    let text = response.text().await?;
    if status == "200 OK" {
        Ok(text)
    } else {
        Err(format!("Error with HTTP status code: {}", status).into())
    }
}


pub async fn send_to_user_id(chat_id:i64, text: String) -> Result<(), Box<dyn std::error::Error>>{
    let data = serde_json::json!({
       "chat_id": chat_id,
       "text": text,
       "parse_mode": "HTML",
    });

    request_to_telegram_api(&"sendMessage", &data).await?;
    Ok(())
}


pub async fn send_to_tg_owner_message(text: String) -> Result<(), Box<dyn std::error::Error>>{
    let data = serde_json::json!({
       "chat_id": CHAT_ID_OWNER,
       "text": text,
       "parse_mode": "HTML",
    });

    request_to_telegram_api(&"sendMessage", &data).await?;
    Ok(())
}


pub async fn send_to_tg_channel(text: String) -> Result<(), Box<dyn std::error::Error>>{
    let data = serde_json::json!({
       "chat_id": CHAT_ID,
       "text": text
    });

    request_to_telegram_api(&"sendMessage", &data).await?;
    Ok(())
}