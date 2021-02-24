use crate::handle::{SQLHDESC};
use crate::{ReadAttr, WriteAttr, SQLINTEGER, SQLULEN, SQLHDBC, SQLHSTMT};
use rs_odbc_derive::{odbc_type, Identifier, StmtAttr};
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

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_QUERY_TIMEOUT {}
unsafe impl<C> WriteAttr<C, SQLULEN> for SQL_ATTR_QUERY_TIMEOUT {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_ROWS {}
unsafe impl<C> WriteAttr<C, SQLULEN> for SQL_ATTR_MAX_ROWS {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl<C> ReadAttr<C, MaybeUninit<Noscan>> for SQL_ATTR_NOSCAN {}
unsafe impl<C> WriteAttr<C, Noscan> for SQL_ATTR_NOSCAN {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_MAX_LENGTH {}
unsafe impl<C> WriteAttr<C, SQLULEN> for SQL_ATTR_MAX_LENGTH {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
unsafe impl<C> ReadAttr<C, MaybeUninit<CursorType>> for SQL_ATTR_CURSOR_TYPE {}
unsafe impl<C> WriteAttr<C, CursorType> for SQL_ATTR_CURSOR_TYPE {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
unsafe impl<C> ReadAttr<C, MaybeUninit<Concurrency>> for SQL_ATTR_CONCURRENCY {}
unsafe impl<C> WriteAttr<C, Concurrency> for SQL_ATTR_CONCURRENCY {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_KEYSET_SIZE {}
unsafe impl<C> WriteAttr<C, SQLULEN> for SQL_ATTR_KEYSET_SIZE {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl<C> ReadAttr<C, MaybeUninit<SimulateCursor>> for SQL_ATTR_SIMULATE_CURSOR {}
unsafe impl<C> WriteAttr<C, SimulateCursor> for SQL_ATTR_SIMULATE_CURSOR {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl<C> ReadAttr<C, MaybeUninit<RetrieveData>> for SQL_ATTR_RETRIEVE_DATA {}
unsafe impl<C> WriteAttr<C, RetrieveData> for SQL_ATTR_RETRIEVE_DATA {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl<C> ReadAttr<C, MaybeUninit<UseBookmarks>> for SQL_ATTR_USE_BOOKMARKS {}
unsafe impl<C> WriteAttr<C, UseBookmarks> for SQL_ATTR_USE_BOOKMARKS {}

//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 15)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ENABLE_AUTO_IPD;
//
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 16)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
//
//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 5)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 17)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 20)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 21)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 22)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 23)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 24)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 25)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 26)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 27)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(SQLINTEGER, 29)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v3_8")]
//#[allow(non_camel_case_types)]
// TODO: This type MUST be Rc or similar
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(SQLINTEGER, 30)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(SQLINTEGER, 31)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(SQLINTEGER, 32)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 33)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
//
//// TODO: Write-only - Cannot be used with SetIdentifier, StmtAttr
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 14)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_ROW_NUMBER;

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10010)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_ROW_DESC;
unsafe impl<'a, C> ReadAttr<C, MaybeUninit<&SQLHDESC<'_, 'a, SQLHSTMT<'_, '_, 'a>>>> for SQL_ATTR_APP_ROW_DESC {}
unsafe impl<C> WriteAttr<C, SQLHDESC<'_, '_, SQLHDBC<'_>>> for SQL_ATTR_APP_ROW_DESC {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10011)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl<'a, C> ReadAttr<C, MaybeUninit<&SQLHDESC<'_, 'a, SQLHSTMT<'_, '_, 'a>>>> for SQL_ATTR_APP_PARAM_DESC {}
unsafe impl<C> WriteAttr<C, SQLHDESC<'_, '_, SQLHDBC<'_>>> for SQL_ATTR_APP_PARAM_DESC {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10012)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl<'a, C> ReadAttr<C, MaybeUninit<&SQLHDESC<'_, 'a, SQLHSTMT<'_, '_, 'a>>>> for SQL_ATTR_IMP_ROW_DESC {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10013)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl<'a, C> ReadAttr<C, MaybeUninit<&SQLHDESC<'_, 'a, SQLHSTMT<'_, '_, 'a>>>> for SQL_ATTR_IMP_PARAM_DESC {}

//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, -1)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SCROLLABLE;
//
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, -2)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SENSITIVITY;
//
//// TODO: Not found in implementation
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCALLBACK
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCONTEXT
//
#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10014)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
//impl<C> ReadAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_METADATA_ID {}
//
#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
//impl<C> ReadAttr<C, MaybeUninit<SQLULEN>> for SQL_ATTR_ASYNC_ENABLE {}

#[odbc_type(SQLULEN)]
pub struct Noscan;
pub const SQL_NOSCAN_OFF: Noscan = Noscan(0);
pub const SQL_NOSCAN_ON: Noscan = Noscan(1);
pub use SQL_NOSCAN_OFF as SQL_NOSCAN_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct CursorType;
pub const SQL_CURSOR_FORWARD_ONLY: CursorType = CursorType(0);
pub const SQL_CURSOR_KEYSET_DRIVEN: CursorType = CursorType(1);
pub const SQL_CURSOR_DYNAMIC: CursorType = CursorType(2);
pub const SQL_CURSOR_STATIC: CursorType = CursorType(3);
pub use SQL_CURSOR_FORWARD_ONLY as SQL_CURSOR_TYPE_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct Concurrency;
pub const SQL_CONCUR_READ_ONLY: Concurrency = Concurrency(1);
pub const SQL_CONCUR_LOCK: Concurrency = Concurrency(2);
pub const SQL_CONCUR_ROWVER: Concurrency = Concurrency(3);
pub const SQL_CONCUR_VALUES: Concurrency = Concurrency(4);
pub use SQL_CONCUR_READ_ONLY as SQL_CONCUR_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct SimulateCursor;
pub const SQL_SC_NON_UNIQUE: SimulateCursor = SimulateCursor(0);
pub const SQL_SC_TRY_UNIQUE: SimulateCursor = SimulateCursor(1);
pub const SQL_SC_UNIQUE: SimulateCursor = SimulateCursor(2);

#[odbc_type(SQLULEN)]
pub struct RetrieveData;
pub const SQL_RD_OFF: RetrieveData = RetrieveData(0);
pub const SQL_RD_ON: RetrieveData = RetrieveData(1);
pub use SQL_RD_ON as SQL_RD_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct UseBookmarks;
pub const SQL_UB_OFF: UseBookmarks = UseBookmarks(0);
pub const SQL_UB_ON: UseBookmarks = UseBookmarks(1);
pub use SQL_UB_OFF as SQL_UB_DEFAULT;
