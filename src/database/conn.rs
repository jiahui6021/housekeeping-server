#[rocket_contrib::database("pg_db")]
pub struct DbConn(diesel::MysqlConnection);

