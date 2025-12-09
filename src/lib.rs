#![cfg_attr(not(feature = "std"), no_std)]

pub mod api;
pub mod attr;
pub mod c_types;
pub mod col;
pub mod conn;
pub mod convert;
pub mod desc;
pub mod diag;
pub mod env;
pub mod handle;
pub mod info;
pub mod sql_types;
pub mod sqlreturn;
pub mod stmt;
pub mod str;

// TODO: Export Ident, odbc_type, odbc_bitmask publicly
// so users can import from this lib directly
use core::{ffi::c_void, fmt::Debug};
use rs_odbc_derive::odbc_type;
pub use {
    BulkOperation::*, CompletionType::*, DriverCompletion::*, FreeStmtOption::*, FunctionId::*,
    IdentifierType::*, LockType::*, Operation::*, Reserved::*, Scope::*, Unique::*,
};
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

// TODO: Is this type required?
//type UWORD = u16;
pub type SQLPOINTER = *mut c_void;

// TODO: Won't be required once GATs are implemented because
// implicit handles will be able to use type constructors
// https://github.com/rust-lang/rust/issues/44265
/// Marker trait for binding lifetimes
pub trait Ref<'a> {}

pub trait Def {}
pub enum OdbcDefined {}
pub enum DriverDefined {}
impl Def for OdbcDefined {}
impl Def for DriverDefined {}

const SQL_IS_POINTER: SQLSMALLINT = -4;
const SQL_IS_UINTEGER: SQLSMALLINT = -5;
const SQL_IS_INTEGER: SQLSMALLINT = -6;
const SQL_IS_USMALLINT: SQLSMALLINT = -7;
const SQL_IS_SMALLINT: SQLSMALLINT = -8;

// TODO: Remove these and implement traits directly on
// SQLLEN/SQLULEN like `impl Attr<A> for SQLLEN`
// WARNING: These are not mentioned in ODBC
const SQL_IS_LEN: SQLSMALLINT = SQL_IS_INTEGER;
const SQL_IS_ULEN: SQLSMALLINT = SQL_IS_UINTEGER;

/// Implementing type must have the same representation as SQLPOINTER
pub trait Ident {
    type Type: Copy;
    const IDENTIFIER: Self::Type;
}
impl Ident for SQLSMALLINT {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_SMALLINT;
}
impl Ident for SQLUSMALLINT {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_USMALLINT;
}
impl Ident for SQLINTEGER {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_INTEGER;
}
impl Ident for SQLUINTEGER {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_UINTEGER;
}
impl Ident for SQLLEN {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_LEN;
}
impl Ident for SQLULEN {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_ULEN;
}
impl<T> Ident for [T] {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_POINTER;
}
impl<T> Ident for &T {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_POINTER;
}
impl<T> Ident for &mut T {
    type Type = SQLSMALLINT;

    const IDENTIFIER: Self::Type = SQL_IS_POINTER;
}

// This invariant is here because of the blanket implementation of `AsMutPtr`
/// Implementing types must support all possible values for T because
/// any valid T value can be written to the obtained raw mut pointer
// TODO: Make private?
pub trait Scalar: Copy {}
impl Scalar for SQLSCHAR {}
impl Scalar for SQLCHAR {}
impl Scalar for SQLSMALLINT {}
impl Scalar for SQLUSMALLINT {}
impl Scalar for SQLINTEGER {}
impl Scalar for SQLUINTEGER {}
impl Scalar for SQLLEN {}
impl Scalar for SQLULEN {}

// TODO: Comapare all attribute types that use OdbcBool: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(u32, true), SQL_ATTR_AUTO_IPD(u32, _)
// WARN: SQL_ATTR_METADATA_ID is SQLULEN
#[odbc_type(SQLUINTEGER)]
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
#[expect(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

#[odbc_type(SQLSMALLINT)]
#[expect(non_camel_case_types)]
pub enum IdentifierType {
    SQL_BEST_ROWID = 1,
    SQL_ROWVER = 2,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum BulkOperation {
    SQL_ADD = 4,
    SQL_UPDATE_BY_BOOKMARK = 5,
    SQL_DELETE_BY_BOOKMARK = 6,
    SQL_FETCH_BY_BOOKMARK = 7,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum Operation {
    SQL_POSITION = 0,
    SQL_REFRESH = 1,
    SQL_UPDATE = 2,
    SQL_DELETE = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum LockType {
    SQL_LOCK_NO_CHANGE = 0,
    SQL_LOCK_EXCLUSIVE = 1,
    SQL_LOCK_UNLOCK = 2,
}

#[odbc_type(SQLSMALLINT)]
#[expect(non_camel_case_types)]
pub enum CompletionType {
    SQL_COMMIT = 0,
    SQL_ROLLBACK = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum FreeStmtOption {
    SQL_CLOSE = 0,
    SQL_UNBIND = 2,
    SQL_RESET_PARAMS = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum Reserved {
    SQL_QUICK = 0,
    SQL_ENSURE = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[expect(non_camel_case_types)]
pub enum Unique {
    SQL_INDEX_UNIQUE = 0,
    SQL_INDEX_ALL = 1,
}

#[odbc_type(SQLSMALLINT)]
#[expect(non_camel_case_types)]
pub enum Scope {
    SQL_SCOPE_CURROW = 0,
    SQL_SCOPE_TRANSACTION = 1,
    SQL_SCOPE_SESSION = 2,
}

#[odbc_type(SQLSMALLINT)]
// TODO: Think about splitting for IO
pub struct IOType;
pub const SQL_PARAM_INPUT: IOType = IOType(1);
pub const SQL_PARAM_INPUT_OUTPUT: IOType = IOType(2);
pub const SQL_PARAM_OUTPUT: IOType = IOType(4);

pub const SQL_PARAM_INPUT_OUTPUT_STREAM: IOType = IOType(8);
pub const SQL_PARAM_OUTPUT_STREAM: IOType = IOType(16);

pub const SQL_PARAM_TYPE_UNKNOWN: IOType = IOType(0);
pub const SQL_RESULT_COL: IOType = IOType(3);
pub const SQL_RETURN_VALUE: IOType = IOType(5);

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
#[expect(non_camel_case_types)]
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

//const SQL_LEN_DATA_AT_EXEC_OFFSET: usize = -100;
//pub const fn SQL_LEN_DATA_AT_EXEC<LEN>(length: LEN) {
//    (-length).checked_add(SQL_LEN_DATA_AT_EXEC_OFFSET).expect()
//}

// TODO: Make it const fn
pub(crate) fn slice_len<T, LEN: TryFrom<usize>>(slice: &[T]) -> LEN
where
    LEN::Error: Debug,
{
    const SLICE_LEN_TOO_LARGE_MSG: &str = "Slice len too large";
    LEN::try_from(slice.len()).expect(SLICE_LEN_TOO_LARGE_MSG)
}

// TODO: Instead of implementing traits for every Option<T>, consider making a blanket impl for all T
