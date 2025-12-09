use core::{cell::UnsafeCell, mem::MaybeUninit};
use rs_odbc::api::{Allocate, Statement};
use rs_odbc::c_types::{SQL_C_CHAR, SQL_C_SSHORT};
use rs_odbc::env::SQL_OV_ODBC3_80;
use rs_odbc::handle::{SQL_NULL_HANDLE, SQLHDBC, SQLHENV, SQLHSTMT};
use rs_odbc::sqlreturn::{SQL_NO_DATA, SQL_SUCCEEDED, SQL_SUCCESS};
use rs_odbc::{SQL_DRIVER_COMPLETE, SQLCHAR};

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

    // At the moment all bound columns must be wrapped into UnsafeCell.
    // This limitation might be relaxed in one of the future releases
    let id_buffer = UnsafeCell::new(18);
    let name_buffer: &UnsafeCell<[SQLCHAR]> = &UnsafeCell::new([0; 24]);

    let (hstmt, res) = SQLHSTMT::SQLAllocHandle(&hdbc);
    assert_eq!(res, SQL_SUCCESS);
    let hstmt = hstmt.unwrap();

    // It is assumed that table Registry(id smallint, name varchar(20)) already exists
    // in the database, otherwise SQLExecDirect will return SQL_ERROR when called
    let res = hstmt.SQLExecDirectA("SELECT id, name from Registry".as_ref());
    assert_eq!(res, SQL_SUCCESS);

    let res = hstmt.SQLBindCol(1, SQL_C_SSHORT, Some(&id_buffer), None);
    assert_eq!(res, SQL_SUCCESS);
    let res = hstmt.SQLBindCol(2, SQL_C_CHAR, Some(name_buffer.as_ref()), None);
    assert_eq!(res, SQL_SUCCESS);

    let res = loop {
        let res = hstmt.SQLFetch();

        if SQL_SUCCEEDED(res) {
            let name_buffer = unsafe { name_buffer.get().as_ref().expect("Non null") };
            let name: &str = core::str::from_utf8(name_buffer).expect("Valid");
            println!("Id: {}, Name: {}", unsafe { *id_buffer.get() }, name);
        } else {
            break res;
        }
    };

    if res != SQL_NO_DATA {
        println!("Failed to fetch result set: {:?}", res);
    }
}
