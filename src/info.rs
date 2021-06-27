use crate::env::{OdbcVersion, SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::{
    Attr, AttrGet, AttrLen, Ident, OdbcDefined, True, SQLCHAR, SQLSMALLINT, SQLUINTEGER,
    SQLUSMALLINT, SQLWCHAR,
};
use rs_odbc_derive::{odbc_bitmask, odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait InfoType<I: Ident, V: OdbcVersion>:
    Attr<I> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>
{
}

// Implement InfoType for all versions of info type attributes
impl<I: Ident, T: Ident> InfoType<I, SQL_OV_ODBC3_80> for T where T: InfoType<I, SQL_OV_ODBC3> {}
impl<I: Ident, T: Ident> InfoType<I, SQL_OV_ODBC4> for T where T: InfoType<I, SQL_OV_ODBC3_80> {}
impl<I: Ident> InfoType<I, SQL_OV_ODBC3_80> for [SQLCHAR] where [SQLCHAR]: InfoType<I, SQL_OV_ODBC3> {}
impl<I: Ident> InfoType<I, SQL_OV_ODBC4> for [SQLCHAR] where [SQLCHAR]: InfoType<I, SQL_OV_ODBC3_80> {}

// Implement InfoType for unicode character info type attributes
impl<V: OdbcVersion, I: Ident> InfoType<I, V> for [SQLWCHAR] where
    [SQLCHAR]: InfoType<I, V, NonBinary = True>
{
}

// Implement InfoType for uninitialized info type attributes
impl<V: OdbcVersion, I: Ident, T: Ident> InfoType<I, V> for MaybeUninit<T>
where
    T: InfoType<I, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}
impl<V: OdbcVersion, I: Ident> InfoType<I, V> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: InfoType<I, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}
impl<V: OdbcVersion, I: Ident> InfoType<I, V> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: InfoType<I, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// These aliases include extensions of abbreviations
pub use SQL_MAX_CATALOG_NAME_LEN as SQL_MAXIMUM_CATALOG_NAME_LENGTH;
pub use SQL_MAX_COLUMNS_IN_GROUP_BY as SQL_MAXIMUM_COLUMNS_IN_GROUP_BY;
pub use SQL_MAX_COLUMNS_IN_ORDER_BY as SQL_MAXIMUM_COLUMNS_IN_ORDER_BY;
pub use SQL_MAX_COLUMNS_IN_SELECT as SQL_MAXIMUM_COLUMNS_IN_SELECT;
pub use SQL_MAX_COLUMNS_IN_TABLE as SQL_MAXIMUM_COLUMNS_IN_TABLE;
pub use SQL_MAX_COLUMN_NAME_LEN as SQL_MAXIMUM_COLUMN_NAME_LENGTH;
pub use SQL_MAX_CONCURRENT_ACTIVITIES as SQL_MAXIMUM_CONCURRENT_ACTIVITIES;
pub use SQL_MAX_CURSOR_NAME_LEN as SQL_MAXIMUM_CURSOR_NAME_LENGTH;
pub use SQL_MAX_DRIVER_CONNECTIONS as SQL_MAXIMUM_DRIVER_CONNECTIONS;
pub use SQL_MAX_IDENTIFIER_LEN as SQL_MAXIMUM_IDENTIFIER_LENGTH;
pub use SQL_MAX_SCHEMA_NAME_LEN as SQL_MAXIMUM_SCHEMA_NAME_LENGTH;
pub use SQL_MAX_STATEMENT_LEN as SQL_MAXIMUM_STATEMENT_LENGTH;
pub use SQL_MAX_TABLES_IN_SELECT as SQL_MAXIMUM_TABLES_IN_SELECT;
pub use SQL_MAX_TABLE_NAME_LEN as SQL_MAXIMUM_TABLE_NAME_LENGTH;
pub use SQL_MAX_USER_NAME_LEN as SQL_MAXIMUM_USER_NAME_LENGTH;
pub use SQL_MULT_RESULT_SETS as SQL_MULTIPLE_RESULT_SETS;
pub use SQL_OJ_CAPABILITIES as SQL_OUTER_JOIN_CAPABILITIES;
pub use SQL_TXN_CAPABLE as SQL_TRANSACTION_CAPABLE;
pub use SQL_TXN_ISOLATION_OPTION as SQL_TRANSACTION_ISOLATION_OPTION;

// TODO: Not mentioned in the specification, only implementation
pub use SQL_MAX_COLUMNS_IN_INDEX as SQL_MAXIMUM_COLUMNS_IN_INDEX;
pub use SQL_MAX_INDEX_SIZE as SQL_MAXIMUM_INDEX_SIZE;
pub use SQL_MAX_ROW_SIZE as SQL_MAXIMUM_ROW_SIZE;

// TODO: Try to categorize all of the following items just like all the other attributes

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 171)]
#[allow(non_camel_case_types)]
pub struct SQL_DM_VER;
impl InfoType<SQL_DM_VER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DM_VER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DM_VER> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10000)]
#[allow(non_camel_case_types)]
pub struct SQL_XOPEN_CLI_YEAR;
impl InfoType<SQL_XOPEN_CLI_YEAR, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_XOPEN_CLI_YEAR> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_XOPEN_CLI_YEAR> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 134)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_VIEW;
impl InfoType<SQL_CREATE_VIEW, SQL_OV_ODBC3> for CreateView {}
unsafe impl Attr<SQL_CREATE_VIEW> for CreateView {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_VIEW> for CreateView {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 155)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_DATETIME_FUNCTIONS;
impl InfoType<SQL_SQL92_DATETIME_FUNCTIONS, SQL_OV_ODBC3> for Sql92DatetimeFunctions {}
unsafe impl Attr<SQL_SQL92_DATETIME_FUNCTIONS> for Sql92DatetimeFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_DATETIME_FUNCTIONS> for Sql92DatetimeFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 156)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_FOREIGN_KEY_DELETE_RULE;
impl InfoType<SQL_SQL92_FOREIGN_KEY_DELETE_RULE, SQL_OV_ODBC3> for Sql92ForeignKeyDeleteRule {}
unsafe impl Attr<SQL_SQL92_FOREIGN_KEY_DELETE_RULE> for Sql92ForeignKeyDeleteRule {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_FOREIGN_KEY_DELETE_RULE> for Sql92ForeignKeyDeleteRule {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 157)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_FOREIGN_KEY_UPDATE_RULE;
impl InfoType<SQL_SQL92_FOREIGN_KEY_UPDATE_RULE, SQL_OV_ODBC3> for Sql92ForeignKeyUpdateRule {}
unsafe impl Attr<SQL_SQL92_FOREIGN_KEY_UPDATE_RULE> for Sql92ForeignKeyUpdateRule {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_FOREIGN_KEY_UPDATE_RULE> for Sql92ForeignKeyUpdateRule {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 158)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_GRANT;
impl InfoType<SQL_SQL92_GRANT, SQL_OV_ODBC3> for Sql92Grant {}
unsafe impl Attr<SQL_SQL92_GRANT> for Sql92Grant {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_GRANT> for Sql92Grant {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 119)]
#[allow(non_camel_case_types)]
pub struct SQL_DATETIME_LITERALS;
impl InfoType<SQL_DATETIME_LITERALS, SQL_OV_ODBC3> for DatetimeLiterals {}
unsafe impl Attr<SQL_DATETIME_LITERALS> for DatetimeLiterals {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DATETIME_LITERALS> for DatetimeLiterals {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 159)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_NUMERIC_VALUE_FUNCTIONS;
impl InfoType<SQL_SQL92_NUMERIC_VALUE_FUNCTIONS, SQL_OV_ODBC3> for Sql92NumericValueFunctions {}
unsafe impl Attr<SQL_SQL92_NUMERIC_VALUE_FUNCTIONS> for Sql92NumericValueFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_NUMERIC_VALUE_FUNCTIONS> for Sql92NumericValueFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 160)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_PREDICATES;
impl InfoType<SQL_SQL92_PREDICATES, SQL_OV_ODBC3> for Sql92Predicates {}
unsafe impl Attr<SQL_SQL92_PREDICATES> for Sql92Predicates {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_PREDICATES> for Sql92Predicates {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 161)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_RELATIONAL_JOIN_OPERATORS;
impl InfoType<SQL_SQL92_RELATIONAL_JOIN_OPERATORS, SQL_OV_ODBC3> for Sql92RelationalJoinOperators {}
unsafe impl Attr<SQL_SQL92_RELATIONAL_JOIN_OPERATORS> for Sql92RelationalJoinOperators {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_RELATIONAL_JOIN_OPERATORS> for Sql92RelationalJoinOperators {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 162)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_REVOKE;
impl InfoType<SQL_SQL92_REVOKE, SQL_OV_ODBC3> for Sql92Revoke {}
unsafe impl Attr<SQL_SQL92_REVOKE> for Sql92Revoke {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_REVOKE> for Sql92Revoke {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 163)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_ROW_VALUE_CONSTRUCTOR;
impl InfoType<SQL_SQL92_ROW_VALUE_CONSTRUCTOR, SQL_OV_ODBC3> for Sql92RowValueConstructor {}
unsafe impl Attr<SQL_SQL92_ROW_VALUE_CONSTRUCTOR> for Sql92RowValueConstructor {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_ROW_VALUE_CONSTRUCTOR> for Sql92RowValueConstructor {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 164)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_STRING_FUNCTIONS;
impl InfoType<SQL_SQL92_STRING_FUNCTIONS, SQL_OV_ODBC3> for Sql92StringFunctions {}
unsafe impl Attr<SQL_SQL92_STRING_FUNCTIONS> for Sql92StringFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_STRING_FUNCTIONS> for Sql92StringFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 165)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL92_VALUE_EXPRESSIONS;
impl InfoType<SQL_SQL92_VALUE_EXPRESSIONS, SQL_OV_ODBC3> for Sql92ValueExpressions {}
unsafe impl Attr<SQL_SQL92_VALUE_EXPRESSIONS> for Sql92ValueExpressions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL92_VALUE_EXPRESSIONS> for Sql92ValueExpressions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 166)]
#[allow(non_camel_case_types)]
pub struct SQL_STANDARD_CLI_CONFORMANCE;
impl InfoType<SQL_STANDARD_CLI_CONFORMANCE, SQL_OV_ODBC3> for StandardCliConformance {}
unsafe impl Attr<SQL_STANDARD_CLI_CONFORMANCE> for StandardCliConformance {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_STANDARD_CLI_CONFORMANCE> for StandardCliConformance {}

// TODO: What about these
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 0)]
//#[allow(non_camel_case_types)]
//pub struct SQL_INFO_FIRST;
//impl InfoType<SQL_INFO_FIRST, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_INFO_FIRST> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_INFO_FIRST> for {}

//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 12)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ODBC_SAG_CLI_CONFORMANCE;
//impl InfoType<SQL_ODBC_SAG_CLI_CONFORMANCE, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_ODBC_SAG_CLI_CONFORMANCE> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_ODBC_SAG_CLI_CONFORMANCE> for {}

//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, SQL_UNION)]
//#[allow(non_camel_case_types)]
//pub struct SQL_UNION_STATEMENT;
//impl InfoType<SQL_UNION_STATEMENT, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_UNION_STATEMENT> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_UNION_STATEMENT> for {}

//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 174)]
//#[allow(non_camel_case_types)]
//pub struct SQL_SCHEMA_INFERENCE;
//impl InfoType<SQL_SCHEMA_INFERENCE, SQL_OV_ODBC4> for {}
//unsafe impl Attr<SQL_SCHEMA_INFERENCE> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_SCHEMA_INFERENCE> for {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 175)]
#[allow(non_camel_case_types)]
pub struct SQL_BINARY_FUNCTIONS;
impl InfoType<SQL_BINARY_FUNCTIONS, SQL_OV_ODBC4> for BinaryFunctions {}
unsafe impl Attr<SQL_BINARY_FUNCTIONS> for BinaryFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_BINARY_FUNCTIONS> for BinaryFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 176)]
#[allow(non_camel_case_types)]
pub struct SQL_ISO_STRING_FUNCTIONS;
impl InfoType<SQL_ISO_STRING_FUNCTIONS, SQL_OV_ODBC4> for Sql92StringFunctions {}
unsafe impl Attr<SQL_ISO_STRING_FUNCTIONS> for Sql92StringFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ISO_STRING_FUNCTIONS> for Sql92StringFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 177)]
#[allow(non_camel_case_types)]
pub struct SQL_ISO_BINARY_FUNCTIONS;
impl InfoType<SQL_ISO_BINARY_FUNCTIONS, SQL_OV_ODBC4> for IsoBinaryFunctions {}
unsafe impl Attr<SQL_ISO_BINARY_FUNCTIONS> for IsoBinaryFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ISO_BINARY_FUNCTIONS> for IsoBinaryFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 178)]
#[allow(non_camel_case_types)]
pub struct SQL_LIMIT_ESCAPE_CLAUSE;
impl InfoType<SQL_LIMIT_ESCAPE_CLAUSE, SQL_OV_ODBC4> for LimitEscapeClause {}
unsafe impl Attr<SQL_LIMIT_ESCAPE_CLAUSE> for LimitEscapeClause {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_LIMIT_ESCAPE_CLAUSE> for LimitEscapeClause {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 179)]
#[allow(non_camel_case_types)]
pub struct SQL_NATIVE_ESCAPE_CLAUSE;
impl InfoType<SQL_NATIVE_ESCAPE_CLAUSE, SQL_OV_ODBC4> for [SQLCHAR] {}
unsafe impl Attr<SQL_NATIVE_ESCAPE_CLAUSE> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_NATIVE_ESCAPE_CLAUSE> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 180)]
#[allow(non_camel_case_types)]
pub struct SQL_RETURN_ESCAPE_CLAUSE;
impl InfoType<SQL_RETURN_ESCAPE_CLAUSE, SQL_OV_ODBC4> for ReturnEscapeClause {}
unsafe impl Attr<SQL_RETURN_ESCAPE_CLAUSE> for ReturnEscapeClause {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_RETURN_ESCAPE_CLAUSE> for ReturnEscapeClause {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 181)]
#[allow(non_camel_case_types)]
pub struct SQL_FORMAT_ESCAPE_CLAUSE;
impl InfoType<SQL_FORMAT_ESCAPE_CLAUSE, SQL_OV_ODBC4> for FormatEscapeClause {}
unsafe impl Attr<SQL_FORMAT_ESCAPE_CLAUSE> for FormatEscapeClause {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_FORMAT_ESCAPE_CLAUSE> for FormatEscapeClause {}

#[cfg(feature = "v4")]
pub use SQL_SQL92_DATETIME_FUNCTIONS as SQL_ISO_DATETIME_FUNCTIONS;

#[cfg(feature = "v4")]
pub use SQL_SQL92_FOREIGN_KEY_DELETE_RULE as SQL_ISO_FOREIGN_KEY_DELETE_RULE;

#[cfg(feature = "v4")]
pub use SQL_SQL92_FOREIGN_KEY_UPDATE_RULE as SQL_ISO_FOREIGN_KEY_UPDATE_RULE;

#[cfg(feature = "v4")]
pub use SQL_SQL92_GRANT as SQL_ISO_GRANT;

#[cfg(feature = "v4")]
pub use SQL_SQL92_NUMERIC_VALUE_FUNCTIONS as SQL_ISO_NUMERIC_VALUE_FUNCTIONS;

#[cfg(feature = "v4")]
pub use SQL_SQL92_PREDICATES as SQL_ISO_PREDICATES;

#[cfg(feature = "v4")]
pub use SQL_SQL92_RELATIONAL_JOIN_OPERATORS as SQL_ISO_RELATIONAL_JOIN_OPERATORS;

#[cfg(feature = "v4")]
pub use SQL_SQL92_REVOKE as SQL_ISO_REVOKE;

#[cfg(feature = "v4")]
pub use SQL_SQL92_ROW_VALUE_CONSTRUCTOR as SQL_ISO_ROW_VALUE_CONSTRUCTOR;

#[cfg(feature = "v4")]
pub use SQL_SQL92_VALUE_EXPRESSIONS as SQL_ISO_VALUE_EXPRESSIONS;

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Driver Information ///////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 116)]
#[allow(non_camel_case_types)]
pub struct SQL_ACTIVE_ENVIRONMENTS;
impl InfoType<SQL_ACTIVE_ENVIRONMENTS, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_ACTIVE_ENVIRONMENTS> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ACTIVE_ENVIRONMENTS> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10023)]
#[allow(non_camel_case_types)]
pub struct SQL_ASYNC_DBC_FUNCTIONS;
impl InfoType<SQL_ASYNC_DBC_FUNCTIONS, SQL_OV_ODBC3_80> for AsyncDbcFunctions {}
unsafe impl Attr<SQL_ASYNC_DBC_FUNCTIONS> for AsyncDbcFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ASYNC_DBC_FUNCTIONS> for AsyncDbcFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10021)]
#[allow(non_camel_case_types)]
pub struct SQL_ASYNC_MODE;
impl InfoType<SQL_ASYNC_MODE, SQL_OV_ODBC3> for AsyncMode {}
unsafe impl Attr<SQL_ASYNC_MODE> for AsyncMode {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ASYNC_MODE> for AsyncMode {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10025)]
#[allow(non_camel_case_types)]
pub struct SQL_ASYNC_NOTIFICATION;
impl InfoType<SQL_ASYNC_NOTIFICATION, SQL_OV_ODBC3_80> for AsyncNotification {}
unsafe impl Attr<SQL_ASYNC_NOTIFICATION> for AsyncNotification {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ASYNC_NOTIFICATION> for AsyncNotification {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 120)]
#[allow(non_camel_case_types)]
pub struct SQL_BATCH_ROW_COUNT;
impl InfoType<SQL_BATCH_ROW_COUNT, SQL_OV_ODBC3> for BatchRowCount {}
unsafe impl Attr<SQL_BATCH_ROW_COUNT> for BatchRowCount {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_BATCH_ROW_COUNT> for BatchRowCount {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 121)]
#[allow(non_camel_case_types)]
pub struct SQL_BATCH_SUPPORT;
impl InfoType<SQL_BATCH_SUPPORT, SQL_OV_ODBC3> for BatchSupport {}
unsafe impl Attr<SQL_BATCH_SUPPORT> for BatchSupport {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_BATCH_SUPPORT> for BatchSupport {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_NAME;
impl InfoType<SQL_DATA_SOURCE_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DATA_SOURCE_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DATA_SOURCE_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10024)]
#[allow(non_camel_case_types)]
pub struct SQL_DRIVER_AWARE_POOLING_SUPPORTED;
impl InfoType<SQL_DRIVER_AWARE_POOLING_SUPPORTED, SQL_OV_ODBC3_80> for DriverAwarePoolingSupported {}
unsafe impl Attr<SQL_DRIVER_AWARE_POOLING_SUPPORTED> for DriverAwarePoolingSupported {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DRIVER_AWARE_POOLING_SUPPORTED> for DriverAwarePoolingSupported {}

// TODO: How are these handles used?
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 3)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DRIVER_HDBC;
//impl InfoType<SQL_DRIVER_HDBC, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_DRIVER_HDBC> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_DRIVER_HDBC> for {}
//
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 135)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DRIVER_HDESC;
//impl InfoType<SQL_DRIVER_HDESC, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_DRIVER_HDESC> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_DRIVER_HDESC> for {}
//
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 4)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DRIVER_HENV;
//impl InfoType<SQL_DRIVER_HENV, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_DRIVER_HENV> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_DRIVER_HENV> for {}
//
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 76)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DRIVER_HLIB;
//impl InfoType<SQL_DRIVER_HLIB, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_DRIVER_HLIB> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_DRIVER_HLIB> for {}
//
//#[derive(Ident)]
//#[identifier(SQLUSMALLINT, 5)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DRIVER_HSTMT;
//impl InfoType<SQL_DRIVER_HSTMT, SQL_OV_ODBC3> for {}
//unsafe impl Attr<SQL_DRIVER_HSTMT> for {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//unsafe impl AttrGet<SQL_DRIVER_HSTMT> for {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_DRIVER_NAME;
impl InfoType<SQL_DRIVER_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DRIVER_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DRIVER_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 77)]
#[allow(non_camel_case_types)]
pub struct SQL_DRIVER_ODBC_VER;
impl InfoType<SQL_DRIVER_ODBC_VER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DRIVER_ODBC_VER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DRIVER_ODBC_VER> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_DRIVER_VER;
impl InfoType<SQL_DRIVER_VER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DRIVER_VER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DRIVER_VER> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 144)]
#[allow(non_camel_case_types)]
pub struct SQL_DYNAMIC_CURSOR_ATTRIBUTES1;
impl InfoType<SQL_DYNAMIC_CURSOR_ATTRIBUTES1, SQL_OV_ODBC3> for CursorAttributes1 {}
unsafe impl Attr<SQL_DYNAMIC_CURSOR_ATTRIBUTES1> for CursorAttributes1 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DYNAMIC_CURSOR_ATTRIBUTES1> for CursorAttributes1 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 145)]
#[allow(non_camel_case_types)]
pub struct SQL_DYNAMIC_CURSOR_ATTRIBUTES2;
impl InfoType<SQL_DYNAMIC_CURSOR_ATTRIBUTES2, SQL_OV_ODBC3> for CursorAttributes2 {}
unsafe impl Attr<SQL_DYNAMIC_CURSOR_ATTRIBUTES2> for CursorAttributes2 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DYNAMIC_CURSOR_ATTRIBUTES2> for CursorAttributes2 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 146)]
#[allow(non_camel_case_types)]
pub struct SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1;
impl InfoType<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1, SQL_OV_ODBC3> for CursorAttributes1 {}
unsafe impl Attr<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1> for CursorAttributes1 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1> for CursorAttributes1 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 147)]
#[allow(non_camel_case_types)]
pub struct SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2;
impl InfoType<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2, SQL_OV_ODBC3> for CursorAttributes2 {}
unsafe impl Attr<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2> for CursorAttributes2 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2> for CursorAttributes2 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 84)]
#[allow(non_camel_case_types)]
pub struct SQL_FILE_USAGE;
impl InfoType<SQL_FILE_USAGE, SQL_OV_ODBC3> for FileUsage {}
unsafe impl Attr<SQL_FILE_USAGE> for FileUsage {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_FILE_USAGE> for FileUsage {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 81)]
#[allow(non_camel_case_types)]
pub struct SQL_GETDATA_EXTENSIONS;
impl InfoType<SQL_GETDATA_EXTENSIONS, SQL_OV_ODBC3> for GetdataExtensions {}
unsafe impl Attr<SQL_GETDATA_EXTENSIONS> for GetdataExtensions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_GETDATA_EXTENSIONS> for GetdataExtensions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 149)]
#[allow(non_camel_case_types)]
pub struct SQL_INFO_SCHEMA_VIEWS;
impl InfoType<SQL_INFO_SCHEMA_VIEWS, SQL_OV_ODBC3> for InfoSchemaViews {}
unsafe impl Attr<SQL_INFO_SCHEMA_VIEWS> for InfoSchemaViews {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_INFO_SCHEMA_VIEWS> for InfoSchemaViews {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 150)]
#[allow(non_camel_case_types)]
pub struct SQL_KEYSET_CURSOR_ATTRIBUTES1;
impl InfoType<SQL_KEYSET_CURSOR_ATTRIBUTES1, SQL_OV_ODBC3> for CursorAttributes1 {}
unsafe impl Attr<SQL_KEYSET_CURSOR_ATTRIBUTES1> for CursorAttributes1 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_KEYSET_CURSOR_ATTRIBUTES1> for CursorAttributes1 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 151)]
#[allow(non_camel_case_types)]
pub struct SQL_KEYSET_CURSOR_ATTRIBUTES2;
impl InfoType<SQL_KEYSET_CURSOR_ATTRIBUTES2, SQL_OV_ODBC3> for CursorAttributes2 {}
unsafe impl Attr<SQL_KEYSET_CURSOR_ATTRIBUTES2> for CursorAttributes2 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_KEYSET_CURSOR_ATTRIBUTES2> for CursorAttributes2 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10022)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_ASYNC_CONCURRENT_STATEMENTS;
impl InfoType<SQL_MAX_ASYNC_CONCURRENT_STATEMENTS, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_ASYNC_CONCURRENT_STATEMENTS> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_ASYNC_CONCURRENT_STATEMENTS> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CONCURRENT_ACTIVITIES;
impl InfoType<SQL_MAX_CONCURRENT_ACTIVITIES, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_CONCURRENT_ACTIVITIES> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_CONCURRENT_ACTIVITIES> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_DRIVER_CONNECTIONS;
impl InfoType<SQL_MAX_DRIVER_CONNECTIONS, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_DRIVER_CONNECTIONS> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_DRIVER_CONNECTIONS> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 152)]
#[allow(non_camel_case_types)]
pub struct SQL_ODBC_INTERFACE_CONFORMANCE;
impl InfoType<SQL_ODBC_INTERFACE_CONFORMANCE, SQL_OV_ODBC3> for OdbcInterfaceConformance {}
unsafe impl Attr<SQL_ODBC_INTERFACE_CONFORMANCE> for OdbcInterfaceConformance {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ODBC_INTERFACE_CONFORMANCE> for OdbcInterfaceConformance {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ODBC_VER;
impl InfoType<SQL_ODBC_VER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_ODBC_VER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ODBC_VER> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 153)]
#[allow(non_camel_case_types)]
pub struct SQL_PARAM_ARRAY_ROW_COUNTS;
impl InfoType<SQL_PARAM_ARRAY_ROW_COUNTS, SQL_OV_ODBC3> for ParamArrayRowCounts {}
unsafe impl Attr<SQL_PARAM_ARRAY_ROW_COUNTS> for ParamArrayRowCounts {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_PARAM_ARRAY_ROW_COUNTS> for ParamArrayRowCounts {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 154)]
#[allow(non_camel_case_types)]
pub struct SQL_PARAM_ARRAY_SELECTS;
impl InfoType<SQL_PARAM_ARRAY_SELECTS, SQL_OV_ODBC3> for ParamArraySelects {}
unsafe impl Attr<SQL_PARAM_ARRAY_SELECTS> for ParamArraySelects {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_PARAM_ARRAY_SELECTS> for ParamArraySelects {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ROW_UPDATES;
impl InfoType<SQL_ROW_UPDATES, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_ROW_UPDATES> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ROW_UPDATES> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 14)]
#[allow(non_camel_case_types)]
pub struct SQL_SEARCH_PATTERN_ESCAPE;
impl InfoType<SQL_SEARCH_PATTERN_ESCAPE, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_SEARCH_PATTERN_ESCAPE> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SEARCH_PATTERN_ESCAPE> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 13)]
#[allow(non_camel_case_types)]
pub struct SQL_SERVER_NAME;
impl InfoType<SQL_SERVER_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_SERVER_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SERVER_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 167)]
#[allow(non_camel_case_types)]
pub struct SQL_STATIC_CURSOR_ATTRIBUTES1;
impl InfoType<SQL_STATIC_CURSOR_ATTRIBUTES1, SQL_OV_ODBC3> for CursorAttributes1 {}
unsafe impl Attr<SQL_STATIC_CURSOR_ATTRIBUTES1> for CursorAttributes1 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_STATIC_CURSOR_ATTRIBUTES1> for CursorAttributes1 {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 168)]
#[allow(non_camel_case_types)]
pub struct SQL_STATIC_CURSOR_ATTRIBUTES2;
impl InfoType<SQL_STATIC_CURSOR_ATTRIBUTES2, SQL_OV_ODBC3> for CursorAttributes2 {}
unsafe impl Attr<SQL_STATIC_CURSOR_ATTRIBUTES2> for CursorAttributes2 {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_STATIC_CURSOR_ATTRIBUTES2> for CursorAttributes2 {}

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////// DBMS Product Information ////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 16)]
#[allow(non_camel_case_types)]
pub struct SQL_DATABASE_NAME;
impl InfoType<SQL_DATABASE_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DATABASE_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DATABASE_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 17)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_NAME;
impl InfoType<SQL_DBMS_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DBMS_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DBMS_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 18)]
#[allow(non_camel_case_types)]
pub struct SQL_DBMS_VER;
impl InfoType<SQL_DBMS_VER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DBMS_VER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DBMS_VER> for [SQLCHAR] {}

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////// Data Source Information /////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 20)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_PROCEDURES;
impl InfoType<SQL_ACCESSIBLE_PROCEDURES, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_ACCESSIBLE_PROCEDURES> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ACCESSIBLE_PROCEDURES> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 19)]
#[allow(non_camel_case_types)]
pub struct SQL_ACCESSIBLE_TABLES;
impl InfoType<SQL_ACCESSIBLE_TABLES, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_ACCESSIBLE_TABLES> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ACCESSIBLE_TABLES> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 82)]
#[allow(non_camel_case_types)]
pub struct SQL_BOOKMARK_PERSISTENCE;
impl InfoType<SQL_BOOKMARK_PERSISTENCE, SQL_OV_ODBC3> for BookmarkPersistence {}
unsafe impl Attr<SQL_BOOKMARK_PERSISTENCE> for BookmarkPersistence {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_BOOKMARK_PERSISTENCE> for BookmarkPersistence {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 42)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_TERM;
impl InfoType<SQL_CATALOG_TERM, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_CATALOG_TERM> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CATALOG_TERM> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10004)]
#[allow(non_camel_case_types)]
pub struct SQL_COLLATION_SEQ;
impl InfoType<SQL_COLLATION_SEQ, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_COLLATION_SEQ> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_COLLATION_SEQ> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 22)]
#[allow(non_camel_case_types)]
pub struct SQL_CONCAT_NULL_BEHAVIOR;
impl InfoType<SQL_CONCAT_NULL_BEHAVIOR, SQL_OV_ODBC3> for ConcatNullBehavior {}
unsafe impl Attr<SQL_CONCAT_NULL_BEHAVIOR> for ConcatNullBehavior {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONCAT_NULL_BEHAVIOR> for ConcatNullBehavior {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 23)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_COMMIT_BEHAVIOR;
impl InfoType<SQL_CURSOR_COMMIT_BEHAVIOR, SQL_OV_ODBC3> for CursorBehavior {}
unsafe impl Attr<SQL_CURSOR_COMMIT_BEHAVIOR> for CursorBehavior {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CURSOR_COMMIT_BEHAVIOR> for CursorBehavior {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 24)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_ROLLBACK_BEHAVIOR;
impl InfoType<SQL_CURSOR_ROLLBACK_BEHAVIOR, SQL_OV_ODBC3> for CursorBehavior {}
unsafe impl Attr<SQL_CURSOR_ROLLBACK_BEHAVIOR> for CursorBehavior {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CURSOR_ROLLBACK_BEHAVIOR> for CursorBehavior {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10001)]
#[allow(non_camel_case_types)]
pub struct SQL_CURSOR_SENSITIVITY;
impl InfoType<SQL_CURSOR_SENSITIVITY, SQL_OV_ODBC3> for CursorSensitivity {}
unsafe impl Attr<SQL_CURSOR_SENSITIVITY> for CursorSensitivity {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CURSOR_SENSITIVITY> for CursorSensitivity {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 25)]
#[allow(non_camel_case_types)]
pub struct SQL_DATA_SOURCE_READ_ONLY;
impl InfoType<SQL_DATA_SOURCE_READ_ONLY, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DATA_SOURCE_READ_ONLY> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DATA_SOURCE_READ_ONLY> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 26)]
#[allow(non_camel_case_types)]
pub struct SQL_DEFAULT_TXN_ISOLATION;
impl InfoType<SQL_DEFAULT_TXN_ISOLATION, SQL_OV_ODBC3> for TxnIsolation {}
unsafe impl Attr<SQL_DEFAULT_TXN_ISOLATION> for TxnIsolation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DEFAULT_TXN_ISOLATION> for TxnIsolation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10002)]
#[allow(non_camel_case_types)]
pub struct SQL_DESCRIBE_PARAMETER;
impl InfoType<SQL_DESCRIBE_PARAMETER, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_DESCRIBE_PARAMETER> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DESCRIBE_PARAMETER> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 36)]
#[allow(non_camel_case_types)]
pub struct SQL_MULT_RESULT_SETS;
impl InfoType<SQL_MULT_RESULT_SETS, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_MULT_RESULT_SETS> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MULT_RESULT_SETS> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 37)]
#[allow(non_camel_case_types)]
pub struct SQL_MULTIPLE_ACTIVE_TXN;
impl InfoType<SQL_MULTIPLE_ACTIVE_TXN, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_MULTIPLE_ACTIVE_TXN> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MULTIPLE_ACTIVE_TXN> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 111)]
#[allow(non_camel_case_types)]
pub struct SQL_NEED_LONG_DATA_LEN;
impl InfoType<SQL_NEED_LONG_DATA_LEN, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_NEED_LONG_DATA_LEN> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_NEED_LONG_DATA_LEN> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 85)]
#[allow(non_camel_case_types)]
pub struct SQL_NULL_COLLATION;
impl InfoType<SQL_NULL_COLLATION, SQL_OV_ODBC3> for NullCollation {}
unsafe impl Attr<SQL_NULL_COLLATION> for NullCollation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_NULL_COLLATION> for NullCollation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 40)]
#[allow(non_camel_case_types)]
pub struct SQL_PROCEDURE_TERM;
impl InfoType<SQL_PROCEDURE_TERM, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_PROCEDURE_TERM> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_PROCEDURE_TERM> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 39)]
#[allow(non_camel_case_types)]
pub struct SQL_SCHEMA_TERM;
impl InfoType<SQL_SCHEMA_TERM, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_SCHEMA_TERM> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SCHEMA_TERM> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 44)]
#[allow(non_camel_case_types)]
pub struct SQL_SCROLL_OPTIONS;
impl InfoType<SQL_SCROLL_OPTIONS, SQL_OV_ODBC3> for ScrollOptions {}
unsafe impl Attr<SQL_SCROLL_OPTIONS> for ScrollOptions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SCROLL_OPTIONS> for ScrollOptions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 45)]
#[allow(non_camel_case_types)]
pub struct SQL_TABLE_TERM;
impl InfoType<SQL_TABLE_TERM, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_TABLE_TERM> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TABLE_TERM> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 46)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_CAPABLE;
impl InfoType<SQL_TXN_CAPABLE, SQL_OV_ODBC3> for TxnCapable {}
unsafe impl Attr<SQL_TXN_CAPABLE> for TxnCapable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TXN_CAPABLE> for TxnCapable {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 72)]
#[allow(non_camel_case_types)]
pub struct SQL_TXN_ISOLATION_OPTION;
impl InfoType<SQL_TXN_ISOLATION_OPTION, SQL_OV_ODBC3> for TxnIsolation {}
unsafe impl Attr<SQL_TXN_ISOLATION_OPTION> for TxnIsolation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TXN_ISOLATION_OPTION> for TxnIsolation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 47)]
#[allow(non_camel_case_types)]
pub struct SQL_USER_NAME;
impl InfoType<SQL_USER_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_USER_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_USER_NAME> for [SQLCHAR] {}

/////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////// Supported SQL //////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 169)]
#[allow(non_camel_case_types)]
pub struct SQL_AGGREGATE_FUNCTIONS;
impl InfoType<SQL_AGGREGATE_FUNCTIONS, SQL_OV_ODBC3> for AggregateFunctions {}
unsafe impl Attr<SQL_AGGREGATE_FUNCTIONS> for AggregateFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_AGGREGATE_FUNCTIONS> for AggregateFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 117)]
#[allow(non_camel_case_types)]
pub struct SQL_ALTER_DOMAIN;
impl InfoType<SQL_ALTER_DOMAIN, SQL_OV_ODBC3> for AlterDomain {}
unsafe impl Attr<SQL_ALTER_DOMAIN> for AlterDomain {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ALTER_DOMAIN> for AlterDomain {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 86)]
#[allow(non_camel_case_types)]
pub struct SQL_ALTER_TABLE;
impl InfoType<SQL_ALTER_TABLE, SQL_OV_ODBC3> for AlterTable {}
unsafe impl Attr<SQL_ALTER_TABLE> for AlterTable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ALTER_TABLE> for AlterTable {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 114)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_LOCATION;
impl InfoType<SQL_CATALOG_LOCATION, SQL_OV_ODBC3> for CatalogLocation {}
unsafe impl Attr<SQL_CATALOG_LOCATION> for CatalogLocation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CATALOG_LOCATION> for CatalogLocation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10003)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_NAME;
impl InfoType<SQL_CATALOG_NAME, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_CATALOG_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CATALOG_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 41)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_NAME_SEPARATOR;
impl InfoType<SQL_CATALOG_NAME_SEPARATOR, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_CATALOG_NAME_SEPARATOR> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CATALOG_NAME_SEPARATOR> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 92)]
#[allow(non_camel_case_types)]
pub struct SQL_CATALOG_USAGE;
impl InfoType<SQL_CATALOG_USAGE, SQL_OV_ODBC3> for CatalogUsage {}
unsafe impl Attr<SQL_CATALOG_USAGE> for CatalogUsage {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CATALOG_USAGE> for CatalogUsage {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 87)]
#[allow(non_camel_case_types)]
pub struct SQL_COLUMN_ALIAS;
impl InfoType<SQL_COLUMN_ALIAS, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_COLUMN_ALIAS> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_COLUMN_ALIAS> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 74)]
#[allow(non_camel_case_types)]
pub struct SQL_CORRELATION_NAME;
impl InfoType<SQL_CORRELATION_NAME, SQL_OV_ODBC3> for CorrelationName {}
unsafe impl Attr<SQL_CORRELATION_NAME> for CorrelationName {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CORRELATION_NAME> for CorrelationName {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 127)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_ASSERTION;
impl InfoType<SQL_CREATE_ASSERTION, SQL_OV_ODBC3> for CreateAssertion {}
unsafe impl Attr<SQL_CREATE_ASSERTION> for CreateAssertion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_ASSERTION> for CreateAssertion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 128)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_CHARACTER_SET;
impl InfoType<SQL_CREATE_CHARACTER_SET, SQL_OV_ODBC3> for CreateCharacterSet {}
unsafe impl Attr<SQL_CREATE_CHARACTER_SET> for CreateCharacterSet {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_CHARACTER_SET> for CreateCharacterSet {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 129)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_COLLATION;
impl InfoType<SQL_CREATE_COLLATION, SQL_OV_ODBC3> for CreateCollation {}
unsafe impl Attr<SQL_CREATE_COLLATION> for CreateCollation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_COLLATION> for CreateCollation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 130)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_DOMAIN;
impl InfoType<SQL_CREATE_DOMAIN, SQL_OV_ODBC3> for CreateDomain {}
unsafe impl Attr<SQL_CREATE_DOMAIN> for CreateDomain {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_DOMAIN> for CreateDomain {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 131)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_SCHEMA;
impl InfoType<SQL_CREATE_SCHEMA, SQL_OV_ODBC3> for CreateSchema {}
unsafe impl Attr<SQL_CREATE_SCHEMA> for CreateSchema {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_SCHEMA> for CreateSchema {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 132)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_TABLE;
impl InfoType<SQL_CREATE_TABLE, SQL_OV_ODBC3> for CreateTable {}
unsafe impl Attr<SQL_CREATE_TABLE> for CreateTable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_TABLE> for CreateTable {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 133)]
#[allow(non_camel_case_types)]
pub struct SQL_CREATE_TRANSLATION;
impl InfoType<SQL_CREATE_TRANSLATION, SQL_OV_ODBC3> for CreateTranslation {}
unsafe impl Attr<SQL_CREATE_TRANSLATION> for CreateTranslation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CREATE_TRANSLATION> for CreateTranslation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 170)]
#[allow(non_camel_case_types)]
pub struct SQL_DDL_INDEX;
impl InfoType<SQL_DDL_INDEX, SQL_OV_ODBC3> for DdlIndex {}
unsafe impl Attr<SQL_DDL_INDEX> for DdlIndex {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DDL_INDEX> for DdlIndex {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 136)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_ASSERTION;
impl InfoType<SQL_DROP_ASSERTION, SQL_OV_ODBC3> for DropAssertion {}
unsafe impl Attr<SQL_DROP_ASSERTION> for DropAssertion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_ASSERTION> for DropAssertion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 137)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_CHARACTER_SET;
impl InfoType<SQL_DROP_CHARACTER_SET, SQL_OV_ODBC3> for DropCharacterSet {}
unsafe impl Attr<SQL_DROP_CHARACTER_SET> for DropCharacterSet {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_CHARACTER_SET> for DropCharacterSet {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 138)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_COLLATION;
impl InfoType<SQL_DROP_COLLATION, SQL_OV_ODBC3> for DropCollation {}
unsafe impl Attr<SQL_DROP_COLLATION> for DropCollation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_COLLATION> for DropCollation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 139)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_DOMAIN;
impl InfoType<SQL_DROP_DOMAIN, SQL_OV_ODBC3> for DropDomain {}
unsafe impl Attr<SQL_DROP_DOMAIN> for DropDomain {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_DOMAIN> for DropDomain {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 140)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_SCHEMA;
impl InfoType<SQL_DROP_SCHEMA, SQL_OV_ODBC3> for DropSchema {}
unsafe impl Attr<SQL_DROP_SCHEMA> for DropSchema {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_SCHEMA> for DropSchema {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 141)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_TABLE;
impl InfoType<SQL_DROP_TABLE, SQL_OV_ODBC3> for DropTable {}
unsafe impl Attr<SQL_DROP_TABLE> for DropTable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_TABLE> for DropTable {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 142)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_TRANSLATION;
impl InfoType<SQL_DROP_TRANSLATION, SQL_OV_ODBC3> for DropTranslation {}
unsafe impl Attr<SQL_DROP_TRANSLATION> for DropTranslation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_TRANSLATION> for DropTranslation {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 143)]
#[allow(non_camel_case_types)]
pub struct SQL_DROP_VIEW;
impl InfoType<SQL_DROP_VIEW, SQL_OV_ODBC3> for DropView {}
unsafe impl Attr<SQL_DROP_VIEW> for DropView {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_DROP_VIEW> for DropView {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 27)]
#[allow(non_camel_case_types)]
pub struct SQL_EXPRESSIONS_IN_ORDERBY;
impl InfoType<SQL_EXPRESSIONS_IN_ORDERBY, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_EXPRESSIONS_IN_ORDERBY> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_EXPRESSIONS_IN_ORDERBY> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 88)]
#[allow(non_camel_case_types)]
pub struct SQL_GROUP_BY;
impl InfoType<SQL_GROUP_BY, SQL_OV_ODBC3> for GroupBy {}
unsafe impl Attr<SQL_GROUP_BY> for GroupBy {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_GROUP_BY> for GroupBy {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 28)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_CASE;
impl InfoType<SQL_IDENTIFIER_CASE, SQL_OV_ODBC3> for IdentifierCase {}
unsafe impl Attr<SQL_IDENTIFIER_CASE> for IdentifierCase {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_IDENTIFIER_CASE> for IdentifierCase {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 29)]
#[allow(non_camel_case_types)]
pub struct SQL_IDENTIFIER_QUOTE_CHAR;
impl InfoType<SQL_IDENTIFIER_QUOTE_CHAR, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_IDENTIFIER_QUOTE_CHAR> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_IDENTIFIER_QUOTE_CHAR> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 148)]
#[allow(non_camel_case_types)]
pub struct SQL_INDEX_KEYWORDS;
impl InfoType<SQL_INDEX_KEYWORDS, SQL_OV_ODBC3> for IndexKeywords {}
unsafe impl Attr<SQL_INDEX_KEYWORDS> for IndexKeywords {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_INDEX_KEYWORDS> for IndexKeywords {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 171)]
#[allow(non_camel_case_types)]
pub struct SQL_INSERT_STATEMENT;
impl InfoType<SQL_INSERT_STATEMENT, SQL_OV_ODBC3> for InsertStatement {}
unsafe impl Attr<SQL_INSERT_STATEMENT> for InsertStatement {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_INSERT_STATEMENT> for InsertStatement {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 73)]
#[allow(non_camel_case_types)]
pub struct SQL_INTEGRITY;
impl InfoType<SQL_INTEGRITY, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_INTEGRITY> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_INTEGRITY> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 89)]
#[allow(non_camel_case_types)]
pub struct SQL_KEYWORDS;
impl InfoType<SQL_KEYWORDS, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_KEYWORDS> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_KEYWORDS> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 113)]
#[allow(non_camel_case_types)]
pub struct SQL_LIKE_ESCAPE_CLAUSE;
impl InfoType<SQL_LIKE_ESCAPE_CLAUSE, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_LIKE_ESCAPE_CLAUSE> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_LIKE_ESCAPE_CLAUSE> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 75)]
#[allow(non_camel_case_types)]
pub struct SQL_NON_NULLABLE_COLUMNS;
impl InfoType<SQL_NON_NULLABLE_COLUMNS, SQL_OV_ODBC3> for NonNullableColumns {}
unsafe impl Attr<SQL_NON_NULLABLE_COLUMNS> for NonNullableColumns {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_NON_NULLABLE_COLUMNS> for NonNullableColumns {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 115)]
#[allow(non_camel_case_types)]
pub struct SQL_OJ_CAPABILITIES;
impl InfoType<SQL_OJ_CAPABILITIES, SQL_OV_ODBC3> for OjCapabilities {}
unsafe impl Attr<SQL_OJ_CAPABILITIES> for OjCapabilities {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_OJ_CAPABILITIES> for OjCapabilities {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 90)]
#[allow(non_camel_case_types)]
pub struct SQL_ORDER_BY_COLUMNS_IN_SELECT;
impl InfoType<SQL_ORDER_BY_COLUMNS_IN_SELECT, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_ORDER_BY_COLUMNS_IN_SELECT> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_ORDER_BY_COLUMNS_IN_SELECT> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 38)]
#[allow(non_camel_case_types)]
pub struct SQL_OUTER_JOINS;
impl InfoType<SQL_OUTER_JOINS, SQL_OV_ODBC3> for OuterJoins {}
unsafe impl Attr<SQL_OUTER_JOINS> for OuterJoins {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_OUTER_JOINS> for OuterJoins {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 21)]
#[allow(non_camel_case_types)]
pub struct SQL_PROCEDURES;
impl InfoType<SQL_PROCEDURES, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_PROCEDURES> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_PROCEDURES> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 93)]
#[allow(non_camel_case_types)]
pub struct SQL_QUOTED_IDENTIFIER_CASE;
impl InfoType<SQL_QUOTED_IDENTIFIER_CASE, SQL_OV_ODBC3> for IdentifierCase {}
unsafe impl Attr<SQL_QUOTED_IDENTIFIER_CASE> for IdentifierCase {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_QUOTED_IDENTIFIER_CASE> for IdentifierCase {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 91)]
#[allow(non_camel_case_types)]
pub struct SQL_SCHEMA_USAGE;
impl InfoType<SQL_SCHEMA_USAGE, SQL_OV_ODBC3> for SchemaUsage {}
unsafe impl Attr<SQL_SCHEMA_USAGE> for SchemaUsage {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SCHEMA_USAGE> for SchemaUsage {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 94)]
#[allow(non_camel_case_types)]
pub struct SQL_SPECIAL_CHARACTERS;
impl InfoType<SQL_SPECIAL_CHARACTERS, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_SPECIAL_CHARACTERS> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SPECIAL_CHARACTERS> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 118)]
#[allow(non_camel_case_types)]
pub struct SQL_SQL_CONFORMANCE;
impl InfoType<SQL_SQL_CONFORMANCE, SQL_OV_ODBC3> for SqlConformance {}
unsafe impl Attr<SQL_SQL_CONFORMANCE> for SqlConformance {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SQL_CONFORMANCE> for SqlConformance {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 95)]
#[allow(non_camel_case_types)]
pub struct SQL_SUBQUERIES;
impl InfoType<SQL_SUBQUERIES, SQL_OV_ODBC3> for Subqueries {}
unsafe impl Attr<SQL_SUBQUERIES> for Subqueries {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SUBQUERIES> for Subqueries {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 96)]
#[allow(non_camel_case_types)]
pub struct SQL_UNION;
impl InfoType<SQL_UNION, SQL_OV_ODBC3> for Union {}
unsafe impl Attr<SQL_UNION> for Union {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_UNION> for Union {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// SQL Limits ///////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 112)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_BINARY_LITERAL_LEN;
impl InfoType<SQL_MAX_BINARY_LITERAL_LEN, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_BINARY_LITERAL_LEN> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_BINARY_LITERAL_LEN> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 34)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CATALOG_NAME_LEN;
impl InfoType<SQL_MAX_CATALOG_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_CATALOG_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_CATALOG_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 108)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CHAR_LITERAL_LEN;
impl InfoType<SQL_MAX_CHAR_LITERAL_LEN, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_CHAR_LITERAL_LEN> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_CHAR_LITERAL_LEN> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 30)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMN_NAME_LEN;
impl InfoType<SQL_MAX_COLUMN_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMN_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMN_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 97)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_GROUP_BY;
impl InfoType<SQL_MAX_COLUMNS_IN_GROUP_BY, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMNS_IN_GROUP_BY> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMNS_IN_GROUP_BY> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 98)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_INDEX;
impl InfoType<SQL_MAX_COLUMNS_IN_INDEX, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMNS_IN_INDEX> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMNS_IN_INDEX> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 99)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_ORDER_BY;
impl InfoType<SQL_MAX_COLUMNS_IN_ORDER_BY, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMNS_IN_ORDER_BY> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMNS_IN_ORDER_BY> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 100)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_SELECT;
impl InfoType<SQL_MAX_COLUMNS_IN_SELECT, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMNS_IN_SELECT> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMNS_IN_SELECT> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_COLUMNS_IN_TABLE;
impl InfoType<SQL_MAX_COLUMNS_IN_TABLE, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_COLUMNS_IN_TABLE> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_COLUMNS_IN_TABLE> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 31)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_CURSOR_NAME_LEN;
impl InfoType<SQL_MAX_CURSOR_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_CURSOR_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_CURSOR_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10005)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_IDENTIFIER_LEN;
impl InfoType<SQL_MAX_IDENTIFIER_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_IDENTIFIER_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_IDENTIFIER_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_INDEX_SIZE;
impl InfoType<SQL_MAX_INDEX_SIZE, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_INDEX_SIZE> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_INDEX_SIZE> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 33)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_PROCEDURE_NAME_LEN;
impl InfoType<SQL_MAX_PROCEDURE_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_PROCEDURE_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_PROCEDURE_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_ROW_SIZE;
impl InfoType<SQL_MAX_ROW_SIZE, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_ROW_SIZE> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_ROW_SIZE> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 103)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_ROW_SIZE_INCLUDES_LONG;
impl InfoType<SQL_MAX_ROW_SIZE_INCLUDES_LONG, SQL_OV_ODBC3> for [SQLCHAR] {}
unsafe impl Attr<SQL_MAX_ROW_SIZE_INCLUDES_LONG> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_ROW_SIZE_INCLUDES_LONG> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 32)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_SCHEMA_NAME_LEN;
impl InfoType<SQL_MAX_SCHEMA_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_SCHEMA_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_SCHEMA_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_STATEMENT_LEN;
impl InfoType<SQL_MAX_STATEMENT_LEN, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl Attr<SQL_MAX_STATEMENT_LEN> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_STATEMENT_LEN> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 35)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLE_NAME_LEN;
impl InfoType<SQL_MAX_TABLE_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_TABLE_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_TABLE_NAME_LEN> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_TABLES_IN_SELECT;
impl InfoType<SQL_MAX_TABLES_IN_SELECT, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_TABLES_IN_SELECT> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_TABLES_IN_SELECT> for SQLUSMALLINT {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 107)]
#[allow(non_camel_case_types)]
pub struct SQL_MAX_USER_NAME_LEN;
impl InfoType<SQL_MAX_USER_NAME_LEN, SQL_OV_ODBC3> for SQLUSMALLINT {}
unsafe impl Attr<SQL_MAX_USER_NAME_LEN> for SQLUSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_MAX_USER_NAME_LEN> for SQLUSMALLINT {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// Scalar Function Information //////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 48)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_FUNCTIONS;
impl InfoType<SQL_CONVERT_FUNCTIONS, SQL_OV_ODBC3> for ConvertFunctions {}
unsafe impl Attr<SQL_CONVERT_FUNCTIONS> for ConvertFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_FUNCTIONS> for ConvertFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 49)]
#[allow(non_camel_case_types)]
pub struct SQL_NUMERIC_FUNCTIONS;
impl InfoType<SQL_NUMERIC_FUNCTIONS, SQL_OV_ODBC3> for NumericFunctions {}
unsafe impl Attr<SQL_NUMERIC_FUNCTIONS> for NumericFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_NUMERIC_FUNCTIONS> for NumericFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 50)]
#[allow(non_camel_case_types)]
pub struct SQL_STRING_FUNCTIONS;
impl InfoType<SQL_STRING_FUNCTIONS, SQL_OV_ODBC3> for StringFunctions {}
unsafe impl Attr<SQL_STRING_FUNCTIONS> for StringFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_STRING_FUNCTIONS> for StringFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 51)]
#[allow(non_camel_case_types)]
pub struct SQL_SYSTEM_FUNCTIONS;
impl InfoType<SQL_SYSTEM_FUNCTIONS, SQL_OV_ODBC3> for SystemFunctions {}
unsafe impl Attr<SQL_SYSTEM_FUNCTIONS> for SystemFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_SYSTEM_FUNCTIONS> for SystemFunctions {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 109)]
#[allow(non_camel_case_types)]
pub struct SQL_TIMEDATE_ADD_INTERVALS;
impl InfoType<SQL_TIMEDATE_ADD_INTERVALS, SQL_OV_ODBC3> for TimedateIntervals {}
unsafe impl Attr<SQL_TIMEDATE_ADD_INTERVALS> for TimedateIntervals {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TIMEDATE_ADD_INTERVALS> for TimedateIntervals {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 110)]
#[allow(non_camel_case_types)]
pub struct SQL_TIMEDATE_DIFF_INTERVALS;
impl InfoType<SQL_TIMEDATE_DIFF_INTERVALS, SQL_OV_ODBC3> for TimedateIntervals {}
unsafe impl Attr<SQL_TIMEDATE_DIFF_INTERVALS> for TimedateIntervals {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TIMEDATE_DIFF_INTERVALS> for TimedateIntervals {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 52)]
#[allow(non_camel_case_types)]
pub struct SQL_TIMEDATE_FUNCTIONS;
impl InfoType<SQL_TIMEDATE_FUNCTIONS, SQL_OV_ODBC3> for TimedateFunctions {}
unsafe impl Attr<SQL_TIMEDATE_FUNCTIONS> for TimedateFunctions {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_TIMEDATE_FUNCTIONS> for TimedateFunctions {}

/////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// Conversion Information /////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 53)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_BIGINT;
impl InfoType<SQL_CONVERT_BIGINT, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_BIGINT> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_BIGINT> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 54)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_BINARY;
impl InfoType<SQL_CONVERT_BINARY, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_BINARY> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_BINARY> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 55)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_BIT;
impl InfoType<SQL_CONVERT_BIT, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_BIT> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_BIT> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 56)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_CHAR;
impl InfoType<SQL_CONVERT_CHAR, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_CHAR> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_CHAR> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 57)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_DATE;
impl InfoType<SQL_CONVERT_DATE, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_DATE> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_DATE> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 58)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_DECIMAL;
impl InfoType<SQL_CONVERT_DECIMAL, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_DECIMAL> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_DECIMAL> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 59)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_DOUBLE;
impl InfoType<SQL_CONVERT_DOUBLE, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_DOUBLE> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_DOUBLE> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 60)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_FLOAT;
impl InfoType<SQL_CONVERT_FLOAT, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_FLOAT> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_FLOAT> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 61)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_INTEGER;
impl InfoType<SQL_CONVERT_INTEGER, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_INTEGER> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_INTEGER> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 123)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_INTERVAL_DAY_TIME;
impl InfoType<SQL_CONVERT_INTERVAL_DAY_TIME, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_INTERVAL_DAY_TIME> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_INTERVAL_DAY_TIME> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 124)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_INTERVAL_YEAR_MONTH;
impl InfoType<SQL_CONVERT_INTERVAL_YEAR_MONTH, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_INTERVAL_YEAR_MONTH> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_INTERVAL_YEAR_MONTH> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 71)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_LONGVARBINARY;
impl InfoType<SQL_CONVERT_LONGVARBINARY, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_LONGVARBINARY> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_LONGVARBINARY> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 62)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_LONGVARCHAR;
impl InfoType<SQL_CONVERT_LONGVARCHAR, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_LONGVARCHAR> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_LONGVARCHAR> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 63)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_NUMERIC;
impl InfoType<SQL_CONVERT_NUMERIC, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_NUMERIC> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_NUMERIC> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 64)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_REAL;
impl InfoType<SQL_CONVERT_REAL, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_REAL> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_REAL> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 65)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_SMALLINT;
impl InfoType<SQL_CONVERT_SMALLINT, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_SMALLINT> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_SMALLINT> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 66)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_TIME;
impl InfoType<SQL_CONVERT_TIME, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_TIME> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_TIME> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 67)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_TIMESTAMP;
impl InfoType<SQL_CONVERT_TIMESTAMP, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_TIMESTAMP> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_TIMESTAMP> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 68)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_TINYINT;
impl InfoType<SQL_CONVERT_TINYINT, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_TINYINT> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_TINYINT> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 69)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_VARBINARY;
impl InfoType<SQL_CONVERT_VARBINARY, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_VARBINARY> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_VARBINARY> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 70)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_VARCHAR;
impl InfoType<SQL_CONVERT_VARCHAR, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_VARCHAR> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_VARCHAR> for Conversion {}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 173)]
#[allow(non_camel_case_types)]
pub struct SQL_CONVERT_GUID;
impl InfoType<SQL_CONVERT_GUID, SQL_OV_ODBC3> for Conversion {}
unsafe impl Attr<SQL_CONVERT_GUID> for Conversion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl AttrGet<SQL_CONVERT_GUID> for Conversion {}

//=====================================================================================//

#[odbc_type(SQLUINTEGER)]
pub struct AsyncDbcFunctions;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: AsyncDbcFunctions = AsyncDbcFunctions(0x0000000);
pub const SQL_ASYNC_DBC_CAPABLE: AsyncDbcFunctions = AsyncDbcFunctions(0x00000001);

#[odbc_type(SQLUINTEGER)]
pub struct AsyncMode;
pub const SQL_AM_NONE: AsyncMode = AsyncMode(0);
pub const SQL_AM_CONNECTION: AsyncMode = AsyncMode(1);
pub const SQL_AM_STATEMENT: AsyncMode = AsyncMode(2);

#[odbc_type(SQLUINTEGER)]
pub struct AsyncNotification;
pub const SQL_ASYNC_NOTIFICATION_NOT_CAPABLE: AsyncNotification = AsyncNotification(0x00000000);
pub const SQL_ASYNC_NOTIFICATION_CAPABLE: AsyncNotification = AsyncNotification(0x00000001);

#[odbc_type(SQLUSMALLINT)]
pub struct ConcatNullBehavior;
pub const SQL_CB_NON_NULL: ConcatNullBehavior = ConcatNullBehavior(0x0000);
pub const SQL_CB_NULL: ConcatNullBehavior = ConcatNullBehavior(0x0001);

#[odbc_type(SQLUSMALLINT)]
pub struct CorrelationName;
pub const SQL_CN_NONE: CorrelationName = CorrelationName(0x0000);
pub const SQL_CN_DIFFERENT: CorrelationName = CorrelationName(0x0001);
pub const SQL_CN_ANY: CorrelationName = CorrelationName(0x0002);

#[odbc_type(SQLUINTEGER)]
pub struct CatalogLocation;
pub const SQL_CL_START: CatalogLocation = CatalogLocation(0x0001);
pub const SQL_CL_END: CatalogLocation = CatalogLocation(0x0002);

#[odbc_type(SQLUSMALLINT)]
pub struct CursorBehavior;
pub const SQL_CB_DELETE: CursorBehavior = CursorBehavior(0);
pub const SQL_CB_CLOSE: CursorBehavior = CursorBehavior(1);
pub const SQL_CB_PRESERVE: CursorBehavior = CursorBehavior(2);

#[odbc_type(SQLUINTEGER)]
pub struct CursorSensitivity;
pub const SQL_UNSPECIFIED: CursorSensitivity = CursorSensitivity(0);
pub const SQL_INSENSITIVE: CursorSensitivity = CursorSensitivity(1);
pub const SQL_SENSITIVE: CursorSensitivity = CursorSensitivity(2);

#[odbc_type(SQLUINTEGER)]
pub struct DdlIndex;
pub const SQL_DI_CREATE_INDEX: DdlIndex = DdlIndex(0x00000001);
pub const SQL_DI_DROP_INDEX: DdlIndex = DdlIndex(0x00000002);

#[odbc_type(SQLUSMALLINT)]
pub struct TxnCapable;
pub const SQL_TC_NONE: TxnCapable = TxnCapable(0);
pub const SQL_TC_DML: TxnCapable = TxnCapable(1);
pub const SQL_TC_ALL: TxnCapable = TxnCapable(2);
pub const SQL_TC_DDL_COMMIT: TxnCapable = TxnCapable(3);
pub const SQL_TC_DDL_IGNORE: TxnCapable = TxnCapable(4);

#[odbc_type(SQLUINTEGER)]
pub struct SqlConformance;
pub const SQL_SC_SQL92_ENTRY: SqlConformance = SqlConformance(0x00000001);
pub const SQL_SC_FIPS127_2_TRANSITIONAL: SqlConformance = SqlConformance(0x00000002);
pub const SQL92_INTERMEDIATE: SqlConformance = SqlConformance(0x00000004);
pub const SQL_SC_SQL92_FULL: SqlConformance = SqlConformance(0x00000008);

#[odbc_type(SQLUINTEGER)]
pub struct ParamArraySelects;
pub const SQL_PAS_BATCH: ParamArraySelects = ParamArraySelects(1);
pub const SQL_PAS_NO_BATCH: ParamArraySelects = ParamArraySelects(2);
pub const SQL_PAS_NO_SELECT: ParamArraySelects = ParamArraySelects(3);

#[odbc_type(SQLUINTEGER)]
pub struct ParamArrayRowCounts;
pub const SQL_PARC_BATCH: ParamArrayRowCounts = ParamArrayRowCounts(1);
pub const SQL_PARC_NO_BATCH: ParamArrayRowCounts = ParamArrayRowCounts(2);

#[odbc_type(SQLUINTEGER)]
pub struct OdbcInterfaceConformance;
pub const SQL_OIC_CORE: OdbcInterfaceConformance = OdbcInterfaceConformance(1);
pub const SQL_OIC_LEVEL1: OdbcInterfaceConformance = OdbcInterfaceConformance(2);
pub const SQL_OIC_LEVEL2: OdbcInterfaceConformance = OdbcInterfaceConformance(3);

#[odbc_type(SQLUSMALLINT)]
pub struct NonNullableColumns;
pub const SQL_NNC_NULL: NonNullableColumns = NonNullableColumns(0x0000);
pub const SQL_NNC_NON_NULL: NonNullableColumns = NonNullableColumns(0x0001);

#[odbc_type(SQLUSMALLINT)]
pub struct IdentifierCase;
pub const SQL_IC_UPPER: IdentifierCase = IdentifierCase(1);
pub const SQL_IC_LOWER: IdentifierCase = IdentifierCase(2);
pub const SQL_IC_SENSITIVE: IdentifierCase = IdentifierCase(3);
pub const SQL_IC_MIXED: IdentifierCase = IdentifierCase(4);

#[odbc_type(SQLUSMALLINT)]
pub struct GroupBy;
pub const SQL_GB_NOT_SUPPORTED: GroupBy = GroupBy(0x0000);
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: GroupBy = GroupBy(0x0001);
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: GroupBy = GroupBy(0x0002);
pub const SQL_GB_NO_RELATION: GroupBy = GroupBy(0x0003);
pub const SQL_GB_COLLATE: GroupBy = GroupBy(0x0004);

#[odbc_type(SQLUSMALLINT)]
pub struct FileUsage;
pub const SQL_FILE_NOT_SUPPORTED: FileUsage = FileUsage(0x0000);
pub const SQL_FILE_TABLE: FileUsage = FileUsage(0x0001);
pub const SQL_FILE_CATALOG: FileUsage = FileUsage(0x0002);

#[odbc_bitmask(SQLUINTEGER)]
pub struct BatchRowCount;
pub const SQL_BRC_PROCEDURES: BatchRowCount = BatchRowCount(0x0000001);
pub const SQL_BRC_EXPLICIT: BatchRowCount = BatchRowCount(0x0000002);
pub const SQL_BRC_ROLLED_UP: BatchRowCount = BatchRowCount(0x0000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct BatchSupport;
pub const SQL_BS_SELECT_EXPLICIT: BatchSupport = BatchSupport(0x00000001);
pub const SQL_BS_ROW_COUNT_EXPLICIT: BatchSupport = BatchSupport(0x00000002);
pub const SQL_BS_SELECT_PROC: BatchSupport = BatchSupport(0x00000004);
pub const SQL_BS_ROW_COUNT_PROC: BatchSupport = BatchSupport(0x00000008);

#[odbc_type(SQLUINTEGER)]
pub struct DriverAwarePoolingSupported;
pub const SQL_DRIVER_AWARE_POOLING_NOT_CAPABLE: DriverAwarePoolingSupported =
    DriverAwarePoolingSupported(0x00000000);
pub const SQL_DRIVER_AWARE_POOLING_CAPABLE: DriverAwarePoolingSupported =
    DriverAwarePoolingSupported(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CursorAttributes1;
pub const SQL_CA1_NEXT: CursorAttributes1 = CursorAttributes1(0x00000001);
pub const SQL_CA1_ABSOLUTE: CursorAttributes1 = CursorAttributes1(0x00000002);
pub const SQL_CA1_RELATIVE: CursorAttributes1 = CursorAttributes1(0x00000004);
pub const SQL_CA1_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00000008);

pub const SQL_CA1_LOCK_NO_CHANGE: CursorAttributes1 = CursorAttributes1(0x00000040);
pub const SQL_CA1_LOCK_EXCLUSIVE: CursorAttributes1 = CursorAttributes1(0x00000080);
pub const SQL_CA1_LOCK_UNLOCK: CursorAttributes1 = CursorAttributes1(0x00000100);

pub const SQL_CA1_POS_POSITION: CursorAttributes1 = CursorAttributes1(0x00000200);
pub const SQL_CA1_POS_UPDATE: CursorAttributes1 = CursorAttributes1(0x00000400);
pub const SQL_CA1_POS_DELETE: CursorAttributes1 = CursorAttributes1(0x00000800);
pub const SQL_CA1_POS_REFRESH: CursorAttributes1 = CursorAttributes1(0x00001000);

pub const SQL_CA1_POSITIONED_UPDATE: CursorAttributes1 = CursorAttributes1(0x00002000);
pub const SQL_CA1_POSITIONED_DELETE: CursorAttributes1 = CursorAttributes1(0x00004000);
pub const SQL_CA1_SELECT_FOR_UPDATE: CursorAttributes1 = CursorAttributes1(0x00008000);

pub const SQL_CA1_BULK_ADD: CursorAttributes1 = CursorAttributes1(0x00010000);
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00020000);
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00040000);
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00080000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CursorAttributes2;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000001);
pub const SQL_CA2_LOCK_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000002);
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000004);
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000008);

pub const SQL_CA2_SENSITIVITY_ADDITIONS: CursorAttributes2 = CursorAttributes2(0x00000010);
pub const SQL_CA2_SENSITIVITY_DELETIONS: CursorAttributes2 = CursorAttributes2(0x00000020);
pub const SQL_CA2_SENSITIVITY_UPDATES: CursorAttributes2 = CursorAttributes2(0x00000040);

pub const SQL_CA2_MAX_ROWS_SELECT: CursorAttributes2 = CursorAttributes2(0x00000080);
pub const SQL_CA2_MAX_ROWS_INSERT: CursorAttributes2 = CursorAttributes2(0x00000100);
pub const SQL_CA2_MAX_ROWS_DELETE: CursorAttributes2 = CursorAttributes2(0x00000200);
pub const SQL_CA2_MAX_ROWS_UPDATE: CursorAttributes2 = CursorAttributes2(0x00000400);
pub const SQL_CA2_MAX_ROWS_CATALOG: CursorAttributes2 = CursorAttributes2(0x00000800);
pub const SQL_CA2_MAX_ROWS_AFFECTS_ALL: CursorAttributes2 = CursorAttributes2(
    SQL_CA2_MAX_ROWS_SELECT.0
        | SQL_CA2_MAX_ROWS_INSERT.0
        | SQL_CA2_MAX_ROWS_DELETE.0
        | SQL_CA2_MAX_ROWS_UPDATE.0
        | SQL_CA2_MAX_ROWS_CATALOG.0,
);

pub const SQL_CA2_CRC_EXACT: CursorAttributes2 = CursorAttributes2(0x00001000);
pub const SQL_CA2_CRC_APPROXIMATE: CursorAttributes2 = CursorAttributes2(0x00002000);

pub const SQL_CA2_SIMULATE_NON_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00004000);
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00008000);
pub const SQL_CA2_SIMULATE_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00010000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct GetdataExtensions;
pub const SQL_GD_ANY_COLUMN: GetdataExtensions = GetdataExtensions(0x00000001);
pub const SQL_GD_ANY_ORDER: GetdataExtensions = GetdataExtensions(0x00000002);
pub const SQL_GD_BLOCK: GetdataExtensions = GetdataExtensions(0x00000004);
pub const SQL_GD_BOUND: GetdataExtensions = GetdataExtensions(0x00000008);
pub const SQL_GD_OUTPUT_PARAMS: GetdataExtensions = GetdataExtensions(0x00000010);
pub const SQL_GD_CONCURRENT: GetdataExtensions = GetdataExtensions(0x00000020);

#[odbc_bitmask(SQLUINTEGER)]
pub struct InfoSchemaViews;
pub const SQL_ISV_ASSERTIONS: InfoSchemaViews = InfoSchemaViews(0x00000001);
pub const SQL_ISV_CHARACTER_SETS: InfoSchemaViews = InfoSchemaViews(0x00000002);
pub const SQL_ISV_CHECK_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00000004);
pub const SQL_ISV_COLLATIONS: InfoSchemaViews = InfoSchemaViews(0x00000008);
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000010);
pub const SQL_ISV_COLUMN_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00000020);
pub const SQL_ISV_COLUMNS: InfoSchemaViews = InfoSchemaViews(0x00000040);
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000080);
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000100);
pub const SQL_ISV_DOMAIN_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00000200);
pub const SQL_ISV_DOMAINS: InfoSchemaViews = InfoSchemaViews(0x00000400);
pub const SQL_ISV_KEY_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000800);
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00001000);
pub const SQL_ISV_SCHEMATA: InfoSchemaViews = InfoSchemaViews(0x00002000);
pub const SQL_ISV_SQL_LANGUAGES: InfoSchemaViews = InfoSchemaViews(0x00004000);
pub const SQL_ISV_TABLE_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00008000);
pub const SQL_ISV_TABLE_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00010000);
pub const SQL_ISV_TABLES: InfoSchemaViews = InfoSchemaViews(0x00020000);
pub const SQL_ISV_TRANSLATIONS: InfoSchemaViews = InfoSchemaViews(0x00040000);
pub const SQL_ISV_USAGE_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00080000);
pub const SQL_ISV_VIEW_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00100000);
pub const SQL_ISV_VIEW_TABLE_USAGE: InfoSchemaViews = InfoSchemaViews(0x00200000);
pub const SQL_ISV_VIEWS: InfoSchemaViews = InfoSchemaViews(0x00400000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct BookmarkPersistence;
pub const SQL_BP_CLOSE: BookmarkPersistence = BookmarkPersistence(0x00000001);
pub const SQL_BP_DELETE: BookmarkPersistence = BookmarkPersistence(0x00000002);
pub const SQL_BP_DROP: BookmarkPersistence = BookmarkPersistence(0x00000004);
pub const SQL_BP_TRANSACTION: BookmarkPersistence = BookmarkPersistence(0x00000008);
pub const SQL_BP_UPDATE: BookmarkPersistence = BookmarkPersistence(0x00000010);
pub const SQL_BP_OTHER_HSTMT: BookmarkPersistence = BookmarkPersistence(0x00000020);
// TODO: should also be supported?
// pub const SQL_BP_SCROLL: BookmarkPersistence = BookmarkPersistence(0x00000040);

#[odbc_type(SQLUSMALLINT)]
pub struct NullCollation;
pub const SQL_NC_HIGH: NullCollation = NullCollation(0);
pub const SQL_NC_LOW: NullCollation = NullCollation(1);
pub const SQL_NC_START: NullCollation = NullCollation(0x0002);
pub const SQL_NC_END: NullCollation = NullCollation(0x0004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct ScrollOptions;
pub const SQL_SO_FORWARD_ONLY: ScrollOptions = ScrollOptions(0x00000001);
pub const SQL_SO_KEYSET_DRIVEN: ScrollOptions = ScrollOptions(0x00000002);
pub const SQL_SO_DYNAMIC: ScrollOptions = ScrollOptions(0x00000004);
pub const SQL_SO_MIXED: ScrollOptions = ScrollOptions(0x00000008);
pub const SQL_SO_STATIC: ScrollOptions = ScrollOptions(0x00000010);

// TODO: This is both an odbc type and bitmask
#[odbc_bitmask(SQLUINTEGER)]
pub struct TxnIsolation;
pub const SQL_TXN_READ_UNCOMMITTED: TxnIsolation = TxnIsolation(0x00000001);
pub const SQL_TXN_READ_COMMITTED: TxnIsolation = TxnIsolation(0x00000002);
pub const SQL_TXN_REPEATABLE_READ: TxnIsolation = TxnIsolation(0x00000004);
pub const SQL_TXN_SERIALIZABLE: TxnIsolation = TxnIsolation(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct AggregateFunctions;
pub const SQL_AF_AVG: AggregateFunctions = AggregateFunctions(0x00000001);
pub const SQL_AF_COUNT: AggregateFunctions = AggregateFunctions(0x00000002);
pub const SQL_AF_MAX: AggregateFunctions = AggregateFunctions(0x00000004);
pub const SQL_AF_MIN: AggregateFunctions = AggregateFunctions(0x00000008);
pub const SQL_AF_SUM: AggregateFunctions = AggregateFunctions(0x00000010);
pub const SQL_AF_DISTINCT: AggregateFunctions = AggregateFunctions(0x00000020);
pub const SQL_AF_ALL: AggregateFunctions = AggregateFunctions(0x00000040);
pub const SQL_AF_EVERY: AggregateFunctions = AggregateFunctions(0x00000080);
pub const SQL_AF_ANY: AggregateFunctions = AggregateFunctions(0x00000100);
pub const SQL_AF_STDEV_OP: AggregateFunctions = AggregateFunctions(0x00000200);
pub const SQL_AF_STDEV_SAMP: AggregateFunctions = AggregateFunctions(0x00000400);
pub const SQL_AF_VAR_SAMP: AggregateFunctions = AggregateFunctions(0x00000800);
pub const SQL_AF_VAR_POP: AggregateFunctions = AggregateFunctions(0x00001000);
pub const SQL_AF_ARRAY_AGG: AggregateFunctions = AggregateFunctions(0x00002000);
pub const SQL_AF_COLLECT: AggregateFunctions = AggregateFunctions(0x00004000);
pub const SQL_AF_FUSION: AggregateFunctions = AggregateFunctions(0x00008000);
pub const SQL_AF_INTERSECTION: AggregateFunctions = AggregateFunctions(0x00010000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct AlterDomain;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: AlterDomain = AlterDomain(0x00000001);
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: AlterDomain = AlterDomain(0x00000002);
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: AlterDomain = AlterDomain(0x00000004);
pub const SQL_AD_ADD_DOMAIN_DEFAULT: AlterDomain = AlterDomain(0x00000008);
pub const SQL_AD_DROP_DOMAIN_DEFAULT: AlterDomain = AlterDomain(0x00000010);
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: AlterDomain = AlterDomain(0x00000020);
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: AlterDomain = AlterDomain(0x00000040);
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: AlterDomain = AlterDomain(0x00000080);
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: AlterDomain = AlterDomain(0x00000100);

#[odbc_bitmask(SQLUINTEGER)]
pub struct AlterTable;
// TODO: Are these two to be supported
//pub const SQL_AT_ADD_COLUMN: AlterTable = AlterTable(0x00000001);
//pub const SQL_AT_DROP_COLUMN: AlterTable = AlterTable(0x00000002);
pub const SQL_AT_ADD_CONSTRAINT: AlterTable = AlterTable(0x00000008);
pub const SQL_AT_ADD_COLUMN_SINGLE: AlterTable = AlterTable(0x00000020);
pub const SQL_AT_ADD_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000040);
pub const SQL_AT_ADD_COLUMN_COLLATION: AlterTable = AlterTable(0x00000080);
pub const SQL_AT_SET_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000100);
pub const SQL_AT_DROP_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000200);
pub const SQL_AT_DROP_COLUMN_CASCADE: AlterTable = AlterTable(0x00000400);
pub const SQL_AT_DROP_COLUMN_RESTRICT: AlterTable = AlterTable(0x00000800);
pub const SQL_AT_ADD_TABLE_CONSTRAINT: AlterTable = AlterTable(0x00001000);
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: AlterTable = AlterTable(0x00002000);
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: AlterTable = AlterTable(0x00004000);
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: AlterTable = AlterTable(0x00008000);
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: AlterTable = AlterTable(0x00010000);
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: AlterTable = AlterTable(0x00020000);
pub const SQL_AT_CONSTRAINT_DEFERRABLE: AlterTable = AlterTable(0x00040000);
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: AlterTable = AlterTable(0x00080000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CatalogUsage;
pub const SQL_CU_DML_STATEMENTS: CatalogUsage = CatalogUsage(0x00000001);
pub const SQL_CU_PROCEDURE_INVOCATION: CatalogUsage = CatalogUsage(0x00000002);
pub const SQL_CU_TABLE_DEFINITION: CatalogUsage = CatalogUsage(0x00000004);
pub const SQL_CU_INDEX_DEFINITION: CatalogUsage = CatalogUsage(0x00000008);
pub const SQL_CU_PRIVILEGE_DEFINITION: CatalogUsage = CatalogUsage(0x00000010);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateAssertion;
pub const SQL_CA_CREATE_ASSERTION: CreateAssertion = CreateAssertion(0x00000001);
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: CreateAssertion = CreateAssertion(0x00000010);
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: CreateAssertion = CreateAssertion(0x00000020);
pub const SQL_CA_CONSTRAINT_DEFERRABLE: CreateAssertion = CreateAssertion(0x00000040);
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: CreateAssertion = CreateAssertion(0x00000080);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateCharacterSet;
pub const SQL_CCS_CREATE_CHARACTER_SET: CreateCharacterSet = CreateCharacterSet(0x00000001);
pub const SQL_CCS_COLLATE_CLAUSE: CreateCharacterSet = CreateCharacterSet(0x00000002);
pub const SQL_CCS_LIMITED_COLLATION: CreateCharacterSet = CreateCharacterSet(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateCollation;
pub const SQL_CCOL_CREATE_COLLATION: CreateCollation = CreateCollation(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateDomain;
pub const SQL_CDO_CREATE_DOMAIN: CreateDomain = CreateDomain(0x00000001);
pub const SQL_CDO_DEFAULT: CreateDomain = CreateDomain(0x00000002);
pub const SQL_CDO_CONSTRAINT: CreateDomain = CreateDomain(0x00000004);
pub const SQL_CDO_COLLATION: CreateDomain = CreateDomain(0x00000008);
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: CreateDomain = CreateDomain(0x00000010);
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: CreateDomain = CreateDomain(0x00000020);
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: CreateDomain = CreateDomain(0x00000040);
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: CreateDomain = CreateDomain(0x00000080);
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: CreateDomain = CreateDomain(0x00000100);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateSchema;
pub const SQL_CS_CREATE_SCHEMA: CreateSchema = CreateSchema(0x00000001);
pub const SQL_CS_AUTHORIZATION: CreateSchema = CreateSchema(0x00000002);
pub const SQL_CS_DEFAULT_CHARACTER_SET: CreateSchema = CreateSchema(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateTable;
pub const SQL_CT_CREATE_TABLE: CreateTable = CreateTable(0x00000001);
pub const SQL_CT_COMMIT_PRESERVE: CreateTable = CreateTable(0x00000002);
pub const SQL_CT_COMMIT_DELETE: CreateTable = CreateTable(0x00000004);
pub const SQL_CT_GLOBAL_TEMPORARY: CreateTable = CreateTable(0x00000008);
pub const SQL_CT_LOCAL_TEMPORARY: CreateTable = CreateTable(0x00000010);
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: CreateTable = CreateTable(0x00000020);
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: CreateTable = CreateTable(0x00000040);
pub const SQL_CT_CONSTRAINT_DEFERRABLE: CreateTable = CreateTable(0x00000080);
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: CreateTable = CreateTable(0x00000100);
pub const SQL_CT_COLUMN_CONSTRAINT: CreateTable = CreateTable(0x00000200);
pub const SQL_CT_COLUMN_DEFAULT: CreateTable = CreateTable(0x00000400);
pub const SQL_CT_COLUMN_COLLATION: CreateTable = CreateTable(0x00000800);
pub const SQL_CT_TABLE_CONSTRAINT: CreateTable = CreateTable(0x00001000);
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: CreateTable = CreateTable(0x00002000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateTranslation;
pub const SQL_CTR_CREATE_TRANSLATION: CreateTranslation = CreateTranslation(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct CreateView;
pub const SQL_CV_CREATE_VIEW: CreateView = CreateView(0x00000001);
pub const SQL_CV_CHECK_OPTION: CreateView = CreateView(0x00000002);
pub const SQL_CV_CASCADED: CreateView = CreateView(0x00000004);
pub const SQL_CV_LOCAL: CreateView = CreateView(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropAssertion;
pub const SQL_DA_DROP_ASSERTION: DropAssertion = DropAssertion(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Conversion;
pub const SQL_CVT_CHAR: Conversion = Conversion(0x00000001);
pub const SQL_CVT_NUMERIC: Conversion = Conversion(0x00000002);
pub const SQL_CVT_DECIMAL: Conversion = Conversion(0x00000004);
pub const SQL_CVT_INTEGER: Conversion = Conversion(0x00000008);
pub const SQL_CVT_SMALLINT: Conversion = Conversion(0x00000010);
pub const SQL_CVT_FLOAT: Conversion = Conversion(0x00000020);
pub const SQL_CVT_REAL: Conversion = Conversion(0x00000040);
pub const SQL_CVT_DOUBLE: Conversion = Conversion(0x00000080);
pub const SQL_CVT_VARCHAR: Conversion = Conversion(0x00000100);
pub const SQL_CVT_LONGVARCHAR: Conversion = Conversion(0x00000200);
pub const SQL_CVT_BINARY: Conversion = Conversion(0x00000400);
pub const SQL_CVT_VARBINARY: Conversion = Conversion(0x00000800);
pub const SQL_CVT_BIT: Conversion = Conversion(0x00001000);
pub const SQL_CVT_TINYINT: Conversion = Conversion(0x00002000);
pub const SQL_CVT_BIGINT: Conversion = Conversion(0x00004000);
pub const SQL_CVT_DATE: Conversion = Conversion(0x00008000);
pub const SQL_CVT_TIME: Conversion = Conversion(0x00010000);
pub const SQL_CVT_TIMESTAMP: Conversion = Conversion(0x00020000);
pub const SQL_CVT_LONGVARBINARY: Conversion = Conversion(0x00040000);

pub const SQL_CVT_INTERVAL_YEAR_MONTH: Conversion = Conversion(0x00080000);
pub const SQL_CVT_INTERVAL_DAY_TIME: Conversion = Conversion(0x00100000);

#[cfg(feature = "v3_5")]
pub const SQL_CVT_GUID: Conversion = Conversion(0x01000000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropCharacterSet;
pub const SQL_DCS_DROP_CHARACTER_SET: DropCharacterSet = DropCharacterSet(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropCollation;
pub const SQL_DC_DROP_COLLATION: DropCollation = DropCollation(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropDomain;
pub const SQL_DD_DROP_DOMAIN: DropDomain = DropDomain(0x00000001);
pub const SQL_DD_RESTRICT: DropDomain = DropDomain(0x00000002);
pub const SQL_DD_CASCADE: DropDomain = DropDomain(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropSchema;
pub const SQL_DS_DROP_SCHEMA: DropSchema = DropSchema(0x00000001);
pub const SQL_DS_RESTRICT: DropSchema = DropSchema(0x00000002);
pub const SQL_DS_CASCADE: DropSchema = DropSchema(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropTable;
pub const SQL_DT_DROP_TABLE: DropTable = DropTable(0x00000001);
pub const SQL_DT_RESTRICT: DropTable = DropTable(0x00000002);
pub const SQL_DT_CASCADE: DropTable = DropTable(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropTranslation;
pub const SQL_DTR_DROP_TRANSLATION: DropTranslation = DropTranslation(0x00000001);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DropView;
pub const SQL_DV_DROP_VIEW: DropView = DropView(0x00000001);
pub const SQL_DV_RESTRICT: DropView = DropView(0x00000002);
pub const SQL_DV_CASCADE: DropView = DropView(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct IndexKeywords;
pub const SQL_IK_NONE: IndexKeywords = IndexKeywords(0x00000000);
pub const SQL_IK_ASC: IndexKeywords = IndexKeywords(0x00000001);
pub const SQL_IK_DESC: IndexKeywords = IndexKeywords(0x00000002);
pub const SQL_IK_ALL: IndexKeywords = IndexKeywords(SQL_IK_ASC.0 | SQL_IK_DESC.0);

#[odbc_bitmask(SQLUINTEGER)]
pub struct InsertStatement;
pub const SQL_IS_INSERT_LITERALS: InsertStatement = InsertStatement(0x00000001);
pub const SQL_IS_INSERT_SEARCHED: InsertStatement = InsertStatement(0x00000002);
pub const SQL_IS_SELECT_INTO: InsertStatement = InsertStatement(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct OjCapabilities;
pub const SQL_OJ_LEFT: OjCapabilities = OjCapabilities(0x00000001);
pub const SQL_OJ_RIGHT: OjCapabilities = OjCapabilities(0x00000002);
pub const SQL_OJ_FULL: OjCapabilities = OjCapabilities(0x00000004);
pub const SQL_OJ_NESTED: OjCapabilities = OjCapabilities(0x00000008);
pub const SQL_OJ_NOT_ORDERED: OjCapabilities = OjCapabilities(0x00000010);
pub const SQL_OJ_INNER: OjCapabilities = OjCapabilities(0x00000020);
pub const SQL_OJ_ALL_COMPARISON_OPS: OjCapabilities = OjCapabilities(0x00000040);

#[odbc_bitmask(SQLUINTEGER)]
pub struct OuterJoins;

#[odbc_bitmask(SQLUINTEGER)]
pub struct SchemaUsage;
pub const SQL_SU_DML_STATEMENTS: SchemaUsage = SchemaUsage(0x00000001);
pub const SQL_SU_PROCEDURE_INVOCATION: SchemaUsage = SchemaUsage(0x00000002);
pub const SQL_SU_TABLE_DEFINITION: SchemaUsage = SchemaUsage(0x00000004);
pub const SQL_SU_INDEX_DEFINITION: SchemaUsage = SchemaUsage(0x00000008);
pub const SQL_SU_PRIVILEGE_DEFINITION: SchemaUsage = SchemaUsage(0x00000010);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Subqueries;
pub const SQL_SQ_COMPARISON: Subqueries = Subqueries(0x00000001);
pub const SQL_SQ_EXISTS: Subqueries = Subqueries(0x00000002);
pub const SQL_SQ_IN: Subqueries = Subqueries(0x00000004);
pub const SQL_SQ_QUANTIFIED: Subqueries = Subqueries(0x00000008);
pub const SQL_SQ_CORRELATED_SUBQUERIES: Subqueries = Subqueries(0x00000010);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Union;
pub const SQL_U_UNION: Union = Union(0x00000001);
pub const SQL_U_UNION_ALL: Union = Union(0x00000002);

#[odbc_bitmask(SQLUINTEGER)]
pub struct ConvertFunctions;
pub const SQL_FN_CVT_CONVERT: ConvertFunctions = ConvertFunctions(0x00000001);
pub const SQL_FN_CVT_CAST: ConvertFunctions = ConvertFunctions(0x00000002);

#[odbc_bitmask(SQLUINTEGER)]
pub struct NumericFunctions;
pub const SQL_FN_NUM_ABS: NumericFunctions = NumericFunctions(0x00000001);
pub const SQL_FN_NUM_ACOS: NumericFunctions = NumericFunctions(0x00000002);
pub const SQL_FN_NUM_ASIN: NumericFunctions = NumericFunctions(0x00000004);
pub const SQL_FN_NUM_ATAN: NumericFunctions = NumericFunctions(0x00000008);
pub const SQL_FN_NUM_ATAN2: NumericFunctions = NumericFunctions(0x00000010);
pub const SQL_FN_NUM_CEILING: NumericFunctions = NumericFunctions(0x00000020);
pub const SQL_FN_NUM_COS: NumericFunctions = NumericFunctions(0x00000040);
pub const SQL_FN_NUM_COT: NumericFunctions = NumericFunctions(0x00000080);
pub const SQL_FN_NUM_EXP: NumericFunctions = NumericFunctions(0x00000100);
pub const SQL_FN_NUM_FLOOR: NumericFunctions = NumericFunctions(0x00000200);
pub const SQL_FN_NUM_LOG: NumericFunctions = NumericFunctions(0x00000400);
pub const SQL_FN_NUM_MOD: NumericFunctions = NumericFunctions(0x00000800);
pub const SQL_FN_NUM_SIGN: NumericFunctions = NumericFunctions(0x00001000);
pub const SQL_FN_NUM_SIN: NumericFunctions = NumericFunctions(0x00002000);
pub const SQL_FN_NUM_SQRT: NumericFunctions = NumericFunctions(0x00004000);
pub const SQL_FN_NUM_TAN: NumericFunctions = NumericFunctions(0x00008000);
pub const SQL_FN_NUM_PI: NumericFunctions = NumericFunctions(0x00010000);
pub const SQL_FN_NUM_RAND: NumericFunctions = NumericFunctions(0x00020000);
pub const SQL_FN_NUM_DEGREES: NumericFunctions = NumericFunctions(0x00040000);
pub const SQL_FN_NUM_LOG10: NumericFunctions = NumericFunctions(0x00080000);
pub const SQL_FN_NUM_POWER: NumericFunctions = NumericFunctions(0x00100000);
pub const SQL_FN_NUM_RADIANS: NumericFunctions = NumericFunctions(0x00200000);
pub const SQL_FN_NUM_ROUND: NumericFunctions = NumericFunctions(0x00400000);
pub const SQL_FN_NUM_TRUNCATE: NumericFunctions = NumericFunctions(0x00800000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct StringFunctions;
pub const SQL_FN_STR_CONCAT: StringFunctions = StringFunctions(0x00000001);
pub const SQL_FN_STR_INSERT: StringFunctions = StringFunctions(0x00000002);
pub const SQL_FN_STR_LEFT: StringFunctions = StringFunctions(0x00000004);
pub const SQL_FN_STR_LTRIM: StringFunctions = StringFunctions(0x00000008);
pub const SQL_FN_STR_LENGTH: StringFunctions = StringFunctions(0x00000010);
pub const SQL_FN_STR_LOCATE: StringFunctions = StringFunctions(0x00000020);
pub const SQL_FN_STR_LCASE: StringFunctions = StringFunctions(0x00000040);
pub const SQL_FN_STR_REPEAT: StringFunctions = StringFunctions(0x00000080);
pub const SQL_FN_STR_REPLACE: StringFunctions = StringFunctions(0x00000100);
pub const SQL_FN_STR_RIGHT: StringFunctions = StringFunctions(0x00000200);
pub const SQL_FN_STR_RTRIM: StringFunctions = StringFunctions(0x00000400);
pub const SQL_FN_STR_SUBSTRING: StringFunctions = StringFunctions(0x00000800);
pub const SQL_FN_STR_UCASE: StringFunctions = StringFunctions(0x00001000);
pub const SQL_FN_STR_ASCII: StringFunctions = StringFunctions(0x00002000);
pub const SQL_FN_STR_CHAR: StringFunctions = StringFunctions(0x00004000);
pub const SQL_FN_STR_DIFFERENCE: StringFunctions = StringFunctions(0x00008000);
pub const SQL_FN_STR_LOCATE_2: StringFunctions = StringFunctions(0x00010000);
pub const SQL_FN_STR_SOUNDEX: StringFunctions = StringFunctions(0x00020000);
pub const SQL_FN_STR_SPACE: StringFunctions = StringFunctions(0x00040000);
pub const SQL_FN_STR_BIT_LENGTH: StringFunctions = StringFunctions(0x00080000);
pub const SQL_FN_STR_CHAR_LENGTH: StringFunctions = StringFunctions(0x00100000);
pub const SQL_FN_STR_CHARACTER_LENGTH: StringFunctions = StringFunctions(0x00200000);
pub const SQL_FN_STR_OCTET_LENGTH: StringFunctions = StringFunctions(0x00400000);
pub const SQL_FN_STR_POSITION: StringFunctions = StringFunctions(0x00800000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct SystemFunctions;
pub const SQL_FN_SYS_USERNAME: SystemFunctions = SystemFunctions(0x00000001);
pub const SQL_FN_SYS_DBNAME: SystemFunctions = SystemFunctions(0x00000002);
pub const SQL_FN_SYS_IFNULL: SystemFunctions = SystemFunctions(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct TimedateIntervals;
pub const SQL_FN_TSI_FRAC_SECOND: TimedateIntervals = TimedateIntervals(0x00000001);
pub const SQL_FN_TSI_SECOND: TimedateIntervals = TimedateIntervals(0x00000002);
pub const SQL_FN_TSI_MINUTE: TimedateIntervals = TimedateIntervals(0x00000004);
pub const SQL_FN_TSI_HOUR: TimedateIntervals = TimedateIntervals(0x00000008);
pub const SQL_FN_TSI_DAY: TimedateIntervals = TimedateIntervals(0x00000010);
pub const SQL_FN_TSI_WEEK: TimedateIntervals = TimedateIntervals(0x00000020);
pub const SQL_FN_TSI_MONTH: TimedateIntervals = TimedateIntervals(0x00000040);
pub const SQL_FN_TSI_QUARTER: TimedateIntervals = TimedateIntervals(0x00000080);
pub const SQL_FN_TSI_YEAR: TimedateIntervals = TimedateIntervals(0x00000100);

#[odbc_bitmask(SQLUINTEGER)]
pub struct TimedateFunctions;
pub const SQL_FN_TD_NOW: TimedateFunctions = TimedateFunctions(0x00000001);
pub const SQL_FN_TD_CURDATE: TimedateFunctions = TimedateFunctions(0x00000002);
pub const SQL_FN_TD_DAYOFMONTH: TimedateFunctions = TimedateFunctions(0x00000004);
pub const SQL_FN_TD_DAYOFWEEK: TimedateFunctions = TimedateFunctions(0x00000008);
pub const SQL_FN_TD_DAYOFYEAR: TimedateFunctions = TimedateFunctions(0x00000010);
pub const SQL_FN_TD_MONTH: TimedateFunctions = TimedateFunctions(0x00000020);
pub const SQL_FN_TD_QUARTER: TimedateFunctions = TimedateFunctions(0x00000040);
pub const SQL_FN_TD_WEEK: TimedateFunctions = TimedateFunctions(0x00000080);
pub const SQL_FN_TD_YEAR: TimedateFunctions = TimedateFunctions(0x00000100);
pub const SQL_FN_TD_CURTIME: TimedateFunctions = TimedateFunctions(0x00000200);
pub const SQL_FN_TD_HOUR: TimedateFunctions = TimedateFunctions(0x00000400);
pub const SQL_FN_TD_MINUTE: TimedateFunctions = TimedateFunctions(0x00000800);
pub const SQL_FN_TD_SECOND: TimedateFunctions = TimedateFunctions(0x00001000);
pub const SQL_FN_TD_TIMESTAMPADD: TimedateFunctions = TimedateFunctions(0x00002000);
pub const SQL_FN_TD_TIMESTAMPDIFF: TimedateFunctions = TimedateFunctions(0x00004000);
pub const SQL_FN_TD_DAYNAME: TimedateFunctions = TimedateFunctions(0x00008000);
pub const SQL_FN_TD_MONTHNAME: TimedateFunctions = TimedateFunctions(0x00010000);
pub const SQL_FN_TD_CURRENT_DATE: TimedateFunctions = TimedateFunctions(0x00020000);
pub const SQL_FN_TD_CURRENT_TIME: TimedateFunctions = TimedateFunctions(0x00040000);
pub const SQL_FN_TD_CURRENT_TIMESTAMP: TimedateFunctions = TimedateFunctions(0x00080000);
pub const SQL_FN_TD_EXTRACT: TimedateFunctions = TimedateFunctions(0x00100000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92DatetimeFunctions;
pub const SQL_SDF_CURRENT_DATE: Sql92DatetimeFunctions = Sql92DatetimeFunctions(0x00000001);
pub const SQL_SDF_CURRENT_TIME: Sql92DatetimeFunctions = Sql92DatetimeFunctions(0x00000002);
pub const SQL_SDF_CURRENT_TIMESTAMP: Sql92DatetimeFunctions = Sql92DatetimeFunctions(0x00000004);

#[odbc_bitmask(SQLUINTEGER)]
pub struct DatetimeLiterals;
pub const SQL_DL_SQL92_DATE: DatetimeLiterals = DatetimeLiterals(0x00000001);
pub const SQL_DL_SQL92_TIME: DatetimeLiterals = DatetimeLiterals(0x00000002);
pub const SQL_DL_SQL92_TIMESTAMP: DatetimeLiterals = DatetimeLiterals(0x00000004);
pub const SQL_DL_SQL92_INTERVAL_YEAR: DatetimeLiterals = DatetimeLiterals(0x00000008);
pub const SQL_DL_SQL92_INTERVAL_MONTH: DatetimeLiterals = DatetimeLiterals(0x00000010);
pub const SQL_DL_SQL92_INTERVAL_DAY: DatetimeLiterals = DatetimeLiterals(0x00000020);
pub const SQL_DL_SQL92_INTERVAL_HOUR: DatetimeLiterals = DatetimeLiterals(0x00000040);
pub const SQL_DL_SQL92_INTERVAL_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00000080);
pub const SQL_DL_SQL92_INTERVAL_SECOND: DatetimeLiterals = DatetimeLiterals(0x00000100);
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: DatetimeLiterals = DatetimeLiterals(0x00000200);
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: DatetimeLiterals = DatetimeLiterals(0x00000400);
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00000800);
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00001000);
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00002000);
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00004000);
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00008000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92ForeignKeyDeleteRule;
pub const SQL_SFKD_CASCADE: Sql92ForeignKeyDeleteRule = Sql92ForeignKeyDeleteRule(0x00000001);
pub const SQL_SFKD_NO_ACTION: Sql92ForeignKeyDeleteRule = Sql92ForeignKeyDeleteRule(0x00000002);
pub const SQL_SFKD_SET_DEFAULT: Sql92ForeignKeyDeleteRule = Sql92ForeignKeyDeleteRule(0x00000004);
pub const SQL_SFKD_SET_NULL: Sql92ForeignKeyDeleteRule = Sql92ForeignKeyDeleteRule(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92ForeignKeyUpdateRule;
pub const SQL_SFKU_CASCADE: Sql92ForeignKeyUpdateRule = Sql92ForeignKeyUpdateRule(0x00000001);
pub const SQL_SFKU_NO_ACTION: Sql92ForeignKeyUpdateRule = Sql92ForeignKeyUpdateRule(0x00000002);
pub const SQL_SFKU_SET_DEFAULT: Sql92ForeignKeyUpdateRule = Sql92ForeignKeyUpdateRule(0x00000004);
pub const SQL_SFKU_SET_NULL: Sql92ForeignKeyUpdateRule = Sql92ForeignKeyUpdateRule(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92Grant;
pub const SQL_SG_USAGE_ON_DOMAIN: Sql92Grant = Sql92Grant(0x00000001);
pub const SQL_SG_USAGE_ON_CHARACTER_SET: Sql92Grant = Sql92Grant(0x00000002);
pub const SQL_SG_USAGE_ON_COLLATION: Sql92Grant = Sql92Grant(0x00000004);
pub const SQL_SG_USAGE_ON_TRANSLATION: Sql92Grant = Sql92Grant(0x00000008);
pub const SQL_SG_WITH_GRANT_OPTION: Sql92Grant = Sql92Grant(0x00000010);
pub const SQL_SG_DELETE_TABLE: Sql92Grant = Sql92Grant(0x00000020);
pub const SQL_SG_INSERT_TABLE: Sql92Grant = Sql92Grant(0x00000040);
pub const SQL_SG_INSERT_COLUMN: Sql92Grant = Sql92Grant(0x00000080);
pub const SQL_SG_REFERENCES_TABLE: Sql92Grant = Sql92Grant(0x00000100);
pub const SQL_SG_REFERENCES_COLUMN: Sql92Grant = Sql92Grant(0x00000200);
pub const SQL_SG_SELECT_TABLE: Sql92Grant = Sql92Grant(0x00000400);
pub const SQL_SG_UPDATE_TABLE: Sql92Grant = Sql92Grant(0x00000800);
pub const SQL_SG_UPDATE_COLUMN: Sql92Grant = Sql92Grant(0x00001000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92NumericValueFunctions;
pub const SQL_SNVF_BIT_LENGTH: Sql92NumericValueFunctions = Sql92NumericValueFunctions(0x00000001);
pub const SQL_SNVF_CHAR_LENGTH: Sql92NumericValueFunctions = Sql92NumericValueFunctions(0x00000002);
pub const SQL_SNVF_CHARACTER_LENGTH: Sql92NumericValueFunctions =
    Sql92NumericValueFunctions(0x00000004);
pub const SQL_SNVF_EXTRACT: Sql92NumericValueFunctions = Sql92NumericValueFunctions(0x00000008);
pub const SQL_SNVF_OCTET_LENGTH: Sql92NumericValueFunctions =
    Sql92NumericValueFunctions(0x00000010);
pub const SQL_SNVF_POSITION: Sql92NumericValueFunctions = Sql92NumericValueFunctions(0x00000020);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92Predicates;
pub const SQL_SP_EXISTS: Sql92Predicates = Sql92Predicates(0x00000001);
pub const SQL_SP_ISNOTNULL: Sql92Predicates = Sql92Predicates(0x00000002);
pub const SQL_SP_ISNULL: Sql92Predicates = Sql92Predicates(0x00000004);
pub const SQL_SP_MATCH_FULL: Sql92Predicates = Sql92Predicates(0x00000008);
pub const SQL_SP_MATCH_PARTIAL: Sql92Predicates = Sql92Predicates(0x00000010);
pub const SQL_SP_MATCH_UNIQUE_FULL: Sql92Predicates = Sql92Predicates(0x00000020);
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: Sql92Predicates = Sql92Predicates(0x00000040);
pub const SQL_SP_OVERLAPS: Sql92Predicates = Sql92Predicates(0x00000080);
pub const SQL_SP_UNIQUE: Sql92Predicates = Sql92Predicates(0x00000100);
pub const SQL_SP_LIKE: Sql92Predicates = Sql92Predicates(0x00000200);
pub const SQL_SP_IN: Sql92Predicates = Sql92Predicates(0x00000400);
pub const SQL_SP_BETWEEN: Sql92Predicates = Sql92Predicates(0x00000800);
pub const SQL_SP_COMPARISON: Sql92Predicates = Sql92Predicates(0x00001000);
pub const SQL_SP_QUANTIFIED_COMPARISON: Sql92Predicates = Sql92Predicates(0x00002000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92RelationalJoinOperators;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000001);
pub const SQL_SRJO_CROSS_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000002);
pub const SQL_SRJO_EXCEPT_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000004);
pub const SQL_SRJO_FULL_OUTER_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000008);
pub const SQL_SRJO_INNER_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000010);
pub const SQL_SRJO_INTERSECT_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000020);
pub const SQL_SRJO_LEFT_OUTER_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000040);
pub const SQL_SRJO_NATURAL_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000080);
pub const SQL_SRJO_RIGHT_OUTER_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000100);
pub const SQL_SRJO_UNION_JOIN: Sql92RelationalJoinOperators =
    Sql92RelationalJoinOperators(0x00000200);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92Revoke;
pub const SQL_SR_USAGE_ON_DOMAIN: Sql92Revoke = Sql92Revoke(0x00000001);
pub const SQL_SR_USAGE_ON_CHARACTER_SET: Sql92Revoke = Sql92Revoke(0x00000002);
pub const SQL_SR_USAGE_ON_COLLATION: Sql92Revoke = Sql92Revoke(0x00000004);
pub const SQL_SR_USAGE_ON_TRANSLATION: Sql92Revoke = Sql92Revoke(0x00000008);
pub const SQL_SR_GRANT_OPTION_FOR: Sql92Revoke = Sql92Revoke(0x00000010);
pub const SQL_SR_CASCADE: Sql92Revoke = Sql92Revoke(0x00000020);
pub const SQL_SR_RESTRICT: Sql92Revoke = Sql92Revoke(0x00000040);
pub const SQL_SR_DELETE_TABLE: Sql92Revoke = Sql92Revoke(0x00000080);
pub const SQL_SR_INSERT_TABLE: Sql92Revoke = Sql92Revoke(0x00000100);
pub const SQL_SR_INSERT_COLUMN: Sql92Revoke = Sql92Revoke(0x00000200);
pub const SQL_SR_REFERENCES_TABLE: Sql92Revoke = Sql92Revoke(0x00000400);
pub const SQL_SR_REFERENCES_COLUMN: Sql92Revoke = Sql92Revoke(0x00000800);
pub const SQL_SR_SELECT_TABLE: Sql92Revoke = Sql92Revoke(0x00001000);
pub const SQL_SR_UPDATE_TABLE: Sql92Revoke = Sql92Revoke(0x00002000);
pub const SQL_SR_UPDATE_COLUMN: Sql92Revoke = Sql92Revoke(0x00004000);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92RowValueConstructor;
pub const SQL_SRVC_VALUE_EXPRESSION: Sql92RowValueConstructor =
    Sql92RowValueConstructor(0x00000001);
pub const SQL_SRVC_NULL: Sql92RowValueConstructor = Sql92RowValueConstructor(0x00000002);
pub const SQL_SRVC_DEFAULT: Sql92RowValueConstructor = Sql92RowValueConstructor(0x00000004);
pub const SQL_SRVC_ROW_SUBQUERY: Sql92RowValueConstructor = Sql92RowValueConstructor(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92StringFunctions;
pub const SQL_SSF_CONVERT: Sql92StringFunctions = Sql92StringFunctions(0x00000001);
pub const SQL_SSF_LOWER: Sql92StringFunctions = Sql92StringFunctions(0x00000002);
pub const SQL_SSF_UPPER: Sql92StringFunctions = Sql92StringFunctions(0x00000004);
pub const SQL_SSF_SUBSTRING: Sql92StringFunctions = Sql92StringFunctions(0x00000008);
pub const SQL_SSF_TRANSLATE: Sql92StringFunctions = Sql92StringFunctions(0x00000010);
pub const SQL_SSF_TRIM_BOTH: Sql92StringFunctions = Sql92StringFunctions(0x00000020);
pub const SQL_SSF_TRIM_LEADING: Sql92StringFunctions = Sql92StringFunctions(0x00000040);
pub const SQL_SSF_TRIM_TRAILING: Sql92StringFunctions = Sql92StringFunctions(0x00000080);
pub const SQL_SSF_OVERLAY: Sql92StringFunctions = Sql92StringFunctions(0x00000100);
pub const SQL_SSF_LENGTH: Sql92StringFunctions = Sql92StringFunctions(0x00000200);
pub const SQL_SSF_POSITION: Sql92StringFunctions = Sql92StringFunctions(0x00000400);
pub const SQL_SSF_CONCAT: Sql92StringFunctions = Sql92StringFunctions(0x00000800);

#[odbc_bitmask(SQLUINTEGER)]
pub struct Sql92ValueExpressions;
pub const SQL_SVE_CASE: Sql92ValueExpressions = Sql92ValueExpressions(0x00000001);
pub const SQL_SVE_CAST: Sql92ValueExpressions = Sql92ValueExpressions(0x00000002);
pub const SQL_SVE_COALESCE: Sql92ValueExpressions = Sql92ValueExpressions(0x00000004);
pub const SQL_SVE_NULLIF: Sql92ValueExpressions = Sql92ValueExpressions(0x00000008);

#[odbc_bitmask(SQLUINTEGER)]
pub struct StandardCliConformance;
pub const SQL_SCC_XOPEN_CLI_VERSION1: StandardCliConformance = StandardCliConformance(0x00000001);
pub const SQL_SCC_ISO92_CLI: StandardCliConformance = StandardCliConformance(0x00000002);

#[odbc_bitmask(SQLUINTEGER)]
pub struct BinaryFunctions;
pub const SQL_FN_BIN_BIT_LENGTH: BinaryFunctions = BinaryFunctions(SQL_FN_STR_BIT_LENGTH.0);
pub const SQL_FN_BIN_CONCAT: BinaryFunctions = BinaryFunctions(SQL_FN_STR_CONCAT.0);
pub const SQL_FN_BIN_INSERT: BinaryFunctions = BinaryFunctions(SQL_FN_STR_INSERT.0);
pub const SQL_FN_BIN_LTRIM: BinaryFunctions = BinaryFunctions(SQL_FN_STR_LTRIM.0);
pub const SQL_FN_BIN_OCTET_LENGTH: BinaryFunctions = BinaryFunctions(SQL_FN_STR_OCTET_LENGTH.0);
pub const SQL_FN_BIN_POSITION: BinaryFunctions = BinaryFunctions(SQL_FN_STR_POSITION.0);
pub const SQL_FN_BIN_RTRIM: BinaryFunctions = BinaryFunctions(SQL_FN_STR_RTRIM.0);
pub const SQL_FN_BIN_SUBSTRING: BinaryFunctions = BinaryFunctions(SQL_FN_STR_SUBSTRING.0);

#[odbc_bitmask(SQLUINTEGER)]
pub struct IsoBinaryFunctions;
pub const SQL_SBF_CONVERT: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_CONVERT.0);
pub const SQL_SBF_SUBSTRING: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_SUBSTRING.0);
pub const SQL_SBF_TRIM_BOTH: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_TRIM_BOTH.0);
pub const SQL_SBF_TRIM_LEADING: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_TRIM_LEADING.0);
pub const SQL_SBF_TRIM_TRAILING: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_TRIM_TRAILING.0);
pub const SQL_SBF_OVERLAY: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_OVERLAY.0);
pub const SQL_SBF_POSITION: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_POSITION.0);
pub const SQL_SBF_CONCAT: IsoBinaryFunctions = IsoBinaryFunctions(SQL_SSF_CONCAT.0);

#[odbc_bitmask(SQLUINTEGER)]
pub struct LimitEscapeClause;
pub const SQL_LC_NONE: LimitEscapeClause = LimitEscapeClause(0x00000000);
pub const SQL_LC_TAKE: LimitEscapeClause = LimitEscapeClause(0x00000001);
pub const SQL_LC_SKIP: LimitEscapeClause = LimitEscapeClause(0x00000003);

#[odbc_bitmask(SQLUINTEGER)]
pub struct ReturnEscapeClause;
pub const SQL_RC_NONE: ReturnEscapeClause = ReturnEscapeClause(0x00000000);
pub const SQL_RC_INSERT_SINGLE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000001);
pub const SQL_RC_INSERT_SINGLE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000002 | SQL_RC_INSERT_SINGLE_ROWID.0);
pub const SQL_RC_INSERT_MULTIPLE_ROWID: ReturnEscapeClause =
    ReturnEscapeClause(0x00000004 | SQL_RC_INSERT_SINGLE_ROWID.0);
pub const SQL_RC_INSERT_MULTIPLE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000008 | SQL_RC_INSERT_MULTIPLE_ROWID.0 | SQL_RC_INSERT_SINGLE_ANY.0);
pub const SQL_RC_INSERT_SELECT_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000010);
pub const SQL_RC_INSERT_SELECT_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000020 | SQL_RC_INSERT_SELECT_ROWID.0);
pub const SQL_RC_UPDATE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000040);
pub const SQL_RC_UPDATE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000080 | SQL_RC_UPDATE_ROWID.0);
pub const SQL_RC_DELETE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000100);
pub const SQL_RC_DELETE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000200 | SQL_RC_DELETE_ROWID.0);
pub const SQL_RC_SELECT_INTO_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000400);
pub const SQL_RC_SELECT_INTO_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000800 | SQL_RC_SELECT_INTO_ROWID.0);

#[odbc_bitmask(SQLUINTEGER)]
pub struct FormatEscapeClause;
pub const SQL_FC_NONE: FormatEscapeClause = FormatEscapeClause(0x00000000);
pub const SQL_FC_JSON: FormatEscapeClause = FormatEscapeClause(0x00000001);
pub const SQL_FC_JSON_BINARY: FormatEscapeClause = FormatEscapeClause(0x00000002);
