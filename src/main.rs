use chrono::{DateTime, Utc};
use surrealdb::{engine::local::Mem, Surreal};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Example {
    pub time: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    let client = Surreal::new::<Mem>(()).await.unwrap();
    client.use_ns("example").use_db("example").await.unwrap();
    let content = Example { time: Utc::now() };

    let a: Example = client
        .create(("table", "a"))
        .content(content)
        .await
        .expect("Ok")
        .expect("Some");
    dbg!(&a);

    // Below returns nothing because it thinks time is not a datetime
    let is_datetime = client
        .query("SELECT * FROM table:a WHERE type::is::datetime(time)")
        .await
        .unwrap();
    dbg!(is_datetime);

    // Below returns something because it thinks time is *not* a datetime
    let is_not_datetime = client
        .query("SELECT * FROM table:a WHERE !type::is::datetime(time)")
        .await
        .unwrap();
    dbg!(is_not_datetime);
}
