use sqlx::{migrate, query, PgPool, Row};

#[derive(Debug, Clone)]
pub struct Subscription {
    pub email: String,
    pub subscribed_at: i64,
}

async fn create_subscription(
    pool: &PgPool,
    sub: Subscription,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO subscriptions (email, subscribed_at) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(&sub.email)
        .bind(&sub.subscribed_at)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_sub(pool: &PgPool, email: String) -> Result<Subscription, Box<dyn std::error::Error>> {
    let query = "SELECT * FROM subscriptions WHERE email = $1";

    let sub = sqlx::query(query).bind(&email).fetch_one(pool).await?;

    let sub = Subscription {
        email: sub.get("email"),
        subscribed_at: sub.get("subscribed_at"),
    };

    Ok(sub)
}

#[tokio::main]
async fn main() {
    dotenvy::from_filename("infra/.env").unwrap();
    let db_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");

    println!("db_name: {:?}", db_name);
    println!("db_user: {:?}", db_user);
    println!("db_password: {:?}", db_password);

    let pool = PgPool::connect(
        format!(
            "postgres://{}:{}@localhost:5432/{}",
            db_user, db_password, db_name
        )
        .as_str(),
    )
    .await
    .unwrap();

    migrate!("./migrations").run(&pool).await.unwrap();

    // let res = query("SELECT 1+1 as sum").fetch_one(&pool).await.unwrap();
    // let sum: i32 = res.get("sum");
    // println!("sum: {}", sum);

    // let sub = Subscription {
    //     email: "dupa".to_string(),
    //     subscribed_at: 123,
    // };

    // create_subscription(&pool, sub).await.unwrap();

    // println!(
    //     "sub: {:?}",
    //     get_sub(&pool, "dupa".to_string()).await.unwrap()
    // );

    // let rows = sqlx::query("SELECT * FROM users")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();

    // for row in rows {
    //     println!("{:?}", );
    // }
}
