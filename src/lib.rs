pub mod api;
pub mod c_types;
pub mod col;
pub mod conn;
pub mod desc;
pub mod diag;
pub mod env;
pub(crate) mod extern_api;
pub mod handle;
pub mod sql_types;
pub mod sqlchar_str;
pub mod sqlreturn;
pub mod stmt;

use std::mem::MaybeUninit;

use rs_odbc_derive::EqSQLSMALLINT;

pub use conn::{
    AccessMode::*, AsyncDbcFunctionsEnable::*, AutoCommit::*, ConnectionDead::*, Trace::*,
    SQL_ASYNC_DBC_ENABLE_DEFAULT, SQL_ATTR_ACCESS_MODE, SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE,
    SQL_ATTR_AUTOCOMMIT, SQL_ATTR_AUTO_IPD, SQL_ATTR_CONNECTION_DEAD, SQL_ATTR_CONNECTION_TIMEOUT,
    SQL_ATTR_CURRENT_CATALOG, SQL_ATTR_LOGIN_TIMEOUT, SQL_ATTR_PACKET_SIZE, SQL_ATTR_TRACE,
    SQL_ATTR_TRACEFILE, SQL_ATTR_TRANSLATE_LIB, SQL_AUTOCOMMIT_DEFAULT, SQL_MODE_DEFAULT,
    SQL_OPT_TRACE_DEFAULT,
};
pub use env::{
    ConnectionPooling::*, CpMatch::*, OdbcVersion::*, SQL_ATTR_CONNECTION_POOLING,
    SQL_ATTR_CP_MATCH, SQL_ATTR_ODBC_VERSION, SQL_CP_DEFAULT, SQL_CP_MATCH_DEFAULT,
};
pub use handle::{
    SQLHANDLE, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQLHWND, SQL_HANDLE_DBC, SQL_HANDLE_DESC,
    SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
}; // TODO: SQLHWND
pub use sqlchar_str::SQLCHARString;
pub use sqlreturn::{
    SQLRETURN, SQL_ERROR, SQL_INVALID_HANDLE, SQL_NEED_DATA, SQL_NO_DATA, SQL_PARAM_DATA_AVAILABLE,
    SQL_STILL_EXECUTING, SQL_SUCCESS, SQL_SUCCESS_WITH_INFO,
};
pub use DriverCompletion::*;
pub use {api::*, c_types::*, sql_types::*};

// TODO: Add support for mingw-x64 on x86 platform

pub type SQLSMALLINT = i16;
pub type SQLUSMALLINT = u16;

pub type SQLINTEGER = i32;
pub type SQLUINTEGER = u32;

pub type SQLREAL = f32;
pub type SQLDOUBLE = f64;
pub type SQLFLOAT = f64;

/// ASCII encoded character
pub type SQLCHAR = u8;
pub type SQLSCHAR = i8;

/// UCS-2 encoded character
pub type SQLWCHAR = u16;

pub type SQLBIGINT = i64;
pub type SQLUBIGINT = u64;

pub type SQLLEN = isize;
pub type SQLULEN = usize;

pub type RETCODE = i16;

#[cfg(target_pointer_width = "32")]
pub type SQLSETPOSIROW = SQLUSMALLINT;
#[cfg(target_pointer_width = "64")]
pub type SQLSETPOSIROW = u64;

pub struct SqlStateA([SQLCHAR; 6]);
pub struct SqlStateW([SQLWCHAR; 6]);

type SQLPOINTER = *mut std::ffi::c_void;

pub const SQL_IS_POINTER: i8 = -4;
pub const SQL_IS_UINTEGER: i8 = -5;
pub const SQL_IS_INTEGER: i8 = -6;
pub const SQL_IS_USMALLINT: i8 = -7;
pub const SQL_IS_SMALLINT: i8 = -8;

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

// TODO: Is it possible to derive this trait?
pub trait AsSQLPOINTER {
    fn as_SQLPOINTER(&self) -> SQLPOINTER;
}
pub trait AsMutSQLPOINTER {
    // TODO: Consider extracting StrLen to a separate trait
    type StrLen;
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER;
}
pub trait Len<AT, LEN> {
    fn len(&self) -> LEN;
}
pub trait AsRawSlice<T, LEN> {
    fn as_raw_slice(&self) -> (*const T, LEN);
}
pub trait AsMutRawSlice<T, LEN> {
    fn as_mut_raw_slice(&mut self) -> (*mut T, LEN);
}

impl<T> AsMutPtr<T> for MaybeUninit<()> {
    fn as_mut_ptr(&mut self) -> *mut T {
        // TODO: If using dangling pointers is ok, this trait can be removed entirely
        // and use MaybeUninit::as_mut_ptr instead as is
        std::ptr::null_mut()
    }
}

impl AsMutSQLPOINTER for MaybeUninit<SQLUINTEGER> {
    // TODO: Either has to be in separate trait or this trait has to be paramterized
    type StrLen = ();

    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}
impl<T, LEN: From<i8>> Len<T, LEN> for MaybeUninit<SQLUINTEGER> {
    fn len(&self) -> LEN {
        LEN::from(SQL_IS_UINTEGER)
    }
}
impl AsRawSlice<SQLCHAR, SQLSMALLINT> for str {
    fn as_raw_slice(&self) -> (*const SQLCHAR, SQLSMALLINT) {
        // TODO: This cast is problematic
        (self.as_ptr(), self.len() as SQLSMALLINT)
    }
}
impl AsSQLPOINTER for str {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        self.as_ptr() as *mut _
    }
}
//impl Len<SQLINTEGER> for str {
//    fn len(&self) -> SQLINTEGER {
//        2 * self.len()
//    }
//}
//const fn SQL_LEN_BINARY_ATTR<LEN>(length: LEN) {
//    const SQL_LEN_BINARY_ATTR_OFFSET: LEN = -100;
//    -length + SQL_LEN_BINARY_ATTR_OFFSET
//}
impl Len<DriverAttr, SQLINTEGER> for [u8] {
    fn len(&self) -> SQLINTEGER {
        // TODO: This is not correct
        self.len() as SQLINTEGER
    }
}

pub enum OdbcAttr {}
pub enum DriverAttr {}

pub trait Identifier {
    type IdentType;

    const IDENTIFIER: Self::IdentType;
}
pub trait GetAttr<C, T> {}
pub trait SetAttr<C, T> {}
pub enum AnsiType {}
pub enum UnicodeType {}

// TODO: Maybe implement something like this?
//impl<const M: usize> AsMutRawSlice<SQLSMALLINT> for [MaybeUninit<SQLCHAR>; M] {
//    type InitializedType = [SQLCHAR; M];
//
//    fn as_mut_raw_SQLCHAR_slice(&mut self) -> (*mut SQLCHAR, SQLSMALLINT) {
//        unimplemented!()
//    }
//    unsafe fn assume_init(self) -> Self::InitializedType {
//        let mut nul_mark_found = false;
//
//        self.iter_mut().for_each(|x| {
//            if nul_mark_found {
//                if *x.as_mut_ptr() == 0 {
//                    nul_mark_found = true;
//                }
//            } else {
//                std::ptr::write(x.as_mut_ptr(), 0);
//            }
//        });
//
//        std::mem::transmute::<_, Self::InitializedType>(self)
//    }
//}

// TODO: Comapare attribute types: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(u32, true), SQL_ATTR_AUTO_IPD(u32, _)
#[allow(non_camel_case_types)]
// TODO: Type equality should be derived such as EqSQLUINTEGER
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcBool {
    SQL_FALSE = 0,
    SQL_TRUE = 1,
}

// TODO
//pub use SQL_COLUMN_SEARCHABLE::SQL_SEARCHABLE as SQL_PRED_SEARCHABLE;
// Special return values for SQLGetData
// SQL_NO_TOTAL = -4,

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

#[derive(EqSQLSMALLINT, Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Nullable {
    SQL_NO_NULLS = 0,
    SQL_NULLABLE = 1,
    // TODO: This value should not be used with SQLSpecialColumns
    SQL_NULLABLE_UNKNOWN = 2,
}

#[allow(non_snake_case)]
pub fn SQL_SUCCEEDED<T: Into<SQLRETURN>>(ret: T) -> bool {
    match ret.into() {
        SQL_SUCCESS | SQL_SUCCESS_WITH_INFO => true,
        _ => false,
    }
}
//pub mod info {
//    pub trait InfoType: Identifier<TypeIdentifier=SQLUSMALLINT> {}
//}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES
