#![cfg_attr(not(feature = "std"), no_std)]

use core::{cell::UnsafeCell, mem::MaybeUninit};
use rs_odbc::{
    api::{Allocate, Statement},
    c_types::SQL_C_SSHORT,
    env::SQL_OV_ODBC3_80,
    handle::{SQLHDBC, SQLHENV, SQLHSTMT, SQL_NULL_HANDLE},
    sqlreturn::{SQL_NO_DATA, SQL_SUCCEEDED, SQL_SUCCESS},
    SQL_DRIVER_COMPLETE,
};

fn main() {
    let (henv, res) = SQLHENV::<SQL_OV_ODBC3_80>::SQLAllocHandle(&SQL_NULL_HANDLE);
    assert_eq!(res, SQL_SUCCESS);
    let henv = henv.unwrap();

    let (hdbc, res) = SQLHDBC::SQLAllocHandle(&henv);
    assert_eq!(res, SQL_SUCCESS);
    let hdbc = hdbc.unwrap();

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstrlen = MaybeUninit::zeroed();
    let (hdbc, res) = hdbc.SQLDriverConnectA(
        None,
        conn_string.as_ref(),
        None,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    assert_eq!(res, SQL_SUCCESS);
    let hdbc = hdbc.unwrap();

    let (hstmt, res) = SQLHSTMT::SQLAllocHandle(&hdbc);
    assert_eq!(res, SQL_SUCCESS);
    let value = UnsafeCell::new(18);
    let hstmt = hstmt.unwrap();

    let res = hstmt.SQLExecDirectA("SELECT num from T".as_ref());
    assert_eq!(res, SQL_SUCCESS);

    let res = hstmt.SQLBindCol(1, SQL_C_SSHORT, Some(&value), None);
    assert_eq!(res, SQL_SUCCESS);

    let res = hstmt.SQLFetch();
    #[cfg(feature = "std")]
    if SQL_SUCCEEDED(res) {
        println!("{:?}", unsafe { *value.get() });
    } else if res != SQL_NO_DATA {
        println!("Failed to fetch result set: {:?}", res);
    }
}
