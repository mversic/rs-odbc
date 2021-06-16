use rs_odbc::handle::{
    SQLHDBC, SQLHSTMT, SQLHENV, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE
};
use rs_odbc::info::{
    TxnIsolation, SQL_TXN_ISOLATION_OPTION, SQL_TXN_READ_COMMITTED, SQL_TXN_READ_UNCOMMITTED,
    SQL_TXN_REPEATABLE_READ, SQL_TXN_SERIALIZABLE,
};
use rs_odbc::{
    SQLAllocHandle, SQLDisconnect, SQLDriverConnectA, SQLFreeHandle, SQLGetEnvAttr,
    SQLGetInfoA, SQLGetStmtAttrA, SQLSetEnvAttr, SQLCHAR,
    SQL_DRIVER_COMPLETE, sqlreturn::SQL_SUCCESS,
};
use rs_odbc::stmt::{SQL_ATTR_APP_ROW_DESC, RefSQLHDESC};
use rs_odbc::env::{SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3, SQL_OV_ODBC3_80};
use std::mem::MaybeUninit;

fn get_env_handle() -> SQLHENV {
    let mut env = MaybeUninit::<SQLHENV>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env = unsafe { env.assume_init() };
    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    assert_eq!(SQL_SUCCESS, res);

    return env;
}

fn connect_to_test_db<'env>(env: &'env mut SQLHENV) -> SQLHDBC<'env> {
    let mut conn = MaybeUninit::<SQLHDBC>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_DBC, env, &mut conn);
    assert_eq!(SQL_SUCCESS, res);

    let mut conn = unsafe { conn.assume_init() };
    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstrlen = MaybeUninit::zeroed();
    let res = SQLDriverConnectA(
        &mut conn,
        None,
        conn_string.as_bytes(),
        None,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);

    return conn;
}

#[test]
fn alloc_env() {
    let mut env = MaybeUninit::<SQLHENV>::zeroed();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn set_get_env_attr() {
    let mut env = MaybeUninit::<SQLHENV>::zeroed();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env = unsafe { env.assume_init() };
    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    assert_eq!(SQL_SUCCESS, res);

    let mut val = SQL_OV_ODBC3;
    let res = SQLGetEnvAttr(&env, SQL_ATTR_ODBC_VERSION, Some(&mut val), None);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(SQL_OV_ODBC3_80, val);
}

#[test]
fn db_connect() {
    let mut env = MaybeUninit::<SQLHENV>::zeroed();
    let mut conn = MaybeUninit::<SQLHDBC>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env = unsafe { env.assume_init() };
    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLAllocHandle(SQL_HANDLE_DBC, &mut env, &mut conn);
    assert_eq!(SQL_SUCCESS, res);

    let mut conn = unsafe { conn.assume_init() };

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr: [MaybeUninit<_>; 1024] = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut outstrlen = MaybeUninit::zeroed();
    let res = SQLDriverConnectA(
        &mut conn,
        None,
        conn_string.as_bytes(),
        Some(&mut outstr[..]),
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);

    let outstrlen: usize = unsafe { outstrlen.assume_init() } as usize;
    assert_eq!(34, outstrlen);

    for i in outstrlen..1024 {
        // Make sure type is properly initialized
        outstr[i] = MaybeUninit::zeroed();
    }

    let outstr: [SQLCHAR; 1024] = unsafe { std::mem::transmute(outstr) };
    assert_eq!(
        "DSN=MariaDB;Database=rs_odbc_test;".as_bytes(),
        &outstr[..outstrlen]
    );

    let res = SQLDisconnect(&mut conn);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn get_handle() {
    let mut env = get_env_handle();
    let mut conn = connect_to_test_db(&mut env);
    let mut stmt = MaybeUninit::<SQLHSTMT>::zeroed();
    let mut desc = MaybeUninit::<RefSQLHDESC<_>>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    assert_eq!(SQL_SUCCESS, res);

    let stmt = unsafe { stmt.assume_init() };
    let res = SQLGetStmtAttrA(&stmt, SQL_ATTR_APP_ROW_DESC, Some(&mut desc), None);
    assert_eq!(SQL_SUCCESS, res);

    SQLFreeHandle(SQL_HANDLE_STMT, stmt);
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLDisconnect(&mut conn);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn get_info() {
    let mut env = get_env_handle();
    let mut conn = connect_to_test_db(&mut env);
    let mut txn_isolation = MaybeUninit::<TxnIsolation>::zeroed();

    SQLGetInfoA(&conn, SQL_TXN_ISOLATION_OPTION, Some(&mut txn_isolation), None);

    let txn_isolation = unsafe{ txn_isolation.assume_init() };
    assert_eq!(0x00000001, SQL_TXN_READ_UNCOMMITTED & txn_isolation);
    assert_eq!(0x00000002, SQL_TXN_READ_COMMITTED & txn_isolation);
    assert_eq!(0x00000004, SQL_TXN_REPEATABLE_READ & txn_isolation);
    assert_eq!(0x00000008, SQL_TXN_SERIALIZABLE & txn_isolation);

    let res = SQLDisconnect(&mut conn);
    assert_eq!(SQL_SUCCESS, res);
}
