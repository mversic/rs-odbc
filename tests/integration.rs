use rs_odbc::{SQL_HANDLE_ENV, SQL_NULL_HANDLE, SQLAllocHandle, SQLHENV, SQL_SUCCESS, SQLGetEnvAttr, SQLSetEnvAttr, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80, SQL_OV_ODBC3};
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
