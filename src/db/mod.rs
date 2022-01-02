use rocket::{Build, Rocket};
pub mod models;

pub async fn init(rocket: Rocket<Build>) -> Rocket<Build> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let pool = sqlx::PgPool::connect(db_url.as_str())
        .await
        .expect("Database failed to connect");

    rocket.manage::<sqlx::PgPool>(pool)
}
