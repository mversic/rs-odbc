use crate::handle::{SQLHENV, SQL_NULL_HANDLE, SQL_HANDLE_ENV};
use crate::sqlreturn::SQL_SUCCESS;
use crate::SQLAllocHandle;
use std::mem::MaybeUninit;

use super::*;

#[test]
fn alloc_env() {
    //let ctx = extern_api::SQLAllocHandle_context();
    let mut env = MaybeUninit::<SQLHENV>::zeroed();
    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    assert_eq!(SQL_SUCCESS, res);
}
