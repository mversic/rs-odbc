use rs_odbc::env::{
    CpMatch, SQL_ATTR_CP_MATCH, SQL_CP_RELAXED_MATCH, SQL_OV_ODBC3, SQL_OV_ODBC3_80,
};
use rs_odbc::handle::{
    C4, SQLHDBC, SQLHENV, SQLHSTMT, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_HANDLE_STMT,
    SQL_NULL_HANDLE, SQLHDESC, SQL_HANDLE_DESC
};
use rs_odbc::info::{
    TxnIsolation, SQL_TXN_ISOLATION_OPTION, SQL_TXN_READ_COMMITTED, SQL_TXN_READ_UNCOMMITTED,
    SQL_TXN_REPEATABLE_READ, SQL_TXN_SERIALIZABLE,
};
use rs_odbc::stmt::{RefSQLHDESC, SQL_ATTR_APP_ROW_DESC};
use rs_odbc::desc::SQL_DESC_ARRAY_SIZE;
use rs_odbc::{
    sqlreturn::SQL_SUCCESS, SQLAllocHandle, SQLDisconnect, SQLDriverConnectA, SQLFreeHandle,
    SQLGetEnvAttr, SQLGetInfoA, SQLGetStmtAttrA, SQLSetEnvAttr, SQLCHAR, SQL_DRIVER_COMPLETE, SQLSetDescFieldA, SQLGetDescFieldA,
    SQLSetStmtAttrA
};
use std::mem::MaybeUninit;

fn get_env_handle() -> SQLHENV<SQL_OV_ODBC3_80> {
    let mut env = MaybeUninit::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    unsafe { env.assume_init() }
}

fn connect_to_test_db<'env>(
    env: &'env mut SQLHENV<SQL_OV_ODBC3_80>,
) -> SQLHDBC<'env, C4, SQL_OV_ODBC3_80> {
    let mut conn = MaybeUninit::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_DBC, env, &mut conn);
    assert_eq!(SQL_SUCCESS, res);

    let conn = unsafe { conn.assume_init() };
    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstrlen = MaybeUninit::zeroed();
    let (conn, res) = SQLDriverConnectA(
        conn,
        None,
        conn_string.as_bytes(),
        None,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);

    return conn.unwrap();
}

#[test]
fn alloc_env() {
    let mut env = MaybeUninit::<SQLHENV<SQL_OV_ODBC3>>::zeroed();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn set_get_env_attr() {
    let mut env = MaybeUninit::zeroed();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env: SQLHENV<SQL_OV_ODBC3_80> = unsafe { env.assume_init() };
    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_CP_MATCH, SQL_CP_RELAXED_MATCH);
    assert_eq!(SQL_SUCCESS, res);

    let mut val = MaybeUninit::zeroed();
    let res = SQLGetEnvAttr(&env, SQL_ATTR_CP_MATCH, Some(&mut val), None);
    assert_eq!(SQL_SUCCESS, res);

    let val: CpMatch = unsafe { val.assume_init() };
    assert_eq!(SQL_CP_RELAXED_MATCH, val);
}

#[test]
fn db_connect() {
    let mut env = MaybeUninit::zeroed();
    let mut conn = MaybeUninit::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);

    let mut env: SQLHENV<SQL_OV_ODBC3_80> = unsafe { env.assume_init() };
    let res = SQLAllocHandle(SQL_HANDLE_DBC, &mut env, &mut conn);
    assert_eq!(SQL_SUCCESS, res);

    let conn = unsafe { conn.assume_init() };

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr: [MaybeUninit<_>; 1024] = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut outstrlen = MaybeUninit::zeroed();
    let (conn, res) = SQLDriverConnectA(
        conn,
        None,
        conn_string.as_bytes(),
        Some(&mut outstr[..]),
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);
    let conn = conn.unwrap();

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

    let (conn, res) = SQLDisconnect(conn);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn stmt_get_desc_handle() {
    let mut env = get_env_handle();
    let conn = connect_to_test_db(&mut env);
    let mut stmt = MaybeUninit::<SQLHSTMT<_>>::zeroed();
    let mut desc = MaybeUninit::<RefSQLHDESC<_, _>>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    assert_eq!(SQL_SUCCESS, res);

    let stmt = unsafe { stmt.assume_init() };
    let res = SQLGetStmtAttrA(&stmt, SQL_ATTR_APP_ROW_DESC, Some(&mut desc), None);
    assert_eq!(SQL_SUCCESS, res);

    let val = 10;
    let mut desc = unsafe { desc.assume_init() };
    let res = SQLSetDescFieldA(&desc, 0, SQL_DESC_ARRAY_SIZE, Some(val));
    assert_eq!(SQL_SUCCESS, res);

    let mut val = 0;
    let res = SQLGetDescFieldA(&mut desc, 0, SQL_DESC_ARRAY_SIZE, Some(&mut val), None);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(10, val);

    let res = SQLFreeHandle(SQL_HANDLE_STMT, stmt);
    assert_eq!(SQL_SUCCESS, res);

    let (conn, res) = SQLDisconnect(conn);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn stmt_set_desc_handle() {
    let mut env = get_env_handle();
    let conn = connect_to_test_db(&mut env);
    let mut stmt = MaybeUninit::<SQLHSTMT<_>>::zeroed();
    let mut desc = MaybeUninit::<SQLHDESC<_, _>>::zeroed();

    let res = SQLAllocHandle(SQL_HANDLE_DESC, &conn, &mut desc);
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    assert_eq!(SQL_SUCCESS, res);

    let desc = unsafe { desc.assume_init() };
    let stmt = unsafe { stmt.assume_init() };
    let res = SQLSetStmtAttrA(&stmt, SQL_ATTR_APP_ROW_DESC, Some(&desc));
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLFreeHandle(SQL_HANDLE_STMT, stmt);
    assert_eq!(SQL_SUCCESS, res);

    let res = SQLFreeHandle(SQL_HANDLE_DESC, desc);
    assert_eq!(SQL_SUCCESS, res);

    let (conn, res) = SQLDisconnect(conn);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn get_info() {
    let mut env = get_env_handle();
    let conn = connect_to_test_db(&mut env);
    let mut txn_isolation = MaybeUninit::<TxnIsolation>::zeroed();

    let res = SQLGetInfoA(
        &conn,
        SQL_TXN_ISOLATION_OPTION,
        Some(&mut txn_isolation),
        None,
    );
    assert_eq!(SQL_SUCCESS, res);

    let txn_isolation = unsafe { txn_isolation.assume_init() };
    assert_eq!(0x00000001, SQL_TXN_READ_UNCOMMITTED & txn_isolation);
    assert_eq!(0x00000002, SQL_TXN_READ_COMMITTED & txn_isolation);
    assert_eq!(0x00000004, SQL_TXN_REPEATABLE_READ & txn_isolation);
    assert_eq!(0x00000008, SQL_TXN_SERIALIZABLE & txn_isolation);
}
