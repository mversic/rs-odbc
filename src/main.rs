use rs_odbc::{
    env::OdbcVersion, handle::Allocate, SQLAllocHandle, SQLCHARString, SQLDisconnect,
    SQLDriverConnectA, SQLFetch, SQLGetData, SQLGetEnvAttr, SQLNumResultCols, SQLSetEnvAttr,
    SQLTablesA, SQLHDBC, SQLHENV, SQLHSTMT, SQLSMALLINT, SQL_ATTR_ODBC_VERSION,
    SQL_DRIVER_COMPLETE, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
    SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_SUCCEEDED, TABLE, VIEW, SQLHDESC, SQL_HANDLE_DESC, SQLSetStmtAttrA, SQLGetStmtAttrA,
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

    let res = SQLSetEnvAttr(&mut env, SQL_ATTR_ODBC_VERSION, &SQL_OV_ODBC3_80);
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
        &mut conn,
        None,
        conn_string,
        &mut outstr,
        &mut outstrlen,
        SQL_DRIVER_COMPLETE,
    );
    println!("KITA: {:?}", res);

    println!("{:?}", unsafe { outstr.assume_init() });

    let res = SQLAllocHandle(SQL_HANDLE_DESC, &conn, &mut desc);
    let mut desc = unsafe { desc.assume_init() };
    println!("{:?}", res);
    let res = SQLAllocHandle(SQL_HANDLE_STMT, &conn, &mut stmt);
    let mut stmt = unsafe { stmt.assume_init() };
    println!("{:?}", res);

    let mut col_cnt = MaybeUninit::uninit();
    SQLTablesA(&mut stmt, "", "", "", &[TABLE, VIEW]);
    SQLNumResultCols(&mut stmt, &mut col_cnt);
    let col_cnt = unsafe { col_cnt.assume_init() };
    let mut row = 0;
    while SQL_SUCCEEDED(SQLFetch(&mut stmt)) {
        println!("Row {}\n", row);

        //for i in 1..col_cnt {
        //    let mut outstr = SQLCHARString::<SQLLEN>::with_capacity(200);
        //    let indicator;

        //    if SQL_SUCCEEDED(SQLGetData(&mut stmt, i, SQL_C_CHAR, outstr, &indicator)) {
        //        /* Handle null columns */
        //        if (indicator == SQL_NULL_DATA) {
        //            strcpy(buf, "NULL");
        //        }
        //        println!("Column {}: {}\n", i, buf);
        //    }
        //}

        row += 1;
    }

    let res = SQLSetStmtAttrA(&mut stmt, rs_odbc::stmt::SQL_ATTR_APP_ROW_DESC, &desc);
    println!("{:?}", res);

    let mut read_desc = MaybeUninit::<&SQLHDESC<SQLHSTMT>>::uninit();
    let res = SQLGetStmtAttrA(&stmt, rs_odbc::stmt::SQL_ATTR_IMP_ROW_DESC, &mut read_desc, &mut MaybeUninit::uninit());
    let read_desc = unsafe {read_desc.assume_init() };
    //println!("{:?}", read_desc);
    println!("KARA: {:?}", res);

    // TODO: This reference is invalid at this point because stmt was dropped
    std::mem::drop(stmt);
    std::mem::drop(read_desc);
    std::mem::drop(desc);
    let res = SQLDisconnect(&mut conn);
    println!("{:?}", res);

    std::mem::drop(conn);
    std::mem::drop(conn2);
    std::mem::drop(env);
}
