use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn init_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    return PgConnection::establish(&*db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
}
