use core::mem::MaybeUninit;

use rs_odbc::{
    attr::QUERY_TIMEOUT,
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
    let stmt = HSTMT::alloc_handle(&conn).unwrap();
    let mut value = MaybeUninit::new(usize::default());

    stmt.get_attr(QUERY_TIMEOUT, Some(&mut value), None);

    drop(stmt);
    drop(value);
}
