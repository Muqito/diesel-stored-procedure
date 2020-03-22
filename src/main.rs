use diesel::sql_types::*;
use diesel::{Connection, ConnectionError, MysqlConnection, QueryableByName, RunQueryDsl};
use std::env;

#[derive(Debug, QueryableByName)]
struct User {
    #[sql_type = "Integer"]
    id: i32,
    #[sql_type = "Varchar"]
    name: String,
    #[sql_type = "Timestamp"]
    created_at: chrono::NaiveDateTime,
}
pub fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
}
fn main() {
    dotenv::dotenv().expect("Couldn't unwrap dotenv");
    let connection = establish_connection().expect("Couldn't establish a connection");
    // This doesn't work
    /*    let user: User = diesel::sql_query("CALL get_user(1)")
    .get_result(&connection)
    .expect("Couldn't get result");*/
    let user: User = diesel::sql_query("SELECT * FROM users WHERE id = ?")
        .bind::<diesel::sql_types::Integer, _>(1)
        .get_result(&connection)
        .expect("Couldn't get result");
    dbg!(user);
}
