mod handle;
mod variable;
mod other;
mod send_to;
mod api_tg;
mod callback;
mod db_threads;

use other::{send_hello_http200};
use send_to::{send_to_tg_owner_message};
use serde_json::Value;
use tokio_postgres::NoTls;
use crate::other::{get_stdin, get_env_vars};
use crate::handle::{handle_message};
use crate::callback::{callback_message};
use crate::db_threads::{get_thread, insert_thread_row, max_id_from_db, insert_action_row, max_id_actions};
use crate::api_tg::{created_thread};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    send_hello_http200();

    let connection_string = "host=host user=user password=password dbname=dbname";
    //
    let (client, connection) =
        tokio_postgres::connect(connection_string, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("database connection error: {}", e);
        }
    });

    let body = get_stdin();
    let (mut header_str, flag): (String, bool) = get_env_vars();

    let body_value: Value = serde_json::from_str(&body)?;


    let max_id = max_id_actions(&client).await? + 1;
    insert_action_row(&client, max_id, &body, false, true).await?;

    if !body_value["message"].is_null() {
        handle_message(&client, &body_value["message"]).await?;
    } else if !body_value["callback_query"].is_null() {
        callback_message(&client, &body_value["callback_query"]).await?;
    }

    insert_action_row(&client, max_id, &body, true, false).await?;

    Ok(())
}