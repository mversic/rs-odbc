use core::mem::MaybeUninit;

use rs_odbc::{
    conn::DriverCompletion,
    env::OV_ODBC3_80,
    handle::{HDBC, HENV, HSTMT},
    sqlreturn::SUCCEEDED,
};

fn main() {
    let statement = "SELECT id, first_name, last_name FROM People ORDER BY 1, 3, 2;";

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

    let mut hstmt = HSTMT::alloc_handle(&hdbc).unwrap();

    let res = hstmt.prepare(statement.as_ref());
    assert!(SUCCEEDED(res));

    // Retrieve number of columns
    let mut num_cols = MaybeUninit::new(i16::default());
    let res = hstmt.num_result_cols(&mut num_cols);
    assert!(SUCCEEDED(res));

    println!("Number of Result Columns {}", unsafe {
        num_cols.assume_init()
    });

    drop(hstmt);
    let _hdbc = hdbc.disconnect().unwrap();
}
