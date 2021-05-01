pub use crate::SQLSMALLINT;
pub use rs_odbc_derive::odbc_type;

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub struct SqlType;

/// Data type cannot be determined
pub const SQL_UNKNOWN_TYPE: SqlType = SqlType(0);

/// Column whose type may vary across rows
#[cfg(feature = "v4")]
pub use SQL_UNKNOWN_TYPE as SQL_VARIANT_TYPE;

/// Character string of fixed string length n.
///
/// SQL data type: CHAR(n)
pub const SQL_CHAR: SqlType = SqlType(1);

/// Variable-length character string with a maximum string length n.
///
/// SQL data type: VARCHAR(n)
pub const SQL_VARCHAR: SqlType = SqlType(12);

/// Variable length character data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARCHAR
pub const SQL_LONGVARCHAR: SqlType = SqlType(-1);

/// Unicode character string of fixed string length n
///
/// SQL data type: WCHAR(n)
pub const SQL_WCHAR: SqlType = SqlType(-8);

/// Unicode variable-length character string with a maximum string length n
///
/// SQL data type: VARWCHAR(n)
pub const SQL_WVARCHAR: SqlType = SqlType(-9);

/// Unicode variable-length character data. Maximum length is data source-dependent
///
/// SQL data type: LONGWVARCHAR
pub const SQL_WLONGVARCHAR: SqlType = SqlType(-10);

/// Signed, exact, numeric value with a precision of at least p and scale s. (The maximum precision is driver-defined.) (1 <= p <= 15; s <= p).
///
/// SQL data type: DECIMAL(p,s)
pub const SQL_DECIMAL: SqlType = SqlType(3);

/// Signed, exact, numeric value with a precision p and scale s (1 <= p <= 15; s <= p).
///
/// SQL data type: NUMERIC(p,s)
pub const SQL_NUMERIC: SqlType = SqlType(2);

/// Exact numeric value with precision 5 and scale 0  (signed:  -32,768 <= n <= 32,767, unsigned:  0 <= n <= 65,535).
///
/// SQL data type: SMALLINT
pub const SQL_SMALLINT: SqlType = SqlType(5);

/// Exact numeric value with precision 10 and scale 0  (signed:  -2[31] <= n <= 2[31] - 1, unsigned:  0 <= n <= 2[32] - 1).
///
/// SQL data type: INTEGER
pub const SQL_INTEGER: SqlType = SqlType(4);

/// Signed, approximate, numeric value with a binary precision 24 (zero or absolute value 10[-38] to 10[38]).
///
/// SQL data type: REAL
pub const SQL_REAL: SqlType = SqlType(7);

/// Signed, approximate, numeric value with a binary precision of at least p. (The maximum precision is driver-defined.)
///
/// SQL data type: FLOAT(p)
pub const SQL_FLOAT: SqlType = SqlType(6);

/// Signed, approximate, numeric value with a binary precision 53 (zero or absolute value 10[-308] to 10[308]).
///
/// SQL data type: DOUBLE PRECISION
pub const SQL_DOUBLE: SqlType = SqlType(8);

/// Single bit binary data.
///
/// SQL data type: BIT
pub const SQL_BIT: SqlType = SqlType(-7);

/// Exact numeric value with precision 3 and scale 0  (signed:  -128 <= n <= 127,  unsigned:  0 <= n <= 255).
///
/// SQL data type: TINYINT
pub const SQL_TINYINT: SqlType = SqlType(-6);

/// Exact numeric value with precision 19 (if signed) or 20 (if unsigned) and scale 0  (signed:  -2[63] <= n <= 2[63] - 1,  unsigned: 0 <= n <= 2[64] - 1),.
///
/// SQL data type: BIGINT
pub const SQL_BIGINT: SqlType = SqlType(-5);

/// Binary data of fixed length n.
///
/// SQL data type: BINARY(n)
pub const SQL_BINARY: SqlType = SqlType(-2);

/// Variable length binary data of maximum length n. The maximum is set by the user.
///
/// SQL data type: VARBINARY(n)
pub const SQL_VARBINARY: SqlType = SqlType(-3);

/// Variable length binary data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARBINARY
pub const SQL_LONGVARBINARY: SqlType = SqlType(-4);

/// Fixed length GUID.
///
/// SQL data type: GUID
#[cfg(feature = "v3_5")]
pub const SQL_GUID: SqlType = SqlType(-11);

/// Year, month, and day fields, conforming to the rules of the Gregorian calendar. (See Constraints of the Gregorian Calendar, later in this appendix.)
///
/// SQL data type: DATE
pub const SQL_TYPE_DATE: SqlType = SqlType(91);

/// Hour, minute, and second fields, with valid values for hours of 00 to 23, valid values for minutes of 00 to 59, and valid values for seconds of 00 to 61. Precision p indicates the seconds precision.
///
/// SQL data type: TIME(p)
pub const SQL_TYPE_TIME: SqlType = SqlType(92);

/// Year, month, day, hour, minute, and second fields, with valid values as defined for the DATE and TIME data types.
///
/// SQL data type: TIMESTAMP(p)
pub const SQL_TYPE_TIMESTAMP: SqlType = SqlType(93);

#[cfg(feature = "v4")]
pub const SQL_TYPE_TIME_WITH_TIMEZONE: SqlType = SqlType(94);

#[cfg(feature = "v4")]
pub const SQL_TYPE_TIMESTAMP_WITH_TIMEZONE: SqlType = SqlType(95);

// TODO: These are not found in the reference implementation but exist in documentation
//    /// Year, month, day, hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision.
//    ///
//    /// SQL data type: UTCDATETIME
//    pub const SQL_TYPE_UTCDATETIME: SqlType = SqlType(x);
//
//    /// Hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision..
//    ///
//    /// SQL data type: UTCTIME
//    pub const SQL_TYPE_UTCTIME: SqlType = SqlType(x);

/// Number of months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL MONTH(p)
pub const SQL_INTERVAL_MONTH: SqlType = SqlType(100 + SQL_CODE_MONTH.0);

/// Number of years between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p)
pub const SQL_INTERVAL_YEAR: SqlType = SqlType(100 + SQL_CODE_YEAR.0);

/// Number of years and months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p) TO MONTH
pub const SQL_INTERVAL_YEAR_TO_MONTH: SqlType = SqlType(100 + SQL_CODE_YEAR_TO_MONTH.0);

/// Number of days between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p)
pub const SQL_INTERVAL_DAY: SqlType = SqlType(100 + SQL_CODE_DAY.0);

/// Number of hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p)
pub const SQL_INTERVAL_HOUR: SqlType = SqlType(100 + SQL_CODE_HOUR.0);

/// Number of minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL MINUTE(p)
pub const SQL_INTERVAL_MINUTE: SqlType = SqlType(100 + SQL_CODE_MINUTE.0);

/// Number of seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL SECOND(p,q)
pub const SQL_INTERVAL_SECOND: SqlType = SqlType(100 + SQL_CODE_SECOND.0);

/// Number of days/hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO HOUR
pub const SQL_INTERVAL_DAY_TO_HOUR: SqlType = SqlType(100 + SQL_CODE_DAY_TO_HOUR.0);

/// Number of days/hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO MINUTE
pub const SQL_INTERVAL_DAY_TO_MINUTE: SqlType = SqlType(100 + SQL_CODE_DAY_TO_MINUTE.0);

/// Number of days/hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL DAY(p) TO SECOND(q)
pub const SQL_INTERVAL_DAY_TO_SECOND: SqlType = SqlType(100 + SQL_CODE_DAY_TO_SECOND.0);

/// Number of hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p) TO MINUTE
pub const SQL_INTERVAL_HOUR_TO_MINUTE: SqlType = SqlType(100 + SQL_CODE_HOUR_TO_MINUTE.0);

/// Number of hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL HOUR(p) TO SECOND(q)
pub const SQL_INTERVAL_HOUR_TO_SECOND: SqlType = SqlType(100 + SQL_CODE_HOUR_TO_SECOND.0);

/// Number of minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL MINUTE(p) TO SECOND(q)
pub const SQL_INTERVAL_MINUTE_TO_SECOND: SqlType = SqlType(100 + SQL_CODE_MINUTE_TO_SECOND.0);

#[cfg(feature = "v4")]
pub const SQL_UDT: SqlType = SqlType(17);

#[cfg(feature = "v4")]
pub const SQL_ROW: SqlType = SqlType(19);

#[cfg(feature = "v4")]
pub const SQL_ARRAY: SqlType = SqlType(50);

#[cfg(feature = "v4")]
pub const SQL_MULTISET: SqlType = SqlType(55);

// =================================================================================== //

/// Datetime verbose type identifier.
pub const SQL_DATETIME: SqlType = SqlType(9);

/// Interval verbose type identifier.
pub const SQL_INTERVAL: SqlType = SqlType(10);

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DATETIME_INTERVAL_CODE;

// Subcodes for the specific verbose datetime data type
pub const SQL_CODE_DATE: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(1);
pub const SQL_CODE_TIME: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(2);
pub const SQL_CODE_TIMESTAMP: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(3);

// Subcode for the specific verbose interval data type
pub const SQL_CODE_YEAR: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(1);
pub const SQL_CODE_MONTH: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(2);
pub const SQL_CODE_DAY: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(3);
pub const SQL_CODE_HOUR: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(4);
pub const SQL_CODE_MINUTE: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(5);
pub const SQL_CODE_SECOND: SQL_DESC_DATETIME_INTERVAL_CODE = SQL_DESC_DATETIME_INTERVAL_CODE(6);
pub const SQL_CODE_YEAR_TO_MONTH: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(7);
pub const SQL_CODE_DAY_TO_HOUR: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(8);
pub const SQL_CODE_DAY_TO_MINUTE: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(9);
pub const SQL_CODE_DAY_TO_SECOND: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(10);
pub const SQL_CODE_HOUR_TO_MINUTE: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(11);
pub const SQL_CODE_HOUR_TO_SECOND: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(12);
pub const SQL_CODE_MINUTE_TO_SECOND: SQL_DESC_DATETIME_INTERVAL_CODE =
    SQL_DESC_DATETIME_INTERVAL_CODE(13);
