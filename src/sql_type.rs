use co3::ReprC;
use rust_spec::RustSpec;

use crate::env::{OV_ODBC3_80, OV_ODBC4, OdbcVersion};

/// A SQL data type identifier returned by an ODBC function.
///
/// Unlike the zero-sized SQL type markers, this stores a type selected at
/// runtime by the driver. The value is intentionally open-ended because
/// drivers may return implementation-defined type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct SqlTypeCode(i16);

impl SqlTypeCode {
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> i16 {
        self.0
    }
}

/// A SQL data type accepted by ODBC version `V`.
///
/// This trait is open so a driver crate can implement it for a local marker type and the
/// versions for which its extended SQL type is available. APIs using the marker also require
/// it to implement [`co3::tag::Tagged`], which supplies the driver-specific tag.
pub trait SqlType<V: OdbcVersion> {}

macro_rules! sql_types {
    (
        v3 { $($(#[$v3_attribute:meta])* $v3_name:ident = $v3_id:expr;)+ }
        v3_8 { $($(#[$v3_8_attribute:meta])* $v3_8_name:ident = $v3_8_id:expr;)+ }
        v4 { $($(#[$v4_attribute:meta])* $v4_name:ident = $v4_id:expr;)+ }
    ) => {
        $(sql_types!(@type $(#[$v3_attribute])* $v3_name = $v3_id;
            crate::env::OV_ODBC3, OV_ODBC3_80, OV_ODBC4);)+
        $(sql_types!(@type $(#[$v3_8_attribute])* $v3_8_name = $v3_8_id;
            OV_ODBC3_80, OV_ODBC4);)+
        $(sql_types!(@type $(#[$v4_attribute])* $v4_name = $v4_id; OV_ODBC4);)+
    };
    (@type $(#[$attribute:meta])* $name:ident = $id:expr; $($version:ty),+ $(,)?) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, co3::Tag)]
        #[tag(i16, unsafe($id))]
        #[allow(non_camel_case_types)]
        pub enum $name {}

        $(impl SqlType<$version> for $name {})+
    };
}

sql_types! {
    v3 {
        /// Data type cannot be determined.
        UNKNOWN_TYPE = 0;
        /// Character string of fixed length `n`.
        CHAR = 1;
        /// Variable-length character string with a maximum length `n`.
        VARCHAR = 12;
        /// Variable-length character data with a data-source-dependent maximum length.
        LONGVARCHAR = -1;
        /// Unicode character string of fixed length `n`.
        WCHAR = -8;
        /// Unicode variable-length character string with a maximum length `n`.
        WVARCHAR = -9;
        /// Unicode variable-length character data with a data-source-dependent maximum length.
        WLONGVARCHAR = -10;
        /// Exact numeric value with driver-dependent precision and scale.
        DECIMAL = 3;
        /// Exact numeric value with precision and scale.
        NUMERIC = 2;
        /// Small integer value.
        SMALLINT = 5;
        /// Integer value.
        INTEGER = 4;
        /// Approximate numeric value with binary precision 24.
        REAL = 7;
        /// Approximate numeric value with driver-dependent binary precision.
        FLOAT = 6;
        /// Double-precision approximate numeric value.
        DOUBLE = 8;
        /// Single-bit binary data.
        BIT = -7;
        /// Tiny integer value.
        TINYINT = -6;
        /// Big integer value.
        BIGINT = -5;
        /// Fixed-length binary data.
        BINARY = -2;
        /// Variable-length binary data with a caller-selected maximum length.
        VARBINARY = -3;
        /// Variable-length binary data with a data-source-dependent maximum length.
        LONGVARBINARY = -4;
        /// Gregorian calendar date.
        TYPE_DATE = 91;
        /// Time of day.
        TYPE_TIME = 92;
        /// Date and time.
        TYPE_TIMESTAMP = 93;
        /// Interval measured in months.
        INTERVAL_MONTH = 100 + SQL_CODE_MONTH as i16;
        /// Interval measured in years.
        INTERVAL_YEAR = 100 + SQL_CODE_YEAR as i16;
        /// Year-to-month interval.
        INTERVAL_YEAR_TO_MONTH = 100 + SQL_CODE_YEAR_TO_MONTH as i16;
        /// Interval measured in days.
        INTERVAL_DAY = 100 + SQL_CODE_DAY as i16;
        /// Interval measured in hours.
        INTERVAL_HOUR = 100 + SQL_CODE_HOUR as i16;
        /// Interval measured in minutes.
        INTERVAL_MINUTE = 100 + SQL_CODE_MINUTE as i16;
        /// Interval measured in seconds.
        INTERVAL_SECOND = 100 + SQL_CODE_SECOND as i16;
        /// Day-to-hour interval.
        INTERVAL_DAY_TO_HOUR = 100 + SQL_CODE_DAY_TO_HOUR as i16;
        /// Day-to-minute interval.
        INTERVAL_DAY_TO_MINUTE = 100 + SQL_CODE_DAY_TO_MINUTE as i16;
        /// Day-to-second interval.
        INTERVAL_DAY_TO_SECOND = 100 + SQL_CODE_DAY_TO_SECOND as i16;
        /// Hour-to-minute interval.
        INTERVAL_HOUR_TO_MINUTE = 100 + SQL_CODE_HOUR_TO_MINUTE as i16;
        /// Hour-to-second interval.
        INTERVAL_HOUR_TO_SECOND = 100 + SQL_CODE_HOUR_TO_SECOND as i16;
        /// Minute-to-second interval.
        INTERVAL_MINUTE_TO_SECOND = 100 + SQL_CODE_MINUTE_TO_SECOND as i16;
        /// Datetime verbose type identifier.
        DATETIME = 9;
        /// Interval verbose type identifier.
        INTERVAL = 10;
    }

    v3_8 {
        // TODO: This is ODBC 3.5.
        /// Fixed-length GUID.
        GUID = -11;
    }

    v4 {
        /// Column whose type may vary across rows.
        VARIANT = 0;
        TYPE_TIME_WITH_TIMEZONE = 94;
        TYPE_TIMESTAMP_WITH_TIMEZONE = 95;
        UDT = 17;
        ROW = 19;
        ARRAY = 50;
        MULTISET = 55;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum DatetimeIntervalCode {
    DATE_OR_YEAR = 1,
    TIME_OR_MONTH = 2,
    TIMESTAMP_OR_DAY = 3,
    HOUR = 4,
    MINUTE = 5,
    SECOND = 6,
    YEAR_TO_MONTH = 7,
    DAY_TO_HOUR = 8,
    DAY_TO_MINUTE = 9,
    DAY_TO_SECOND = 10,
    HOUR_TO_MINUTE = 11,
    HOUR_TO_SECOND = 12,
    MINUTE_TO_SECOND = 13,
}

// Subcodes for the specific verbose datetime data type
//const SQL_CODE_DATE: DatetimeIntervalCode = DatetimeIntervalCode::DATE_OR_YEAR;
//const SQL_CODE_TIME: DatetimeIntervalCode = DatetimeIntervalCode::TIME_OR_MONTH;
//const SQL_CODE_TIMESTAMP: DatetimeIntervalCode = DatetimeIntervalCode::TIMESTAMP_OR_DAY;

// Subcode for the specific verbose interval data type
const SQL_CODE_YEAR: DatetimeIntervalCode = DatetimeIntervalCode::DATE_OR_YEAR;
const SQL_CODE_MONTH: DatetimeIntervalCode = DatetimeIntervalCode::TIME_OR_MONTH;
const SQL_CODE_DAY: DatetimeIntervalCode = DatetimeIntervalCode::TIMESTAMP_OR_DAY;
const SQL_CODE_HOUR: DatetimeIntervalCode = DatetimeIntervalCode::HOUR;
const SQL_CODE_MINUTE: DatetimeIntervalCode = DatetimeIntervalCode::MINUTE;
const SQL_CODE_SECOND: DatetimeIntervalCode = DatetimeIntervalCode::SECOND;
const SQL_CODE_YEAR_TO_MONTH: DatetimeIntervalCode = DatetimeIntervalCode::YEAR_TO_MONTH;
const SQL_CODE_DAY_TO_HOUR: DatetimeIntervalCode = DatetimeIntervalCode::DAY_TO_HOUR;
const SQL_CODE_DAY_TO_MINUTE: DatetimeIntervalCode = DatetimeIntervalCode::DAY_TO_MINUTE;
const SQL_CODE_DAY_TO_SECOND: DatetimeIntervalCode = DatetimeIntervalCode::DAY_TO_SECOND;
const SQL_CODE_HOUR_TO_MINUTE: DatetimeIntervalCode = DatetimeIntervalCode::HOUR_TO_MINUTE;
const SQL_CODE_HOUR_TO_SECOND: DatetimeIntervalCode = DatetimeIntervalCode::HOUR_TO_SECOND;
const SQL_CODE_MINUTE_TO_SECOND: DatetimeIntervalCode = DatetimeIntervalCode::MINUTE_TO_SECOND;
