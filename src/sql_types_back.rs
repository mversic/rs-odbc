pub use crate::SQLSMALLINT;

/// Each DBMS defines its own SQL types. Each ODBC driver exposes only those SQL data
/// types that the associated DBMS defines.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/sql-data-types
#[repr(transparent)]
pub struct SQL_DESC_TYPE(SQLSMALLINT);

impl SQL_DESC_TYPE {
    pub const fn raw_value(&self) -> SQLSMALLINT {
        self.0
    }
}

/// Data type cannot be determined
pub const SQL_UNKNOWN_TYPE: SQL_DESC_TYPE = SQL_DESC_TYPE(0);

/// Column whose type may vary across rows
#[cfg(feature = "v4")]
pub use SQL_UNKNOWN_TYPE as SQL_VARIANT_TYPE;

/// Character string of fixed string length n.
///
/// SQL data type: CHAR(n)
pub const SQL_CHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(1);

/// Variable-length character string with a maximum string length n.
///
/// SQL data type: VARCHAR(n)
pub const SQL_VARCHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(12);

/// Variable length character data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARCHAR
pub const SQL_LONGVARCHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(-1);

/// Unicode character string of fixed string length n
///
/// SQL data type: WCHAR(n)
pub const SQL_WCHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(-8);

/// Unicode variable-length character string with a maximum string length n
///
/// SQL data type: VARWCHAR(n)
pub const SQL_WVARCHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(-9);

/// Unicode variable-length character data. Maximum length is data source-dependent
///
/// SQL data type: LONGWVARCHAR
pub const SQL_WLONGVARCHAR: SQL_DESC_TYPE = SQL_DESC_TYPE(-10);

/// Signed, exact, numeric value with a precision of at least p and scale s. (The maximum precision is driver-defined.) (1 <= p <= 15; s <= p).
///
/// SQL data type: DECIMAL(p,s)
pub const SQL_DECIMAL: SQL_DESC_TYPE = SQL_DESC_TYPE(3);

/// Signed, exact, numeric value with a precision p and scale s (1 <= p <= 15; s <= p).
///
/// SQL data type: NUMERIC(p,s)
pub const SQL_NUMERIC: SQL_DESC_TYPE = SQL_DESC_TYPE(2);

/// Exact numeric value with precision 5 and scale 0  (signed:  -32,768 <= n <= 32,767, unsigned:  0 <= n <= 65,535).
///
/// SQL data type: SMALLINT
pub const SQL_SMALLINT: SQL_DESC_TYPE = SQL_DESC_TYPE(5);

/// Exact numeric value with precision 10 and scale 0  (signed:  -2[31] <= n <= 2[31] - 1, unsigned:  0 <= n <= 2[32] - 1).
///
/// SQL data type: INTEGER
pub const SQL_INTEGER: SQL_DESC_TYPE = SQL_DESC_TYPE(4);

/// Signed, approximate, numeric value with a binary precision 24 (zero or absolute value 10[-38] to 10[38]).
///
/// SQL data type: REAL
pub const SQL_REAL: SQL_DESC_TYPE = SQL_DESC_TYPE(7);

/// Signed, approximate, numeric value with a binary precision of at least p. (The maximum precision is driver-defined.)
///
/// SQL data type: FLOAT(p)
pub const SQL_FLOAT: SQL_DESC_TYPE = SQL_DESC_TYPE(6);

/// Signed, approximate, numeric value with a binary precision 53 (zero or absolute value 10[-308] to 10[308]).
///
/// SQL data type: DOUBLE PRECISION
pub const SQL_DOUBLE: SQL_DESC_TYPE = SQL_DESC_TYPE(8);

/// Single bit binary data.
///
/// SQL data type: BIT
pub const SQL_BIT: SQL_DESC_TYPE = SQL_DESC_TYPE(-7);

/// Exact numeric value with precision 3 and scale 0  (signed:  -128 <= n <= 127,  unsigned:  0 <= n <= 255).
///
/// SQL data type: TINYINT
pub const SQL_TINYINT: SQL_DESC_TYPE = SQL_DESC_TYPE(-6);

/// Exact numeric value with precision 19 (if signed) or 20 (if unsigned) and scale 0  (signed:  -2[63] <= n <= 2[63] - 1,  unsigned: 0 <= n <= 2[64] - 1),.
///
/// SQL data type: BIGINT
pub const SQL_BIGINT: SQL_DESC_TYPE = SQL_DESC_TYPE(-5);

/// Binary data of fixed length n.
///
/// SQL data type: BINARY(n)
pub const SQL_BINARY: SQL_DESC_TYPE = SQL_DESC_TYPE(-2);

/// Variable length binary data of maximum length n. The maximum is set by the user.
///
/// SQL data type: VARBINARY(n)
pub const SQL_VARBINARY: SQL_DESC_TYPE = SQL_DESC_TYPE(-3);

/// Variable length binary data. Maximum length is data source-dependent.
///
/// SQL data type: LONG VARBINARY
pub const SQL_LONGVARBINARY: SQL_DESC_TYPE = SQL_DESC_TYPE(-4);

/// Fixed length GUID.
///
/// SQL data type: GUID
#[cfg(feature = "v3_5")]
pub const SQL_GUID: SQL_DESC_TYPE = SQL_DESC_TYPE(-11);

/// Year, month, and day fields, conforming to the rules of the Gregorian calendar. (See Constraints of the Gregorian Calendar, later in this appendix.)
///
/// SQL data type: DATE
pub const SQL_TYPE_DATE: SQL_DESC_TYPE = SQL_DESC_TYPE(91);

/// Hour, minute, and second fields, with valid values for hours of 00 to 23, valid values for minutes of 00 to 59, and valid values for seconds of 00 to 61. Precision p indicates the seconds precision.
///
/// SQL data type: TIME(p)
pub const SQL_TYPE_TIME: SQL_DESC_TYPE = SQL_DESC_TYPE(92);

/// Year, month, day, hour, minute, and second fields, with valid values as defined for the DATE and TIME data types.
///
/// SQL data type: TIMESTAMP(p)
pub const SQL_TYPE_TIMESTAMP: SQL_DESC_TYPE = SQL_DESC_TYPE(93);

#[cfg(feature = "v4")]
pub const SQL_TYPE_TIME_WITH_TIMEZONE: SQL_DESC_TYPE = SQL_DESC_TYPE(94);

#[cfg(feature = "v4")]
pub const SQL_TYPE_TIMESTAMP_WITH_TIMEZONE: SQL_DESC_TYPE = SQL_DESC_TYPE(95);

// TODO: These are not found in the reference implementation but exist in documentation
//    /// Year, month, day, hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision.
//    ///
//    /// SQL data type: UTCDATETIME
//    pub const SQL_TYPE_UTCDATETIME: SQL_DESC_TYPE = SQL_DESC_TYPE(x);
//
//    /// Hour, minute, second, utchour, and utcminute fields. The utchour and utcminute fields have 1/10 microsecond precision..
//    ///
//    /// SQL data type: UTCTIME
//    pub const SQL_TYPE_UTCTIME: SQL_DESC_TYPE = SQL_DESC_TYPE(x);

/// Number of months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL MONTH(p)
pub const SQL_INTERVAL_MONTH: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_MONTH.raw_value());

/// Number of years between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p)
pub const SQL_INTERVAL_YEAR: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_YEAR.raw_value());

/// Number of years and months between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL YEAR(p) TO MONTH
pub const SQL_INTERVAL_YEAR_TO_MONTH: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_YEAR_TO_MONTH.raw_value());

/// Number of days between two dates; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p)
pub const SQL_INTERVAL_DAY: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_DAY.raw_value());

/// Number of hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p)
pub const SQL_INTERVAL_HOUR: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_HOUR.raw_value());

/// Number of minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL MINUTE(p)
pub const SQL_INTERVAL_MINUTE: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_MINUTE.raw_value());

/// Number of seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL SECOND(p,q)
pub const SQL_INTERVAL_SECOND: SQL_DESC_TYPE = SQL_DESC_TYPE(100 + SQL_CODE_SECOND.raw_value());

/// Number of days/hours between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO HOUR
pub const SQL_INTERVAL_DAY_TO_HOUR: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_DAY_TO_HOUR.raw_value());

/// Number of days/hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL DAY(p) TO MINUTE
pub const SQL_INTERVAL_DAY_TO_MINUTE: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_DAY_TO_MINUTE.raw_value());

/// Number of days/hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL DAY(p) TO SECOND(q)
pub const SQL_INTERVAL_DAY_TO_SECOND: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_DAY_TO_SECOND.raw_value());

/// Number of hours/minutes between two date/times; p is the interval leading precision.
///
/// SQL data type: INTERVAL HOUR(p) TO MINUTE
pub const SQL_INTERVAL_HOUR_TO_MINUTE: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_HOUR_TO_MINUTE.raw_value());

/// Number of hours/minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL HOUR(p) TO SECOND(q)
pub const SQL_INTERVAL_HOUR_TO_SECOND: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_HOUR_TO_SECOND.raw_value());

/// Number of minutes/seconds between two date/times; p is the interval leading precision and q is the interval seconds precision.
///
/// SQL data type: INTERVAL MINUTE(p) TO SECOND(q)
pub const SQL_INTERVAL_MINUTE_TO_SECOND: SQL_DESC_TYPE =
    SQL_DESC_TYPE(100 + SQL_CODE_MINUTE_TO_SECOND.raw_value());

#[cfg(feature = "v4")]
pub const SQL_UDT: SQL_DESC_TYPE = SQL_DESC_TYPE(17);

#[cfg(feature = "v4")]
pub const SQL_ROW: SQL_DESC_TYPE = SQL_DESC_TYPE(19);

#[cfg(feature = "v4")]
pub const SQL_ARRAY: SQL_DESC_TYPE = SQL_DESC_TYPE(50);

#[cfg(feature = "v4")]
pub const SQL_MULTISET: SQL_DESC_TYPE = SQL_DESC_TYPE(55);

// TODO: Should this be SQL_DESC_TYPE?
/// Datetime verbose type identifier.
pub const SQL_DATETIME: SQLSMALLINT = 9;

// TODO: Should this be SQL_DESC_TYPE?
/// Interval verbose type identifier.
pub const SQL_INTERVAL: SQLSMALLINT = 10;

// =================================================================================== //

pub struct SQL_DESC_TYPE;
// Subcodes for the specific verbose datetime data type
pub const SQL_CODE_DATE: SQL_DATETIME = SQL_DATETIME(1);
pub const SQL_CODE_TIME: SQL_DATETIME = SQL_DATETIME(2);
pub const SQL_CODE_TIMESTAMP: SQL_DATETIME = SQL_DATETIME(3);

// Subcode for the specific verbose interval data type
pub const SQL_CODE_YEAR: SQL_DESC_TYPE = SQL_DESC_TYPE(1);
pub const SQL_CODE_MONTH: SQL_DESC_TYPE = SQL_DESC_TYPE(2);
pub const SQL_CODE_DAY: SQL_DESC_TYPE = SQL_DESC_TYPE(3);
pub const SQL_CODE_HOUR: SQL_DESC_TYPE = SQL_DESC_TYPE(4);
pub const SQL_CODE_MINUTE: SQL_DESC_TYPE = SQL_DESC_TYPE(5);
pub const SQL_CODE_SECOND: SQL_DESC_TYPE = SQL_DESC_TYPE(6);
pub const SQL_CODE_YEAR_TO_MONTH: SQL_DESC_TYPE = SQL_DESC_TYPE(7);
pub const SQL_CODE_DAY_TO_HOUR: SQL_DESC_TYPE = SQL_DESC_TYPE(8);
pub const SQL_CODE_DAY_TO_MINUTE: SQL_DESC_TYPE = SQL_DESC_TYPE(9);
pub const SQL_CODE_DAY_TO_SECOND: SQL_DESC_TYPE = SQL_DESC_TYPE(10);
pub const SQL_CODE_HOUR_TO_MINUTE: SQL_DESC_TYPE = SQL_DESC_TYPE(11);
pub const SQL_CODE_HOUR_TO_SECOND: SQL_DESC_TYPE = SQL_DESC_TYPE(12);
pub const SQL_CODE_MINUTE_TO_SECOND: SQL_DESC_TYPE = SQL_DESC_TYPE(13);
