use serde_json::Value;
use crate::db_threads::{get_thread, insert_thread_row, max_id_from_db};
use crate::send_to::{request_to_telegram_api, send_to_user_id};
use tokio_postgres::{Client};


pub async fn callback_message(client: &Client, callback: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let data = callback["data"].as_str().unwrap();
    let chat_id = callback["message"]["chat"]["id"].as_i64().unwrap_or(0);
    let message_id = callback["message"]["message_id"].as_i64().unwrap_or(0);
    let callback_id = callback["id"].as_str().unwrap_or("");
    println!("{}", callback_id);

    if data == "--//--" {
        match get_thread(client, chat_id).await? {
            Some(row) => {
                let id: i64 = row.get(0);
                let chat_id: i64 = row.get(1);
                let thread_id: i64 = row.get(2);
                let active: bool = row.get(3);
                if !active {
                    request_to_telegram_api(&"answerCallbackQuery", &serde_json::json!({"callback_query_id": callback_id})).await?;


                    let first_name = callback["from"]["first_name"].as_str().unwrap();
                    let username = callback["from"]["username"].as_str().unwrap();

                    insert_thread_row(client, thread_id, chat_id, !active).await?;
                    send_to_user_id(chat_id, "<b>Напиши ваш вопрос</b>. \nВам ответит первый освободившийся специалист".to_string()).await?;
                    send_to_thread(thread_id, "<b>Чат Начат</b>".to_string()).await?;

                } else {
                    let callback_data = serde_json::json!({
                        "callback_query_id": callback["id"].as_str().unwrap(),
                        "text": "Вы уже находитесь в чате с поддержкой, опишите вашу проблему.",
                        "show_alert": true
                    });
                    request_to_telegram_api(&"answerCallbackQuery", &callback_data).await?;
                }
            },
            None => {
                request_to_telegram_api(&"answerCallbackQuery", &serde_json::json!({"callback_query_id": callback_id})).await?;
                let max_id = max_id_from_db(client).await? + 1;
                let username = callback["from"]["username"].as_str().unwrap();
                let thread_id = created_thread(format!("Чат №{} с {}", max_id, username)).await?;
                insert_thread_row(client, thread_id, chat_id, true).await?;
                send_to_user_id(chat_id, "<b>Напишите ваш вопрос</b>. \nВам ответит первый освободившийся специалист".to_string()).await?;
                send_to_thread(thread_id, format!("<b>Пользователь:</b>  @{}", username)).await?;
                send_to_thread(thread_id, "<b>Чат Начат</b>".to_string()).await?;
            }
        }
    }

    // Удалил другие обработчики callback

    Ok(())
}