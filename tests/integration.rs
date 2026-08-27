use core::mem::MaybeUninit;

use rs_odbc::{
    attr::{CP_MATCH, CURRENT_CATALOG},
    conn::C4,
    conn::DriverCompletion::DRIVER_COMPLETE,
    data::CHAR,
    env::{CpMatch, OV_ODBC3, OV_ODBC3_80},
    handle::{HDBC, HENV, OwnedHDBC, OwnedHENV},
    info::MAX_DRIVER_CONNECTIONS,
    sqlreturn::SUCCEEDED,
};

fn get_env_handle() -> OwnedHENV<OV_ODBC3_80> {
    HENV::<OV_ODBC3_80>::alloc_handle().unwrap()
}

fn connect_to_test_db<'env>(env: &'env OwnedHENV<OV_ODBC3_80>) -> OwnedHDBC<'env, OV_ODBC3_80, C4> {
    let conn = HDBC::alloc_handle(env).unwrap();

    let conn_string = "DSN=MariaDB;";
    let outcome = conn.driver_connect(None, conn_string.as_ref(), None, None, DRIVER_COMPLETE);
    let mut conn = outcome.unwrap();

    let db_name = "rs_odbc_test";
    let res = conn.set_attr::<CURRENT_CATALOG, CHAR>(db_name.as_ref());
    assert!(SUCCEEDED(res));

    conn
}

#[test]
fn alloc_env() {
    let _env = HENV::<OV_ODBC3>::alloc_handle().unwrap();
}

#[test]
fn set_get_env_attr() {
    let mut env = HENV::<OV_ODBC3_80>::alloc_handle().unwrap();
    let res = env.set_attr::<CP_MATCH>(CpMatch::CP_RELAXED_MATCH);
    assert!(SUCCEEDED(res));

    let mut val = MaybeUninit::new(CpMatch::CP_STRICT_MATCH);
    let res = env.get_attr::<CP_MATCH>(Some(&mut val), None);
    assert!(SUCCEEDED(res));

    let val: CpMatch = unsafe { val.assume_init() };
    assert_eq!(CpMatch::CP_RELAXED_MATCH, val);
}

#[test]
fn db_connect() {
    let env = HENV::<OV_ODBC3_80>::alloc_handle().unwrap();
    let conn = HDBC::alloc_handle(&env).unwrap();

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr = [MaybeUninit::new(CHAR::default()); 1024];
    let mut outstrlen = MaybeUninit::new(0);
    let outcome = conn.driver_connect(
        None,
        conn_string.as_ref(),
        Some(outstr[..].as_mut()),
        Some(&mut outstrlen),
        DRIVER_COMPLETE,
    );
    let conn = outcome.unwrap();

    let outstrlen: usize = unsafe { outstrlen.assume_init() } as usize;
    assert_eq!(34, outstrlen);

    for slot in outstr.iter_mut().skip(outstrlen) {
        // Make sure type is properly initialized
        *slot = MaybeUninit::new(0);
    }

    let outstr: [CHAR; 1024] = unsafe { core::mem::transmute(outstr) };
    assert_eq!(
        "DSN=MariaDB;Database=rs_odbc_test;".as_bytes(),
        &outstr[..outstrlen]
    );

    let _conn = conn.disconnect().unwrap();
}

// FIXME: Re-enable after APP_ROW_DESC get_attr/set_attr support has a non-owning
// descriptor-handle representation with the correct statement/descriptor lifetimes.
#[cfg(any())]
mod descriptor_tests {
    use super::*;
    use rs_odbc::{
        attr::APP_ROW_DESC,
        desc::ARRAY_SIZE,
        handle::{HDESC, HSTMT},
    };

    #[test]
    fn stmt_get_desc_handle() {
        let env = get_env_handle();
        let conn = connect_to_test_db(&env);
        let mut stmt = HSTMT::alloc_handle(&conn).unwrap();
        let mut desc = MaybeUninit::uninit();

        let res = stmt.get_attr::<APP_ROW_DESC, CHAR>(Some(&mut desc), None);
        assert!(SUCCEEDED(res));

        let desc = unsafe { desc.assume_init() };
        let res = desc.set_desc_field::<ARRAY_SIZE, CHAR>(0, 10);
        assert!(SUCCEEDED(res));

        let mut val = MaybeUninit::new(usize::default());
        let res = desc.get_desc_field::<ARRAY_SIZE, CHAR>(0, Some(&mut val), None);
        assert!(SUCCEEDED(res));
        assert_eq!(10, unsafe { val.assume_init() });

        drop(desc);
        drop(stmt);
        let _conn = conn.disconnect().unwrap();
    }

    #[test]
    fn stmt_set_desc_handle() {
        let env = get_env_handle();
        let conn = connect_to_test_db(&env);
        let desc = HDESC::alloc_handle(&conn).unwrap();
        let mut stmt = HSTMT::alloc_handle(&conn).unwrap();

        let res = stmt.set_attr::<APP_ROW_DESC, CHAR>(Some(&desc));
        assert!(SUCCEEDED(res));

        drop(desc);
        drop(stmt);
        let _conn = conn.disconnect().unwrap();
    }
}

#[test]
fn get_info() {
    let env = get_env_handle();
    let conn = connect_to_test_db(&env);
    let mut max_driver_connections = MaybeUninit::new(u16::default());

    let res =
        conn.get_info::<MAX_DRIVER_CONNECTIONS, CHAR>(Some(&mut max_driver_connections), None);
    assert!(SUCCEEDED(res));
}
