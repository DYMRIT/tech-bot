use serde_json::Value;
use crate::send_to::request_to_telegram_api;
use crate::variable::{CHAT_ID};


pub async fn created_thread(name: String) -> Result<(i64), Box<dyn std::error::Error>> {
    let data = serde_json::json!({
       "chat_id": CHAT_ID,
       "name": name
    });
    let res = request_to_telegram_api("createForumTopic", &data).await?;
    let js:Value = serde_json::from_str(&res)?;
    let thread_id = js.get("result").unwrap().get("message_thread_id").unwrap();
    Ok(thread_id.as_i64().unwrap())
    //Ok(serde_json::to_string(thread_id).unwrap())
}


pub async fn send_to_thread(thread_id: i64, text: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({
       "chat_id": CHAT_ID,
       "text": text,
       "message_thread_id": thread_id,
       "parse_mode": "HTML",
    });

    request_to_telegram_api("sendMessage", &data).await?;
    Ok(())
}


pub async fn send_start_message(chat_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"sendMessage", &data).await?;

    Ok(())
}

pub async fn edit_start_message(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}

pub async fn edit_zakaz_bsk(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}

pub async fn edit_already_completed_order(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_fall_order_number(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_have_not_receipt(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_status_order(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_fall_time_ready_bsk(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_not_ready_completed_bsk(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_can_not_completed_bsk(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_refill_card(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_edin_for_month(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_podororojnik(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_1(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_2(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_3(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_4(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_5(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_6(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_7(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}

pub async fn edit_text_8(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_9(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}

pub async fn edit_text_10(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_11(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_12(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_13(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_14(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_15(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_16(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_17(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_18(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}


pub async fn edit_text_19(chat_id: &i64, message_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::json!({});
    request_to_telegram_api(&"editMessageText", &data).await?;

    Ok(())
}

