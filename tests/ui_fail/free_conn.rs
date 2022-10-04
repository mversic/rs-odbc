use rs_odbc::api::Allocate;
use rs_odbc::env::SQL_OV_ODBC3_80;
use rs_odbc::handle::{SQLHDBC, SQLHENV, SQL_NULL_HANDLE};

fn get_env_handle() -> SQLHENV<SQL_OV_ODBC3_80> {
    let (env, _) = SQLHENV::SQLAllocHandle(&SQL_NULL_HANDLE);
    env.unwrap()
}

fn main() {
    let env = get_env_handle();
    let (conn, _) = SQLHDBC::SQLAllocHandle(&env);
    let conn = conn.unwrap();

    env.SQLFreeHandle();
    conn.SQLFreeHandle();
}
