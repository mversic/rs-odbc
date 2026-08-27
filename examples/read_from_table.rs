use core::{cell::UnsafeCell, mem::MaybeUninit};

use rs_odbc::{
    c_type,
    conn::DriverCompletion,
    data::CHAR,
    env::OV_ODBC3_80,
    handle::{HDBC, HENV, HSTMT},
    sqlreturn::{RETURN, SUCCEEDED},
    str::OdbcStr,
};

fn main() {
    let henv = HENV::<OV_ODBC3_80>::alloc_handle().unwrap();

    let hdbc = HDBC::alloc_handle(&henv).unwrap();

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let outcome = hdbc.driver_connect(
        None,
        conn_string.as_ref(),
        None,
        None,
        DriverCompletion::DRIVER_COMPLETE,
    );
    let hdbc = outcome.unwrap();

    // Keep a valid fallback because SQLFetch may leave the target untouched for NULL data.
    let id_buffer = UnsafeCell::new(MaybeUninit::new(0_i16));
    let mut name_storage = [MaybeUninit::new(0); 24];

    let mut hstmt = HSTMT::alloc_handle(&hdbc).unwrap();

    // It is assumed that table Registry(id smallint, name varchar(20)) already exists
    // in the database, otherwise SQLExecDirect will return ERROR when called
    let res = hstmt.exec_direct("SELECT id, name from Registry".as_ref());
    assert!(SUCCEEDED(res));

    let res = hstmt.bind_col::<c_type::SSHORT>(1, Some(&id_buffer), None);
    assert!(SUCCEEDED(res));
    let name_buffer: &mut OdbcStr<MaybeUninit<CHAR>> = name_storage.as_mut().as_mut();
    let name_buffer = UnsafeCell::from_mut(name_buffer);
    // FIXME: Re-enable once co3 can encode a persistent
    // `&UnsafeCell<OdbcStr<MaybeUninit<CHAR>>>` binding.
    // let res = hstmt.bind_col::<c_type::CHAR>(2, Some(name_buffer), None);
    // assert!(SUCCEEDED(res));

    let res = loop {
        let res = hstmt.fetch();

        if SUCCEEDED(res) {
            let name = unsafe { &*name_buffer.get() }
                .iter()
                .map(|character| unsafe { character.assume_init() })
                .take_while(|&character| character != 0)
                .collect::<Vec<_>>();
            let name = core::str::from_utf8(&name).expect("Valid UTF-8");
            println!(
                "Id: {}, Name: {}",
                unsafe { (*id_buffer.get()).assume_init() },
                name
            );
        } else {
            break res;
        }
    };

    if res != RETURN::NO_DATA {
        println!("Failed to fetch result set: {:?}", res);
    }
}
