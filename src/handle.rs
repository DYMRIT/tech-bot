use serde_json::Value;
use crate::send_to::{request_to_telegram_api, send_to_tg_owner_message, send_to_user_id};
use crate::db_threads::{get_thread, get_chat_id, insert_thread_row};
use crate::api_tg::{send_start_message, send_to_thread};
use crate::variable::{CHAT_ID};
use tokio_postgres::{Client, Error};


pub async fn handle_message(client: &Client, message: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let message_id = message["message_id"].as_i64().unwrap();
    let chat_id = message["chat"]["id"].as_i64().unwrap_or(-1);
    let text = message["text"].as_str().unwrap_or("");
    let thread_id = message["reply_to_message"]["message_thread_id"].as_i64().unwrap_or(-1);

    if message["photo"].is_null() && message["voice"].is_null()
        && message["sticker"].is_null() && message["document"].is_null()
        && message["sticker"].is_null() && message["location"].is_null()
        && message["poll"].is_null() && message["contact"].is_null(){

        if !message["chat"]["username"].is_null() {
            //handle message from user
            let name = message["chat"]["first_name"].as_str().unwrap();
            let username = message["chat"]["username"].as_str().unwrap();
            if text == "/start" {
                send_start_message(chat_id).await?;
            } else {
                match get_thread(client, chat_id).await? {
                    Some(row) => {
                        let id: i64 = row.get(0);
                        let chat_id: i64 = row.get(1);
                        let thread_id: i64 = row.get(2);
                        let active: bool = row.get(3);
                        if active {
                            send_to_thread(thread_id, text.to_string()).await?;
                        } else {
                            send_start_message(chat_id).await?;
                        }
                    },
                    None => send_start_message(chat_id).await?
                }
            }
        } else if !message["chat"]["title"].is_null() {
            //handle message from chat
            let name_from = message["from"]["first_name"].as_str().unwrap();
            let username_from = message["from"]["username"].as_str().unwrap();


            if text == "/exit" && !message["is_topic_message"].is_null() {
                match get_chat_id(client, thread_id).await? {
                    Some(row) => {
                        let chat_id: i64 = row.get(1);
                        let thread_id: i64 = row.get(2);
                        let active: bool = row.get(3);
                        if active {
                            insert_thread_row(client, thread_id, chat_id, !active).await?;
                            println!("{}", chat_id);
                            send_to_user_id(chat_id, "Спасибо за ваше обращение.".to_string()).await?;
                            send_to_user_id(chat_id, "<i>Чат закрыт</i>".to_string()).await?;
                            send_to_thread(thread_id, "<i>Чат закрыт</i>".to_string()).await?;
                        } else {
                            send_to_thread(thread_id, "<i>Сейчас чат неактивен</i>".to_string()).await?;
                        }
                    },
                    None => {},
                }
            } else if !message["is_topic_message"].is_null() {
                match get_chat_id(client, thread_id).await? {
                    Some(row) => {
                        let chat_id: i64 = row.get(1);
                        let active: bool = row.get(3);
                        if active {
                            send_to_user_id(chat_id, text.to_string()).await?;
                        }
                    },
                    None => {},
                }
            }
        }
    } else {
        match get_thread(client, chat_id).await? {
            Some(row) => {
                let thread_id: i64 = row.get(2);
                let active: bool = row.get(3);
                if active {
                    let data = serde_json::json!({
                       "chat_id": CHAT_ID,
                       "from_chat_id": chat_id,
                       "message_thread_id": thread_id,
                       "message_id": message_id,
                    });
                    request_to_telegram_api("forwardMessage", &data).await?;
                } else {
                    send_start_message(chat_id).await?;
                }
            },
            None => send_start_message(chat_id).await?
        }
    }

    Ok(())
}