use crate::sql_types::*;

// TODO: Add support for mingw-x64 on x86 platform
use std::convert::TryFrom;
use ucs2;

/// UCS-2 encoded character
#[repr(transparent)]
pub struct SQLWCHAR(u16);
//impl TryFrom<&str> for &mut [SQLWCHAR] {
//    type Error = u16;
//
//    fn try_from(source: &str) -> Result<Self, Self::Error> {
//        unimplemented!()
//    }
//}

pub type SQLSMALLINT = i16;
pub type SQLUSMALLINT = u16;

pub type SQLINTEGER = i32;
pub type SQLUINTEGER = u32;

pub type SQLREAL = f32;
pub type SQLDOUBLE = f64;
pub type SQLFLOAT = f64;

pub type SQLCHAR = u8;
pub type SQLSCHAR = i8;

pub type SQLBIGINT = i64;
pub type SQLUBIGINT = u64;

pub type SQLLEN = isize;
pub type SQLULEN = usize;

pub type RETCODE = i16;
pub type SQLPOINTER = *mut std::ffi::c_void;

#[cfg(target_pointer_width = "32")]
pub type SQLSETPOSIROW = SQLUSMALLINT;
#[cfg(target_pointer_width = "64")]
pub type SQLSETPOSIROW = u64;

// TODO: Fix these
//These types can never be instantiated in Rust code.
pub enum Obj {}
pub enum Env {}
pub enum Dbc {}
pub enum Stmt {}
pub enum Description {}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// void* in unixODBC
pub type SQLHANDLE = *mut Obj;
// SQLHANDLE in c
pub type SQLHENV = *mut Env;
// SQLHANDLE in c
pub type SQLHDESC = *mut Description;
// SQLHANDLE in c
pub type SQLHDBC = *mut Dbc;
// SQLHANDLE in c
pub type SQLHSTMT = *mut Stmt;

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
pub type SQLHWND = SQLPOINTER;

/// ODBC C data types indicate the data type of C buffers used to store data in the
/// application.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/c-data-types?view=sql-server-ver15
#[repr(transparent)]
pub struct CTypeIdentifier(SQLSMALLINT);

impl CTypeIdentifier {
    pub const fn raw_value(&self) -> SQLSMALLINT {
        self.0
    }
}

const SQL_C_LONG: CTypeIdentifier = CTypeIdentifier(SQL_INTEGER.raw_value());
const SQL_C_SHORT: CTypeIdentifier = CTypeIdentifier(SQL_SMALLINT.raw_value());

const SQL_SIGNED_OFFSET: CTypeIdentifier = CTypeIdentifier(-20);
const SQL_UNSIGNED_OFFSET: CTypeIdentifier = CTypeIdentifier(-22);

pub const SQL_C_CHAR: CTypeIdentifier = CTypeIdentifier(SQL_CHAR.raw_value());

pub const SQL_C_WCHAR: CTypeIdentifier = CTypeIdentifier(SQL_WCHAR.raw_value());

pub const SQL_C_SSHORT: CTypeIdentifier = CTypeIdentifier(SQL_C_SHORT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_USHORT: CTypeIdentifier = CTypeIdentifier(SQL_C_SHORT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_SLONG: CTypeIdentifier = CTypeIdentifier(SQL_C_LONG.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_ULONG: CTypeIdentifier = CTypeIdentifier(SQL_C_LONG.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_FLOAT: CTypeIdentifier = CTypeIdentifier(SQL_REAL.raw_value());

pub const SQL_C_DOUBLE: CTypeIdentifier = CTypeIdentifier(SQL_DOUBLE.raw_value());

pub const SQL_C_BIT: CTypeIdentifier = CTypeIdentifier(SQL_BIT.raw_value());

pub const SQL_C_STINYINT: CTypeIdentifier = CTypeIdentifier(SQL_TINYINT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_UTINYINT: CTypeIdentifier = CTypeIdentifier(SQL_TINYINT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_SBIGINT: CTypeIdentifier = CTypeIdentifier(SQL_BIGINT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_UBIGINT: CTypeIdentifier = CTypeIdentifier(SQL_BIGINT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_BINARY: CTypeIdentifier = CTypeIdentifier(SQL_BINARY.raw_value());

pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;
