#[rocket_contrib::database("pg_db")]
pub struct PgConn(diesel::MysqlConnection);

