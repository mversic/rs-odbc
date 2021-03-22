use rs_odbc::{
    env::OdbcVersion, handle::Allocate, SQLAllocHandle, SQLCHARString, SQLDisconnect,
    SQLDriverConnectA, SQLFetch, SQLGetData, SQLGetEnvAttr, SQLNumResultCols, SQLSetEnvAttr,
    SQLTablesA, SQLHDBC, SQLHENV, SQLHSTMT, SQLSMALLINT, SQL_ATTR_ODBC_VERSION,
    SQL_DRIVER_COMPLETE, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
    SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_SUCCEEDED, TABLE, VIEW, SQLHDESC, SQL_HANDLE_DESC, SQLSetStmtAttrA, SQLGetStmtAttrA,
    SQLFreeHandle
};
use std::convert::TryInto;
use std::mem::MaybeUninit;

fn main() {
    let mut env = MaybeUninit::uninit();
    let mut conn = SQLHDBC::uninit();
    let mut stmt = SQLHSTMT::uninit();
    let stmt2 = SQLHSTMT::uninit();
    let mut desc = SQLHDESC::uninit();

    let res = SQLAllocHandle(SQL_HANDLE_ENV, &mut SQL_NULL_HANDLE, &mut env);
    let mut env = unsafe { env.assume_init() };
    println!("{:?}", res);

    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80);
    println!("{:?}", res);

    let mut val = MaybeUninit::uninit();
    let res = SQLGetEnvAttr(
        &env,
        SQL_ATTR_ODBC_VERSION,
        &mut val,
        &mut MaybeUninit::uninit(),
    );
    match unsafe { val.assume_init() } {
        SQL_OV_ODBC3_80 => {
            println!("V3_8")
        }
        SQL_OV_ODBC3 => {
            println!("kita")
        }
        unknown => panic!("unknown value {:?}", unknown),
    }
    let val: OdbcVersion = unsafe { val.assume_init() }.try_into().unwrap();
    println!("{:?}", res);
    println!("{:?}", val);

    let res = SQLAllocHandle(SQL_HANDLE_DBC, &env, &mut conn);
    let mut conn = unsafe { conn.assume_init() };
    println!("{:?}", res);

    let mut conn2 = SQLHDBC::uninit();
    let res = SQLAllocHandle(SQL_HANDLE_DBC, &env, &mut conn2);
    let conn2 = unsafe { conn2.assume_init() };
    println!("{:?}", res);

    let conn_string = "Driver=mysql;User=kita;Password=moja;Database=test;";
    let mut outstr = SQLCHARString::<SQLSMALLINT>::with_capacity(200);
    let mut outstrlen = MaybeUninit::uninit();
    let res = SQLDriverConnectA(
        &conn,
        None,
        conn_string,
        &mut outstr,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    println!("KITA: {:?}", res);

    println!("{:?}", unsafe { outstr.assume_init() });

    let res = SQLAllocHandle(SQL_HANDLE_DESC, &conn, &mut desc);
    let mut desc = std::rc::Rc::new(unsafe { desc.assume_init() });
    println!("{:?}", res);
    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    let mut stmt = unsafe { stmt.assume_init() };
    println!("{:?}", res);

    //let mut col_cnt = MaybeUninit::uninit();
    //SQLTablesA(&mut stmt, "", "", "T", &[TABLE]);
    //SQLNumResultCols(&mut stmt, &mut col_cnt);
    let val = std::cell::UnsafeCell::new(12);
    let ref_val = &val;
    let Statement = "SELECT a from test.T";
    let k = rs_odbc::SQLPrepareA(&stmt, Statement);
    println!("PREPARE: {:?}", k);
    let mut kita = MaybeUninit::uninit();
    let k = rs_odbc::SQLBindCol(&stmt, 1, rs_odbc::c_types::SQL_C_SLONG, Some(ref_val), &mut kita);
    //let col_cnt = unsafe { col_cnt.assume_init() };
    //println!("col_cnt: {}", col_cnt);
    let mut row = 0;
    let x = SQLFetch(&stmt);
    println!("{:?}", x);
    while SQL_SUCCEEDED(x) {
        println!("Row {}, val: {:?}\n", row, val);

        //for i in 1..col_cnt {
        //    //let mut outstr = SQLCHARString::<SQLLEN>::with_capacity(200);
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

    let res = SQLSetStmtAttrA(&stmt, rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC, &desc);
    println!("{:?}", res);

    let mut read_desc = MaybeUninit::uninit();
    let res = SQLGetStmtAttrA(&stmt, rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC, &mut read_desc, &mut MaybeUninit::uninit());
    let read_desc = unsafe {read_desc.assume_init() };
    SQLFetch(&stmt);
    println!("KARA: {:?}", res);

    std::mem::drop(read_desc);
    SQLFreeHandle(SQL_HANDLE_STMT, stmt);
    SQLFreeHandle(SQL_HANDLE_DESC, std::rc::Rc::try_unwrap(desc).ok().unwrap());
    let res = SQLDisconnect(&mut conn);
    println!("{:?}", res);

    SQLFreeHandle(SQL_HANDLE_DBC, conn);
    SQLFreeHandle(SQL_HANDLE_DBC, conn2);
    SQLFreeHandle(SQL_HANDLE_ENV, env);
}
