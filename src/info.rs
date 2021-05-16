use crate::{
    Attr, AttrLen, Ident, OdbcDefined, True, SQLCHAR, SQLSMALLINT, SQLUSMALLINT, SQLWCHAR,
};
use rs_odbc_derive::Ident;
use std::mem::MaybeUninit;

pub trait InfoType<I: Ident>:
    Attr<I> + AttrLen<<Self as Attr<I>>::DefinedBy, <Self as Attr<I>>::NonBinary, SQLSMALLINT>
{
}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_DRIVER_CONNECTIONS;
unsafe impl Attr<SQL_MAX_DRIVER_CONNECTIONS> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl InfoType<SQL_MAX_DRIVER_CONNECTIONS> for SQLUSMALLINT {}
pub use SQL_MAX_DRIVER_CONNECTIONS as SQL_MAXIMUM_DRIVER_CONNECTIONS;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CONCURRENT_ACTIVITIES;

pub use SQL_MAX_CONCURRENT_ACTIVITIES as SQL_MAXIMUM_CONCURRENT_ACTIVITIES;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_FETCH_DIRECTION;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 13)]
#[allow(non_camel_case_types)]
pub struct SQL_SERVER_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 14)]
#[allow(non_camel_case_types)]
pub struct SQL_SEARCH_PATTERN_ESCAPE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 17)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 18)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_VER;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 19)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_TABLES;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 20)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_PROCEDURES;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 23)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_COMMIT_BEHAVIOR;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 25)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_READ_ONLY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 26)]
#[allow(non_camel_case_types)]
pub struct SQL_DEFAULT_TXN_ISOLATION;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 28)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_CASE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 29)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_QUOTE_CHAR;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 30)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMN_NAME_LEN;

pub use SQL_MAX_COLUMN_NAME_LEN as SQL_MAXIMUM_COLUMN_NAME_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 31)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CURSOR_NAME_LEN;

pub use SQL_MAX_CURSOR_NAME_LEN as SQL_MAXIMUM_CURSOR_NAME_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 32)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_SCHEMA_NAME_LEN;

pub use SQL_MAX_SCHEMA_NAME_LEN as SQL_MAXIMUM_SCHEMA_NAME_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 34)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CATALOG_NAME_LEN;

pub use SQL_MAX_CATALOG_NAME_LEN as SQL_MAXIMUM_CATALOG_NAME_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 35)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLE_NAME_LEN;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 43)]
#[allow(non_camel_case_types)]
pub struct SQL_SCROLL_CONCURRENCY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 46)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_CAPABLE;

pub use SQL_TXN_CAPABLE as SQL_TRANSACTION_CAPABLE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 47)]
#[allow(non_camel_case_types)]
pub struct SQL_USER_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 72)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_ISOLATION_OPTION;

pub use SQL_TXN_ISOLATION_OPTION as SQL_TRANSACTION_ISOLATION_OPTION;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 73)]
#[allow(non_camel_case_types)]
pub struct SQL_INTEGRITY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 81)]
#[allow(non_camel_case_types)]
pub struct SQL_GETDATA_EXTENSIONS;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 85)]
#[allow(non_camel_case_types)]
pub struct SQL_NULL_COLLATION;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 86)]
#[allow(non_camel_case_types)]
pub struct SQL_ALTER_TABLE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 90)]
#[allow(non_camel_case_types)]
pub struct SQL_ORDER_BY_COLUMNS_IN_SELECT;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 94)]
#[allow(non_camel_case_types)]
pub struct SQL_SPECIAL_CHARACTERS;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 97)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_GROUP_BY;

pub use SQL_MAX_COLUMNS_IN_GROUP_BY as SQL_MAXIMUM_COLUMNS_IN_GROUP_BY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 98)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_INDEX;

pub use SQL_MAX_COLUMNS_IN_INDEX as SQL_MAXIMUM_COLUMNS_IN_INDEX;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 99)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_ORDER_BY;

pub use SQL_MAX_COLUMNS_IN_ORDER_BY as SQL_MAXIMUM_COLUMNS_IN_ORDER_BY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 100)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_SELECT;

pub use SQL_MAX_COLUMNS_IN_SELECT as SQL_MAXIMUM_COLUMNS_IN_SELECT;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_TABLE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_INDEX_SIZE;

pub use SQL_MAX_INDEX_SIZE as SQL_MAXIMUM_INDEX_SIZE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_ROW_SIZE;

pub use SQL_MAX_ROW_SIZE as SQL_MAXIMUM_ROW_SIZE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_STATEMENT_LEN;

pub use SQL_MAX_STATEMENT_LEN as SQL_MAXIMUM_STATEMENT_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLES_IN_SELECT;

pub use SQL_MAX_TABLES_IN_SELECT as SQL_MAXIMUM_TABLES_IN_SELECT;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 107)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_USER_NAME_LEN;

pub use SQL_MAX_USER_NAME_LEN as SQL_MAXIMUM_USER_NAME_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 115)]
#[allow(non_camel_case_types)]
pub struct SQL_OJ_CAPABILITIES;

pub use SQL_OJ_CAPABILITIES as SQL_OUTER_JOIN_CAPABILITIES;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10000)]
#[allow(non_camel_case_types)]
pub struct SQL_XOPEN_CLI_YEAR;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10001)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_SENSITIVITY;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10002)]
#[allow(non_camel_case_types)]
pub struct SQL_DESCRIBE_PARAMETER;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10003)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10004)]
#[allow(non_camel_case_types)]
pub struct SQL_COLLATION_SEQ;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10005)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_IDENTIFIER_LEN;

pub use SQL_MAX_IDENTIFIER_LEN as SQL_MAXIMUM_IDENTIFIER_LENGTH;

impl<I: Ident> InfoType<I> for [SQLWCHAR]
where
    [SQLCHAR]: InfoType<I, NonBinary = True>,
    [SQLWCHAR]: AttrLen<<Self as Attr<I>>::DefinedBy, <Self as Attr<I>>::NonBinary, SQLSMALLINT>,
{
}

impl<I: Ident> InfoType<I> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: InfoType<I>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}
impl<I: Ident> InfoType<I> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: InfoType<I>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}
impl<I: Ident, T: Ident> InfoType<I> for MaybeUninit<T>
where
    T: InfoType<I>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}
impl<I: Ident> InfoType<I> for &[SQLCHAR] where [SQLCHAR]: InfoType<I> {}
impl<I: Ident> InfoType<I> for &[SQLWCHAR] where [SQLWCHAR]: InfoType<I> {}
