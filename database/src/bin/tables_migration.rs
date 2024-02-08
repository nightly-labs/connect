use database::db::Db;

#[tokio::main]
async fn main() {
    let db = Db::connect_to_the_pool().await;
    db.migrate_tables().await.unwrap();
}
