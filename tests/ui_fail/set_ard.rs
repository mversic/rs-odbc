use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use rs_odbc::{
    c_type,
    conn::C4,
    conn::DriverCompletion::DRIVER_COMPLETE,
    env::OV_ODBC3_80,
    handle::{HDBC, HENV, HSTMT, OwnedHDBC, OwnedHENV},
};

fn get_env_handle() -> OwnedHENV<OV_ODBC3_80> {
    HENV::<OV_ODBC3_80>::alloc_handle().unwrap()
}

fn connect_to_test_db<'env>(env: &'env OwnedHENV<OV_ODBC3_80>) -> OwnedHDBC<'env, OV_ODBC3_80, C4> {
    HDBC::alloc_handle(env)
        .unwrap()
        .driver_connect(None, "".as_ref(), None, None, DRIVER_COMPLETE)
        .unwrap()
}

fn main() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);

    let mut stmt = HSTMT::alloc_handle(&conn).unwrap();
    let value = UnsafeCell::new(MaybeUninit::new(12i32));

    stmt.bind_col::<c_type::SLONG>(1, Some(&value), None);

    drop(value);
    drop(stmt);
}
