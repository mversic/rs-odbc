use rs_odbc::{
    conn::{C4, DriverCompletion::DRIVER_COMPLETE},
    env::OV_ODBC3_80,
    handle::{HDBC, HDESC, HENV, OwnedHDBC, OwnedHENV},
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

    let desc = HDESC::alloc_handle(&conn).unwrap();

    drop(conn);
    drop(desc);
}
