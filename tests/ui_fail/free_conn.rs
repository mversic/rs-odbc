use rs_odbc::{
    env::OV_ODBC3_80,
    handle::{HDBC, HENV, OwnedHENV},
};

fn get_env_handle() -> OwnedHENV<OV_ODBC3_80> {
    HENV::<OV_ODBC3_80>::alloc_handle().unwrap()
}

fn main() {
    let env = get_env_handle();
    let conn = HDBC::alloc_handle(&env).unwrap();

    drop(env);
    drop(conn);
}
