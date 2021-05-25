use rs_odbc::{SQL_HANDLE_ENV, SQL_NULL_HANDLE, SQLAllocHandle, SQLHENV, SQL_SUCCESS};
use std::mem::MaybeUninit;

#[test]
fn alloc_env() {
    let mut env = MaybeUninit::<SQLHENV>::uninit();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res)
}
