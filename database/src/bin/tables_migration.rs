use database::db::Db;

#[tokio::main]
async fn main() {
    println!("Connecting to the database...");
    let db = Db::connect_to_the_pool().await;
    println!("Starting migration of tables...");
    db.migrate_tables().await.unwrap();
    println!("Migration completed.");
}
