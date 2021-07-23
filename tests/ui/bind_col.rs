use rs_odbc::c_types::SQL_C_SLONG;
use rs_odbc::conn::C4;
use rs_odbc::env::SQL_OV_ODBC3_80;
use rs_odbc::handle::{
    SQLHDBC, SQLHENV, SQLHSTMT, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
};
use rs_odbc::{SQLAllocHandle, SQLBindCol, SQLDriverConnectA, SQLFreeHandle, SQL_DRIVER_COMPLETE};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

fn get_env_handle() -> SQLHENV<SQL_OV_ODBC3_80> {
    let mut env = MaybeUninit::uninit();
    SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    unsafe { env.assume_init() }
}

fn connect_to_test_db<'env>(
    env: &'env mut SQLHENV<SQL_OV_ODBC3_80>,
) -> SQLHDBC<'env, C4, SQL_OV_ODBC3_80> {
    let mut conn = MaybeUninit::uninit();
    SQLAllocHandle(SQL_HANDLE_DBC, env, &mut conn);

    let conn = unsafe { conn.assume_init() };
    let mut outstrlen = MaybeUninit::uninit();
    let (conn, _) = SQLDriverConnectA(
        conn,
        None,
        "".as_ref(),
        None,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );

    return conn.unwrap();
}

fn main() {
    let mut env = get_env_handle();
    let conn = connect_to_test_db(&mut env);
    let mut stmt = MaybeUninit::uninit();

    SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    let stmt: SQLHSTMT<_> = unsafe { stmt.assume_init() };

    let val = UnsafeCell::new(12i32);
    SQLBindCol(&stmt, 1, SQL_C_SLONG, Some(&val), None);

    drop(val);
    SQLFreeHandle(SQL_HANDLE_STMT, stmt);
}