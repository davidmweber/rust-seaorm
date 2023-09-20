use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr};
use migration::{Migrator, MigratorTrait};
use entity::pickle;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    println!("Hello, world!");
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection =
        Database::connect(db_url).await?;

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");
    assert!(db.ping().await.is_ok());
    println!("Database connectivity is good");
    
    let p = pickle::ActiveModel{
        id: ActiveValue::NotSet,
        flavour: ActiveValue::Set("My first seaorm".to_owned()),
    };
    p.insert(&db).await?;
    
    db.clone().close().await?;
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
    println!("Database is disconnected");
    Ok(())
}
