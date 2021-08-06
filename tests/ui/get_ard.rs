use rs_odbc::api::{Allocate, Handle};
use rs_odbc::conn::C4;
use rs_odbc::env::SQL_OV_ODBC3_80;
use rs_odbc::handle::{RefSQLHDESC, SQLHDBC, SQLHENV, SQLHSTMT, SQL_NULL_HANDLE};
use rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC;
use rs_odbc::SQL_DRIVER_COMPLETE;
use std::mem::MaybeUninit;

fn get_env_handle() -> SQLHENV<SQL_OV_ODBC3_80> {
    let (env, _) = SQLHENV::SQLAllocHandle(&SQL_NULL_HANDLE);
    env.unwrap()
}

fn connect_to_test_db<'env>(
    env: &'env SQLHENV<SQL_OV_ODBC3_80>,
) -> SQLHDBC<'env, C4, SQL_OV_ODBC3_80> {
    let (conn, _) = SQLHDBC::SQLAllocHandle(env);
    let conn = conn.unwrap();
    let mut outstrlen = MaybeUninit::uninit();

    let (conn, _) =
        conn.SQLDriverConnectA(None, "".as_ref(), None, &mut outstrlen, SQL_DRIVER_COMPLETE);

    conn.unwrap()
}

fn main() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);
    let mut desc = MaybeUninit::uninit();

    let (stmt, _) = SQLHSTMT::SQLAllocHandle(&conn);
    let stmt = stmt.unwrap();

    stmt.SQLGetStmtAttrA(SQL_ATTR_APP_ROW_DESC, Some(&mut desc), None);
    let desc: RefSQLHDESC<_, _> = unsafe { desc.assume_init() };

    stmt.SQLFreeHandle();
    drop(desc);
}
