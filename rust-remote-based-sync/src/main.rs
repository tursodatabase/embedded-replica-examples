use std::time::Instant;

use libsql::{Database, Value};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let db_path = std::env::var("DB_PATH").unwrap();
    let auth_token = std::env::var("AUTH_TOKEN").unwrap();
    let sync_url = std::env::var("SYNC_URL").unwrap();

    let db = Database::open_with_remote_sync("hello.db", sync_url, auth_token)
        .await
        .unwrap();
    db.sync().await.unwrap();

    let conn = db.connect().unwrap();

    conn.execute("CREATE TABLE foo4 (x TEXT)", ())
        .await
        .unwrap();

    for _ in 0..10 {
        let now = Instant::now();

        let mut stmt = conn
            .prepare(
                "SELECT u.name, k.expired FROM users u JOIN keycards as k ON u.user_id = k.user_id",
            )
            .await
            .unwrap();

        let mut rows = stmt.query(()).await.unwrap();

        let delta = now.elapsed().as_micros();

        while let Some(row) = rows.next().unwrap() {
            let name = row.get_str(0).unwrap();
            let Value::Integer(expired) = row.get_value(1).unwrap() else {
                panic!()
            };

            let expired = if expired == 0 { "valid" } else { "expired" };

            println!(
                "The keycard for user {} is {}. Query took {}us",
                name, expired, delta
            );
        }
    }
}
