use crate::{
    AnsiType, AsMutRawSlice, ReadAttr, UnicodeType, SQLCHAR, SQLSMALLINT, SQLUSMALLINT, SQLWCHAR,
};
use rs_odbc_derive::{Identifier, InfoType};

pub trait InfoType: crate::Identifier<IdentType = SQLUSMALLINT> {
    type AttrType;
}

#[identifier(SQLUSMALLINT, 0)]
#[derive(Identifier, InfoType)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_DRIVER_CONNECTIONS;

pub use SQL_MAX_DRIVER_CONNECTIONS as SQL_MAXIMUM_DRIVER_CONNECTIONS;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CONCURRENT_ACTIVITIES;

pub use SQL_MAX_CONCURRENT_ACTIVITIES as SQL_MAXIMUM_CONCURRENT_ACTIVITIES;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_NAME;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_FETCH_DIRECTION;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 13)]
#[allow(non_camel_case_types)]
pub struct SQL_SERVER_NAME;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 14)]
#[allow(non_camel_case_types)]
pub struct SQL_SEARCH_PATTERN_ESCAPE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 17)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_NAME;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 18)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_VER;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 19)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_TABLES;
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLSMALLINT>> ReadAttr<AnsiType, T> for SQL_ACCESSIBLE_TABLES {}
unsafe impl<T: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>> ReadAttr<UnicodeType, T> for SQL_ACCESSIBLE_TABLES {}

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 20)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_PROCEDURES;
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLSMALLINT>> ReadAttr<AnsiType, T> for SQL_ACCESSIBLE_PROCEDURES {}
unsafe impl<T: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>> ReadAttr<UnicodeType, T>
    for SQL_ACCESSIBLE_PROCEDURES
{
}

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 23)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_COMMIT_BEHAVIOR;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 25)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_READ_ONLY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 26)]
#[allow(non_camel_case_types)]
pub struct SQL_DEFAULT_TXN_ISOLATION;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 28)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_CASE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 29)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_QUOTE_CHAR;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 30)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMN_NAME_LEN;

pub use SQL_MAX_COLUMN_NAME_LEN as SQL_MAXIMUM_COLUMN_NAME_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 31)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CURSOR_NAME_LEN;

pub use SQL_MAX_CURSOR_NAME_LEN as SQL_MAXIMUM_CURSOR_NAME_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 32)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_SCHEMA_NAME_LEN;

pub use SQL_MAX_SCHEMA_NAME_LEN as SQL_MAXIMUM_SCHEMA_NAME_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 34)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CATALOG_NAME_LEN;

pub use SQL_MAX_CATALOG_NAME_LEN as SQL_MAXIMUM_CATALOG_NAME_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 35)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLE_NAME_LEN;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 43)]
#[allow(non_camel_case_types)]
pub struct SQL_SCROLL_CONCURRENCY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 46)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_CAPABLE;

pub use SQL_TXN_CAPABLE as SQL_TRANSACTION_CAPABLE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 47)]
#[allow(non_camel_case_types)]
pub struct SQL_USER_NAME;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 72)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_ISOLATION_OPTION;

pub use SQL_TXN_ISOLATION_OPTION as SQL_TRANSACTION_ISOLATION_OPTION;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 73)]
#[allow(non_camel_case_types)]
pub struct SQL_INTEGRITY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 81)]
#[allow(non_camel_case_types)]
pub struct SQL_GETDATA_EXTENSIONS;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 85)]
#[allow(non_camel_case_types)]
pub struct SQL_NULL_COLLATION;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 86)]
#[allow(non_camel_case_types)]
pub struct SQL_ALTER_TABLE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 90)]
#[allow(non_camel_case_types)]
pub struct SQL_ORDER_BY_COLUMNS_IN_SELECT;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 94)]
#[allow(non_camel_case_types)]
pub struct SQL_SPECIAL_CHARACTERS;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 97)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_GROUP_BY;

pub use SQL_MAX_COLUMNS_IN_GROUP_BY as SQL_MAXIMUM_COLUMNS_IN_GROUP_BY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 98)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_INDEX;

pub use SQL_MAX_COLUMNS_IN_INDEX as SQL_MAXIMUM_COLUMNS_IN_INDEX;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 99)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_ORDER_BY;

pub use SQL_MAX_COLUMNS_IN_ORDER_BY as SQL_MAXIMUM_COLUMNS_IN_ORDER_BY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 100)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_SELECT;

pub use SQL_MAX_COLUMNS_IN_SELECT as SQL_MAXIMUM_COLUMNS_IN_SELECT;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_TABLE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_INDEX_SIZE;

pub use SQL_MAX_INDEX_SIZE as SQL_MAXIMUM_INDEX_SIZE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_ROW_SIZE;

pub use SQL_MAX_ROW_SIZE as SQL_MAXIMUM_ROW_SIZE;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_STATEMENT_LEN;

pub use SQL_MAX_STATEMENT_LEN as SQL_MAXIMUM_STATEMENT_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLES_IN_SELECT;

pub use SQL_MAX_TABLES_IN_SELECT as SQL_MAXIMUM_TABLES_IN_SELECT;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 107)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_USER_NAME_LEN;

pub use SQL_MAX_USER_NAME_LEN as SQL_MAXIMUM_USER_NAME_LENGTH;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 115)]
#[allow(non_camel_case_types)]
pub struct SQL_OJ_CAPABILITIES;

pub use SQL_OJ_CAPABILITIES as SQL_OUTER_JOIN_CAPABILITIES;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10000)]
#[allow(non_camel_case_types)]
pub struct SQL_XOPEN_CLI_YEAR;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10001)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_SENSITIVITY;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10002)]
#[allow(non_camel_case_types)]
pub struct SQL_DESCRIBE_PARAMETER;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10003)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_NAME;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10004)]
#[allow(non_camel_case_types)]
pub struct SQL_COLLATION_SEQ;

#[derive(Identifier, InfoType)]
#[identifier(SQLUSMALLINT, 10005)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_IDENTIFIER_LEN;

pub use SQL_MAX_IDENTIFIER_LEN as SQL_MAXIMUM_IDENTIFIER_LENGTH;
