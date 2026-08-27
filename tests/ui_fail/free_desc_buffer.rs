use core::{cell::UnsafeCell, mem::MaybeUninit};

use rs_odbc::{
    conn::{C4, DriverCompletion::DRIVER_COMPLETE},
    desc::OCTET_LENGTH_PTR,
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
    let mut desc = HDESC::alloc_handle(&conn).unwrap();
    let octet_length = UnsafeCell::new(MaybeUninit::new(isize::default()));

    desc.set_desc_field::<OCTET_LENGTH_PTR, u8>(1, Some(&octet_length));

    drop(octet_length);
    drop(desc);
}
