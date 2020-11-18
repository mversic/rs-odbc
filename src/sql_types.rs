pub use crate::SQLSMALLINT;

/// Each DBMS defines its own SQL types. Each ODBC driver exposes only those SQL data
/// types that the associated DBMS defines.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/sql-data-types
#[repr(transparent)]
pub struct SqlTypeIdentifier(SQLSMALLINT);

impl SqlTypeIdentifier {
    pub const fn raw_value(&self) -> SQLSMALLINT {
        self.0
    }
}

const SQL_UNKNOWN_TYPE: SqlTypeIdentifier = SqlTypeIdentifier(0);

/// Character string of fixed string length n.
///
/// SQL data type: CHAR(n)
pub const SQL_CHAR: SqlTypeIdentifier = SqlTypeIdentifier(1);

/// Variable-length character string with a maximum string length n.
///
/// SQL data type: VARCHAR(n)
pub const SQL_VARCHAR: SqlTypeIdentifier = SqlTypeIdentifier(12);

/// Variable length character data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARCHAR
pub const SQL_LONGVARCHAR: SqlTypeIdentifier = SqlTypeIdentifier(-1);

/// Unicode character string of fixed string length n
///
/// SQL data type: WCHAR(n)
pub const SQL_WCHAR: SqlTypeIdentifier = SqlTypeIdentifier(-8);

/// Unicode variable-length character string with a maximum string length n
///
/// SQL data type: VARWCHAR(n)
pub const SQL_WVARCHAR: SqlTypeIdentifier = SqlTypeIdentifier(-9);

/// Unicode variable-length character data. Maximum length is data source-dependent
///
/// SQL data type: LONGWVARCHAR
pub const SQL_WLONGVARCHAR: SqlTypeIdentifier = SqlTypeIdentifier(-10);

/// Signed, exact, numeric value with a precision of at least p and scale s. (The maximum precision is driver-defined.) (1 <= p <= 15; s <= p).
///
/// SQL data type: DECIMAL(p,s)
pub const SQL_DECIMAL: SqlTypeIdentifier = SqlTypeIdentifier(3);

/// Signed, exact, numeric value with a precision p and scale s (1 <= p <= 15; s <= p).
///
/// SQL data type: NUMERIC(p,s)
pub const SQL_NUMERIC: SqlTypeIdentifier = SqlTypeIdentifier(2);

/// Exact numeric value with precision 5 and scale 0  (signed:  -32,768 <= n <= 32,767, unsigned:  0 <= n <= 65,535).
///
/// SQL data type: SMALLINT
pub const SQL_SMALLINT: SqlTypeIdentifier = SqlTypeIdentifier(5);

/// Exact numeric value with precision 10 and scale 0  (signed:  -2[31] <= n <= 2[31] - 1, unsigned:  0 <= n <= 2[32] - 1).
///
/// SQL data type: INTEGER
pub const SQL_INTEGER: SqlTypeIdentifier = SqlTypeIdentifier(4);

/// Signed, approximate, numeric value with a binary precision 24 (zero or absolute value 10[-38] to 10[38]).
///
/// SQL data type: REAL
pub const SQL_REAL: SqlTypeIdentifier = SqlTypeIdentifier(7);

/// Signed, approximate, numeric value with a binary precision of at least p. (The maximum precision is driver-defined.)
///
/// SQL data type: FLOAT(p)
pub const SQL_FLOAT: SqlTypeIdentifier = SqlTypeIdentifier(6);

/// Signed, approximate, numeric value with a binary precision 53 (zero or absolute value 10[-308] to 10[308]).
///
/// SQL data type: DOUBLE PRECISION
pub const SQL_DOUBLE: SqlTypeIdentifier = SqlTypeIdentifier(8);

/// Single bit binary data.
///
/// SQL data type: BIT
pub const SQL_BIT: SqlTypeIdentifier = SqlTypeIdentifier(-7);

/// Exact numeric value with precision 3 and scale 0  (signed:  -128 <= n <= 127,  unsigned:  0 <= n <= 255).
///
/// SQL data type: TINYINT
pub const SQL_TINYINT: SqlTypeIdentifier = SqlTypeIdentifier(-6);

/// Exact numeric value with precision 19 (if signed) or 20 (if unsigned) and scale 0  (signed:  -2[63] <= n <= 2[63] - 1,  unsigned: 0 <= n <= 2[64] - 1),.
///
/// SQL data type: BIGINT
pub const SQL_BIGINT: SqlTypeIdentifier = SqlTypeIdentifier(-5);

/// Binary data of fixed length n.
///
/// SQL data type: BINARY(n)
pub const SQL_BINARY: SqlTypeIdentifier = SqlTypeIdentifier(-2);

/// Variable length binary data of maximum length n. The maximum is set by the user.
///
/// SQL data type: VARBINARY(n)
pub const SQL_VARBINARY: SqlTypeIdentifier = SqlTypeIdentifier(-3);

/// Variable length binary data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARBINARY
pub const SQL_LONGVARBINARY: SqlTypeIdentifier = SqlTypeIdentifier(-4);

/// Year, month, and day fields, conforming to the rules of the Gregorian calendar. (See Constraints of the Gregorian Calendar, later in this appendix.)
///
/// SQL data type: DATE
pub const SQL_TYPE_DATE: SqlTypeIdentifier = SqlTypeIdentifier(91);

/// Hour, minute, and second fields, with valid values for hours of 00 to 23, valid values for minutes of 00 to 59, and valid values for seconds of 00 to 61. Precision p indicates the seconds precision.
///
/// SQL data type: TIME(p)
pub const SQL_TYPE_TIME: SqlTypeIdentifier = SqlTypeIdentifier(92);

/// Year, month, day, hour, minute, and second fields, with valid values as defined for the DATE and TIME data types.
///
/// SQL data type: TIMESTAMP(p)
pub const SQL_TYPE_TIMESTAMP: SqlTypeIdentifier = SqlTypeIdentifier(93);

// TODO: These are not found in the reference implementation
//    /// Year, month, day, hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision.
//    ///
//    /// SQL data type: UTCDATETIME
//    pub const SQL_TYPE_UTCDATETIME: SqlTypeIdentifier = SqlTypeIdentifier(x);
//
//    /// Hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision..
//    ///
//    /// SQL data type: UTCTIME
//    pub const SQL_TYPE_UTCTIME: SqlTypeIdentifier = SqlTypeIdentifier(x);

/// Number of months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL MONTH(p)
pub const SQL_INTERVAL_MONTH: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_MONTH.raw_value());

/// Number of years between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p)
pub const SQL_INTERVAL_YEAR: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_YEAR.raw_value());

/// Number of years and months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p) TO MONTH
pub const SQL_INTERVAL_YEAR_TO_MONTH: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_YEAR_TO_MONTH.raw_value());

/// Number of days between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p)
pub const SQL_INTERVAL_DAY: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_DAY.raw_value());

/// Number of hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p)
pub const SQL_INTERVAL_HOUR: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_HOUR.raw_value());

/// Number of minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL MINUTE(p)
pub const SQL_INTERVAL_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_MINUTE.raw_value());

/// Number of seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL SECOND(p,q)
pub const SQL_INTERVAL_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_SECOND.raw_value());

/// Number of days/hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO HOUR
pub const SQL_INTERVAL_DAY_TO_HOUR: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_DAY_TO_HOUR.raw_value());

/// Number of days/hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO MINUTE
pub const SQL_INTERVAL_DAY_TO_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_DAY_TO_MINUTE.raw_value());

/// Number of days/hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL DAY(p) TO SECOND(q)
pub const SQL_INTERVAL_DAY_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_DAY_TO_SECOND.raw_value());

/// Number of hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p) TO MINUTE
pub const SQL_INTERVAL_HOUR_TO_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_HOUR_TO_MINUTE.raw_value());

/// Number of hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL HOUR(p) TO SECOND(q)
pub const SQL_INTERVAL_HOUR_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_HOUR_TO_SECOND.raw_value());

/// Number of minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL MINUTE(p) TO SECOND(q)
pub const SQL_INTERVAL_MINUTE_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(100 + SQL_CODE_MINUTE_TO_SECOND.raw_value());

/// Fixed length GUID.
///
/// SQL data type: GUID
#[cfg(feature = "odbc_version_3_5")]
pub const SQL_GUID: SqlTypeIdentifier = SqlTypeIdentifier(-11);

#[cfg(feature = "odbc_version_4")]
pub use SQL_UNKNOWN_TYPE as SQL_VARIANT_TYPE;

#[cfg(feature = "odbc_version_4")]
pub const SQL_UDT: SqlTypeIdentifier = SqlTypeIdentifier(17);

#[cfg(feature = "odbc_version_4")]
pub const SQL_ROW: SqlTypeIdentifier = SqlTypeIdentifier(19);

#[cfg(feature = "odbc_version_4")]
pub const SQL_ARRAY: SqlTypeIdentifier = SqlTypeIdentifier(50);

#[cfg(feature = "odbc_version_4")]
pub const SQL_MULTISET: SqlTypeIdentifier = SqlTypeIdentifier(55);

#[cfg(feature = "odbc_version_4")]
pub const SQL_TYPE_TIME_WITH_TIMEZONE: SqlTypeIdentifier = SqlTypeIdentifier(94);

#[cfg(feature = "odbc_version_4")]
pub const SQL_TYPE_TIMESTAMP_WITH_TIMEZONE: SqlTypeIdentifier = SqlTypeIdentifier(95);

// TODO: These are verbose types, not exact types
/// SQL_DESC_CONCISE_TYPE cannot use these values, neither can DATA_TYPE in SQLGetTypeInfo
pub const SQL_DATETIME: SQLSMALLINT = 9;
pub const SQL_INTERVAL: SQLSMALLINT = 10;

// Subcodes for the specific verbose datetime data type
const SQL_CODE_DATE: SqlTypeIdentifier = SqlTypeIdentifier(1);
const SQL_CODE_TIME: SqlTypeIdentifier = SqlTypeIdentifier(2);
const SQL_CODE_TIMESTAMP: SqlTypeIdentifier = SqlTypeIdentifier(3);

// Subcode for the specific verbose interval data type
const SQL_CODE_YEAR: SqlTypeIdentifier = SqlTypeIdentifier(1);
const SQL_CODE_MONTH: SqlTypeIdentifier = SqlTypeIdentifier(2);
const SQL_CODE_DAY: SqlTypeIdentifier = SqlTypeIdentifier(3);
const SQL_CODE_HOUR: SqlTypeIdentifier = SqlTypeIdentifier(4);
const SQL_CODE_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(5);
const SQL_CODE_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(6);
const SQL_CODE_YEAR_TO_MONTH: SqlTypeIdentifier = SqlTypeIdentifier(7);
const SQL_CODE_DAY_TO_HOUR: SqlTypeIdentifier = SqlTypeIdentifier(8);
const SQL_CODE_DAY_TO_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(9);
const SQL_CODE_DAY_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(10);
const SQL_CODE_HOUR_TO_MINUTE: SqlTypeIdentifier = SqlTypeIdentifier(11);
const SQL_CODE_HOUR_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(12);
const SQL_CODE_MINUTE_TO_SECOND: SqlTypeIdentifier = SqlTypeIdentifier(13);
