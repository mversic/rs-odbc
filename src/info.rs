#![expect(non_camel_case_types)]

use core::mem::MaybeUninit;

use co3::{ReprC, Tag};
use rust_spec::RustSpec;

use crate::{
    env::{OV_ODBC3, OV_ODBC3_80, OV_ODBC4, OdbcVersion},
    str::{OdbcChar, OdbcStr},
};

/// Marker for identifiers accepted by `SQLGetInfo`.
///
/// # Safety
///
/// The identifier and buffer must match the representation and size prescribed for the
/// information type by ODBC or by the driver specification.
pub unsafe trait InfoType<V: OdbcVersion>: crate::Defined {
    /// Buffer argument accepted by `SQLGetInfo` for this identifier.
    type Buffer<C: OdbcChar>: ?Sized;
}

unsafe impl<T: InfoType<OV_ODBC3>> InfoType<OV_ODBC3_80> for T {
    type Buffer<C: OdbcChar> = <T as InfoType<OV_ODBC3>>::Buffer<C>;
}

unsafe impl<T: InfoType<OV_ODBC3_80>> InfoType<OV_ODBC4> for T {
    type Buffer<C: OdbcChar> = <T as InfoType<OV_ODBC3_80>>::Buffer<C>;
}

macro_rules! impl_info_type {
    ($version:ty, $info:ty => OdbcStr<C>) => {
        impl crate::Defined for $info {
            type By = crate::OdbcDefined;
        }
        unsafe impl InfoType<$version> for $info {
            type Buffer<C: OdbcChar> = OdbcStr<MaybeUninit<C>>;
        }
    };
    ($version:ty, $info:ty => $value:ty) => {
        impl crate::Defined for $info {
            type By = crate::OdbcDefined;
        }
        unsafe impl InfoType<$version> for $info {
            type Buffer<C: OdbcChar> = MaybeUninit<$value>;
        }
    };
}

macro_rules! odbc_bitmask_impl {
    ($vis:vis struct $name:ident, $rust:ty) => {
        #[derive(Debug, Clone, Copy, RustSpec, ReprC)]
        #[reprC(identity)]
        #[repr(transparent)]
        $vis struct $name($rust);

        impl core::ops::BitAnd<$name> for $name {
            type Output = $rust;

            fn bitand(self, other: $name) -> Self::Output {
                self.0 & other.0
            }
        }
    };
}

macro_rules! odbc_bitmask {
    (INTEGER, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, i32);
    };
    (UINTEGER, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, u32);
    };
    (SMALLINT, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, i16);
    };
    (USMALLINT, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, u16);
    };
    (LEN, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, isize);
    };
    (ULEN, $vis:vis struct $name:ident) => {
        odbc_bitmask_impl!($vis struct $name, usize);
    };
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Tag)]
#[tag(u16, unsafe(171))]
pub struct DM_VER;

#[derive(Tag)]
#[tag(u16, unsafe(10000))]
pub struct XOPEN_CLI_YEAR;

#[derive(Tag)]
#[tag(u16, unsafe(134))]
pub struct CREATE_VIEW;

#[derive(Tag)]
#[tag(u16, unsafe(155))]
pub struct SQL92_DATETIME_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(156))]
pub struct SQL92_FOREIGN_KEY_DELETE_RULE;

#[derive(Tag)]
#[tag(u16, unsafe(157))]
pub struct SQL92_FOREIGN_KEY_UPDATE_RULE;

#[derive(Tag)]
#[tag(u16, unsafe(158))]
pub struct SQL92_GRANT;

#[derive(Tag)]
#[tag(u16, unsafe(119))]
pub struct DATETIME_LITERALS;

#[derive(Tag)]
#[tag(u16, unsafe(159))]
pub struct SQL92_NUMERIC_VALUE_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(160))]
pub struct SQL92_PREDICATES;

#[derive(Tag)]
#[tag(u16, unsafe(161))]
pub struct SQL92_RELATIONAL_JOIN_OPERATORS;

#[derive(Tag)]
#[tag(u16, unsafe(162))]
pub struct SQL92_REVOKE;

#[derive(Tag)]
#[tag(u16, unsafe(163))]
pub struct SQL92_ROW_VALUE_CONSTRUCTOR;

#[derive(Tag)]
#[tag(u16, unsafe(164))]
pub struct SQL92_STRING_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(165))]
pub struct SQL92_VALUE_EXPRESSIONS;

#[derive(Tag)]
#[tag(u16, unsafe(166))]
pub struct STANDARD_CLI_CONFORMANCE;

#[derive(Tag)]
#[tag(u16, unsafe(174))]
pub struct SCHEMA_INFERENCE;

#[derive(Tag)]
#[tag(u16, unsafe(175))]
pub struct BINARY_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(176))]
pub struct ISO_STRING_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(177))]
pub struct ISO_BINARY_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(178))]
pub struct LIMIT_ESCAPE_CLAUSE;

#[derive(Tag)]
#[tag(u16, unsafe(179))]
pub struct NATIVE_ESCAPE_CLAUSE;

#[derive(Tag)]
#[tag(u16, unsafe(180))]
pub struct RETURN_ESCAPE_CLAUSE;

#[derive(Tag)]
#[tag(u16, unsafe(181))]
pub struct FORMAT_ESCAPE_CLAUSE;

#[derive(Tag)]
#[tag(u16, unsafe(155))]
pub struct ISO_DATETIME_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(156))]
pub struct ISO_FOREIGN_KEY_DELETE_RULE;

#[derive(Tag)]
#[tag(u16, unsafe(157))]
pub struct ISO_FOREIGN_KEY_UPDATE_RULE;

#[derive(Tag)]
#[tag(u16, unsafe(158))]
pub struct ISO_GRANT;

#[derive(Tag)]
#[tag(u16, unsafe(159))]
pub struct ISO_NUMERIC_VALUE_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(160))]
pub struct ISO_PREDICATES;

#[derive(Tag)]
#[tag(u16, unsafe(161))]
pub struct ISO_RELATIONAL_JOIN_OPERATORS;

#[derive(Tag)]
#[tag(u16, unsafe(162))]
pub struct ISO_REVOKE;

#[derive(Tag)]
#[tag(u16, unsafe(163))]
pub struct ISO_ROW_VALUE_CONSTRUCTOR;

#[derive(Tag)]
#[tag(u16, unsafe(165))]
pub struct ISO_VALUE_EXPRESSIONS;

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// Driver Information ///////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(116))]
pub struct ACTIVE_ENVIRONMENTS;

#[derive(Tag)]
#[tag(u16, unsafe(10023))]
pub struct ASYNC_DBC_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(10021))]
pub struct ASYNC_MODE;

#[derive(Tag)]
#[tag(u16, unsafe(10025))]
pub struct ASYNC_NOTIFICATION;

#[derive(Tag)]
#[tag(u16, unsafe(120))]
pub struct BATCH_ROW_COUNT;

#[derive(Tag)]
#[tag(u16, unsafe(121))]
pub struct BATCH_SUPPORT;

#[derive(Tag)]
#[tag(u16, unsafe(10024))]
pub struct DRIVER_AWARE_POOLING_SUPPORTED;
//
#[derive(Tag)]
#[tag(u16, unsafe(135))]
pub struct DRIVER_HDESC;

#[derive(Tag)]
#[tag(u16, unsafe(2))]
pub struct DATA_SOURCE_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(3))]
pub struct DRIVER_HDBC;

#[derive(Tag)]
#[tag(u16, unsafe(4))]
pub struct DRIVER_HENV;

#[derive(Tag)]
#[tag(u16, unsafe(5))]
pub struct DRIVER_HSTMT;

#[derive(Tag)]
#[tag(u16, unsafe(6))]
pub struct DRIVER_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(7))]
pub struct DRIVER_VER;

#[derive(Tag)]
#[tag(u16, unsafe(76))]
pub struct DRIVER_HLIB;

#[derive(Tag)]
#[tag(u16, unsafe(77))]
pub struct DRIVER_ODBC_VER;

#[derive(Tag)]
#[tag(u16, unsafe(144))]
pub struct DYNAMIC_CURSOR_ATTRIBUTES1;

#[derive(Tag)]
#[tag(u16, unsafe(145))]
pub struct DYNAMIC_CURSOR_ATTRIBUTES2;

#[derive(Tag)]
#[tag(u16, unsafe(146))]
pub struct FORWARD_ONLY_CURSOR_ATTRIBUTES1;

#[derive(Tag)]
#[tag(u16, unsafe(147))]
pub struct FORWARD_ONLY_CURSOR_ATTRIBUTES2;

#[derive(Tag)]
#[tag(u16, unsafe(84))]
pub struct FILE_USAGE;

#[derive(Tag)]
#[tag(u16, unsafe(81))]
pub struct GETDATA_EXTENSIONS;

#[derive(Tag)]
#[tag(u16, unsafe(149))]
pub struct INFO_SCHEMA_VIEWS;

#[derive(Tag)]
#[tag(u16, unsafe(150))]
pub struct KEYSET_CURSOR_ATTRIBUTES1;

#[derive(Tag)]
#[tag(u16, unsafe(151))]
pub struct KEYSET_CURSOR_ATTRIBUTES2;

#[derive(Tag)]
#[tag(u16, unsafe(10022))]
pub struct MAX_ASYNC_CONCURRENT_STATEMENTS;

#[derive(Tag)]
#[tag(u16, unsafe(1))]
pub struct MAX_CONCURRENT_ACTIVITIES;

#[derive(Tag)]
#[tag(u16, unsafe(0))]
pub struct MAX_DRIVER_CONNECTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(152))]
pub struct ODBC_INTERFACE_CONFORMANCE;

#[derive(Tag)]
#[tag(u16, unsafe(10))]
pub struct ODBC_VER;

#[derive(Tag)]
#[tag(u16, unsafe(153))]
pub struct PARAM_ARRAY_ROW_COUNTS;

#[derive(Tag)]
#[tag(u16, unsafe(154))]
pub struct PARAM_ARRAY_SELECTS;

#[derive(Tag)]
#[tag(u16, unsafe(11))]
pub struct ROW_UPDATES;

#[derive(Tag)]
#[tag(u16, unsafe(14))]
pub struct SEARCH_PATTERN_ESCAPE;

#[derive(Tag)]
#[tag(u16, unsafe(13))]
pub struct SERVER_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(167))]
pub struct STATIC_CURSOR_ATTRIBUTES1;

#[derive(Tag)]
#[tag(u16, unsafe(168))]
pub struct STATIC_CURSOR_ATTRIBUTES2;

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////// DBMS Product Information ////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(16))]
pub struct DATABASE_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(17))]
pub struct DBMS_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(18))]
pub struct DBMS_VER;

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////// Data Source Information /////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(20))]
pub struct ACCESSIBLE_PROCEDURES;

#[derive(Tag)]
#[tag(u16, unsafe(19))]
pub struct ACCESSIBLE_TABLES;

#[derive(Tag)]
#[tag(u16, unsafe(82))]
pub struct BOOKMARK_PERSISTENCE;

#[derive(Tag)]
#[tag(u16, unsafe(42))]
pub struct CATALOG_TERM;

#[derive(Tag)]
#[tag(u16, unsafe(10004))]
pub struct COLLATION_SEQ;

#[derive(Tag)]
#[tag(u16, unsafe(22))]
pub struct CONCAT_NULL_BEHAVIOR;

#[derive(Tag)]
#[tag(u16, unsafe(23))]
pub struct CURSOR_COMMIT_BEHAVIOR;

#[derive(Tag)]
#[tag(u16, unsafe(24))]
pub struct CURSOR_ROLLBACK_BEHAVIOR;

#[derive(Tag)]
#[tag(u16, unsafe(10001))]
pub struct CURSOR_SENSITIVITY;

#[derive(Tag)]
#[tag(u16, unsafe(25))]
pub struct DATA_SOURCE_READ_ONLY;

#[derive(Tag)]
#[tag(u16, unsafe(26))]
pub struct DEFAULT_TXN_ISOLATION;

#[derive(Tag)]
#[tag(u16, unsafe(10002))]
pub struct DESCRIBE_PARAMETER;

#[derive(Tag)]
#[tag(u16, unsafe(36))]
pub struct MULT_RESULT_SETS;

#[derive(Tag)]
#[tag(u16, unsafe(37))]
pub struct MULTIPLE_ACTIVE_TXN;

#[derive(Tag)]
#[tag(u16, unsafe(111))]
pub struct NEED_LONG_DATA_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(85))]
pub struct NULL_COLLATION;

#[derive(Tag)]
#[tag(u16, unsafe(40))]
pub struct PROCEDURE_TERM;

#[derive(Tag)]
#[tag(u16, unsafe(39))]
pub struct SCHEMA_TERM;

#[derive(Tag)]
#[tag(u16, unsafe(44))]
pub struct SCROLL_OPTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(45))]
pub struct TABLE_TERM;

#[derive(Tag)]
#[tag(u16, unsafe(46))]
pub struct TXN_CAPABLE;

#[derive(Tag)]
#[tag(u16, unsafe(72))]
pub struct TXN_ISOLATION_OPTION;

#[derive(Tag)]
#[tag(u16, unsafe(47))]
pub struct USER_NAME;

/////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////// Supported SQL //////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(169))]
pub struct AGGREGATE_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(117))]
pub struct ALTER_DOMAIN;

#[derive(Tag)]
#[tag(u16, unsafe(86))]
pub struct ALTER_TABLE;

#[derive(Tag)]
#[tag(u16, unsafe(114))]
pub struct CATALOG_LOCATION;

#[derive(Tag)]
#[tag(u16, unsafe(10003))]
pub struct CATALOG_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(41))]
pub struct CATALOG_NAME_SEPARATOR;

#[derive(Tag)]
#[tag(u16, unsafe(92))]
pub struct CATALOG_USAGE;

#[derive(Tag)]
#[tag(u16, unsafe(87))]
pub struct COLUMN_ALIAS;

#[derive(Tag)]
#[tag(u16, unsafe(74))]
pub struct CORRELATION_NAME;

#[derive(Tag)]
#[tag(u16, unsafe(127))]
pub struct CREATE_ASSERTION;

#[derive(Tag)]
#[tag(u16, unsafe(128))]
pub struct CREATE_CHARACTER_SET;

#[derive(Tag)]
#[tag(u16, unsafe(129))]
pub struct CREATE_COLLATION;

#[derive(Tag)]
#[tag(u16, unsafe(130))]
pub struct CREATE_DOMAIN;

#[derive(Tag)]
#[tag(u16, unsafe(131))]
pub struct CREATE_SCHEMA;

#[derive(Tag)]
#[tag(u16, unsafe(132))]
pub struct CREATE_TABLE;

#[derive(Tag)]
#[tag(u16, unsafe(133))]
pub struct CREATE_TRANSLATION;

#[derive(Tag)]
#[tag(u16, unsafe(170))]
pub struct DDL_INDEX;

#[derive(Tag)]
#[tag(u16, unsafe(136))]
pub struct DROP_ASSERTION;

#[derive(Tag)]
#[tag(u16, unsafe(137))]
pub struct DROP_CHARACTER_SET;

#[derive(Tag)]
#[tag(u16, unsafe(138))]
pub struct DROP_COLLATION;

#[derive(Tag)]
#[tag(u16, unsafe(139))]
pub struct DROP_DOMAIN;

#[derive(Tag)]
#[tag(u16, unsafe(140))]
pub struct DROP_SCHEMA;

#[derive(Tag)]
#[tag(u16, unsafe(141))]
pub struct DROP_TABLE;

#[derive(Tag)]
#[tag(u16, unsafe(142))]
pub struct DROP_TRANSLATION;

#[derive(Tag)]
#[tag(u16, unsafe(143))]
pub struct DROP_VIEW;

#[derive(Tag)]
#[tag(u16, unsafe(27))]
pub struct EXPRESSIONS_IN_ORDERBY;

#[derive(Tag)]
#[tag(u16, unsafe(88))]
pub struct GROUP_BY;

#[derive(Tag)]
#[tag(u16, unsafe(28))]
pub struct IDENTIFIER_CASE;

#[derive(Tag)]
#[tag(u16, unsafe(29))]
pub struct IDENTIFIER_QUOTE_CHAR;

#[derive(Tag)]
#[tag(u16, unsafe(148))]
pub struct INDEX_KEYWORDS;

#[derive(Tag)]
#[tag(u16, unsafe(172))]
pub struct INSERT_STATEMENT;

#[derive(Tag)]
#[tag(u16, unsafe(73))]
pub struct INTEGRITY;

#[derive(Tag)]
#[tag(u16, unsafe(89))]
pub struct KEYWORDS;

#[derive(Tag)]
#[tag(u16, unsafe(113))]
pub struct LIKE_ESCAPE_CLAUSE;

#[derive(Tag)]
#[tag(u16, unsafe(75))]
pub struct NON_NULLABLE_COLUMNS;

#[derive(Tag)]
#[tag(u16, unsafe(115))]
pub struct OJ_CAPABILITIES;

#[derive(Tag)]
#[tag(u16, unsafe(90))]
pub struct ORDER_BY_COLUMNS_IN_SELECT;

#[derive(Tag)]
#[tag(u16, unsafe(38))]
pub struct OUTER_JOINS;

#[derive(Tag)]
#[tag(u16, unsafe(21))]
pub struct PROCEDURES;

#[derive(Tag)]
#[tag(u16, unsafe(93))]
pub struct QUOTED_IDENTIFIER_CASE;

#[derive(Tag)]
#[tag(u16, unsafe(91))]
pub struct SCHEMA_USAGE;

#[derive(Tag)]
#[tag(u16, unsafe(94))]
pub struct SPECIAL_CHARACTERS;

#[derive(Tag)]
#[tag(u16, unsafe(118))]
pub struct CONFORMANCE;

#[derive(Tag)]
#[tag(u16, unsafe(95))]
pub struct SUBQUERIES;

#[derive(Tag)]
#[tag(u16, unsafe(96))]
pub struct UNION;

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// SQL Limits ///////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(112))]
pub struct MAX_BINARY_LITERAL_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(34))]
pub struct MAX_CATALOG_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(108))]
pub struct MAX_CHAR_LITERAL_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(30))]
pub struct MAX_COLUMN_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(97))]
pub struct MAX_COLUMNS_IN_GROUP_BY;

#[derive(Tag)]
#[tag(u16, unsafe(98))]
pub struct MAX_COLUMNS_IN_INDEX;

#[derive(Tag)]
#[tag(u16, unsafe(99))]
pub struct MAX_COLUMNS_IN_ORDER_BY;

#[derive(Tag)]
#[tag(u16, unsafe(100))]
pub struct MAX_COLUMNS_IN_SELECT;

#[derive(Tag)]
#[tag(u16, unsafe(101))]
pub struct MAX_COLUMNS_IN_TABLE;

#[derive(Tag)]
#[tag(u16, unsafe(31))]
pub struct MAX_CURSOR_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(10005))]
pub struct MAX_IDENTIFIER_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(102))]
pub struct MAX_INDEX_SIZE;

#[derive(Tag)]
#[tag(u16, unsafe(33))]
pub struct MAX_PROCEDURE_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(104))]
pub struct MAX_ROW_SIZE;

#[derive(Tag)]
#[tag(u16, unsafe(103))]
pub struct MAX_ROW_SIZE_INCLUDES_LONG;

#[derive(Tag)]
#[tag(u16, unsafe(32))]
pub struct MAX_SCHEMA_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(105))]
pub struct MAX_STATEMENT_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(35))]
pub struct MAX_TABLE_NAME_LEN;

#[derive(Tag)]
#[tag(u16, unsafe(106))]
pub struct MAX_TABLES_IN_SELECT;

#[derive(Tag)]
#[tag(u16, unsafe(107))]
pub struct MAX_USER_NAME_LEN;

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// Scalar Function Information //////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(48))]
pub struct CONVERT_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(49))]
pub struct NUMERIC_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(50))]
pub struct STRING_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(51))]
pub struct SYSTEM_FUNCTIONS;

#[derive(Tag)]
#[tag(u16, unsafe(109))]
pub struct TIMEDATE_ADD_INTERVALS;

#[derive(Tag)]
#[tag(u16, unsafe(110))]
pub struct TIMEDATE_DIFF_INTERVALS;

#[derive(Tag)]
#[tag(u16, unsafe(52))]
pub struct TIMEDATE_FUNCTIONS;

/////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// Conversion Information /////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Tag)]
#[tag(u16, unsafe(53))]
pub struct CONVERT_BIGINT;

#[derive(Tag)]
#[tag(u16, unsafe(54))]
pub struct CONVERT_BINARY;

#[derive(Tag)]
#[tag(u16, unsafe(55))]
pub struct CONVERT_BIT;

#[derive(Tag)]
#[tag(u16, unsafe(56))]
pub struct CONVERT_CHAR;

#[derive(Tag)]
#[tag(u16, unsafe(57))]
pub struct CONVERT_DATE;

#[derive(Tag)]
#[tag(u16, unsafe(58))]
pub struct CONVERT_DECIMAL;

#[derive(Tag)]
#[tag(u16, unsafe(59))]
pub struct CONVERT_DOUBLE;

#[derive(Tag)]
#[tag(u16, unsafe(60))]
pub struct CONVERT_FLOAT;

#[derive(Tag)]
#[tag(u16, unsafe(61))]
pub struct CONVERT_INTEGER;

#[derive(Tag)]
#[tag(u16, unsafe(123))]
pub struct CONVERT_INTERVAL_DAY_TIME;

#[derive(Tag)]
#[tag(u16, unsafe(124))]
pub struct CONVERT_INTERVAL_YEAR_MONTH;

#[derive(Tag)]
#[tag(u16, unsafe(71))]
pub struct CONVERT_LONGVARBINARY;

#[derive(Tag)]
#[tag(u16, unsafe(62))]
pub struct CONVERT_LONGVARCHAR;

#[derive(Tag)]
#[tag(u16, unsafe(63))]
pub struct CONVERT_NUMERIC;

#[derive(Tag)]
#[tag(u16, unsafe(64))]
pub struct CONVERT_REAL;

#[derive(Tag)]
#[tag(u16, unsafe(65))]
pub struct CONVERT_SMALLINT;

#[derive(Tag)]
#[tag(u16, unsafe(66))]
pub struct CONVERT_TIME;

#[derive(Tag)]
#[tag(u16, unsafe(67))]
pub struct CONVERT_TIMESTAMP;

#[derive(Tag)]
#[tag(u16, unsafe(68))]
pub struct CONVERT_TINYINT;

#[derive(Tag)]
#[tag(u16, unsafe(69))]
pub struct CONVERT_VARBINARY;

#[derive(Tag)]
#[tag(u16, unsafe(70))]
pub struct CONVERT_VARCHAR;

#[derive(Tag)]
#[tag(u16, unsafe(122))]
pub struct CONVERT_WCHAR;

#[derive(Tag)]
#[tag(u16, unsafe(125))]
pub struct CONVERT_WLONGVARCHAR;

#[derive(Tag)]
#[tag(u16, unsafe(126))]
pub struct CONVERT_WVARCHAR;

#[derive(Tag)]
#[tag(u16, unsafe(173))]
pub struct CONVERT_GUID;

impl_info_type!(OV_ODBC3, DM_VER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, XOPEN_CLI_YEAR => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CREATE_VIEW => CreateView);
impl_info_type!(OV_ODBC3, SQL92_DATETIME_FUNCTIONS => DatetimeFunctions);
impl_info_type!(OV_ODBC3, SQL92_FOREIGN_KEY_DELETE_RULE => ForeignKeyDeleteRule);
impl_info_type!(OV_ODBC3, SQL92_FOREIGN_KEY_UPDATE_RULE => ForeignKeyUpdateRule);
impl_info_type!(OV_ODBC3, SQL92_GRANT => Grant);
impl_info_type!(OV_ODBC3, DATETIME_LITERALS => DatetimeLiterals);
impl_info_type!(OV_ODBC3, SQL92_NUMERIC_VALUE_FUNCTIONS => NumericValueFunctions);
impl_info_type!(OV_ODBC3, SQL92_PREDICATES => Predicates);
impl_info_type!(OV_ODBC3, SQL92_RELATIONAL_JOIN_OPERATORS => RelationalJoinOperators);
impl_info_type!(OV_ODBC3, SQL92_REVOKE => Revoke);
impl_info_type!(OV_ODBC3, SQL92_ROW_VALUE_CONSTRUCTOR => RowValueConstructor);
impl_info_type!(OV_ODBC3, SQL92_STRING_FUNCTIONS => StringScalarFunctions);
impl_info_type!(OV_ODBC3, SQL92_VALUE_EXPRESSIONS => ValueExpressions);
impl_info_type!(OV_ODBC3, STANDARD_CLI_CONFORMANCE => StandardCliConformance);
impl_info_type!(OV_ODBC4, BINARY_FUNCTIONS => BinaryFunctions);
impl_info_type!(OV_ODBC4, ISO_STRING_FUNCTIONS => StringScalarFunctions);
impl_info_type!(OV_ODBC4, ISO_BINARY_FUNCTIONS => IsoBinaryFunctions);
impl_info_type!(OV_ODBC4, LIMIT_ESCAPE_CLAUSE => LimitEscapeClause);
impl_info_type!(OV_ODBC4, NATIVE_ESCAPE_CLAUSE => OdbcStr<C>);
impl_info_type!(OV_ODBC4, RETURN_ESCAPE_CLAUSE => ReturnEscapeClause);
impl_info_type!(OV_ODBC4, FORMAT_ESCAPE_CLAUSE => FormatEscapeClause);
impl_info_type!(OV_ODBC4, ISO_DATETIME_FUNCTIONS => DatetimeFunctions);
impl_info_type!(OV_ODBC4, ISO_FOREIGN_KEY_DELETE_RULE => ForeignKeyDeleteRule);
impl_info_type!(OV_ODBC4, ISO_FOREIGN_KEY_UPDATE_RULE => ForeignKeyUpdateRule);
impl_info_type!(OV_ODBC4, ISO_GRANT => Grant);
impl_info_type!(OV_ODBC4, ISO_NUMERIC_VALUE_FUNCTIONS => NumericValueFunctions);
impl_info_type!(OV_ODBC4, ISO_PREDICATES => Predicates);
impl_info_type!(OV_ODBC4, ISO_RELATIONAL_JOIN_OPERATORS => RelationalJoinOperators);
impl_info_type!(OV_ODBC4, ISO_REVOKE => Revoke);
impl_info_type!(OV_ODBC4, ISO_ROW_VALUE_CONSTRUCTOR => RowValueConstructor);
impl_info_type!(OV_ODBC4, ISO_VALUE_EXPRESSIONS => ValueExpressions);
impl_info_type!(OV_ODBC3, ACTIVE_ENVIRONMENTS => u16);
impl_info_type!(OV_ODBC3_80, ASYNC_DBC_FUNCTIONS => AsyncDbcFunctions);
impl_info_type!(OV_ODBC3, ASYNC_MODE => AsyncMode);
impl_info_type!(OV_ODBC3_80, ASYNC_NOTIFICATION => AsyncNotification);
impl_info_type!(OV_ODBC3, BATCH_ROW_COUNT => BatchRowCount);
impl_info_type!(OV_ODBC3, BATCH_SUPPORT => BatchSupport);
impl_info_type!(OV_ODBC3, DATA_SOURCE_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3_80, DRIVER_AWARE_POOLING_SUPPORTED => DriverAwarePoolingSupported);
impl_info_type!(OV_ODBC3, DRIVER_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DRIVER_ODBC_VER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DRIVER_VER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DYNAMIC_CURSOR_ATTRIBUTES1 => CursorAttributes1);
impl_info_type!(OV_ODBC3, DYNAMIC_CURSOR_ATTRIBUTES2 => CursorAttributes2);
impl_info_type!(OV_ODBC3, FORWARD_ONLY_CURSOR_ATTRIBUTES1 => CursorAttributes1);
impl_info_type!(OV_ODBC3, FORWARD_ONLY_CURSOR_ATTRIBUTES2 => CursorAttributes2);
impl_info_type!(OV_ODBC3, FILE_USAGE => FileUsage);
impl_info_type!(OV_ODBC3, GETDATA_EXTENSIONS => GetdataExtensions);
impl_info_type!(OV_ODBC3, INFO_SCHEMA_VIEWS => InfoSchemaViews);
impl_info_type!(OV_ODBC3, KEYSET_CURSOR_ATTRIBUTES1 => CursorAttributes1);
impl_info_type!(OV_ODBC3, KEYSET_CURSOR_ATTRIBUTES2 => CursorAttributes2);
impl_info_type!(OV_ODBC3, MAX_ASYNC_CONCURRENT_STATEMENTS => u32);
impl_info_type!(OV_ODBC3, MAX_CONCURRENT_ACTIVITIES => u16);
impl_info_type!(OV_ODBC3, MAX_DRIVER_CONNECTIONS => u16);
impl_info_type!(OV_ODBC3, ODBC_INTERFACE_CONFORMANCE => OdbcInterfaceConformance);
impl_info_type!(OV_ODBC3, ODBC_VER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, PARAM_ARRAY_ROW_COUNTS => ParamArrayRowCounts);
impl_info_type!(OV_ODBC3, PARAM_ARRAY_SELECTS => ParamArraySelects);
impl_info_type!(OV_ODBC3, ROW_UPDATES => OdbcStr<C>);
impl_info_type!(OV_ODBC3, SEARCH_PATTERN_ESCAPE => OdbcStr<C>);
impl_info_type!(OV_ODBC3, SERVER_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, STATIC_CURSOR_ATTRIBUTES1 => CursorAttributes1);
impl_info_type!(OV_ODBC3, STATIC_CURSOR_ATTRIBUTES2 => CursorAttributes2);
impl_info_type!(OV_ODBC3, DATABASE_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DBMS_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DBMS_VER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, ACCESSIBLE_PROCEDURES => OdbcStr<C>);
impl_info_type!(OV_ODBC3, ACCESSIBLE_TABLES => OdbcStr<C>);
impl_info_type!(OV_ODBC3, BOOKMARK_PERSISTENCE => BookmarkPersistence);
impl_info_type!(OV_ODBC3, CATALOG_TERM => OdbcStr<C>);
impl_info_type!(OV_ODBC3, COLLATION_SEQ => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CONCAT_NULL_BEHAVIOR => ConcatNullBehavior);
impl_info_type!(OV_ODBC3, CURSOR_COMMIT_BEHAVIOR => CursorBehavior);
impl_info_type!(OV_ODBC3, CURSOR_ROLLBACK_BEHAVIOR => CursorBehavior);
impl_info_type!(OV_ODBC3, CURSOR_SENSITIVITY => CursorSensitivity);
impl_info_type!(OV_ODBC3, DATA_SOURCE_READ_ONLY => OdbcStr<C>);
impl_info_type!(OV_ODBC3, DEFAULT_TXN_ISOLATION => Option<TxnIsolation>);
impl_info_type!(OV_ODBC3, DESCRIBE_PARAMETER => OdbcStr<C>);
impl_info_type!(OV_ODBC3, MULT_RESULT_SETS => OdbcStr<C>);
impl_info_type!(OV_ODBC3, MULTIPLE_ACTIVE_TXN => OdbcStr<C>);
impl_info_type!(OV_ODBC3, NEED_LONG_DATA_LEN => OdbcStr<C>);
impl_info_type!(OV_ODBC3, NULL_COLLATION => NullCollation);
impl_info_type!(OV_ODBC3, PROCEDURE_TERM => OdbcStr<C>);
impl_info_type!(OV_ODBC3, SCHEMA_TERM => OdbcStr<C>);
impl_info_type!(OV_ODBC3, SCROLL_OPTIONS => ScrollOptions);
impl_info_type!(OV_ODBC3, TABLE_TERM => OdbcStr<C>);
impl_info_type!(OV_ODBC3, TXN_CAPABLE => TxnCapable);
impl_info_type!(OV_ODBC3, TXN_ISOLATION_OPTION => TxnIsolationOptions);
impl_info_type!(OV_ODBC3, USER_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, AGGREGATE_FUNCTIONS => AggregateFunctions);
impl_info_type!(OV_ODBC3, ALTER_DOMAIN => AlterDomain);
impl_info_type!(OV_ODBC3, ALTER_TABLE => AlterTable);
impl_info_type!(OV_ODBC3, CATALOG_LOCATION => CatalogLocation);
impl_info_type!(OV_ODBC3, CATALOG_NAME => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CATALOG_NAME_SEPARATOR => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CATALOG_USAGE => CatalogUsage);
impl_info_type!(OV_ODBC3, COLUMN_ALIAS => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CORRELATION_NAME => CorrelationName);
impl_info_type!(OV_ODBC3, CREATE_ASSERTION => CreateAssertion);
impl_info_type!(OV_ODBC3, CREATE_CHARACTER_SET => CreateCharacterSet);
impl_info_type!(OV_ODBC3, CREATE_COLLATION => CreateCollation);
impl_info_type!(OV_ODBC3, CREATE_DOMAIN => CreateDomain);
impl_info_type!(OV_ODBC3, CREATE_SCHEMA => CreateSchema);
impl_info_type!(OV_ODBC3, CREATE_TABLE => CreateTable);
impl_info_type!(OV_ODBC3, CREATE_TRANSLATION => CreateTranslation);
impl_info_type!(OV_ODBC3, DDL_INDEX => DdlIndex);
impl_info_type!(OV_ODBC3, DROP_ASSERTION => DropAssertion);
impl_info_type!(OV_ODBC3, DROP_CHARACTER_SET => DropCharacterSet);
impl_info_type!(OV_ODBC3, DROP_COLLATION => DropCollation);
impl_info_type!(OV_ODBC3, DROP_DOMAIN => DropDomain);
impl_info_type!(OV_ODBC3, DROP_SCHEMA => DropSchema);
impl_info_type!(OV_ODBC3, DROP_TABLE => DropTable);
impl_info_type!(OV_ODBC3, DROP_TRANSLATION => DropTranslation);
impl_info_type!(OV_ODBC3, DROP_VIEW => DropView);
impl_info_type!(OV_ODBC3, EXPRESSIONS_IN_ORDERBY => OdbcStr<C>);
impl_info_type!(OV_ODBC3, GROUP_BY => GroupBy);
impl_info_type!(OV_ODBC3, IDENTIFIER_CASE => IdentifierCase);
impl_info_type!(OV_ODBC3, IDENTIFIER_QUOTE_CHAR => OdbcStr<C>);
impl_info_type!(OV_ODBC3, INDEX_KEYWORDS => IndexKeywords);
impl_info_type!(OV_ODBC3, INSERT_STATEMENT => InsertStatement);
impl_info_type!(OV_ODBC3, INTEGRITY => OdbcStr<C>);
impl_info_type!(OV_ODBC3, KEYWORDS => OdbcStr<C>);
impl_info_type!(OV_ODBC3, LIKE_ESCAPE_CLAUSE => OdbcStr<C>);
impl_info_type!(OV_ODBC3, NON_NULLABLE_COLUMNS => NonNullableColumns);
impl_info_type!(OV_ODBC3, OJ_CAPABILITIES => OjCapabilities);
impl_info_type!(OV_ODBC3, ORDER_BY_COLUMNS_IN_SELECT => OdbcStr<C>);
impl_info_type!(OV_ODBC3, OUTER_JOINS => OdbcStr<C>);
impl_info_type!(OV_ODBC3, PROCEDURES => OdbcStr<C>);
impl_info_type!(OV_ODBC3, QUOTED_IDENTIFIER_CASE => IdentifierCase);
impl_info_type!(OV_ODBC3, SCHEMA_USAGE => SchemaUsage);
impl_info_type!(OV_ODBC3, SPECIAL_CHARACTERS => OdbcStr<C>);
impl_info_type!(OV_ODBC3, CONFORMANCE => Option<SqlConformance>);
impl_info_type!(OV_ODBC3, SUBQUERIES => Subqueries);
impl_info_type!(OV_ODBC3, UNION => Union);
impl_info_type!(OV_ODBC3, MAX_BINARY_LITERAL_LEN => u32);
impl_info_type!(OV_ODBC3, MAX_CATALOG_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_CHAR_LITERAL_LEN => u32);
impl_info_type!(OV_ODBC3, MAX_COLUMN_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_COLUMNS_IN_GROUP_BY => u16);
impl_info_type!(OV_ODBC3, MAX_COLUMNS_IN_INDEX => u16);
impl_info_type!(OV_ODBC3, MAX_COLUMNS_IN_ORDER_BY => u16);
impl_info_type!(OV_ODBC3, MAX_COLUMNS_IN_SELECT => u16);
impl_info_type!(OV_ODBC3, MAX_COLUMNS_IN_TABLE => u16);
impl_info_type!(OV_ODBC3, MAX_CURSOR_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_IDENTIFIER_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_INDEX_SIZE => u32);
impl_info_type!(OV_ODBC3, MAX_PROCEDURE_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_ROW_SIZE => u32);
impl_info_type!(OV_ODBC3, MAX_ROW_SIZE_INCLUDES_LONG => OdbcStr<C>);
impl_info_type!(OV_ODBC3, MAX_SCHEMA_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_STATEMENT_LEN => u32);
impl_info_type!(OV_ODBC3, MAX_TABLE_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, MAX_TABLES_IN_SELECT => u16);
impl_info_type!(OV_ODBC3, MAX_USER_NAME_LEN => u16);
impl_info_type!(OV_ODBC3, CONVERT_FUNCTIONS => ConvertFunctions);
impl_info_type!(OV_ODBC3, NUMERIC_FUNCTIONS => NumericFunctions);
impl_info_type!(OV_ODBC3, STRING_FUNCTIONS => StringFunctions);
impl_info_type!(OV_ODBC3, SYSTEM_FUNCTIONS => SystemFunctions);
impl_info_type!(OV_ODBC3, TIMEDATE_ADD_INTERVALS => TimedateIntervals);
impl_info_type!(OV_ODBC3, TIMEDATE_DIFF_INTERVALS => TimedateIntervals);
impl_info_type!(OV_ODBC3, TIMEDATE_FUNCTIONS => TimedateFunctions);
impl_info_type!(OV_ODBC3, CONVERT_BIGINT => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_BINARY => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_BIT => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_CHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_DATE => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_DECIMAL => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_DOUBLE => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_FLOAT => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_INTEGER => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_INTERVAL_DAY_TIME => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_INTERVAL_YEAR_MONTH => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_LONGVARBINARY => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_LONGVARCHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_NUMERIC => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_REAL => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_SMALLINT => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_TIME => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_TIMESTAMP => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_TINYINT => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_VARBINARY => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_VARCHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_WCHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_WLONGVARCHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_WVARCHAR => Conversion);
impl_info_type!(OV_ODBC3, CONVERT_GUID => Conversion);
impl_info_type!(OV_ODBC4, SCHEMA_INFERENCE => SchemaInference);

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(u32)]
pub enum SchemaInference {
    FALSE = 0,
    TRUE = 1,
}

impl From<bool> for SchemaInference {
    fn from(value: bool) -> Self {
        if value { Self::TRUE } else { Self::FALSE }
    }
}

impl From<SchemaInference> for bool {
    fn from(value: SchemaInference) -> Self {
        value == SchemaInference::TRUE
    }
}

//=====================================================================================//

macro_rules! info_enum {
    ($name:ident, $repr:ty, { $($variant:ident = $value:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
        #[repr($repr)]
        pub enum $name {
            $($variant = $value),+
        }

        $(pub const $variant: $name = $name::$variant;)+
    };
}

info_enum!(AsyncDbcFunctions, u32, {
    ASYNC_DBC_NOT_CAPABLE = 0x0000000,
    ASYNC_DBC_CAPABLE = 0x00000001,
});
info_enum!(AsyncMode, u32, {
    AM_NONE = 0,
    AM_CONNECTION = 1,
    AM_STATEMENT = 2,
});
info_enum!(AsyncNotification, u32, {
    ASYNC_NOTIFICATION_NOT_CAPABLE = 0x00000000,
    ASYNC_NOTIFICATION_CAPABLE = 0x00000001,
});
info_enum!(ConcatNullBehavior, u16, {
    CB_NON_NULL = 0x0000,
    CB_NULL = 0x0001,
});
info_enum!(CorrelationName, u16, {
    CN_NONE = 0x0000,
    CN_DIFFERENT = 0x0001,
    CN_ANY = 0x0002,
});
info_enum!(CatalogLocation, u16, { CL_START = 0x0001, CL_END = 0x0002 });
info_enum!(CursorBehavior, u16, {
    CB_DELETE = 0,
    CB_CLOSE = 1,
    CB_PRESERVE = 2,
});
info_enum!(CursorSensitivity, u32, {
    UNSPECIFIED = 0,
    INSENSITIVE = 1,
    SENSITIVE = 2,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct DdlIndex(pub(crate) u32);
pub const DI_CREATE_INDEX: DdlIndex = DdlIndex(0x00000001);
pub const DI_DROP_INDEX: DdlIndex = DdlIndex(0x00000002);

info_enum!(TxnCapable, u16, {
    TC_NONE = 0,
    TC_DML = 1,
    TC_ALL = 2,
    TC_DDL_COMMIT = 3,
    TC_DDL_IGNORE = 4,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(NICHE_VALUE = CSqlConformance(0))]
#[rust_spec(with_custom_niche)]
#[repr(transparent)]
pub struct SqlConformance(pub(crate) u32);
pub const SC_SQL92_ENTRY: SqlConformance = SqlConformance(0x00000001);
pub const SC_FIPS127_2_TRANSITIONAL: SqlConformance = SqlConformance(0x00000002);
pub const SC_SQL92_INTERMEDIATE: SqlConformance = SqlConformance(0x00000004);
pub const SC_SQL92_FULL: SqlConformance = SqlConformance(0x00000008);

info_enum!(ParamArraySelects, u32, {
    PAS_BATCH = 1,
    PAS_NO_BATCH = 2,
    PAS_NO_SELECT = 3,
});
info_enum!(ParamArrayRowCounts, u32, { PARC_BATCH = 1, PARC_NO_BATCH = 2 });
info_enum!(OdbcInterfaceConformance, u32, {
    OIC_CORE = 1,
    OIC_LEVEL1 = 2,
    OIC_LEVEL2 = 3,
});
info_enum!(NonNullableColumns, u16, { NNC_NULL = 0x0000, NNC_NON_NULL = 0x0001 });
info_enum!(IdentifierCase, u16, {
    IC_UPPER = 1,
    IC_LOWER = 2,
    IC_SENSITIVE = 3,
    IC_MIXED = 4,
});
info_enum!(GroupBy, u16, {
    GB_NOT_SUPPORTED = 0x0000,
    GB_GROUP_BY_EQUALS_SELECT = 0x0001,
    GB_GROUP_BY_CONTAINS_SELECT = 0x0002,
    GB_NO_RELATION = 0x0003,
    GB_COLLATE = 0x0004,
});
info_enum!(FileUsage, u16, {
    FILE_NOT_SUPPORTED = 0x0000,
    FILE_TABLE = 0x0001,
    FILE_CATALOG = 0x0002,
});

odbc_bitmask!(UINTEGER, pub struct BatchRowCount);
pub const BRC_PROCEDURES: BatchRowCount = BatchRowCount(0x0000001);
pub const BRC_EXPLICIT: BatchRowCount = BatchRowCount(0x0000002);
pub const BRC_ROLLED_UP: BatchRowCount = BatchRowCount(0x0000004);

odbc_bitmask!(UINTEGER, pub struct BatchSupport);
pub const BS_SELECT_EXPLICIT: BatchSupport = BatchSupport(0x00000001);
pub const BS_ROW_COUNT_EXPLICIT: BatchSupport = BatchSupport(0x00000002);
pub const BS_SELECT_PROC: BatchSupport = BatchSupport(0x00000004);
pub const BS_ROW_COUNT_PROC: BatchSupport = BatchSupport(0x00000008);

info_enum!(DriverAwarePoolingSupported, u32, {
    DRIVER_AWARE_POOLING_NOT_CAPABLE = 0x00000000,
    DRIVER_AWARE_POOLING_CAPABLE = 0x00000001,
});

odbc_bitmask!(UINTEGER, pub struct CursorAttributes1);
pub const CA1_NEXT: CursorAttributes1 = CursorAttributes1(0x00000001);
pub const CA1_ABSOLUTE: CursorAttributes1 = CursorAttributes1(0x00000002);
pub const CA1_RELATIVE: CursorAttributes1 = CursorAttributes1(0x00000004);
pub const CA1_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00000008);

pub const CA1_LOCK_NO_CHANGE: CursorAttributes1 = CursorAttributes1(0x00000040);
pub const CA1_LOCK_EXCLUSIVE: CursorAttributes1 = CursorAttributes1(0x00000080);
pub const CA1_LOCK_UNLOCK: CursorAttributes1 = CursorAttributes1(0x00000100);

pub const CA1_POS_POSITION: CursorAttributes1 = CursorAttributes1(0x00000200);
pub const CA1_POS_UPDATE: CursorAttributes1 = CursorAttributes1(0x00000400);
pub const CA1_POS_DELETE: CursorAttributes1 = CursorAttributes1(0x00000800);
pub const CA1_POS_REFRESH: CursorAttributes1 = CursorAttributes1(0x00001000);

pub const CA1_POSITIONED_UPDATE: CursorAttributes1 = CursorAttributes1(0x00002000);
pub const CA1_POSITIONED_DELETE: CursorAttributes1 = CursorAttributes1(0x00004000);
pub const CA1_SELECT_FOR_UPDATE: CursorAttributes1 = CursorAttributes1(0x00008000);

pub const CA1_BULK_ADD: CursorAttributes1 = CursorAttributes1(0x00010000);
pub const CA1_BULK_UPDATE_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00020000);
pub const CA1_BULK_DELETE_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00040000);
pub const CA1_BULK_FETCH_BY_BOOKMARK: CursorAttributes1 = CursorAttributes1(0x00080000);

odbc_bitmask!(UINTEGER, pub struct CursorAttributes2);
pub const CA2_READ_ONLY_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000001);
pub const CA2_LOCK_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000002);
pub const CA2_OPT_ROWVER_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000004);
pub const CA2_OPT_VALUES_CONCURRENCY: CursorAttributes2 = CursorAttributes2(0x00000008);

pub const CA2_SENSITIVITY_ADDITIONS: CursorAttributes2 = CursorAttributes2(0x00000010);
pub const CA2_SENSITIVITY_DELETIONS: CursorAttributes2 = CursorAttributes2(0x00000020);
pub const CA2_SENSITIVITY_UPDATES: CursorAttributes2 = CursorAttributes2(0x00000040);

pub const CA2_MAX_ROWS_SELECT: CursorAttributes2 = CursorAttributes2(0x00000080);
pub const CA2_MAX_ROWS_INSERT: CursorAttributes2 = CursorAttributes2(0x00000100);
pub const CA2_MAX_ROWS_DELETE: CursorAttributes2 = CursorAttributes2(0x00000200);
pub const CA2_MAX_ROWS_UPDATE: CursorAttributes2 = CursorAttributes2(0x00000400);
pub const CA2_MAX_ROWS_CATALOG: CursorAttributes2 = CursorAttributes2(0x00000800);
pub const CA2_MAX_ROWS_AFFECTS_ALL: CursorAttributes2 = CursorAttributes2(
    CA2_MAX_ROWS_SELECT.0
        | CA2_MAX_ROWS_INSERT.0
        | CA2_MAX_ROWS_DELETE.0
        | CA2_MAX_ROWS_UPDATE.0
        | CA2_MAX_ROWS_CATALOG.0,
);

pub const CA2_CRC_EXACT: CursorAttributes2 = CursorAttributes2(0x00001000);
pub const CA2_CRC_APPROXIMATE: CursorAttributes2 = CursorAttributes2(0x00002000);

pub const CA2_SIMULATE_NON_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00004000);
pub const CA2_SIMULATE_TRY_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00008000);
pub const CA2_SIMULATE_UNIQUE: CursorAttributes2 = CursorAttributes2(0x00010000);

odbc_bitmask!(UINTEGER, pub struct GetdataExtensions);
pub const GD_ANY_COLUMN: GetdataExtensions = GetdataExtensions(0x00000001);
pub const GD_ANY_ORDER: GetdataExtensions = GetdataExtensions(0x00000002);
pub const GD_BLOCK: GetdataExtensions = GetdataExtensions(0x00000004);
pub const GD_BOUND: GetdataExtensions = GetdataExtensions(0x00000008);
pub const GD_OUTPUT_PARAMS: GetdataExtensions = GetdataExtensions(0x00000010);
pub const GD_CONCURRENT: GetdataExtensions = GetdataExtensions(0x00000020);

odbc_bitmask!(UINTEGER, pub struct InfoSchemaViews);
pub const ISV_ASSERTIONS: InfoSchemaViews = InfoSchemaViews(0x00000001);
pub const ISV_CHARACTER_SETS: InfoSchemaViews = InfoSchemaViews(0x00000002);
pub const ISV_CHECK_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00000004);
pub const ISV_COLLATIONS: InfoSchemaViews = InfoSchemaViews(0x00000008);
pub const ISV_COLUMN_DOMAIN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000010);
pub const ISV_COLUMN_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00000020);
pub const ISV_COLUMNS: InfoSchemaViews = InfoSchemaViews(0x00000040);
pub const ISV_CONSTRAINT_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000080);
pub const ISV_CONSTRAINT_TABLE_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000100);
pub const ISV_DOMAIN_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00000200);
pub const ISV_DOMAINS: InfoSchemaViews = InfoSchemaViews(0x00000400);
pub const ISV_KEY_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00000800);
pub const ISV_REFERENTIAL_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00001000);
pub const ISV_SCHEMATA: InfoSchemaViews = InfoSchemaViews(0x00002000);
pub const ISV_SQL_LANGUAGES: InfoSchemaViews = InfoSchemaViews(0x00004000);
pub const ISV_TABLE_CONSTRAINTS: InfoSchemaViews = InfoSchemaViews(0x00008000);
pub const ISV_TABLE_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00010000);
pub const ISV_TABLES: InfoSchemaViews = InfoSchemaViews(0x00020000);
pub const ISV_TRANSLATIONS: InfoSchemaViews = InfoSchemaViews(0x00040000);
pub const ISV_USAGE_PRIVILEGES: InfoSchemaViews = InfoSchemaViews(0x00080000);
pub const ISV_VIEW_COLUMN_USAGE: InfoSchemaViews = InfoSchemaViews(0x00100000);
pub const ISV_VIEW_TABLE_USAGE: InfoSchemaViews = InfoSchemaViews(0x00200000);
pub const ISV_VIEWS: InfoSchemaViews = InfoSchemaViews(0x00400000);

odbc_bitmask!(UINTEGER, pub struct BookmarkPersistence);
pub const BP_CLOSE: BookmarkPersistence = BookmarkPersistence(0x00000001);
pub const BP_DELETE: BookmarkPersistence = BookmarkPersistence(0x00000002);
pub const BP_DROP: BookmarkPersistence = BookmarkPersistence(0x00000004);
pub const BP_TRANSACTION: BookmarkPersistence = BookmarkPersistence(0x00000008);
pub const BP_UPDATE: BookmarkPersistence = BookmarkPersistence(0x00000010);
pub const BP_OTHER_HSTMT: BookmarkPersistence = BookmarkPersistence(0x00000020);
pub const BP_SCROLL: BookmarkPersistence = BookmarkPersistence(0x00000040);

info_enum!(NullCollation, u16, {
    NC_HIGH = 0,
    NC_LOW = 1,
    NC_START = 0x0002,
    NC_END = 0x0004,
});

odbc_bitmask!(UINTEGER, pub struct ScrollOptions);
pub const SO_FORWARD_ONLY: ScrollOptions = ScrollOptions(0x00000001);
pub const SO_KEYSET_DRIVEN: ScrollOptions = ScrollOptions(0x00000002);
pub const SO_DYNAMIC: ScrollOptions = ScrollOptions(0x00000004);
pub const SO_MIXED: ScrollOptions = ScrollOptions(0x00000008);
pub const SO_STATIC: ScrollOptions = ScrollOptions(0x00000010);

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(NICHE_VALUE = CTxnIsolation(0))]
#[rust_spec(with_custom_niche)]
#[repr(transparent)]
pub struct TxnIsolation(u32);
pub const TXN_READ_UNCOMMITTED: TxnIsolation = TxnIsolation(0x00000001);
pub const TXN_READ_COMMITTED: TxnIsolation = TxnIsolation(0x00000002);
pub const TXN_REPEATABLE_READ: TxnIsolation = TxnIsolation(0x00000004);
pub const TXN_SERIALIZABLE: TxnIsolation = TxnIsolation(0x00000008);
crate::attr::impl_odbc_scalar_attr_unpack!(TxnIsolation => u32);

odbc_bitmask!(UINTEGER, pub struct TxnIsolationOptions);

impl From<TxnIsolation> for TxnIsolationOptions {
    fn from(level: TxnIsolation) -> Self {
        Self(level.0)
    }
}

impl TryFrom<TxnIsolationOptions> for TxnIsolation {
    type Error = TxnIsolationOptions;

    fn try_from(options: TxnIsolationOptions) -> Result<Self, Self::Error> {
        if options.0.count_ones() == 1 {
            Ok(Self(options.0))
        } else {
            Err(options)
        }
    }
}

impl core::ops::BitAnd<TxnIsolation> for TxnIsolationOptions {
    type Output = u32;

    fn bitand(self, level: TxnIsolation) -> Self::Output {
        self.0 & level.0
    }
}

odbc_bitmask!(UINTEGER, pub struct AggregateFunctions);
pub const AF_AVG: AggregateFunctions = AggregateFunctions(0x00000001);
pub const AF_COUNT: AggregateFunctions = AggregateFunctions(0x00000002);
pub const AF_MAX: AggregateFunctions = AggregateFunctions(0x00000004);
pub const AF_MIN: AggregateFunctions = AggregateFunctions(0x00000008);
pub const AF_SUM: AggregateFunctions = AggregateFunctions(0x00000010);
pub const AF_DISTINCT: AggregateFunctions = AggregateFunctions(0x00000020);
pub const AF_ALL: AggregateFunctions = AggregateFunctions(0x00000040);
pub const AF_EVERY: AggregateFunctions = AggregateFunctions(0x00000080);
pub const AF_ANY: AggregateFunctions = AggregateFunctions(0x00000100);
pub const AF_STDEV_OP: AggregateFunctions = AggregateFunctions(0x00000200);
pub const AF_STDEV_SAMP: AggregateFunctions = AggregateFunctions(0x00000400);
pub const AF_VAR_SAMP: AggregateFunctions = AggregateFunctions(0x00000800);
pub const AF_VAR_POP: AggregateFunctions = AggregateFunctions(0x00001000);
pub const AF_ARRAY_AGG: AggregateFunctions = AggregateFunctions(0x00002000);
pub const AF_COLLECT: AggregateFunctions = AggregateFunctions(0x00004000);
pub const AF_FUSION: AggregateFunctions = AggregateFunctions(0x00008000);
pub const AF_INTERSECTION: AggregateFunctions = AggregateFunctions(0x00010000);

odbc_bitmask!(UINTEGER, pub struct AlterDomain);
pub const AD_CONSTRAINT_NAME_DEFINITION: AlterDomain = AlterDomain(0x00000001);
pub const AD_ADD_DOMAIN_CONSTRAINT: AlterDomain = AlterDomain(0x00000002);
pub const AD_DROP_DOMAIN_CONSTRAINT: AlterDomain = AlterDomain(0x00000004);
pub const AD_ADD_DOMAIN_DEFAULT: AlterDomain = AlterDomain(0x00000008);
pub const AD_DROP_DOMAIN_DEFAULT: AlterDomain = AlterDomain(0x00000010);
pub const AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: AlterDomain = AlterDomain(0x00000020);
pub const AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: AlterDomain = AlterDomain(0x00000040);
pub const AD_ADD_CONSTRAINT_DEFERRABLE: AlterDomain = AlterDomain(0x00000080);
pub const AD_ADD_CONSTRAINT_NON_DEFERRABLE: AlterDomain = AlterDomain(0x00000100);

odbc_bitmask!(UINTEGER, pub struct AlterTable);
pub const AT_ADD_CONSTRAINT: AlterTable = AlterTable(0x00000008);
pub const AT_ADD_COLUMN_SINGLE: AlterTable = AlterTable(0x00000020);
pub const AT_ADD_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000040);
pub const AT_ADD_COLUMN_COLLATION: AlterTable = AlterTable(0x00000080);
pub const AT_SET_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000100);
pub const AT_DROP_COLUMN_DEFAULT: AlterTable = AlterTable(0x00000200);
pub const AT_DROP_COLUMN_CASCADE: AlterTable = AlterTable(0x00000400);
pub const AT_DROP_COLUMN_RESTRICT: AlterTable = AlterTable(0x00000800);
pub const AT_ADD_TABLE_CONSTRAINT: AlterTable = AlterTable(0x00001000);
pub const AT_DROP_TABLE_CONSTRAINT_CASCADE: AlterTable = AlterTable(0x00002000);
pub const AT_DROP_TABLE_CONSTRAINT_RESTRICT: AlterTable = AlterTable(0x00004000);
pub const AT_CONSTRAINT_NAME_DEFINITION: AlterTable = AlterTable(0x00008000);
pub const AT_CONSTRAINT_INITIALLY_DEFERRED: AlterTable = AlterTable(0x00010000);
pub const AT_CONSTRAINT_INITIALLY_IMMEDIATE: AlterTable = AlterTable(0x00020000);
pub const AT_CONSTRAINT_DEFERRABLE: AlterTable = AlterTable(0x00040000);
pub const AT_CONSTRAINT_NON_DEFERRABLE: AlterTable = AlterTable(0x00080000);

odbc_bitmask!(UINTEGER, pub struct CatalogUsage);
pub const CU_DML_STATEMENTS: CatalogUsage = CatalogUsage(0x00000001);
pub const CU_PROCEDURE_INVOCATION: CatalogUsage = CatalogUsage(0x00000002);
pub const CU_TABLE_DEFINITION: CatalogUsage = CatalogUsage(0x00000004);
pub const CU_INDEX_DEFINITION: CatalogUsage = CatalogUsage(0x00000008);
pub const CU_PRIVILEGE_DEFINITION: CatalogUsage = CatalogUsage(0x00000010);

odbc_bitmask!(UINTEGER, pub struct CreateAssertion);
pub const CA_CREATE_ASSERTION: CreateAssertion = CreateAssertion(0x00000001);
pub const CA_CONSTRAINT_INITIALLY_DEFERRED: CreateAssertion = CreateAssertion(0x00000010);
pub const CA_CONSTRAINT_INITIALLY_IMMEDIATE: CreateAssertion = CreateAssertion(0x00000020);
pub const CA_CONSTRAINT_DEFERRABLE: CreateAssertion = CreateAssertion(0x00000040);
pub const CA_CONSTRAINT_NON_DEFERRABLE: CreateAssertion = CreateAssertion(0x00000080);

odbc_bitmask!(UINTEGER, pub struct CreateCharacterSet);
pub const CCS_CREATE_CHARACTER_SET: CreateCharacterSet = CreateCharacterSet(0x00000001);
pub const CCS_COLLATE_CLAUSE: CreateCharacterSet = CreateCharacterSet(0x00000002);
pub const CCS_LIMITED_COLLATION: CreateCharacterSet = CreateCharacterSet(0x00000004);

odbc_bitmask!(UINTEGER, pub struct CreateCollation);
pub const CCOL_CREATE_COLLATION: CreateCollation = CreateCollation(0x00000001);

odbc_bitmask!(UINTEGER, pub struct CreateDomain);
pub const CDO_CREATE_DOMAIN: CreateDomain = CreateDomain(0x00000001);
pub const CDO_DEFAULT: CreateDomain = CreateDomain(0x00000002);
pub const CDO_CONSTRAINT: CreateDomain = CreateDomain(0x00000004);
pub const CDO_COLLATION: CreateDomain = CreateDomain(0x00000008);
pub const CDO_CONSTRAINT_NAME_DEFINITION: CreateDomain = CreateDomain(0x00000010);
pub const CDO_CONSTRAINT_INITIALLY_DEFERRED: CreateDomain = CreateDomain(0x00000020);
pub const CDO_CONSTRAINT_INITIALLY_IMMEDIATE: CreateDomain = CreateDomain(0x00000040);
pub const CDO_CONSTRAINT_DEFERRABLE: CreateDomain = CreateDomain(0x00000080);
pub const CDO_CONSTRAINT_NON_DEFERRABLE: CreateDomain = CreateDomain(0x00000100);

odbc_bitmask!(UINTEGER, pub struct CreateSchema);
pub const CS_CREATE_SCHEMA: CreateSchema = CreateSchema(0x00000001);
pub const CS_AUTHORIZATION: CreateSchema = CreateSchema(0x00000002);
pub const CS_DEFAULT_CHARACTER_SET: CreateSchema = CreateSchema(0x00000004);

odbc_bitmask!(UINTEGER, pub struct CreateTable);
pub const CT_CREATE_TABLE: CreateTable = CreateTable(0x00000001);
pub const CT_COMMIT_PRESERVE: CreateTable = CreateTable(0x00000002);
pub const CT_COMMIT_DELETE: CreateTable = CreateTable(0x00000004);
pub const CT_GLOBAL_TEMPORARY: CreateTable = CreateTable(0x00000008);
pub const CT_LOCAL_TEMPORARY: CreateTable = CreateTable(0x00000010);
pub const CT_CONSTRAINT_INITIALLY_DEFERRED: CreateTable = CreateTable(0x00000020);
pub const CT_CONSTRAINT_INITIALLY_IMMEDIATE: CreateTable = CreateTable(0x00000040);
pub const CT_CONSTRAINT_DEFERRABLE: CreateTable = CreateTable(0x00000080);
pub const CT_CONSTRAINT_NON_DEFERRABLE: CreateTable = CreateTable(0x00000100);
pub const CT_COLUMN_CONSTRAINT: CreateTable = CreateTable(0x00000200);
pub const CT_COLUMN_DEFAULT: CreateTable = CreateTable(0x00000400);
pub const CT_COLUMN_COLLATION: CreateTable = CreateTable(0x00000800);
pub const CT_TABLE_CONSTRAINT: CreateTable = CreateTable(0x00001000);
pub const CT_CONSTRAINT_NAME_DEFINITION: CreateTable = CreateTable(0x00002000);

odbc_bitmask!(UINTEGER, pub struct CreateTranslation);
pub const CTR_CREATE_TRANSLATION: CreateTranslation = CreateTranslation(0x00000001);

odbc_bitmask!(UINTEGER, pub struct CreateView);
pub const CV_CREATE_VIEW: CreateView = CreateView(0x00000001);
pub const CV_CHECK_OPTION: CreateView = CreateView(0x00000002);
pub const CV_CASCADED: CreateView = CreateView(0x00000004);
pub const CV_LOCAL: CreateView = CreateView(0x00000008);

odbc_bitmask!(UINTEGER, pub struct DropAssertion);
pub const DA_DROP_ASSERTION: DropAssertion = DropAssertion(0x00000001);

odbc_bitmask!(UINTEGER, pub struct Conversion);
pub const CVT_CHAR: Conversion = Conversion(0x00000001);
pub const CVT_NUMERIC: Conversion = Conversion(0x00000002);
pub const CVT_DECIMAL: Conversion = Conversion(0x00000004);
pub const CVT_INTEGER: Conversion = Conversion(0x00000008);
pub const CVT_SMALLINT: Conversion = Conversion(0x00000010);
pub const CVT_FLOAT: Conversion = Conversion(0x00000020);
pub const CVT_REAL: Conversion = Conversion(0x00000040);
pub const CVT_DOUBLE: Conversion = Conversion(0x00000080);
pub const CVT_VARCHAR: Conversion = Conversion(0x00000100);
pub const CVT_LONGVARCHAR: Conversion = Conversion(0x00000200);
pub const CVT_BINARY: Conversion = Conversion(0x00000400);
pub const CVT_VARBINARY: Conversion = Conversion(0x00000800);
pub const CVT_BIT: Conversion = Conversion(0x00001000);
pub const CVT_TINYINT: Conversion = Conversion(0x00002000);
pub const CVT_BIGINT: Conversion = Conversion(0x00004000);
pub const CVT_DATE: Conversion = Conversion(0x00008000);
pub const CVT_TIME: Conversion = Conversion(0x00010000);
pub const CVT_TIMESTAMP: Conversion = Conversion(0x00020000);
pub const CVT_LONGVARBINARY: Conversion = Conversion(0x00040000);

pub const CVT_INTERVAL_YEAR_MONTH: Conversion = Conversion(0x00080000);
pub const CVT_INTERVAL_DAY_TIME: Conversion = Conversion(0x00100000);

pub const CVT_GUID: Conversion = Conversion(0x01000000);

odbc_bitmask!(UINTEGER, pub struct DropCharacterSet);
pub const DCS_DROP_CHARACTER_SET: DropCharacterSet = DropCharacterSet(0x00000001);

odbc_bitmask!(UINTEGER, pub struct DropCollation);
pub const DC_DROP_COLLATION: DropCollation = DropCollation(0x00000001);

odbc_bitmask!(UINTEGER, pub struct DropDomain);
pub const DD_DROP_DOMAIN: DropDomain = DropDomain(0x00000001);
pub const DD_RESTRICT: DropDomain = DropDomain(0x00000002);
pub const DD_CASCADE: DropDomain = DropDomain(0x00000004);

odbc_bitmask!(UINTEGER, pub struct DropSchema);
pub const DS_DROP_SCHEMA: DropSchema = DropSchema(0x00000001);
pub const DS_RESTRICT: DropSchema = DropSchema(0x00000002);
pub const DS_CASCADE: DropSchema = DropSchema(0x00000004);

odbc_bitmask!(UINTEGER, pub struct DropTable);
pub const DT_DROP_TABLE: DropTable = DropTable(0x00000001);
pub const DT_RESTRICT: DropTable = DropTable(0x00000002);
pub const DT_CASCADE: DropTable = DropTable(0x00000004);

odbc_bitmask!(UINTEGER, pub struct DropTranslation);
pub const DTR_DROP_TRANSLATION: DropTranslation = DropTranslation(0x00000001);

odbc_bitmask!(UINTEGER, pub struct DropView);
pub const DV_DROP_VIEW: DropView = DropView(0x00000001);
pub const DV_RESTRICT: DropView = DropView(0x00000002);
pub const DV_CASCADE: DropView = DropView(0x00000004);

odbc_bitmask!(UINTEGER, pub struct IndexKeywords);
pub const IK_NONE: IndexKeywords = IndexKeywords(0x00000000);
pub const IK_ASC: IndexKeywords = IndexKeywords(0x00000001);
pub const IK_DESC: IndexKeywords = IndexKeywords(0x00000002);
pub const IK_ALL: IndexKeywords = IndexKeywords(IK_ASC.0 | IK_DESC.0);

odbc_bitmask!(UINTEGER, pub struct InsertStatement);
pub const IS_INSERT_LITERALS: InsertStatement = InsertStatement(0x00000001);
pub const IS_INSERT_SEARCHED: InsertStatement = InsertStatement(0x00000002);
pub const IS_SELECT_INTO: InsertStatement = InsertStatement(0x00000004);

odbc_bitmask!(UINTEGER, pub struct OjCapabilities);
pub const OJ_LEFT: OjCapabilities = OjCapabilities(0x00000001);
pub const OJ_RIGHT: OjCapabilities = OjCapabilities(0x00000002);
pub const OJ_FULL: OjCapabilities = OjCapabilities(0x00000004);
pub const OJ_NESTED: OjCapabilities = OjCapabilities(0x00000008);
pub const OJ_NOT_ORDERED: OjCapabilities = OjCapabilities(0x00000010);
pub const OJ_INNER: OjCapabilities = OjCapabilities(0x00000020);
pub const OJ_ALL_COMPARISON_OPS: OjCapabilities = OjCapabilities(0x00000040);

odbc_bitmask!(UINTEGER, pub struct SchemaUsage);
pub const SU_DML_STATEMENTS: SchemaUsage = SchemaUsage(0x00000001);
pub const SU_PROCEDURE_INVOCATION: SchemaUsage = SchemaUsage(0x00000002);
pub const SU_TABLE_DEFINITION: SchemaUsage = SchemaUsage(0x00000004);
pub const SU_INDEX_DEFINITION: SchemaUsage = SchemaUsage(0x00000008);
pub const SU_PRIVILEGE_DEFINITION: SchemaUsage = SchemaUsage(0x00000010);

odbc_bitmask!(UINTEGER, pub struct Subqueries);
pub const SQ_COMPARISON: Subqueries = Subqueries(0x00000001);
pub const SQ_EXISTS: Subqueries = Subqueries(0x00000002);
pub const SQ_IN: Subqueries = Subqueries(0x00000004);
pub const SQ_QUANTIFIED: Subqueries = Subqueries(0x00000008);
pub const SQ_CORRELATED_SUBQUERIES: Subqueries = Subqueries(0x00000010);

odbc_bitmask!(UINTEGER, pub struct Union);
pub const U_UNION: Union = Union(0x00000001);
pub const U_UNION_ALL: Union = Union(0x00000002);

odbc_bitmask!(UINTEGER, pub struct ConvertFunctions);
pub const FN_CVT_CONVERT: ConvertFunctions = ConvertFunctions(0x00000001);
pub const FN_CVT_CAST: ConvertFunctions = ConvertFunctions(0x00000002);

odbc_bitmask!(UINTEGER, pub struct NumericFunctions);
pub const FN_NUM_ABS: NumericFunctions = NumericFunctions(0x00000001);
pub const FN_NUM_ACOS: NumericFunctions = NumericFunctions(0x00000002);
pub const FN_NUM_ASIN: NumericFunctions = NumericFunctions(0x00000004);
pub const FN_NUM_ATAN: NumericFunctions = NumericFunctions(0x00000008);
pub const FN_NUM_ATAN2: NumericFunctions = NumericFunctions(0x00000010);
pub const FN_NUM_CEILING: NumericFunctions = NumericFunctions(0x00000020);
pub const FN_NUM_COS: NumericFunctions = NumericFunctions(0x00000040);
pub const FN_NUM_COT: NumericFunctions = NumericFunctions(0x00000080);
pub const FN_NUM_EXP: NumericFunctions = NumericFunctions(0x00000100);
pub const FN_NUM_FLOOR: NumericFunctions = NumericFunctions(0x00000200);
pub const FN_NUM_LOG: NumericFunctions = NumericFunctions(0x00000400);
pub const FN_NUM_MOD: NumericFunctions = NumericFunctions(0x00000800);
pub const FN_NUM_SIGN: NumericFunctions = NumericFunctions(0x00001000);
pub const FN_NUM_SIN: NumericFunctions = NumericFunctions(0x00002000);
pub const FN_NUM_SQRT: NumericFunctions = NumericFunctions(0x00004000);
pub const FN_NUM_TAN: NumericFunctions = NumericFunctions(0x00008000);
pub const FN_NUM_PI: NumericFunctions = NumericFunctions(0x00010000);
pub const FN_NUM_RAND: NumericFunctions = NumericFunctions(0x00020000);
pub const FN_NUM_DEGREES: NumericFunctions = NumericFunctions(0x00040000);
pub const FN_NUM_LOG10: NumericFunctions = NumericFunctions(0x00080000);
pub const FN_NUM_POWER: NumericFunctions = NumericFunctions(0x00100000);
pub const FN_NUM_RADIANS: NumericFunctions = NumericFunctions(0x00200000);
pub const FN_NUM_ROUND: NumericFunctions = NumericFunctions(0x00400000);
pub const FN_NUM_TRUNCATE: NumericFunctions = NumericFunctions(0x00800000);

odbc_bitmask!(UINTEGER, pub struct StringFunctions);
pub const FN_STR_CONCAT: StringFunctions = StringFunctions(0x00000001);
pub const FN_STR_INSERT: StringFunctions = StringFunctions(0x00000002);
pub const FN_STR_LEFT: StringFunctions = StringFunctions(0x00000004);
pub const FN_STR_LTRIM: StringFunctions = StringFunctions(0x00000008);
pub const FN_STR_LENGTH: StringFunctions = StringFunctions(0x00000010);
pub const FN_STR_LOCATE: StringFunctions = StringFunctions(0x00000020);
pub const FN_STR_LCASE: StringFunctions = StringFunctions(0x00000040);
pub const FN_STR_REPEAT: StringFunctions = StringFunctions(0x00000080);
pub const FN_STR_REPLACE: StringFunctions = StringFunctions(0x00000100);
pub const FN_STR_RIGHT: StringFunctions = StringFunctions(0x00000200);
pub const FN_STR_RTRIM: StringFunctions = StringFunctions(0x00000400);
pub const FN_STR_SUBSTRING: StringFunctions = StringFunctions(0x00000800);
pub const FN_STR_UCASE: StringFunctions = StringFunctions(0x00001000);
pub const FN_STR_ASCII: StringFunctions = StringFunctions(0x00002000);
pub const FN_STR_CHAR: StringFunctions = StringFunctions(0x00004000);
pub const FN_STR_DIFFERENCE: StringFunctions = StringFunctions(0x00008000);
pub const FN_STR_LOCATE_2: StringFunctions = StringFunctions(0x00010000);
pub const FN_STR_SOUNDEX: StringFunctions = StringFunctions(0x00020000);
pub const FN_STR_SPACE: StringFunctions = StringFunctions(0x00040000);
pub const FN_STR_BIT_LENGTH: StringFunctions = StringFunctions(0x00080000);
pub const FN_STR_CHAR_LENGTH: StringFunctions = StringFunctions(0x00100000);
pub const FN_STR_CHARACTER_LENGTH: StringFunctions = StringFunctions(0x00200000);
pub const FN_STR_OCTET_LENGTH: StringFunctions = StringFunctions(0x00400000);
pub const FN_STR_POSITION: StringFunctions = StringFunctions(0x00800000);

odbc_bitmask!(UINTEGER, pub struct SystemFunctions);
pub const FN_SYS_USERNAME: SystemFunctions = SystemFunctions(0x00000001);
pub const FN_SYS_DBNAME: SystemFunctions = SystemFunctions(0x00000002);
pub const FN_SYS_IFNULL: SystemFunctions = SystemFunctions(0x00000004);

odbc_bitmask!(UINTEGER, pub struct TimedateIntervals);
pub const FN_TSI_FRAC_SECOND: TimedateIntervals = TimedateIntervals(0x00000001);
pub const FN_TSI_SECOND: TimedateIntervals = TimedateIntervals(0x00000002);
pub const FN_TSI_MINUTE: TimedateIntervals = TimedateIntervals(0x00000004);
pub const FN_TSI_HOUR: TimedateIntervals = TimedateIntervals(0x00000008);
pub const FN_TSI_DAY: TimedateIntervals = TimedateIntervals(0x00000010);
pub const FN_TSI_WEEK: TimedateIntervals = TimedateIntervals(0x00000020);
pub const FN_TSI_MONTH: TimedateIntervals = TimedateIntervals(0x00000040);
pub const FN_TSI_QUARTER: TimedateIntervals = TimedateIntervals(0x00000080);
pub const FN_TSI_YEAR: TimedateIntervals = TimedateIntervals(0x00000100);

odbc_bitmask!(UINTEGER, pub struct TimedateFunctions);
pub const FN_TD_NOW: TimedateFunctions = TimedateFunctions(0x00000001);
pub const FN_TD_CURDATE: TimedateFunctions = TimedateFunctions(0x00000002);
pub const FN_TD_DAYOFMONTH: TimedateFunctions = TimedateFunctions(0x00000004);
pub const FN_TD_DAYOFWEEK: TimedateFunctions = TimedateFunctions(0x00000008);
pub const FN_TD_DAYOFYEAR: TimedateFunctions = TimedateFunctions(0x00000010);
pub const FN_TD_MONTH: TimedateFunctions = TimedateFunctions(0x00000020);
pub const FN_TD_QUARTER: TimedateFunctions = TimedateFunctions(0x00000040);
pub const FN_TD_WEEK: TimedateFunctions = TimedateFunctions(0x00000080);
pub const FN_TD_YEAR: TimedateFunctions = TimedateFunctions(0x00000100);
pub const FN_TD_CURTIME: TimedateFunctions = TimedateFunctions(0x00000200);
pub const FN_TD_HOUR: TimedateFunctions = TimedateFunctions(0x00000400);
pub const FN_TD_MINUTE: TimedateFunctions = TimedateFunctions(0x00000800);
pub const FN_TD_SECOND: TimedateFunctions = TimedateFunctions(0x00001000);
pub const FN_TD_TIMESTAMPADD: TimedateFunctions = TimedateFunctions(0x00002000);
pub const FN_TD_TIMESTAMPDIFF: TimedateFunctions = TimedateFunctions(0x00004000);
pub const FN_TD_DAYNAME: TimedateFunctions = TimedateFunctions(0x00008000);
pub const FN_TD_MONTHNAME: TimedateFunctions = TimedateFunctions(0x00010000);
pub const FN_TD_CURRENT_DATE: TimedateFunctions = TimedateFunctions(0x00020000);
pub const FN_TD_CURRENT_TIME: TimedateFunctions = TimedateFunctions(0x00040000);
pub const FN_TD_CURRENT_TIMESTAMP: TimedateFunctions = TimedateFunctions(0x00080000);
pub const FN_TD_EXTRACT: TimedateFunctions = TimedateFunctions(0x00100000);

odbc_bitmask!(UINTEGER, pub struct DatetimeFunctions);
pub const SDF_CURRENT_DATE: DatetimeFunctions = DatetimeFunctions(0x00000001);
pub const SDF_CURRENT_TIME: DatetimeFunctions = DatetimeFunctions(0x00000002);
pub const SDF_CURRENT_TIMESTAMP: DatetimeFunctions = DatetimeFunctions(0x00000004);

odbc_bitmask!(UINTEGER, pub struct DatetimeLiterals);
pub const DL_SQL92_DATE: DatetimeLiterals = DatetimeLiterals(0x00000001);
pub const DL_SQL92_TIME: DatetimeLiterals = DatetimeLiterals(0x00000002);
pub const DL_SQL92_TIMESTAMP: DatetimeLiterals = DatetimeLiterals(0x00000004);
pub const DL_SQL92_INTERVAL_YEAR: DatetimeLiterals = DatetimeLiterals(0x00000008);
pub const DL_SQL92_INTERVAL_MONTH: DatetimeLiterals = DatetimeLiterals(0x00000010);
pub const DL_SQL92_INTERVAL_DAY: DatetimeLiterals = DatetimeLiterals(0x00000020);
pub const DL_SQL92_INTERVAL_HOUR: DatetimeLiterals = DatetimeLiterals(0x00000040);
pub const DL_SQL92_INTERVAL_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00000080);
pub const DL_SQL92_INTERVAL_SECOND: DatetimeLiterals = DatetimeLiterals(0x00000100);
pub const DL_SQL92_INTERVAL_YEAR_TO_MONTH: DatetimeLiterals = DatetimeLiterals(0x00000200);
pub const DL_SQL92_INTERVAL_DAY_TO_HOUR: DatetimeLiterals = DatetimeLiterals(0x00000400);
pub const DL_SQL92_INTERVAL_DAY_TO_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00000800);
pub const DL_SQL92_INTERVAL_DAY_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00001000);
pub const DL_SQL92_INTERVAL_HOUR_TO_MINUTE: DatetimeLiterals = DatetimeLiterals(0x00002000);
pub const DL_SQL92_INTERVAL_HOUR_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00004000);
pub const DL_SQL92_INTERVAL_MINUTE_TO_SECOND: DatetimeLiterals = DatetimeLiterals(0x00008000);

odbc_bitmask!(UINTEGER, pub struct ForeignKeyDeleteRule);
pub const SFKD_CASCADE: ForeignKeyDeleteRule = ForeignKeyDeleteRule(0x00000001);
pub const SFKD_NO_ACTION: ForeignKeyDeleteRule = ForeignKeyDeleteRule(0x00000002);
pub const SFKD_SET_DEFAULT: ForeignKeyDeleteRule = ForeignKeyDeleteRule(0x00000004);
pub const SFKD_SET_NULL: ForeignKeyDeleteRule = ForeignKeyDeleteRule(0x00000008);

odbc_bitmask!(UINTEGER, pub struct ForeignKeyUpdateRule);
pub const SFKU_CASCADE: ForeignKeyUpdateRule = ForeignKeyUpdateRule(0x00000001);
pub const SFKU_NO_ACTION: ForeignKeyUpdateRule = ForeignKeyUpdateRule(0x00000002);
pub const SFKU_SET_DEFAULT: ForeignKeyUpdateRule = ForeignKeyUpdateRule(0x00000004);
pub const SFKU_SET_NULL: ForeignKeyUpdateRule = ForeignKeyUpdateRule(0x00000008);

odbc_bitmask!(UINTEGER, pub struct Grant);
pub const SG_USAGE_ON_DOMAIN: Grant = Grant(0x00000001);
pub const SG_USAGE_ON_CHARACTER_SET: Grant = Grant(0x00000002);
pub const SG_USAGE_ON_COLLATION: Grant = Grant(0x00000004);
pub const SG_USAGE_ON_TRANSLATION: Grant = Grant(0x00000008);
pub const SG_WITH_GRANT_OPTION: Grant = Grant(0x00000010);
pub const SG_DELETE_TABLE: Grant = Grant(0x00000020);
pub const SG_INSERT_TABLE: Grant = Grant(0x00000040);
pub const SG_INSERT_COLUMN: Grant = Grant(0x00000080);
pub const SG_REFERENCES_TABLE: Grant = Grant(0x00000100);
pub const SG_REFERENCES_COLUMN: Grant = Grant(0x00000200);
pub const SG_SELECT_TABLE: Grant = Grant(0x00000400);
pub const SG_UPDATE_TABLE: Grant = Grant(0x00000800);
pub const SG_UPDATE_COLUMN: Grant = Grant(0x00001000);

odbc_bitmask!(UINTEGER, pub struct NumericValueFunctions);
pub const SNVF_BIT_LENGTH: NumericValueFunctions = NumericValueFunctions(0x00000001);
pub const SNVF_CHAR_LENGTH: NumericValueFunctions = NumericValueFunctions(0x00000002);
pub const SNVF_CHARACTER_LENGTH: NumericValueFunctions = NumericValueFunctions(0x00000004);
pub const SNVF_EXTRACT: NumericValueFunctions = NumericValueFunctions(0x00000008);
pub const SNVF_OCTET_LENGTH: NumericValueFunctions = NumericValueFunctions(0x00000010);
pub const SNVF_POSITION: NumericValueFunctions = NumericValueFunctions(0x00000020);

odbc_bitmask!(UINTEGER, pub struct Predicates);
pub const SP_EXISTS: Predicates = Predicates(0x00000001);
pub const SP_ISNOTNULL: Predicates = Predicates(0x00000002);
pub const SP_ISNULL: Predicates = Predicates(0x00000004);
pub const SP_MATCH_FULL: Predicates = Predicates(0x00000008);
pub const SP_MATCH_PARTIAL: Predicates = Predicates(0x00000010);
pub const SP_MATCH_UNIQUE_FULL: Predicates = Predicates(0x00000020);
pub const SP_MATCH_UNIQUE_PARTIAL: Predicates = Predicates(0x00000040);
pub const SP_OVERLAPS: Predicates = Predicates(0x00000080);
pub const SP_UNIQUE: Predicates = Predicates(0x00000100);
pub const SP_LIKE: Predicates = Predicates(0x00000200);
pub const SP_IN: Predicates = Predicates(0x00000400);
pub const SP_BETWEEN: Predicates = Predicates(0x00000800);
pub const SP_COMPARISON: Predicates = Predicates(0x00001000);
pub const SP_QUANTIFIED_COMPARISON: Predicates = Predicates(0x00002000);

odbc_bitmask!(UINTEGER, pub struct RelationalJoinOperators);
pub const SRJO_CORRESPONDING_CLAUSE: RelationalJoinOperators = RelationalJoinOperators(0x00000001);
pub const SRJO_CROSS_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000002);
pub const SRJO_EXCEPT_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000004);
pub const SRJO_FULL_OUTER_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000008);
pub const SRJO_INNER_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000010);
pub const SRJO_INTERSECT_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000020);
pub const SRJO_LEFT_OUTER_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000040);
pub const SRJO_NATURAL_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000080);
pub const SRJO_RIGHT_OUTER_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000100);
pub const SRJO_UNION_JOIN: RelationalJoinOperators = RelationalJoinOperators(0x00000200);

odbc_bitmask!(UINTEGER, pub struct Revoke);
pub const SR_USAGE_ON_DOMAIN: Revoke = Revoke(0x00000001);
pub const SR_USAGE_ON_CHARACTER_SET: Revoke = Revoke(0x00000002);
pub const SR_USAGE_ON_COLLATION: Revoke = Revoke(0x00000004);
pub const SR_USAGE_ON_TRANSLATION: Revoke = Revoke(0x00000008);
pub const SR_GRANT_OPTION_FOR: Revoke = Revoke(0x00000010);
pub const SR_CASCADE: Revoke = Revoke(0x00000020);
pub const SR_RESTRICT: Revoke = Revoke(0x00000040);
pub const SR_DELETE_TABLE: Revoke = Revoke(0x00000080);
pub const SR_INSERT_TABLE: Revoke = Revoke(0x00000100);
pub const SR_INSERT_COLUMN: Revoke = Revoke(0x00000200);
pub const SR_REFERENCES_TABLE: Revoke = Revoke(0x00000400);
pub const SR_REFERENCES_COLUMN: Revoke = Revoke(0x00000800);
pub const SR_SELECT_TABLE: Revoke = Revoke(0x00001000);
pub const SR_UPDATE_TABLE: Revoke = Revoke(0x00002000);
pub const SR_UPDATE_COLUMN: Revoke = Revoke(0x00004000);

odbc_bitmask!(UINTEGER, pub struct RowValueConstructor);
pub const SRVC_VALUE_EXPRESSION: RowValueConstructor = RowValueConstructor(0x00000001);
pub const SRVC_NULL: RowValueConstructor = RowValueConstructor(0x00000002);
pub const SRVC_DEFAULT: RowValueConstructor = RowValueConstructor(0x00000004);
pub const SRVC_ROW_SUBQUERY: RowValueConstructor = RowValueConstructor(0x00000008);

odbc_bitmask!(UINTEGER, pub struct StringScalarFunctions);
pub const SSF_CONVERT: StringScalarFunctions = StringScalarFunctions(0x00000001);
pub const SSF_LOWER: StringScalarFunctions = StringScalarFunctions(0x00000002);
pub const SSF_UPPER: StringScalarFunctions = StringScalarFunctions(0x00000004);
pub const SSF_SUBSTRING: StringScalarFunctions = StringScalarFunctions(0x00000008);
pub const SSF_TRANSLATE: StringScalarFunctions = StringScalarFunctions(0x00000010);
pub const SSF_TRIM_BOTH: StringScalarFunctions = StringScalarFunctions(0x00000020);
pub const SSF_TRIM_LEADING: StringScalarFunctions = StringScalarFunctions(0x00000040);
pub const SSF_TRIM_TRAILING: StringScalarFunctions = StringScalarFunctions(0x00000080);
pub const SSF_OVERLAY: StringScalarFunctions = StringScalarFunctions(0x00000100);
pub const SSF_LENGTH: StringScalarFunctions = StringScalarFunctions(0x00000200);
pub const SSF_POSITION: StringScalarFunctions = StringScalarFunctions(0x00000400);
pub const SSF_CONCAT: StringScalarFunctions = StringScalarFunctions(0x00000800);

odbc_bitmask!(UINTEGER, pub struct ValueExpressions);
pub const SVE_CASE: ValueExpressions = ValueExpressions(0x00000001);
pub const SVE_CAST: ValueExpressions = ValueExpressions(0x00000002);
pub const SVE_COALESCE: ValueExpressions = ValueExpressions(0x00000004);
pub const SVE_NULLIF: ValueExpressions = ValueExpressions(0x00000008);

odbc_bitmask!(UINTEGER, pub struct StandardCliConformance);
pub const SCC_XOPEN_CLI_VERSION1: StandardCliConformance = StandardCliConformance(0x00000001);
pub const SCC_ISO92_CLI: StandardCliConformance = StandardCliConformance(0x00000002);

odbc_bitmask!(UINTEGER, pub struct BinaryFunctions);
pub const FN_BIN_BIT_LENGTH: BinaryFunctions = BinaryFunctions(FN_STR_BIT_LENGTH.0);
pub const FN_BIN_CONCAT: BinaryFunctions = BinaryFunctions(FN_STR_CONCAT.0);
pub const FN_BIN_INSERT: BinaryFunctions = BinaryFunctions(FN_STR_INSERT.0);
pub const FN_BIN_LTRIM: BinaryFunctions = BinaryFunctions(FN_STR_LTRIM.0);
pub const FN_BIN_OCTET_LENGTH: BinaryFunctions = BinaryFunctions(FN_STR_OCTET_LENGTH.0);
pub const FN_BIN_POSITION: BinaryFunctions = BinaryFunctions(FN_STR_POSITION.0);
pub const FN_BIN_RTRIM: BinaryFunctions = BinaryFunctions(FN_STR_RTRIM.0);
pub const FN_BIN_SUBSTRING: BinaryFunctions = BinaryFunctions(FN_STR_SUBSTRING.0);

odbc_bitmask!(UINTEGER, pub struct IsoBinaryFunctions);
pub const SBF_CONVERT: IsoBinaryFunctions = IsoBinaryFunctions(SSF_CONVERT.0);
pub const SBF_SUBSTRING: IsoBinaryFunctions = IsoBinaryFunctions(SSF_SUBSTRING.0);
pub const SBF_TRIM_BOTH: IsoBinaryFunctions = IsoBinaryFunctions(SSF_TRIM_BOTH.0);
pub const SBF_TRIM_LEADING: IsoBinaryFunctions = IsoBinaryFunctions(SSF_TRIM_LEADING.0);
pub const SBF_TRIM_TRAILING: IsoBinaryFunctions = IsoBinaryFunctions(SSF_TRIM_TRAILING.0);
pub const SBF_OVERLAY: IsoBinaryFunctions = IsoBinaryFunctions(SSF_OVERLAY.0);
pub const SBF_POSITION: IsoBinaryFunctions = IsoBinaryFunctions(SSF_POSITION.0);
pub const SBF_CONCAT: IsoBinaryFunctions = IsoBinaryFunctions(SSF_CONCAT.0);

odbc_bitmask!(UINTEGER, pub struct LimitEscapeClause);
pub const LC_NONE: LimitEscapeClause = LimitEscapeClause(0x00000000);
pub const LC_TAKE: LimitEscapeClause = LimitEscapeClause(0x00000001);
pub const LC_SKIP: LimitEscapeClause = LimitEscapeClause(0x00000003);

odbc_bitmask!(UINTEGER, pub struct ReturnEscapeClause);
pub const RC_NONE: ReturnEscapeClause = ReturnEscapeClause(0x00000000);
pub const RC_INSERT_SINGLE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000001);
pub const RC_INSERT_SINGLE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000002 | RC_INSERT_SINGLE_ROWID.0);
pub const RC_INSERT_MULTIPLE_ROWID: ReturnEscapeClause =
    ReturnEscapeClause(0x00000004 | RC_INSERT_SINGLE_ROWID.0);
pub const RC_INSERT_MULTIPLE_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000008 | RC_INSERT_MULTIPLE_ROWID.0 | RC_INSERT_SINGLE_ANY.0);
pub const RC_INSERT_SELECT_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000010);
pub const RC_INSERT_SELECT_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000020 | RC_INSERT_SELECT_ROWID.0);
pub const RC_UPDATE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000040);
pub const RC_UPDATE_ANY: ReturnEscapeClause = ReturnEscapeClause(0x00000080 | RC_UPDATE_ROWID.0);
pub const RC_DELETE_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000100);
pub const RC_DELETE_ANY: ReturnEscapeClause = ReturnEscapeClause(0x00000200 | RC_DELETE_ROWID.0);
pub const RC_SELECT_INTO_ROWID: ReturnEscapeClause = ReturnEscapeClause(0x00000400);
pub const RC_SELECT_INTO_ANY: ReturnEscapeClause =
    ReturnEscapeClause(0x00000800 | RC_SELECT_INTO_ROWID.0);

odbc_bitmask!(UINTEGER, pub struct FormatEscapeClause);
pub const FC_NONE: FormatEscapeClause = FormatEscapeClause(0x00000000);
pub const FC_JSON: FormatEscapeClause = FormatEscapeClause(0x00000001);
pub const FC_JSON_BINARY: FormatEscapeClause = FormatEscapeClause(0x00000002);
