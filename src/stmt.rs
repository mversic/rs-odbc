use crate::{AsRawParts, Attribute, GetAttr, SetAttr, SQLINTEGER, SQLPOINTER, OdbcAttr, SQLULEN};
use rs_odbc_derive::{EqSQLULEN, StmtAttr};
use std::mem::MaybeUninit;

pub trait StmtAttr: Attribute<IdentType = SQLINTEGER> {}

// TODO: These seem to be from v2.0
//#[deprecated]
//enum StmtOption {
//    SQL_ROWSET_SIZE = 9,
//    SQL_GET_BOOKMARK = 13,
//}

#[identifier(0)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_QUERY_TIMEOUT {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_QUERY_TIMEOUT {}

#[identifier(1)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_ROWS {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_MAX_ROWS {}

#[identifier(2)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
pub use Noscan::SQL_NOSCAN_OFF as SQL_NOSCAN_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_NOSCAN {}
impl<C> SetAttr<C, Noscan> for SQL_ATTR_NOSCAN {}

#[identifier(3)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_LENGTH {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_MAX_LENGTH {}

#[identifier(6)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
pub use CursorType::SQL_CURSOR_FORWARD_ONLY as SQL_CURSOR_TYPE_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_CURSOR_TYPE {}
impl<C> SetAttr<C, CursorType> for SQL_ATTR_CURSOR_TYPE {}

#[identifier(7)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
pub use Concurrency::SQL_CONCUR_READ_ONLY as SQL_CONCUR_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_CONCURRENCY {}
impl<C> SetAttr<C, Concurrency> for SQL_ATTR_CONCURRENCY {}

#[identifier(8)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_KEYSET_SIZE {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_KEYSET_SIZE {}

#[identifier(10)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_SIMULATE_CURSOR {}
impl<C> SetAttr<C, SimulateCursor> for SQL_ATTR_SIMULATE_CURSOR {}

#[identifier(11)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
pub use RetrieveData::SQL_RD_ON as SQL_RD_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_RETRIEVE_DATA {}
impl<C> SetAttr<C, RetrieveData> for SQL_ATTR_RETRIEVE_DATA {}

#[identifier(12)]
#[derive(StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
pub use UseBookmarks::SQL_UB_OFF as SQL_UB_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_USE_BOOKMARKS {}
impl<C> SetAttr<C, RetrieveData> for SQL_ATTR_USE_BOOKMARKS {}

//#[identifier(15)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ENABLE_AUTO_IPD;
//
//#[identifier(16)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
//
//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[identifier(5)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[identifier(17)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[identifier(18)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(18)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(20)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[identifier(21)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[identifier(22)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[identifier(23)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(24)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(25)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[identifier(26)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[identifier(27)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(29)]
//#[cfg(feature = "v3_8")]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(30)]
//#[cfg(feature = "v4")]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(31)]
//#[cfg(feature = "v4")]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(32)]
//#[cfg(feature = "v4")]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(33)]
//#[cfg(feature = "v4")]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
//
//#[identifier(10010)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_APP_ROW_DESC;
//
//#[identifier(10010)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_APP_PARAM_DESC;
//
//// TODO: Write-only - Cannot be used with SetStmtAttr
//#[identifier(14)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_NUMBER;
//
//// TODO: Write-only - Cannot be used with SetStmtAttr
//#[identifier(10012)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_IMP_ROW_DESC;
//
//// TODO: Write-only - Cannot be used with SetStmtAttr
//#[identifier(10013)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_IMP_PARAM_DESC;
//
//#[identifier(-1)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SCROLLABLE;
//
//#[identifier(-2)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SENSITIVITY;
//
//// TODO: Not found in implementation
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCALLBACK
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCONTEXT
//
//#[identifier(10014)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
//impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_METADATA_ID {}
//
//#[identifier(4)]
//#[derive(StmtAttr)]
//#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
//impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_ASYNC_ENABLE {}

#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Noscan {
    SQL_NOSCAN_OFF = 0,
    SQL_NOSCAN_ON = 1,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for Noscan {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLULEN as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CursorType {
    SQL_CURSOR_FORWARD_ONLY = 0,
    SQL_CURSOR_KEYSET_DRIVEN = 1,
    SQL_CURSOR_DYNAMIC = 2,
    SQL_CURSOR_STATIC = 3,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for CursorType {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLULEN as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Concurrency {
    SQL_CONCUR_READ_ONLY = 1,
    SQL_CONCUR_LOCK = 2,
    SQL_CONCUR_ROWVER = 3,
    SQL_CONCUR_VALUES = 4,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for Concurrency {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLULEN as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum SimulateCursor {
    SQL_SC_NON_UNIQUE = 0,
    SQL_SC_TRY_UNIQUE = 1,
    SQL_SC_UNIQUE = 2,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for SimulateCursor {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLULEN as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum RetrieveData {
    SQL_RD_OFF = 0,
    SQL_RD_ON = 1,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for RetrieveData {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLULEN as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UseBookmarks {
    SQL_UB_OFF = 0,
    SQL_UB_ON = 1,
}

