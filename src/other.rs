use std::env;
use std::io::{self, Read};
use serde_json::Value;
use crate::variable::{TOKEN, CHAT_ID_OWNER};


pub fn send_hello_http200() {
    println!("HTTP/1.1 200 OK\r");
    println!("Content-Type: application/json\r\n");
    println!("{{\"text\": \"Hello world\"}}");
}


pub fn get_env_vars() -> (String, bool) {
    let mut flag: bool = false;

    let mut header_str = String::new();
    for (key, value) in env::vars() {
        header_str.push_str(&format!("{}: {}\n", key, value));
        if key == "HTTP_X_TELEGRAM_BOT_API_SECRET_TOKEN" && value == "any key if exist" {
            flag = true;
        }
    }
    header_str.push_str("\n");
    (header_str, flag)
}


pub fn get_stdin() -> String {
    let content_length = std::env::var("CONTENT_LENGTH")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<usize>()
        .expect("CONTENT_LENGTH не является числом");

    // Создаем буфер для чтения данных
    let mut buffer = vec![0; content_length];

    // Читаем данные из stdin в буфер
    io::stdin()
        .read_exact(&mut buffer)
        .expect("Ошибка при чтении из stdin");

    // Преобразуем байты в строку UTF-8
    let body = String::from_utf8(buffer)
        .expect("Данные не являются допустимой UTF-8 строкой");

    body
}