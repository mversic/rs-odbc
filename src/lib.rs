pub mod api;
pub mod env;
pub mod desc;
pub mod diag;
pub mod conn;
pub mod stmt;
pub mod col;
pub mod c_types;
pub mod handle;
pub mod sql_types;
pub mod sqlchar_str;
pub mod sqlreturn;

use std::mem::MaybeUninit;

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
    SQL_ATTR_CP_MATCH, SQL_ATTR_ODBC_VERSION, SQL_CP_DEFAULT,
    SQL_CP_MATCH_DEFAULT,
};
pub use handle::{
    SQLHANDLE, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQL_HANDLE_DBC, SQL_HANDLE_DESC,
    SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
}; // TODO: SQLHWND
pub use sqlchar_str::SQLCHARString;
pub use DriverCompletion::*;
pub use {api::*, c_types::*, sql_types::*, sqlreturn::*};

type SQLPOINTER = *mut std::ffi::c_void;

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}
// TODO: Is it possible to derive this trait?
pub trait AsRawParts<T, LEN> {
    // TODO: Is it possible to not have P type?
    fn as_raw_parts(&self) -> (SQLPOINTER, LEN);
}
pub trait AsMutRawSlice<T, LEN> {
    // TODO: Is it possible to not have P type?
    // TODO: Consider extracting StrLen to a separate trait
    type StrLen;
    fn as_mut_raw_slice(&mut self) -> (SQLPOINTER, LEN);
}
pub trait AsRawSQLCHARSlice<LEN> {
    fn as_raw_SQLCHAR_slice(&self) -> (*const SQLCHAR, LEN);
}
pub trait AsRawSQLWCHARSlice<LEN> {
    fn as_raw_SQLWCHAR_slice(&self) -> (*const SQLWCHAR, LEN);
}
pub trait AsMutRawSQLCHARSlice<LEN> {
    fn as_mut_raw_SQLCHAR_slice(&mut self) -> (*mut SQLCHAR, LEN);
}
pub trait AsMutRawSQLWCHARSlice<LEN> {
    fn as_mut_raw_SQLWCHAR_slice(&mut self) -> (*mut SQLWCHAR, LEN);
}

impl AsMutPtr<SQLINTEGER> for MaybeUninit<SQLINTEGER> {
    fn as_mut_ptr(&mut self) -> *mut SQLINTEGER {
        self.as_mut_ptr()
    }
}
impl<T> AsMutPtr<T> for MaybeUninit<()> {
    fn as_mut_ptr(&mut self) -> *mut T {
        // TODO: Is this dangling pointer of?
        // std::ptr::NonNull::dangling().as_ptr()
        std::ptr::null_mut()
    }
}

impl AsMutRawSlice<OdbcAttr, SQLINTEGER> for MaybeUninit<SQLUINTEGER> {
    type StrLen = ();
    fn as_mut_raw_slice(&mut self) -> (SQLPOINTER, SQLINTEGER) {
        (self.as_mut_ptr().cast(), 0)
    }
}
impl AsRawSQLCHARSlice<SQLSMALLINT> for str {
    fn as_raw_SQLCHAR_slice(&self) -> (*const SQLCHAR, SQLSMALLINT) {
        // TODO: This cast is problematic
        (self.as_ptr(), self.len() as SQLSMALLINT)
    }
}

pub trait Attribute {
    type AttrType;
    type IdentType;

    const IDENTIFIER: Self::IdentType;
}
pub enum OdbcAttr {}
pub enum DriverAttr {}
pub trait GetAttr<C, T> {}
pub trait SetAttr<C, T> {}
pub enum AnsiType {}
pub enum UnicodeType {}

// TODO: Maybe implement something like this?
//impl<const M: usize> AsMutRawSQLCHARSlice<SQLSMALLINT> for [MaybeUninit<SQLCHAR>; M] {
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

#[allow(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

//pub mod info {
//    pub trait InfoType: Attribute<TypeIdentifier=SQLUSMALLINT> {}
//}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES
