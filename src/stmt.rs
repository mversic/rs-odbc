use crate::{AsSQLPOINTER, GetAttr, Len, OdbcAttr, SetAttr, SQLINTEGER, SQLPOINTER, SQLULEN};
use rs_odbc_derive::{EqSQLULEN, Identifier, StmtAttr};
use std::mem::MaybeUninit;

pub trait StmtAttr: crate::Identifier<IdentType = SQLINTEGER> {
    type AttrType;
}

// TODO: These seem to be from v2.0
//#[deprecated]
//enum StmtOption {
//    SQL_ROWSET_SIZE = 9,
//    SQL_GET_BOOKMARK = 13,
//}

#[identifier(SQLINTEGER, 0)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_QUERY_TIMEOUT {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_QUERY_TIMEOUT {}

#[identifier(SQLINTEGER, 1)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_ROWS {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_MAX_ROWS {}

#[identifier(SQLINTEGER, 2)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
pub use Noscan::SQL_NOSCAN_OFF as SQL_NOSCAN_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_NOSCAN {}
impl<C> SetAttr<C, Noscan> for SQL_ATTR_NOSCAN {}

#[identifier(SQLINTEGER, 3)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_LENGTH {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_MAX_LENGTH {}

#[identifier(SQLINTEGER, 6)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
pub use CursorType::SQL_CURSOR_FORWARD_ONLY as SQL_CURSOR_TYPE_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_CURSOR_TYPE {}
impl<C> SetAttr<C, CursorType> for SQL_ATTR_CURSOR_TYPE {}

#[identifier(SQLINTEGER, 7)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
pub use Concurrency::SQL_CONCUR_READ_ONLY as SQL_CONCUR_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_CONCURRENCY {}
impl<C> SetAttr<C, Concurrency> for SQL_ATTR_CONCURRENCY {}

#[identifier(SQLINTEGER, 8)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_KEYSET_SIZE {}
impl<C> SetAttr<C, SQLULEN> for SQL_ATTR_KEYSET_SIZE {}

#[identifier(SQLINTEGER, 10)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_SIMULATE_CURSOR {}
impl<C> SetAttr<C, SimulateCursor> for SQL_ATTR_SIMULATE_CURSOR {}

#[identifier(SQLINTEGER, 11)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
pub use RetrieveData::SQL_RD_ON as SQL_RD_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_RETRIEVE_DATA {}
impl<C> SetAttr<C, RetrieveData> for SQL_ATTR_RETRIEVE_DATA {}

#[identifier(SQLINTEGER, 12)]
#[derive(Identifier, StmtAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
pub use UseBookmarks::SQL_UB_OFF as SQL_UB_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_USE_BOOKMARKS {}
impl<C> SetAttr<C, RetrieveData> for SQL_ATTR_USE_BOOKMARKS {}

//#[identifier(SQLINTEGER, 15)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ENABLE_AUTO_IPD;
//
//#[identifier(SQLINTEGER, 16)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
//
//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[identifier(SQLINTEGER, 5)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[identifier(SQLINTEGER, 17)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[identifier(SQLINTEGER, 18)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(SQLINTEGER, 18)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(SQLINTEGER, 20)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[identifier(SQLINTEGER, 21)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[identifier(SQLINTEGER, 22)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[identifier(SQLINTEGER, 23)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(SQLINTEGER, 24)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[identifier(SQLINTEGER, 25)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[identifier(SQLINTEGER, 26)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[identifier(SQLINTEGER, 27)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(SQLINTEGER, 29)]
//#[cfg(feature = "v3_8")]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(SQLINTEGER, 30)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(SQLINTEGER, 31)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(SQLINTEGER, 32)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 33)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 10010)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_APP_ROW_DESC;
//
//#[identifier(SQLINTEGER, 10010)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_APP_PARAM_DESC;
//
//// TODO: Write-only - Cannot be used with SetIdentifier, StmtAttr
//#[identifier(SQLINTEGER, 14)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_ROW_NUMBER;
//
//// TODO: Write-only - Cannot be used with SetIdentifier, StmtAttr
//#[identifier(SQLINTEGER, 10012)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_IMP_ROW_DESC;
//
//// TODO: Write-only - Cannot be used with SetIdentifier, StmtAttr
//#[identifier(SQLINTEGER, 10013)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_IMP_PARAM_DESC;
//
//#[identifier(SQLINTEGER, -1)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SCROLLABLE;
//
//#[identifier(SQLINTEGER, -2)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SENSITIVITY;
//
//// TODO: Not found in implementation
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCALLBACK
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCONTEXT
//
//#[identifier(SQLINTEGER, 10014)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
//impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_METADATA_ID {}
//
//#[identifier(SQLINTEGER, 4)]
//#[derive(Identifier, StmtAttr)]
//#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
//impl<C> GetAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_ASYNC_ENABLE {}

#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Noscan {
    SQL_NOSCAN_OFF = 0,
    SQL_NOSCAN_ON = 1,
}
impl AsSQLPOINTER for Noscan {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for Noscan {
    fn len(&self) -> LEN {
        Default::default()
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
impl AsSQLPOINTER for CursorType {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for CursorType {
    fn len(&self) -> LEN {
        Default::default()
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
impl AsSQLPOINTER for Concurrency {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for Concurrency {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum SimulateCursor {
    SQL_SC_NON_UNIQUE = 0,
    SQL_SC_TRY_UNIQUE = 1,
    SQL_SC_UNIQUE = 2,
}
impl AsSQLPOINTER for SimulateCursor {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for SimulateCursor {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum RetrieveData {
    SQL_RD_OFF = 0,
    SQL_RD_ON = 1,
}
impl AsSQLPOINTER for RetrieveData {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for RetrieveData {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLULEN, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UseBookmarks {
    SQL_UB_OFF = 0,
    SQL_UB_ON = 1,
}

impl AsSQLPOINTER for UseBookmarks {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLULEN as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for UseBookmarks {
    fn len(&self) -> LEN {
        Default::default()
    }
}
