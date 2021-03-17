pub mod api;
pub mod c_types;
pub mod col;
pub mod conn;
pub mod desc;
pub mod diag;
pub mod env;
pub(crate) mod extern_api;
pub mod handle;
pub mod info;
pub mod sql_types;
pub mod sqlchar_str;
pub mod sqlreturn;
pub mod stmt;

use std::cell::{RefCell, UnsafeCell};
use std::mem::MaybeUninit;

pub use conn::{
    SQL_ASYNC_DBC_ENABLE_DEFAULT, SQL_ATTR_ACCESS_MODE, SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE,
    SQL_ATTR_AUTOCOMMIT, SQL_ATTR_AUTO_IPD, SQL_ATTR_CONNECTION_DEAD, SQL_ATTR_CONNECTION_TIMEOUT,
    SQL_ATTR_CURRENT_CATALOG, SQL_ATTR_LOGIN_TIMEOUT, SQL_ATTR_PACKET_SIZE, SQL_ATTR_TRACE,
    SQL_ATTR_TRACEFILE, SQL_ATTR_TRANSLATE_LIB, SQL_AUTOCOMMIT_DEFAULT, SQL_MODE_DEFAULT,
    SQL_OPT_TRACE_DEFAULT,
};
pub use env::{
    SQL_ATTR_CONNECTION_POOLING, SQL_ATTR_CP_MATCH, SQL_ATTR_ODBC_VERSION, SQL_CP_DEFAULT,
    SQL_CP_DRIVER_AWARE, SQL_CP_MATCH_DEFAULT, SQL_CP_OFF, SQL_CP_ONE_PER_DRIVER,
    SQL_CP_ONE_PER_HENV, SQL_CP_RELAXED_MATCH, SQL_CP_STRICT_MATCH, SQL_OV_ODBC3, SQL_OV_ODBC3_80,
};
pub use handle::{
    SQLHANDLE, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQLHWND, SQL_HANDLE_DBC, SQL_HANDLE_DESC,
    SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
}; // TODO: SQLHWND
pub use rs_odbc_derive::odbc_type;
pub use sql_types::*;
pub use sqlchar_str::SQLCHARString;
pub use sqlreturn::{
    SQLRETURN, SQL_ERROR, SQL_INVALID_HANDLE, SQL_NEED_DATA, SQL_NO_DATA, SQL_PARAM_DATA_AVAILABLE,
    SQL_STILL_EXECUTING, SQL_SUCCEEDED, SQL_SUCCESS, SQL_SUCCESS_WITH_INFO,
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
pub use SQLDOUBLE as SQLFLOAT;

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

type UWORD = u16;
type SQLPOINTER = *mut std::ffi::c_void;

const SQL_IS_POINTER: SQLSMALLINT = -4;
const SQL_IS_UINTEGER: SQLSMALLINT = -5;
const SQL_IS_INTEGER: SQLSMALLINT = -6;
const SQL_IS_USMALLINT: SQLSMALLINT = -7;
const SQL_IS_SMALLINT: SQLSMALLINT = -8;

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

pub unsafe trait IntoSQLPOINTER where Self: Copy {
    #[allow(non_snake_case)]
    fn into_SQLPOINTER(self) -> SQLPOINTER;
}
pub unsafe trait AsSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_SQLPOINTER(&self) -> SQLPOINTER;
}
/// If type implementing this trait is a reference allocated inside Driver Manager, then
/// it must be constrained by the given lifetime parameter 'a. Such references are never
/// owned (and therefore never dropped) by the Rust code
pub unsafe trait AsMutSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER;
}
pub unsafe trait Len<AT, LEN: Copy> {
    type StrLen;
    // TODO: consider returning MaybeUninit here. This should be entirely valid
    // It could be difficult to implement because of conflict with odbc_type macro
    fn len(&self) -> LEN;
}
pub unsafe trait AsRawSlice<T, LEN: Copy> {
    fn as_raw_slice(&self) -> (*const T, LEN);
}
pub unsafe trait AsMutRawSlice<T, LEN: Copy> {
    fn as_mut_raw_slice(&mut self) -> (*mut T, LEN);
}

impl<T> AsMutPtr<T> for MaybeUninit<()> {
    fn as_mut_ptr(&mut self) -> *mut T {
        // TODO: If using dangling pointers is ok, this trait can be removed entirely and use MaybeUninit::as_mut_ptr instead as is
        // However, it is SAFER to use null pointers because it is likely that implementation will null-check before dereferencing
        std::ptr::null_mut()
    }
}

unsafe impl AsRawSlice<SQLCHAR, SQLSMALLINT> for str {
    fn as_raw_slice(&self) -> (*const SQLCHAR, SQLSMALLINT) {
        // TODO: This cast is problematic
        (self.as_ptr(), self.len() as SQLSMALLINT)
    }
}
unsafe impl IntoSQLPOINTER for &str {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.as_ptr() as *mut _
    }
}
impl Identifier for SQLSMALLINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_IS_SMALLINT;
}
impl Identifier for SQLUSMALLINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_IS_USMALLINT;
}
impl Identifier for SQLINTEGER {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_IS_INTEGER;
}
impl Identifier for SQLUINTEGER {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_IS_UINTEGER;
}
impl Identifier for SQLLEN {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQLINTEGER::IDENTIFIER;
}
impl Identifier for SQLULEN {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQLUINTEGER::IDENTIFIER;
}
impl Identifier for SQLPOINTER {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_IS_POINTER;
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLSMALLINT
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLSMALLINT::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLUSMALLINT
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLUSMALLINT::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLINTEGER
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLINTEGER::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLUINTEGER
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLUINTEGER::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLLEN
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLLEN::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for SQLULEN
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLULEN::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLSMALLINT
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLSMALLINT::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLUSMALLINT
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLUSMALLINT::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLINTEGER
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLINTEGER::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLUINTEGER
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLUINTEGER::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLLEN
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLLEN::IDENTIFIER)
    }
}
unsafe impl<LEN: Copy> Len<DriverAttr, LEN> for SQLULEN
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(SQLULEN::IDENTIFIER)
    }
}

pub enum OdbcAttr {}
pub enum DriverAttr {}

pub trait Identifier {
    type IdentType: Copy;

    const IDENTIFIER: Self::IdentType;
}
pub unsafe trait ReadAttr<T, C> {}
pub unsafe trait WriteAttr<T, C> {}
pub enum AnsiType {}
pub enum UnicodeType {}

// TODO: Comapare attribute types: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(u32, true), SQL_ATTR_AUTO_IPD(u32, _)
#[odbc_type(SQLUINTEGER)]
#[allow(non_camel_case_types)]
pub struct OdbcBool;
pub const SQL_FALSE: OdbcBool = OdbcBool(0);
pub const SQL_TRUE: OdbcBool = OdbcBool(1);

// TODO
//pub use SQL_COLUMN_SEARCHABLE::SQL_SEARCHABLE as SQL_PRED_SEARCHABLE;
// Special return values for SQLGetData
// SQL_NO_TOTAL = -4,

#[odbc_type(SQLSMALLINT)]
// TODO: See how to name this struct
pub struct NullAllowed;
pub const SQL_NO_NULLS: NullAllowed = NullAllowed(0);
pub const SQL_NULLABLE: NullAllowed = NullAllowed(1);
// TODO: This value should not be used with SQLSpecialColumns
pub const SQL_NULLABLE_UNKNOWN: NullAllowed = NullAllowed(2);

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum IdentifierType {
    SQL_BEST_ROWID = 1,
    SQL_ROWVER = 2,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum BulkOperation {
    SQL_ADD = 4,
    SQL_UPDATE_BY_BOOKMARK = 5,
    SQL_DELETE_BY_BOOKMARK = 6,
    SQL_FETCH_BY_BOOKMARK = 7,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Operation {
    SQL_POSITION = 0,
    SQL_REFRESH = 1,
    SQL_UPDATE = 2,
    SQL_DELETE = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum LockType {
    SQL_LOCK_NO_CHANGE = 0,
    SQL_LOCK_EXCLUSIVE = 1,
    SQL_LOCK_UNLOCK = 2,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum CompletionType {
    SQL_COMMIT = 0,
    SQL_ROLLBACK = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum FreeStmtOption {
    SQL_CLOSE = 0,
    SQL_UNBIND = 2,
    SQL_RESET_PARAMS = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Reserved {
    SQL_QUICK = 0,
    SQL_ENSURE = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Unique {
    SQL_INDEX_UNIQUE = 0,
    SQL_INDEX_ALL = 1,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Scope {
    SQL_SCOPE_CURROW = 0,
    SQL_SCOPE_TRANSACTION = 1,
    SQL_SCOPE_SESSION = 2,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
// TODO: Think about splitting for IO
pub enum ParameterType {
    SQL_PARAM_INPUT = 1,
    SQL_PARAM_INPUT_OUTPUT = 2,
    SQL_PARAM_OUTPUT = 4,

    SQL_PARAM_INPUT_OUTPUT_STREAM = 8,
    SQL_PARAM_OUTPUT_STREAM = 16,

    SQL_PARAM_TYPE_UNKNOWN = 0,
    SQL_RESULT_COL = 3,
    SQL_RETURN_VALUE = 5,
}

//pub const fn SQL_LEN_BINARY_ATTR<LEN>(length: LEN) {
//    const SQL_LEN_BINARY_ATTR_OFFSET: LEN = -100;
//    -length + SQL_LEN_BINARY_ATTR_OFFSET
//}

pub trait GetData<T>: Identifier<IdentType = SQLSMALLINT> {}
impl<T, TT: OutCType<T>> GetData<T> for TT {}

//#[derive(Identifier)]
//#[identifier(SQLSMALLINT, -99)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ARD_TYPE;
//impl<T> GetData<T> for SQL_ARD_TYPE {}
//
//#[derive(Identifier)]
//#[identifier(SQLSMALLINT, -100)]
//#[allow(non_camel_case_types)]
//pub struct SQL_APD_TYPE;
//impl<T> GetData<T> for SQL_APD_TYPE {}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES

// TODO: and what about SQLCHAR vs SQLWCHAR?
pub const SQL_ALL_CATALOGS: &str = "%";
pub const SQL_ALL_SCHEMAS: &str = "%";
pub const SQL_ALL_TABLE_TYPES: &str = "%";

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum FunctionId {
    SQL_API_ODBC3_ALL_FUNCTIONS = 999,
    SQL_API_SQLALLOCCONNECT = 1,
    SQL_API_SQLALLOCENV = 2,
    SQL_API_SQLALLOCHANDLE = 1001,
    SQL_API_SQLALLOCSTMT = 3,
    SQL_API_SQLBINDCOL = 4,
    SQL_API_SQLBINDPARAM = 1002,
    SQL_API_SQLCANCEL = 5,
    SQL_API_SQLCLOSECURSOR = 1003,
    SQL_API_SQLCOLATTRIBUTE = 6,
    SQL_API_SQLCOLUMNS = 40,
    SQL_API_SQLCONNECT = 7,
    SQL_API_SQLCOPYDESC = 1004,
    SQL_API_SQLDATASOURCES = 57,
    SQL_API_SQLDESCRIBECOL = 8,
    SQL_API_SQLDISCONNECT = 9,
    SQL_API_SQLENDTRAN = 1005,
    SQL_API_SQLERROR = 10,
    SQL_API_SQLEXECDIRECT = 11,
    SQL_API_SQLEXECUTE = 12,
    SQL_API_SQLFETCH = 13,
    SQL_API_SQLFETCHSCROLL = 1021,
    SQL_API_SQLFREECONNECT = 14,
    SQL_API_SQLFREEENV = 15,
    SQL_API_SQLFREEHANDLE = 1006,
    SQL_API_SQLFREESTMT = 16,
    SQL_API_SQLGETCONNECTATTR = 1007,
    SQL_API_SQLGETCONNECTOPTION = 42,
    SQL_API_SQLGETCURSORNAME = 17,
    SQL_API_SQLGETDATA = 43,
    SQL_API_SQLGETDESCFIELD = 1008,
    SQL_API_SQLGETDESCREC = 1009,
    SQL_API_SQLGETDIAGFIELD = 1010,
    SQL_API_SQLGETDIAGREC = 1011,
    SQL_API_SQLGETENVATTR = 1012,
    SQL_API_SQLGETFUNCTIONS = 44,
    SQL_API_SQLGETINFO = 45,
    SQL_API_SQLGETSTMTATTR = 1014,
    SQL_API_SQLGETSTMTOPTION = 46,
    SQL_API_SQLGETTYPEINFO = 47,
    SQL_API_SQLNUMRESULTCOLS = 18,
    SQL_API_SQLPARAMDATA = 48,
    SQL_API_SQLPREPARE = 19,
    SQL_API_SQLPUTDATA = 49,
    SQL_API_SQLROWCOUNT = 20,
    SQL_API_SQLSETCONNECTATTR = 1016,
    SQL_API_SQLSETCONNECTOPTION = 50,
    SQL_API_SQLSETCURSORNAME = 21,
    SQL_API_SQLSETDESCFIELD = 1017,
    SQL_API_SQLSETDESCREC = 1018,
    SQL_API_SQLSETENVATTR = 1019,
    SQL_API_SQLSETPARAM = 22,
    SQL_API_SQLSETSTMTATTR = 1020,
    SQL_API_SQLSETSTMTOPTION = 51,
    SQL_API_SQLSPECIALCOLUMNS = 52,
    SQL_API_SQLSTATISTICS = 53,
    SQL_API_SQLTABLES = 54,
    SQL_API_SQLTRANSACT = 23,
    SQL_API_SQLCANCELHANDLE = 1550,
    SQL_API_SQLCOMPLETEASYNC = 1551,
}

//pub const fn SQL_FUNC_EXISTS(pfExists: SQLUSMALLINT, uwAPI: SQLUSMALLINT) -> OdbcBool {
//    if *((pfExists as *const UWORD).offset((uwAPI >> 4) as isize)) & (1 << (uwAPI & 0x000F)) {
//        return SQL_TRUE;
//    }
//
//    SQL_FALSE
//}

// TODO: Please try to use odbc_type derive. The problem is that str doesn't implement Identifier
pub struct TableType(&'static str);
impl TableType {
    pub const fn driver_specific(val: &'static str) -> Self {
        Self(val)
    }
}
pub const TABLE: TableType = TableType("TABLE");
pub const VIEW: TableType = TableType("VIEW");

unsafe impl Len<OdbcAttr, i32> for MaybeUninit<usize> {
    type StrLen = ();
    // TODO: consider returning MaybeUninit here. This should be entirely valid
    // It could be difficult to implement because of conflict with odbc_type macro
    fn len(&self) -> i32 {
        0 as i32
    }
}

unsafe impl AsMutSQLPOINTER for MaybeUninit<usize> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        unimplemented!()
    }
}

unsafe impl<A, T, C> ReadAttr<MaybeUninit<T>, C> for A where A: ReadAttr<T, C> {}
