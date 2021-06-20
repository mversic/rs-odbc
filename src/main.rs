use rs_odbc::{
    env::OdbcVersion,
    env::{SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3, SQL_OV_ODBC3_80},
    handle::{
        SQLHDBC, SQL_HANDLE_DBC, SQL_HANDLE_DESC, SQL_HANDLE_ENV,
        SQL_HANDLE_STMT, SQL_NULL_HANDLE,
    },
    sqlreturn::SQL_SUCCEEDED,
    SQLAllocHandle, SQLDisconnect, SQLDriverConnectA, SQLFetch, SQLFreeHandle, SQLGetConnectAttrW,
    SQLGetData, SQLGetEnvAttr, SQLGetStmtAttrA, SQLNumResultCols, SQLSetConnectAttrW,
    SQLSetEnvAttr, SQLSetStmtAttrA, SQLTablesA, StrLenOrInd, SQLCHAR, SQLWCHAR,
    SQL_DRIVER_COMPLETE,
};
use std::convert::TryInto;
use std::mem::MaybeUninit;

fn main() {
    let mut env = MaybeUninit::uninit();
    let mut conn = MaybeUninit::uninit();
    let mut stmt = MaybeUninit::uninit();
    let mut desc = MaybeUninit::uninit();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    let mut env = unsafe { env.assume_init() };
    println!("{:?}", res);

    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    println!("{:?}", res);

    let mut val = SQL_OV_ODBC3_80;
    let res = SQLGetEnvAttr(&env, SQL_ATTR_ODBC_VERSION, Some(&mut val), None);
    match val {
        SQL_OV_ODBC3_80 => {
            println!("V3_8")
        }
        SQL_OV_ODBC3 => {
            println!("kita")
        }
        unknown => panic!("unknown value {:?}", unknown),
    }
    let val: OdbcVersion = val.try_into().unwrap();
    println!("{:?}", res);
    println!("{:?}", val);

    let res = SQLAllocHandle(SQL_HANDLE_DBC, &env, &mut conn);
    let mut conn = unsafe { conn.assume_init() };
    println!("{:?}", res);

    let mut conn2 = MaybeUninit::uninit();
    let res = SQLAllocHandle(SQL_HANDLE_DBC, &env, &mut conn2);
    let conn2: SQLHDBC<()> = unsafe { conn2.assume_init() };
    println!("{:?}", res);

    let conn_string = "DSN=MariaDB;Database=rs_odbc_test;";
    let mut outstr: [MaybeUninit<_>; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
    let cap = outstr.len();
    let outstrr: &mut [MaybeUninit<_>] = &mut outstr;
    let mut outstrlen = MaybeUninit::uninit();
    let res = SQLDriverConnectA(
        &mut conn,
        None,
        conn_string.as_bytes(),
        Some(outstrr),
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    let outstr = unsafe {
        String::from_raw_parts(
            std::mem::ManuallyDrop::new(Box::new(outstr))
                .as_mut_ptr()
                .cast(),
            unsafe { outstrlen.assume_init() } as usize,
            cap,
        )
    };
    //let outstr = std::mem::ManuallyDrop::new(outstr);
    println!("KITA: {:?}", &outstr);
    println!("KITA: {:?}", res);

    let res = SQLAllocHandle(SQL_HANDLE_DESC, &conn, &mut desc);
    let mut desc = unsafe { desc.assume_init() };
    println!("{:?}", res);
    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    let mut stmt = unsafe { stmt.assume_init() };
    println!("{:?}", res);

    //let mut col_cnt = MaybeUninit::uninit();
    //SQLTablesA(&mut stmt, "", "", "T", &[TABLE]);
    //SQLNumResultCols(&mut stmt, &mut col_cnt);
    let val = std::cell::UnsafeCell::new(12i32);
    let ref_val = &val;
    let Statement = "SELECT a from test.T";
    let k = rs_odbc::SQLPrepareA(&stmt, Statement.as_bytes());
    println!("PREPARE: {:?}", k);
    //let mut kita = std::cell::UnsafeCell::new(StrLenOrInd::from);
    let k = rs_odbc::SQLBindCol(&stmt, 1, rs_odbc::c_types::SQL_C_SLONG, Some(ref_val), None);
    //let col_cnt = unsafe { col_cnt.assume_init() };
    //println!("col_cnt: {}", col_cnt);
    let mut row = 0;
    let x = unsafe { SQLFetch(&stmt) };
    println!("{:?}", x);
    while SQL_SUCCEEDED(x) {
        println!("Row {}, val: {:?}\n", row, val);

        //for i in 1..col_cnt {
        //    //let mut outstr = SQLCHARString::<SQLLEN>::with_capacity(1024);
        //    //let indicator;

        //    //if SQL_SUCCEEDED(SQLGetData(&mut stmt, i, SQL_C_CHAR, outstr, &indicator)) {
        //    //    ///* Handle null columns */
        //    //    //if (indicator == SQL_NULL_DATA) {
        //    //    //    strcpy(buf, "NULL");
        //    //    //}
        //    //    println!("Column {}: {}\n", i, buf);
        //    //}
        //}

        row += 1;
    }

    let mut slice: [SQLCHAR; 0] = [];
    let mut slice: &mut [SQLCHAR] = &mut slice;
    let mut slice2 = [
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
        MaybeUninit::uninit(),
    ];
    let mut slice3: &mut [_] = &mut slice2;
    let mut len = MaybeUninit::uninit();
    let res = SQLGetConnectAttrW(
        &conn,
        rs_odbc::conn::SQL_ATTR_CURRENT_CATALOG,
        Some(slice3),
        Some(&mut len),
    );
    let slice2: [SQLWCHAR; 5] = unsafe { std::mem::transmute(slice2) };
    let slice2: &[_] = &slice2;
    println!("{:?}", slice2);
    println!("{:?}", unsafe { len.assume_init() });
    let res = SQLSetConnectAttrW(&conn, rs_odbc::conn::SQL_ATTR_CURRENT_CATALOG, slice2);

    let res = SQLSetStmtAttrA(&stmt, rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC, Some(&desc));
    println!("{:?}", res);

    let mut read_desc = MaybeUninit::uninit();
    let res = SQLGetStmtAttrA(
        &stmt,
        rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC,
        Some(&mut read_desc),
        None,
    );
    let read_desc: rs_odbc::stmt::RefSQLHDESC<(), _> = unsafe { read_desc.assume_init() };
    unsafe {
        SQLFetch(&stmt);
    }
    println!("KARA: {:?}", res);

    println!("{:?}", read_desc);
    std::mem::drop(read_desc);
    SQLFreeHandle(SQL_HANDLE_STMT, stmt);
    SQLFreeHandle(SQL_HANDLE_DESC, desc);
    let res = SQLDisconnect(&mut conn);
    println!("{:?}", res);

    SQLFreeHandle(SQL_HANDLE_DBC, conn);
    SQLFreeHandle(SQL_HANDLE_DBC, conn2);
    SQLFreeHandle(SQL_HANDLE_ENV, env);
}
