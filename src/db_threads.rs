use tokio_postgres::{Client, Row};


pub async fn insert_thread_row(
    client: &Client,
    thread_id: i64,
    chat_id: i64,
    active: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let id: i64 = max_id_from_db(client).await? + 1;
    let rows_affected = client
        .execute(
            "INSERT INTO --//-- (id, thread_id, chat_id, active)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (chat_id) DO UPDATE
             SET thread_id = $2, active = $4",
            &[&id, &thread_id, &chat_id, &active],
        )
        .await?;

    Ok(())
}


pub async fn get_thread(
    client: &Client,
    chat_id: i64
) -> Result<Option<Row>,Box<dyn std::error::Error>> {
    let row = client.query_opt(
        "SELECT * FROM --//-- WHERE chat_id = $1",
        &[&chat_id]).await?;
    Ok(row)
}


pub async fn get_chat_id(
    client: &Client,
    thread_id: i64
) -> Result<Option<Row>,Box<dyn std::error::Error>> {
    let row = client.query_opt(
        "SELECT * FROM --//-- WHERE thread_id = $1",
        &[&thread_id]).await?;
    Ok(row)
}


pub async fn max_id_from_db(
    client: &Client
) -> Result<i64,Box<dyn std::error::Error>> {
    let row = client.query_opt(
        "SELECT max(id) FROM --//--",
        &[]).await?;
    if let Some(value) = row.expect("Faile to get max_id").get::<usize, Option<i64>>(0) {
        Ok(value)
    } else {
        Ok(0)
    }
}


pub async fn insert_action_row(
    client: &Client,
    id: i64,
    text: &String,
    status: bool,
    insert: bool
) -> Result<(), Box<dyn std::error::Error>> {
    if insert == true {
        let rows_affected = client
            .execute(
                "INSERT INTO --//-- (id, text)
                          VALUES ($1, $2)",
                &[&id, &text],
            )
            .await?;
    } else {
        let rows_affected = client
            .execute(
                "UPDATE --//--
                          SET status = $2
                          WHERE id = $1",
                &[&id, &status],
            )
            .await?;
    }

    Ok(())
}


pub async fn max_id_actions(
    client: &Client
) -> Result<i64,Box<dyn std::error::Error>> {
    let row = client.query_opt(
        "SELECT max(id) FROM --//--",
        &[]).await?;
    if let Some(value) = row.expect("Faile to get max_id").get::<usize, Option<i64>>(0) {
        Ok(value)
    } else {
        Ok(0)
    }
}