use std::cell::UnsafeCell;
use crate::sql_types::*;

// TODO: Add support for mingw-x64 on x86 platform
//use std::convert::TryFrom;
//use ucs2;

/// UCS-2 encoded character
pub type SQLWCHAR = u16;
//impl TryFrom<&str> for &mut [SQLWCHAR] {
//    type Error = u16;
//
//    fn try_from(source: &str) -> Result<Self, Self::Error> {
//        unimplemented!()
//    }
//}

pub type SQLSMALLINT = i16;
pub type SQLUSMALLINT = u16;

pub type SQLINTEGER = i32;
pub type SQLUINTEGER = u32;

pub type SQLREAL = f32;
pub type SQLDOUBLE = f64;
pub type SQLFLOAT = f64;

/// ASCII encoded character
pub type SQLCHAR = u8;
pub type SQLSCHAR = i8;

pub type SQLBIGINT = i64;
pub type SQLUBIGINT = u64;

pub type SQLLEN = isize;
pub type SQLULEN = usize;

pub type RETCODE = i16;

#[cfg(target_pointer_width = "32")]
pub type SQLSETPOSIROW = SQLUSMALLINT;
#[cfg(target_pointer_width = "64")]
pub type SQLSETPOSIROW = u64;

pub trait AsSqlCharStr {
    fn as_sql_str(&self) -> &[UnsafeCell<SQLCHAR>];
}
pub trait AsMutSqlCharStr {
    fn as_mut_sql_str(&mut self) -> &mut[UnsafeCell<SQLCHAR>];
}
pub trait AsSqlWcharStr {
    fn as_sql_str(&self) -> &[UnsafeCell<SQLWCHAR>];
}
pub trait AsMutSqlWcharStr {
    fn as_mut_sql_str(&mut self) -> &mut[UnsafeCell<SQLWCHAR>];
}

pub struct SqlStateA([SQLCHAR; 6]);
pub struct SqlStateW([SQLWCHAR; 6]);

//impl AsAsciiStr for std::ffi::CStr {
//    fn as_ascii_str(&self) -> Result<&[UnsafeCell<SQLCHAR>], Error> {
//        self.to_bytes()
//    }
//}
//impl AsMutAsciiStr for std::ffi::CStr {
//    fn as_mut_ascii_str(&mut self) -> Result<&mut [UnsafeCell<SQLCHAR>], Error> {
//        self.to_bytes()
//    }
//}

/// ODBC C data types indicate the data type of C buffers used to store data in the
/// application.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/c-data-types
#[repr(transparent)]
pub struct CTypeIdentifier(SQLSMALLINT);

impl CTypeIdentifier {
    pub const fn raw_value(&self) -> SQLSMALLINT {
        self.0
    }

    /// Used to construct a driver-specific C type identifier
    #[cfg(feature = "v3_8")]
    pub fn driver_specific(source: SQLSMALLINT) -> Self {
        CTypeIdentifier(source)
    }
}

// This value is discouraged from being used
#[allow(dead_code)]
const SQL_C_DEFAULT: CTypeIdentifier = CTypeIdentifier(99);

const SQL_SIGNED_OFFSET: CTypeIdentifier = CTypeIdentifier(-20);

const SQL_UNSIGNED_OFFSET: CTypeIdentifier = CTypeIdentifier(-22);

pub const SQL_C_CHAR: CTypeIdentifier = CTypeIdentifier(SQL_CHAR.raw_value());

pub const SQL_C_WCHAR: CTypeIdentifier = CTypeIdentifier(SQL_WCHAR.raw_value());

/// Replaced by SQL_C_SSHORT and SQL_C_USHORT
#[deprecated]
const SQL_C_SHORT: CTypeIdentifier = CTypeIdentifier(SQL_SMALLINT.raw_value());

#[allow(deprecated)]
pub const SQL_C_SSHORT: CTypeIdentifier =
    CTypeIdentifier(SQL_C_SHORT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

#[allow(deprecated)]
pub const SQL_C_USHORT: CTypeIdentifier =
    CTypeIdentifier(SQL_C_SHORT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

/// Replaced by SQL_C_SLONG and SQL_C_ULONG
#[deprecated]
const SQL_C_LONG: CTypeIdentifier = CTypeIdentifier(SQL_INTEGER.raw_value());

#[allow(deprecated)]
pub const SQL_C_SLONG: CTypeIdentifier =
    CTypeIdentifier(SQL_C_LONG.raw_value() + SQL_SIGNED_OFFSET.raw_value());

#[allow(deprecated)]
pub const SQL_C_ULONG: CTypeIdentifier =
    CTypeIdentifier(SQL_C_LONG.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_FLOAT: CTypeIdentifier = CTypeIdentifier(SQL_REAL.raw_value());

pub const SQL_C_DOUBLE: CTypeIdentifier = CTypeIdentifier(SQL_DOUBLE.raw_value());

pub const SQL_C_BIT: CTypeIdentifier = CTypeIdentifier(SQL_BIT.raw_value());

#[deprecated]
#[allow(dead_code)]
const SQL_C_TINYINT: CTypeIdentifier = CTypeIdentifier(SQL_TINYINT.raw_value());

pub const SQL_C_STINYINT: CTypeIdentifier =
    CTypeIdentifier(SQL_TINYINT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_UTINYINT: CTypeIdentifier =
    CTypeIdentifier(SQL_TINYINT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_SBIGINT: CTypeIdentifier =
    CTypeIdentifier(SQL_BIGINT.raw_value() + SQL_SIGNED_OFFSET.raw_value());

pub const SQL_C_UBIGINT: CTypeIdentifier =
    CTypeIdentifier(SQL_BIGINT.raw_value() + SQL_UNSIGNED_OFFSET.raw_value());

pub const SQL_C_BINARY: CTypeIdentifier = CTypeIdentifier(SQL_BINARY.raw_value());

pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

pub const SQL_C_NUMERIC: CTypeIdentifier = CTypeIdentifier(SQL_NUMERIC.raw_value());

#[cfg(feature = "v3_5")]
pub const SQL_C_GUID: CTypeIdentifier = CTypeIdentifier(SQL_GUID.raw_value());

pub const SQL_C_TYPE_DATE: CTypeIdentifier = CTypeIdentifier(SQL_TYPE_DATE.raw_value());

pub const SQL_C_TYPE_TIME: CTypeIdentifier = CTypeIdentifier(SQL_TYPE_TIME.raw_value());

pub const SQL_C_TYPE_TIMESTAMP: CTypeIdentifier = CTypeIdentifier(SQL_TYPE_TIMESTAMP.raw_value());

#[cfg(feature = "v4")]
pub const SQL_C_TYPE_TIME_WITH_TIMEZONE: CTypeIdentifier =
    CTypeIdentifier(SQL_TYPE_TIME_WITH_TIMEZONE.raw_value());

#[cfg(feature = "v4")]
pub const SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE: CTypeIdentifier =
    CTypeIdentifier(SQL_TYPE_TIMESTAMP_WITH_TIMEZONE.raw_value());

pub const SQL_C_INTERVAL_YEAR: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_YEAR.raw_value());
pub const SQL_C_INTERVAL_MONTH: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_MONTH.raw_value());
pub const SQL_C_INTERVAL_DAY: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_DAY.raw_value());
pub const SQL_C_INTERVAL_HOUR: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_HOUR.raw_value());
pub const SQL_C_INTERVAL_MINUTE: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_MINUTE.raw_value());
pub const SQL_C_INTERVAL_SECOND: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_SECOND.raw_value());
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_YEAR_TO_MONTH.raw_value());
pub const SQL_C_INTERVAL_DAY_TO_HOUR: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_DAY_TO_HOUR.raw_value());
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_DAY_TO_MINUTE.raw_value());
pub const SQL_C_INTERVAL_DAY_TO_SECOND: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_DAY_TO_SECOND.raw_value());
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_HOUR_TO_MINUTE.raw_value());
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_HOUR_TO_SECOND.raw_value());
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: CTypeIdentifier =
    CTypeIdentifier(SQL_INTERVAL_MINUTE_TO_SECOND.raw_value());

// =================================================================================== //

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SQL_NUMERIC_STRUCT {
    pub precision: SQLCHAR,
    pub scale: SQLSCHAR,
    /// The sign field is 1 if positive, 0 if negative.
    pub sign: SQLCHAR,
    pub val: [SQLCHAR; SQL_MAX_NUMERIC_LEN],
}

pub const SQL_MAX_NUMERIC_LEN: usize = 16;

#[repr(C)]
#[cfg(feature = "v3_5")]
#[allow(non_camel_case_types, non_snake_case)]
pub struct SQLGUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SQL_DATE_STRUCT {
    year: SQLSMALLINT,
    month: SQLUSMALLINT,
    day: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SQL_TIME_STRUCT {
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SQL_TIMESTAMP_STRUCT {
    year: SQLSMALLINT,
    month: SQLUSMALLINT,
    day: SQLUSMALLINT,
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    fraction: SQLUINTEGER,
}

#[repr(C)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_TIME_WITH_TIMEZONE_STRUCT {
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
    timezone_hours: SQLSMALLINT,
    timezone_minutes: SQLUSMALLINT,
}

#[repr(C)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {
    year: SQLSMALLINT,
    month: SQLUSMALLINT,
    day: SQLUSMALLINT,
    hour: SQLUSMALLINT,
    minute: SQLUSMALLINT,
    second: SQLUSMALLINT,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    fraction: SQLUINTEGER,
    timezone_hours: SQLSMALLINT,
    timezone_minutes: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SQL_INTERVAL_STRUCT {
    interval_type: SQLINTERVAL,
    interval_sign: SQLSMALLINT,
    interval: IntervalUnion,
}

// TODO: Probably shouldn't use an enum with C FFI
#[repr(C)]
enum SQLINTERVAL {
    SQL_IS_YEAR = 1,
    SQL_IS_MONTH = 2,
    SQL_IS_DAY = 3,
    SQL_IS_HOUR = 4,
    SQL_IS_MINUTE = 5,
    SQL_IS_SECOND = 6,
    SQL_IS_YEAR_TO_MONTH = 7,
    SQL_IS_DAY_TO_HOUR = 8,
    SQL_IS_DAY_TO_MINUTE = 9,
    SQL_IS_DAY_TO_SECOND = 10,
    SQL_IS_HOUR_TO_MINUTE = 11,
    SQL_IS_HOUR_TO_SECOND = 12,
    SQL_IS_MINUTE_TO_SECOND = 13,
}

#[repr(C)]
// TODO: Should this be public?
union IntervalUnion {
    year_month: SQL_YEAR_MONTH_STRUCT,
    day_second: SQL_DAY_SECOND_STRUCT,
}

// TODO: Must be copy because it's used in uinon
// Maybe it's would bt ok in nightly
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
struct SQL_YEAR_MONTH_STRUCT {
    year: SQLUINTEGER,
    month: SQLUINTEGER,
}

// TODO: Must be copy because it's used in uinon
// Maybe it's would bt ok in nightly
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
struct SQL_DAY_SECOND_STRUCT {
    day: SQLUINTEGER,
    month: SQLUINTEGER,
    minute: SQLUINTEGER,
    second: SQLUINTEGER,
    fraction: SQLUINTEGER,
}
