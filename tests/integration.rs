use core::mem::MaybeUninit;
use rs_odbc::api::{Allocate, Descriptor, Statement};
use rs_odbc::conn::{C4, SQL_ATTR_CURRENT_CATALOG};
use rs_odbc::desc::SQL_DESC_ARRAY_SIZE;
use rs_odbc::env::{
    CpMatch, SQL_ATTR_CP_MATCH, SQL_CP_RELAXED_MATCH, SQL_CP_STRICT_MATCH, SQL_OV_ODBC3,
    SQL_OV_ODBC3_80,
};
use rs_odbc::handle::{RefSQLHDESC, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQL_NULL_HANDLE};
use rs_odbc::info::{
    TxnIsolation, SQL_TXN_ISOLATION_OPTION, SQL_TXN_READ_COMMITTED, SQL_TXN_READ_UNCOMMITTED,
    SQL_TXN_REPEATABLE_READ, SQL_TXN_SERIALIZABLE,
};
use rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC;
use rs_odbc::{sqlreturn::SQL_SUCCESS, SQLCHAR, SQL_DRIVER_COMPLETE};

fn get_env_handle() -> SQLHENV<SQL_OV_ODBC3_80> {
    let (env, res) = SQLHENV::SQLAllocHandle(&SQL_NULL_HANDLE);

    assert_eq!(SQL_SUCCESS, res);
    env.unwrap()
}

fn connect_to_test_db<'env>(
    env: &'env SQLHENV<SQL_OV_ODBC3_80>,
) -> SQLHDBC<'env, C4, SQL_OV_ODBC3_80> {
    let (conn, res) = SQLHDBC::SQLAllocHandle(env);

    assert_eq!(SQL_SUCCESS, res);
    let conn = conn.unwrap();

    let conn_string = "DSN=MariaDB;";
    let mut outstrlen = MaybeUninit::zeroed();
    let (conn, res) = conn.SQLDriverConnectA(
        None,
        conn_string.as_ref(),
        None,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);
    let conn = conn.unwrap();

    let db_name = "rs_odbc_test";
    let res = conn.SQLSetConnectAttrA(SQL_ATTR_CURRENT_CATALOG, db_name.as_ref());
    assert_eq!(SQL_SUCCESS, res);

    conn
}

#[test]
fn alloc_env() {
    let (_, res) = SQLHENV::<SQL_OV_ODBC3>::SQLAllocHandle(&SQL_NULL_HANDLE);
    assert_eq!(SQL_SUCCESS, res);
}

#[test]
fn set_get_env_attr() {
    let (env, res) = SQLHENV::<SQL_OV_ODBC3_80>::SQLAllocHandle(&SQL_NULL_HANDLE);

    assert_eq!(SQL_SUCCESS, res);
    let mut env = env.unwrap();

    let res = env.SQLSetEnvAttr(SQL_ATTR_CP_MATCH, SQL_CP_RELAXED_MATCH);
    assert_eq!(SQL_SUCCESS, res);

    let mut val = MaybeUninit::new(SQL_CP_STRICT_MATCH);
    let res = env.SQLGetEnvAttr(SQL_ATTR_CP_MATCH, Some(&mut val), None);
    assert_eq!(SQL_SUCCESS, res);

    let val: CpMatch = unsafe { val.assume_init() };
    assert_eq!(SQL_CP_RELAXED_MATCH, val);
}

#[test]
fn db_connect() {
    let (env, res) = SQLHENV::<SQL_OV_ODBC3_80>::SQLAllocHandle(&SQL_NULL_HANDLE);

    assert_eq!(SQL_SUCCESS, res);
    let env = env.unwrap();

    let (conn, res) = SQLHDBC::SQLAllocHandle(&env);

    assert_eq!(SQL_SUCCESS, res);
    let conn = conn.unwrap();

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr: [MaybeUninit<_>; 1024] = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut outstrlen = MaybeUninit::new(0);
    let (conn, res) = conn.SQLDriverConnectA(
        None,
        conn_string.as_ref(),
        Some(outstr[..].as_mut()),
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(SQL_SUCCESS, res);
    let conn = conn.unwrap();

    let outstrlen: usize = unsafe { outstrlen.assume_init() } as usize;
    assert_eq!(34, outstrlen);

    for i in outstrlen..1024 {
        // Make sure type is properly initialized
        outstr[i] = MaybeUninit::new(0);
    }

    let outstr: [SQLCHAR; 1024] = unsafe { core::mem::transmute(outstr) };
    assert_eq!(
        "DSN=MariaDB;Database=rs_odbc_test;".as_bytes(),
        &outstr[..outstrlen]
    );

    let (conn, res) = conn.SQLDisconnect();
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn stmt_get_desc_handle() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);

    let (stmt, res) = SQLHSTMT::SQLAllocHandle(&conn);

    assert_eq!(SQL_SUCCESS, res);
    let stmt = stmt.unwrap();

    let mut desc = MaybeUninit::<RefSQLHDESC<_, _>>::zeroed();
    let res = stmt.SQLGetStmtAttrA(SQL_ATTR_APP_ROW_DESC, Some(&mut desc), None);
    assert_eq!(SQL_SUCCESS, res);

    let val = 10;
    let desc = unsafe { desc.assume_init() };
    let res = desc.SQLSetDescFieldA(0, SQL_DESC_ARRAY_SIZE, Some(val));
    assert_eq!(SQL_SUCCESS, res);

    let mut val = 0;
    let res = desc.SQLGetDescFieldA(0, SQL_DESC_ARRAY_SIZE, Some(&mut val), None);
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(10, val);

    stmt.SQLFreeHandle();
    assert_eq!(SQL_SUCCESS, res);

    let (conn, res) = conn.SQLDisconnect();
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn stmt_set_desc_handle() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);

    let (desc, res) = SQLHDESC::SQLAllocHandle(&conn);

    assert_eq!(SQL_SUCCESS, res);
    let desc = desc.unwrap();

    let (stmt, res) = SQLHSTMT::SQLAllocHandle(&conn);

    assert_eq!(SQL_SUCCESS, res);
    let stmt = stmt.unwrap();

    let res = stmt.SQLSetStmtAttrA(SQL_ATTR_APP_ROW_DESC, Some(&desc));
    assert_eq!(SQL_SUCCESS, res);

    stmt.SQLFreeHandle();
    assert_eq!(SQL_SUCCESS, res);

    desc.SQLFreeHandle();
    assert_eq!(SQL_SUCCESS, res);

    let (conn, res) = conn.SQLDisconnect();
    assert_eq!(SQL_SUCCESS, res);
    assert_eq!(true, conn.is_ok());
}

#[test]
fn get_info() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);
    let mut txn_isolation = MaybeUninit::<TxnIsolation>::zeroed();

    let res = conn.SQLGetInfoA(SQL_TXN_ISOLATION_OPTION, Some(&mut txn_isolation), None);
    assert_eq!(SQL_SUCCESS, res);

    let txn_isolation = unsafe { txn_isolation.assume_init() };
    assert_eq!(0x00000001, SQL_TXN_READ_UNCOMMITTED & txn_isolation);
    assert_eq!(0x00000002, SQL_TXN_READ_COMMITTED & txn_isolation);
    assert_eq!(0x00000004, SQL_TXN_REPEATABLE_READ & txn_isolation);
    assert_eq!(0x00000008, SQL_TXN_SERIALIZABLE & txn_isolation);
}
