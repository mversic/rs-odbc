use rs_odbc::{
    SQLAllocHandle, SQLDriverConnectA, SQLDisconnect, SQLGetEnvAttr, SQLSetEnvAttr, SQLCHAR,
    SQLHENV, SQL_ATTR_ODBC_VERSION, SQL_DRIVER_COMPLETE, SQL_HANDLE_DBC, SQL_HANDLE_ENV,
    SQL_NULL_HANDLE, SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_SUCCESS,
};
use std::mem::MaybeUninit;

#[test]
fn alloc_env() {
    let mut env = MaybeUninit::<SQLHENV>::uninit();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn set_get_env_attr() {
    let mut env = MaybeUninit::uninit();
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
    let mut env = MaybeUninit::uninit();
    let mut conn = MaybeUninit::uninit();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env = unsafe { env.assume_init() };
    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLAllocHandle(SQL_HANDLE_DBC, &mut env, &mut conn);
    assert_eq!(SQL_SUCCESS, res);

    let mut conn = unsafe { conn.assume_init() };

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr: [MaybeUninit<SQLCHAR>; 200] = unsafe { MaybeUninit::uninit().assume_init() };
    let outstrr: &mut [MaybeUninit<SQLCHAR>] = &mut outstr;
    let mut outstrlen = MaybeUninit::uninit();
    let res = SQLDriverConnectA(
        &mut conn,
        None,
        conn_string.as_bytes(),
        outstrr,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLDisconnect(&mut conn);
    assert_eq!(SQL_SUCCESS, res);
}
