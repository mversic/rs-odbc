use core::mem::MaybeUninit;
use rs_odbc::api::{Allocate, Statement};
use rs_odbc::env::SQL_OV_ODBC3_80;
use rs_odbc::handle::{SQLHDBC, SQLHENV, SQLHSTMT, SQL_NULL_HANDLE};
use rs_odbc::sqlreturn::SQL_SUCCESS;
use rs_odbc::SQL_DRIVER_COMPLETE;

fn main() {
    let statement = "SELECT id, first_name, last_name FROM People ORDER BY 1, 3, 2;";

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
    let hstmt = hstmt.unwrap();

    let res = hstmt.SQLPrepareA(statement.as_ref());
    assert_eq!(res, SQL_SUCCESS);

    // Retrieve number of columns
    let mut num_cols = MaybeUninit::zeroed();
    let res = hstmt.SQLNumResultCols(&mut num_cols);
    assert_eq!(res, SQL_SUCCESS);

    let num_cols = unsafe { num_cols.assume_init() };
    println!("Number of Result Columns {}", num_cols);
}
