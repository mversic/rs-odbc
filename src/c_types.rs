use crate::sql_types::*;
use crate::{AsMutRawSlice, AsRawSlice, Identifier};
use crate::{
    SQLBIGINT, SQLCHAR, SQLDOUBLE, SQLINTEGER, SQLLEN, SQLREAL, SQLSCHAR, SQLSMALLINT, SQLUBIGINT,
    SQLUINTEGER, SQLUSMALLINT, SQLWCHAR, IntoSQLPOINTER, SQLPOINTER
};
use std::cell::UnsafeCell;
use rs_odbc_derive::CType;

// TODO: Make these traits unsafe
pub trait CType: Identifier<IdentType = SQLSMALLINT> {}
pub trait InCType<T>: CType {}
pub trait OutCType<T>: CType {}

const SQL_UNSIGNED_OFFSET: SQLSMALLINT = -22;
const SQL_SIGNED_OFFSET: SQLSMALLINT = -20;
const SQL_C_SHORT: SQLSMALLINT = SQL_SMALLINT.identifier();
const SQL_C_LONG: SQLSMALLINT = SQL_INTEGER.identifier();
const SQL_C_TINYINT: SQLSMALLINT = SQL_TINYINT.identifier();

// TODO: This value is discouraged from being used
//#[derive(Identifier)]
//#[identifier(SQLSMALLINT, 99)]
//#[allow(non_camel_case_types)]
//struct SQL_C_DEFAULT;

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_CHAR;
impl Identifier for SQL_C_CHAR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_CHAR.identifier();
}
impl<T: AsMutRawSlice<UnsafeCell<SQLCHAR>, SQLLEN>> OutCType<T> for SQL_C_CHAR {}
impl<T: AsRawSlice<SQLCHAR, SQLLEN>> InCType<T> for SQL_C_CHAR {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_WCHAR;
impl Identifier for SQL_C_WCHAR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_WCHAR.identifier();
}
impl<T: AsMutRawSlice<UnsafeCell<SQLWCHAR>, SQLLEN>> OutCType<T> for SQL_C_WCHAR {}
impl<T: AsRawSlice<SQLWCHAR, SQLLEN>> InCType<T> for SQL_C_WCHAR {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SSHORT;
impl Identifier for SQL_C_SSHORT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_SHORT + SQL_SIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLSMALLINT>> for SQL_C_SSHORT {}
impl InCType<SQLSMALLINT> for SQL_C_SSHORT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_USHORT;
impl Identifier for SQL_C_USHORT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_SHORT + SQL_UNSIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLUSMALLINT>> for SQL_C_USHORT {}
impl InCType<SQLUSMALLINT> for SQL_C_USHORT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SLONG;
impl Identifier for SQL_C_SLONG {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_LONG + SQL_SIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLINTEGER>> for SQL_C_SLONG {}
impl InCType<SQLINTEGER> for SQL_C_SLONG {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_ULONG;
impl Identifier for SQL_C_ULONG {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_LONG + SQL_UNSIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLUINTEGER>> for SQL_C_ULONG {}
impl InCType<SQLUINTEGER> for SQL_C_ULONG {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_FLOAT;
impl Identifier for SQL_C_FLOAT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_REAL.identifier();
}
impl OutCType<UnsafeCell<SQLREAL>> for SQL_C_FLOAT {}
impl InCType<SQLREAL> for SQL_C_FLOAT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_DOUBLE;
impl Identifier for SQL_C_DOUBLE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_DOUBLE.identifier();
}
impl OutCType<UnsafeCell<SQLDOUBLE>> for SQL_C_DOUBLE {}
impl InCType<SQLDOUBLE> for SQL_C_DOUBLE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_BIT;
impl Identifier for SQL_C_BIT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_BIT.identifier();
}
impl OutCType<UnsafeCell<SQLCHAR>> for SQL_C_BIT {}
impl InCType<SQLCHAR> for SQL_C_BIT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_STINYINT;
impl Identifier for SQL_C_STINYINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_TINYINT + SQL_SIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLSCHAR>> for SQL_C_STINYINT {}
impl InCType<SQLSCHAR> for SQL_C_STINYINT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_UTINYINT;
impl Identifier for SQL_C_UTINYINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_C_TINYINT + SQL_UNSIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLCHAR>> for SQL_C_UTINYINT {}
impl InCType<SQLCHAR> for SQL_C_UTINYINT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_SBIGINT;
impl Identifier for SQL_C_SBIGINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_BIGINT.identifier() + SQL_SIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLBIGINT>> for SQL_C_SBIGINT {}
impl InCType<SQLBIGINT> for SQL_C_SBIGINT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_UBIGINT;
impl Identifier for SQL_C_UBIGINT {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_BIGINT.identifier() + SQL_UNSIGNED_OFFSET;
}
impl OutCType<UnsafeCell<SQLUBIGINT>> for SQL_C_UBIGINT {}
impl InCType<SQLUBIGINT> for SQL_C_UBIGINT {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_BINARY;
impl Identifier for SQL_C_BINARY {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_BINARY.identifier();
}
// TODO: Has to be SQLCHAR * actually, not string
impl<T: AsMutRawSlice<UnsafeCell<SQLCHAR>, SQLLEN>> OutCType<T> for SQL_C_BINARY {}
impl<T: AsRawSlice<SQLCHAR, SQLLEN>> InCType<T> for SQL_C_BINARY {}

// TODO: Weird?
pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_NUMERIC;
impl Identifier for SQL_C_NUMERIC {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_NUMERIC.identifier();
}
impl OutCType<UnsafeCell<SQL_NUMERIC_STRUCT>> for SQL_C_NUMERIC {}
impl InCType<SQL_NUMERIC_STRUCT> for SQL_C_NUMERIC {}

#[derive(CType)]
#[cfg(feature = "v3_5")]
#[allow(non_camel_case_types)]
pub struct SQL_C_GUID;
impl Identifier for SQL_C_GUID {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_GUID.identifier();
}
impl OutCType<UnsafeCell<SQLGUID>> for SQL_C_GUID {}
impl InCType<SQLGUID> for SQL_C_GUID {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_DATE;
impl Identifier for SQL_C_TYPE_DATE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_TYPE_DATE.identifier();
}
impl OutCType<UnsafeCell<SQL_DATE_STRUCT>> for SQL_C_TYPE_DATE {}
impl InCType<SQL_DATE_STRUCT> for SQL_C_TYPE_DATE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME;
impl Identifier for SQL_C_TYPE_TIME {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_TYPE_TIME.identifier();
}
impl OutCType<UnsafeCell<SQL_TIME_STRUCT>> for SQL_C_TYPE_TIME {}
impl InCType<SQL_TIME_STRUCT> for SQL_C_TYPE_TIME {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP;
impl Identifier for SQL_C_TYPE_TIMESTAMP {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_TYPE_TIMESTAMP.identifier();
}
impl OutCType<UnsafeCell<SQL_TIMESTAMP_STRUCT>> for SQL_C_TYPE_TIMESTAMP {}
impl InCType<SQL_TIMESTAMP_STRUCT> for SQL_C_TYPE_TIMESTAMP {}

#[derive(CType)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Identifier for SQL_C_TYPE_TIME_WITH_TIMEZONE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIME_WITH_TIMEZONE.identifier();
}
#[cfg(feature = "v4")]
impl OutCType<UnsafeCell<SQL_TIME_WITH_TIMEZONE_STRUCT>> for SQL_C_TYPE_TIME_WITH_TIMEZONE {}
#[cfg(feature = "v4")]
impl InCType<SQL_TIME_WITH_TIMEZONE_STRUCT> for SQL_C_TYPE_TIME_WITH_TIMEZONE {}

#[derive(CType)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Identifier for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIMESTAMP_WITH_TIMEZONE.identifier();
}
#[cfg(feature = "v4")]
impl OutCType<UnsafeCell<SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT>>
    for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE
{
}
#[cfg(feature = "v4")]
impl InCType<SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT> for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR;
impl Identifier for SQL_C_INTERVAL_YEAR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_YEAR.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_YEAR {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_YEAR {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MONTH;
impl Identifier for SQL_C_INTERVAL_MONTH {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_MONTH.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_MONTH {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_MONTH {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY;
impl Identifier for SQL_C_INTERVAL_DAY {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_DAY.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_DAY {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_DAY {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR;
impl Identifier for SQL_C_INTERVAL_HOUR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_HOUR.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_HOUR {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_HOUR {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE;
impl Identifier for SQL_C_INTERVAL_MINUTE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_MINUTE.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_MINUTE {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_MINUTE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_SECOND;
impl Identifier for SQL_C_INTERVAL_SECOND {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_SECOND.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_SECOND {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_SECOND {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR_TO_MONTH;
impl Identifier for SQL_C_INTERVAL_YEAR_TO_MONTH {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_YEAR_TO_MONTH.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_YEAR_TO_MONTH {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_YEAR_TO_MONTH {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_HOUR;
impl Identifier for SQL_C_INTERVAL_DAY_TO_HOUR {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_DAY_TO_HOUR.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_DAY_TO_HOUR {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_DAY_TO_HOUR {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_MINUTE;
impl Identifier for SQL_C_INTERVAL_DAY_TO_MINUTE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_DAY_TO_MINUTE.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_DAY_TO_MINUTE {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_DAY_TO_MINUTE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_SECOND;
impl Identifier for SQL_C_INTERVAL_DAY_TO_SECOND {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_DAY_TO_SECOND.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_DAY_TO_SECOND {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_DAY_TO_SECOND {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_MINUTE;
impl Identifier for SQL_C_INTERVAL_HOUR_TO_MINUTE {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_HOUR_TO_MINUTE.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_HOUR_TO_MINUTE {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_HOUR_TO_MINUTE {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_SECOND;
impl Identifier for SQL_C_INTERVAL_HOUR_TO_SECOND {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_HOUR_TO_SECOND.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_HOUR_TO_SECOND {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_HOUR_TO_SECOND {}

#[derive(CType)]
#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE_TO_SECOND;
impl Identifier for SQL_C_INTERVAL_MINUTE_TO_SECOND {
    type IdentType = SQLSMALLINT;
    const IDENTIFIER: Self::IdentType = SQL_INTERVAL_MINUTE_TO_SECOND.identifier();
}
impl OutCType<UnsafeCell<SQL_INTERVAL_STRUCT>> for SQL_C_INTERVAL_MINUTE_TO_SECOND {}
impl InCType<SQL_INTERVAL_STRUCT> for SQL_C_INTERVAL_MINUTE_TO_SECOND {}

// =================================================================================== //

pub const SQL_MAX_NUMERIC_LEN: usize = 16;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_NUMERIC_STRUCT {
    pub precision: SQLCHAR,
    pub scale: SQLSCHAR,
    /// The sign field is 1 if positive, 0 if negative.
    pub sign: SQLCHAR,
    pub val: [SQLCHAR; SQL_MAX_NUMERIC_LEN],
}

#[repr(C)]
#[cfg(feature = "v3_5")]
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQLGUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_DATE_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_TIME_STRUCT {
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_TIMESTAMP_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    pub fraction: SQLUINTEGER,
}

#[repr(C)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_TIME_WITH_TIMEZONE_STRUCT {
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    pub timezone_hours: SQLSMALLINT,
    pub timezone_minutes: SQLUSMALLINT,
}

#[repr(C)]
#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    pub fraction: SQLUINTEGER,
    pub timezone_hours: SQLSMALLINT,
    pub timezone_minutes: SQLUSMALLINT,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct SQL_INTERVAL_STRUCT {
    pub interval_type: SQLINTERVAL,
    pub interval_sign: SQLSMALLINT,
    // TODO: Make public
    interval: IntervalUnion,
}
impl Eq for SQL_INTERVAL_STRUCT {}
impl PartialEq<SQL_INTERVAL_STRUCT> for SQL_INTERVAL_STRUCT {
    fn eq(&self, other: &SQL_INTERVAL_STRUCT) -> bool {
        unimplemented!()
    }
}
impl std::fmt::Debug for SQL_INTERVAL_STRUCT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

// TODO: Probably shouldn't use an enum with C FFI
// But it's literally defined as enum in ODBC
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQLINTERVAL {
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
// TODO: Remove Copy
#[derive(Clone, Copy)]
union IntervalUnion {
    pub year_month: SQL_YEAR_MONTH_STRUCT,
    pub day_second: SQL_DAY_SECOND_STRUCT,
}

// TODO: Must be copy because it's used in uinon
// Maybe it would bt ok in nightly
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
struct SQL_YEAR_MONTH_STRUCT {
    pub year: SQLUINTEGER,
    pub month: SQLUINTEGER,
}

// TODO: Must be copy because it's used in uinon
// Maybe it would bt ok in nightly
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
struct SQL_DAY_SECOND_STRUCT {
    pub day: SQLUINTEGER,
    pub month: SQLUINTEGER,
    pub minute: SQLUINTEGER,
    pub second: SQLUINTEGER,
    pub fraction: SQLUINTEGER,
}

// TODO: How to implement for uninitialized data. How does UnsafeCell compose with MaybeUninit?
// impl<A, T> OutCType<std::mem::MaybeUninit<T>> for A where A: OutCType<T> {}

impl<A: InCType<T>, T: Identifier> InCType<UnsafeCell<T>> for A {}

unsafe impl<T: Identifier> IntoSQLPOINTER for &UnsafeCell<T> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        // Transforming into reference can cause UB so it is avoided under the assumption
        // that the underlaying type T has the same representation as SQLPOINTER
        // which should be true for any type implementing Identifier trait
        self.get().cast()
    }
}
