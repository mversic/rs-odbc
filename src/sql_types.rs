pub use crate::SQLSMALLINT;
pub use rs_odbc_derive::{Identifier, SqlType};

pub trait SqlType: crate::Identifier<IdentType = SQLSMALLINT> {}

/// Data type cannot be determined
#[identifier(SQLSMALLINT, 0)]
#[derive(Identifier, SqlType)]
pub struct SQL_UNKNOWN_TYPE;

// TODO: Is ok?
/// Column whose type may vary across rows
#[cfg(feature = "v4")]
pub use SQL_UNKNOWN_TYPE as SQL_VARIANT_TYPE;

/// Character string of fixed string length n.
///
/// SQL data type: CHAR(n)
#[identifier(SQLSMALLINT, 1)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_CHAR;

/// Variable-length character string with a maximum string length n.
///
/// SQL data type: VARCHAR(n)
#[identifier(SQLSMALLINT, 12)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_VARCHAR;

/// Variable length character data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARCHAR
#[identifier(SQLSMALLINT, -1)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_LONGVARCHAR;

/// Unicode character string of fixed string length n
///
/// SQL data type: WCHAR(n)
#[identifier(SQLSMALLINT, -8)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_WCHAR;

/// Unicode variable-length character string with a maximum string length n
///
/// SQL data type: VARWCHAR(n)
#[identifier(SQLSMALLINT, -9)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_WVARCHAR;

/// Unicode variable-length character data. Maximum length is data source-dependent
///
/// SQL data type: LONGWVARCHAR
#[identifier(SQLSMALLINT, -10)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_WLONGVARCHAR;

/// Signed, exact, numeric value with a precision of at least p and scale s. (The maximum precision is driver-defined.) (1 <= p <= 15; s <= p).
///
/// SQL data type: DECIMAL(p,s)
#[identifier(SQLSMALLINT, 3)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_DECIMAL;

/// Signed, exact, numeric value with a precision p and scale s (1 <= p <= 15; s <= p).
///
/// SQL data type: NUMERIC(p,s)
#[identifier(SQLSMALLINT, 2)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_NUMERIC;

/// Exact numeric value with precision 5 and scale 0  (signed:  -32,768 <= n <= 32,767, unsigned:  0 <= n <= 65,535).
///
/// SQL data type: SMALLINT
#[identifier(SQLSMALLINT, 5)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_SMALLINT;

/// Exact numeric value with precision 10 and scale 0  (signed:  -2[31] <= n <= 2[31] - 1, unsigned:  0 <= n <= 2[32] - 1).
///
/// SQL data type: INTEGER
#[identifier(SQLSMALLINT, 4)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTEGER;

/// Signed, approximate, numeric value with a binary precision 24 (zero or absolute value 10[-38] to 10[38]).
///
/// SQL data type: REAL
#[identifier(SQLSMALLINT, 7)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_REAL;

/// Signed, approximate, numeric value with a binary precision of at least p. (The maximum precision is driver-defined.)
///
/// SQL data type: FLOAT(p)
#[identifier(SQLSMALLINT, 6)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_FLOAT;

/// Signed, approximate, numeric value with a binary precision 53 (zero or absolute value 10[-308] to 10[308]).
///
/// SQL data type: DOUBLE PRECISION
#[identifier(SQLSMALLINT, 8)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_DOUBLE;

/// Single bit binary data.
///
/// SQL data type: BIT
#[identifier(SQLSMALLINT, -7)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_BIT;

/// Exact numeric value with precision 3 and scale 0  (signed:  -128 <= n <= 127,  unsigned:  0 <= n <= 255).
///
/// SQL data type: TINYINT
#[identifier(SQLSMALLINT, -6)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TINYINT;

/// Exact numeric value with precision 19 (if signed) or 20 (if unsigned) and scale 0  (signed:  -2[63] <= n <= 2[63] - 1,  unsigned: 0 <= n <= 2[64] - 1).
///
/// SQL data type: BIGINT
#[identifier(SQLSMALLINT, -5)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_BIGINT;

/// Binary data of fixed length n.
///
/// SQL data type: BINARY(n)
#[identifier(SQLSMALLINT, -2)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_BINARY;

/// Variable length binary data of maximum length n. The maximum is set by the user.
///
/// SQL data type: VARBINARY(n)
#[identifier(SQLSMALLINT, -3)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_VARBINARY;

/// Variable length binary data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARBINARY
#[identifier(SQLSMALLINT, -4)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_LONGVARBINARY;

/// Year, month, and day fields, conforming to the rules of the Gregorian calendar. (See Constraints of the Gregorian Calendar, later in this appendix.)
///
/// SQL data type: DATE
#[identifier(SQLSMALLINT, 92)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TYPE_DATE;

/// Hour, minute, and second fields, with valid values for hours of 00 to 23, valid values for minutes of 00 to 59, and valid values for seconds of 00 to 61. Precision p indicates the seconds precision.
///
/// SQL data type: TIME(p)
#[identifier(SQLSMALLINT, 92)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TYPE_TIME;

/// Year, month, day, hour, minute, and second fields, with valid values as defined for the DATE and TIME data types.
///
/// SQL data type: TIMESTAMP(p)
#[identifier(SQLSMALLINT, 93)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TYPE_TIMESTAMP;

// TODO: These are not found in the reference implementation but exist in documentation
///// Year, month, day, hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision.
/////
///// SQL data type: UTCDATETIME
//#[identifier(SQLSMALLINT, x)]
//#[derive(Identifier, SqlType)]
//#[allow(non_camel_case_types)]
//pub struct SQL_TYPE_UTCDATETIME;
//
///// Hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision..
/////
///// SQL data type: UTCTIME
//#[identifier(SQLSMALLINT, x)]
//#[derive(Identifier, SqlType)]
//#[allow(non_camel_case_types)]
//pub struct SQL_TYPE_UTCTIME;

/// Number of months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL MONTH(p)
#[identifier(SQLSMALLINT, 102)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_MONTH;

/// Number of years between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p)
#[identifier(SQLSMALLINT, 101)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_YEAR;

/// Number of years and months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p) TO MONTH
#[identifier(SQLSMALLINT, 107)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_YEAR_TO_MONTH;

/// Number of days between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p)
#[identifier(SQLSMALLINT, 103)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_DAY;

/// Number of hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p)
#[identifier(SQLSMALLINT, 104)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_HOUR;

/// Number of minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL MINUTE(p)
#[identifier(SQLSMALLINT, 105)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_MINUTE;

/// Number of seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL SECOND(p,q)
#[identifier(SQLSMALLINT, 106)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_SECOND;

/// Number of days/hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO HOUR
#[identifier(SQLSMALLINT, 108)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_DAY_TO_HOUR;

/// Number of days/hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO MINUTE
#[identifier(SQLSMALLINT, 109)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_DAY_TO_MINUTE;

/// Number of days/hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL DAY(p) TO SECOND(q)
#[identifier(SQLSMALLINT, 110)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_DAY_TO_SECOND;

/// Number of hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p) TO MINUTE
#[identifier(SQLSMALLINT, 111)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_HOUR_TO_MINUTE;

/// Number of hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL HOUR(p) TO SECOND(q)
#[identifier(SQLSMALLINT, 112)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_HOUR_TO_SECOND;

/// Number of minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL MINUTE(p) TO SECOND(q)
#[identifier(SQLSMALLINT, 113)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_MINUTE_TO_SECOND;

/// Fixed length GUID.
///
/// SQL data type: GUID
#[cfg(feature = "v3_5")]
#[identifier(SQLSMALLINT, -11)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_GUID;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 94)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TYPE_TIME_WITH_TIMEZONE;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 95)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_TYPE_TIMESTAMP_WITH_TIMEZONE;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 17)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_UDT;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 19)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_ROW;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 50)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_ARRAY;

#[cfg(feature = "v4")]
#[identifier(SQLSMALLINT, 55)]
#[derive(Identifier, SqlType)]
#[allow(non_camel_case_types)]
pub struct SQL_MULTISET;

// TODO: Should this be SqlTypeIdentifier?
/// Datetime verbose type identifier.
pub const SQL_DATETIME: SQLSMALLINT = 9;

// TODO: Should this be SqlTypeIdentifier?
/// Interval verbose type identifier.
pub const SQL_INTERVAL: SQLSMALLINT = 10;
