use crate::{SQLSMALLINT, SQLCHAR, SQLUINTEGER, SQLUSMALLINT, SQLSCHAR};
use rs_odbc_derive::{Identifier as Ident, CType};
use crate::sql_types::*;
use crate::Identifier;

pub trait CType: Identifier<IdentType = SQLSMALLINT> {}

const SQL_UNSIGNED_OFFSET: SQLSMALLINT = -22;
const SQL_SIGNED_OFFSET: SQLSMALLINT = -20;
const SQL_C_SHORT: SQLSMALLINT = SQL_SMALLINT::IDENTIFIER;
const SQL_C_LONG: SQLSMALLINT = SQL_INTEGER::IDENTIFIER;
const SQL_C_TINYINT: SQLSMALLINT = SQL_TINYINT::IDENTIFIER;

// TODO: This value is discouraged from being used
#[derive(Ident, CType)]
#[identifier(SQLSMALLINT, 99)]
#[allow(non_camel_case_types)]
struct SQL_C_DEFAULT;

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_CHAR;
impl  Identifier for SQL_C_CHAR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_CHAR::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_WCHAR;
impl Identifier for SQL_C_WCHAR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_WCHAR::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SSHORT;
impl Identifier for SQL_C_SSHORT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_SHORT + SQL_SIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_USHORT;
impl Identifier for SQL_C_USHORT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_SHORT + SQL_UNSIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SLONG;
impl Identifier for SQL_C_SLONG {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_LONG + SQL_SIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_ULONG;
impl Identifier for SQL_C_ULONG {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_LONG + SQL_UNSIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_FLOAT;
impl Identifier for SQL_C_FLOAT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_REAL::IDENTIFIER;

}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_DOUBLE;
impl Identifier for SQL_C_DOUBLE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_DOUBLE::IDENTIFIER;
}
#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_BIT;
impl Identifier for SQL_C_BIT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_BIT::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_STINYINT;
impl Identifier for SQL_C_STINYINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_TINYINT + SQL_SIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_UTINYINT;
impl Identifier for SQL_C_UTINYINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_C_TINYINT + SQL_UNSIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SBIGINT;
impl Identifier for SQL_C_SBIGINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_BIGINT::IDENTIFIER + SQL_SIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_UBIGINT;
impl Identifier for SQL_C_UBIGINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_BIGINT::IDENTIFIER + SQL_UNSIGNED_OFFSET;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_BINARY;
impl Identifier for SQL_C_BINARY {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_BINARY::IDENTIFIER;
}

// TODO: Weird?
pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_NUMERIC;
impl Identifier for SQL_C_NUMERIC {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_NUMERIC::IDENTIFIER;
}

#[derive(CType)]
#[cfg(feature = "v3_5")]
#[allow(non_camel_case_types)]
pub struct SQL_C_GUID;
impl Identifier for SQL_C_GUID {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_GUID::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_DATE;
impl Identifier for SQL_C_TYPE_DATE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_DATE::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME;
impl Identifier for SQL_C_TYPE_TIME {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIME::IDENTIFIER;
}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP;
impl Identifier for SQL_C_TYPE_TIMESTAMP {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIMESTAMP::IDENTIFIER;
}

#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Identifier for SQL_C_TYPE_TIME_WITH_TIMEZONE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIME_WITH_TIMEZONE::IDENTIFIER;
}

#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Identifier for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIMESTAMP_WITH_TIMEZONE;
}

//pub const SQL_C_INTERVAL_YEAR: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_YEAR.raw_value());
//pub const SQL_C_INTERVAL_MONTH: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_MONTH.raw_value());
//pub const SQL_C_INTERVAL_DAY: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_DAY.raw_value());
//pub const SQL_C_INTERVAL_HOUR: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_HOUR.raw_value());
//pub const SQL_C_INTERVAL_MINUTE: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_MINUTE.raw_value());
//pub const SQL_C_INTERVAL_SECOND: CTypeIdentifier = CTypeIdentifier(SQL_INTERVAL_SECOND.raw_value());
//pub const SQL_C_INTERVAL_YEAR_TO_MONTH: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_YEAR_TO_MONTH.raw_value());
//pub const SQL_C_INTERVAL_DAY_TO_HOUR: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_DAY_TO_HOUR.raw_value());
//pub const SQL_C_INTERVAL_DAY_TO_MINUTE: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_DAY_TO_MINUTE.raw_value());
//pub const SQL_C_INTERVAL_DAY_TO_SECOND: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_DAY_TO_SECOND.raw_value());
//pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_HOUR_TO_MINUTE.raw_value());
//pub const SQL_C_INTERVAL_HOUR_TO_SECOND: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_HOUR_TO_SECOND.raw_value());
//pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: CTypeIdentifier =
//    CTypeIdentifier(SQL_INTERVAL_MINUTE_TO_SECOND.raw_value());

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

// TODO: Why is this required?
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
// Maybe it would bt ok in nightly
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
