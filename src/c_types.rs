use crate::Ident;
use crate::SQLPOINTER;
use crate::convert::AsSQLPOINTER;
use crate::env::{OdbcVersion, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::handle::{RefSQLHDESC, RefUnsafeSQLHDESC};
use crate::sql_types::*;
use crate::str::{OdbcChar, OdbcStr};
use crate::{
    SQLBIGINT, SQLCHAR, SQLDOUBLE, SQLINTEGER, SQLLEN, SQLREAL, SQLSCHAR, SQLSMALLINT, SQLUBIGINT,
    SQLUINTEGER, SQLUSMALLINT, SQLWCHAR,
};
use core::{cell::UnsafeCell, mem::MaybeUninit};

pub trait CData<TT: Ident, V: OdbcVersion>: CDataLen {}

/// Care must be taken because references to DeferredBuf might be written to
// TODO: Do I need to disambiguate between BindCol and BindParameters deferred buffers
// TODO: add bound D: Descriptor
pub unsafe trait DeferredBuf<D, TT: Ident, V: OdbcVersion>: CDataLen + AsSQLPOINTER {}

impl<TT: Ident, T: CScalar, V: OdbcVersion> CData<TT, V> for MaybeUninit<T> where T: CData<TT, V> {}

impl<TT: Ident, T, V: OdbcVersion> CData<TT, V> for [MaybeUninit<T>] where [T]: CData<TT, V> {}

impl<TT: Ident, T: OdbcChar, V: OdbcVersion> CData<TT, V> for OdbcStr<MaybeUninit<T>> where
    OdbcStr<T>: CData<TT, V>
{
}

unsafe impl<DT, TT: Ident, T: CScalar, V: OdbcVersion> DeferredBuf<RefSQLHDESC<'_, DT, V>, TT, V>
    for UnsafeCell<T>
where
    T: CData<TT, V>,
{
}
unsafe impl<DT, TT: Ident, CH: OdbcChar, V: OdbcVersion> DeferredBuf<RefSQLHDESC<'_, DT, V>, TT, V>
    for OdbcStr<UnsafeCell<CH>>
where
    OdbcStr<CH>: CData<TT, V>,
{
}

unsafe impl<'conn, DT, TT: Ident, T: CScalar, V: OdbcVersion>
    DeferredBuf<RefUnsafeSQLHDESC<'conn, DT, V>, TT, V> for UnsafeCell<T>
where
    T: DeferredBuf<RefSQLHDESC<'conn, DT, V>, TT, V>,
{
}
unsafe impl<'conn, DT, TT: Ident, T: CScalar, V: OdbcVersion>
    DeferredBuf<RefUnsafeSQLHDESC<'conn, DT, V>, TT, V> for [UnsafeCell<T>]
where
    [T]: DeferredBuf<RefSQLHDESC<'conn, DT, V>, TT, V>,
{
}
unsafe impl<'conn, DT, TT: Ident, CH: OdbcChar, V: OdbcVersion>
    DeferredBuf<RefUnsafeSQLHDESC<'conn, DT, V>, TT, V> for OdbcStr<UnsafeCell<CH>>
where
    OdbcStr<CH>: DeferredBuf<RefSQLHDESC<'conn, DT, V>, TT, V>,
{
}
unsafe impl<DT, TT: Ident, V: OdbcVersion> DeferredBuf<RefUnsafeSQLHDESC<'_, DT, V>, TT, V>
    for (SQLPOINTER, SQLLEN)
{
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct StrLenOrInd(pub(crate) SQLLEN);
impl StrLenOrInd {
    pub unsafe fn set_len(&mut self, len: SQLLEN) {
        if len < 0 {
            panic!("len must be non-negative: {}", len);
        }

        self.0 = len;
    }
}
pub const SQL_NULL_DATA: StrLenOrInd = StrLenOrInd(-1);
// Output constants
pub const SQL_NO_TOTAL: StrLenOrInd = StrLenOrInd(-4);
// Input constants
pub const SQL_NTS: StrLenOrInd = StrLenOrInd(-3);
pub const SQL_DATA_AT_EXEC: StrLenOrInd = StrLenOrInd(-2);
pub const SQL_COLUMN_IGNORE: StrLenOrInd = StrLenOrInd(-6);

// TODO: Do something
// pub fn SQL_LEN_DATA_AT_EXEC(length: LEN) -> LEN {
//    const SQL_LEN_DATA_AT_EXEC_OFFSET: LEN = -100;
//    (-length).checked_add(SQL_LEN_DATA_AT_EXEC_OFFSET).expect()
//}

// SQLBindParameter only
pub const SQL_DEFAULT_PARAM: StrLenOrInd = StrLenOrInd(-5);

const SQL_UNSIGNED_OFFSET: SQLSMALLINT = -22;
const SQL_SIGNED_OFFSET: SQLSMALLINT = -20;
const SQL_C_SHORT: SQLSMALLINT = SqlTypeV3::identifier(&SQL_SMALLINT);
const SQL_C_LONG: SQLSMALLINT = SqlTypeV3::identifier(&SQL_INTEGER);
const SQL_C_TINYINT: SQLSMALLINT = SqlTypeV3::identifier(&SQL_TINYINT);

// TODO: This value is discouraged from being used
//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 99)]
//#[expect(non_camel_case_types)]
//struct SQL_C_DEFAULT;

#[expect(non_camel_case_types)]
pub struct SQL_C_CHAR;
impl Ident for SQL_C_CHAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_CHAR);
}
impl<V: OdbcVersion> CData<SQL_C_CHAR, V> for OdbcStr<SQLCHAR> {}

#[expect(non_camel_case_types)]
pub struct SQL_C_WCHAR;
impl Ident for SQL_C_WCHAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_WCHAR);
}
impl<V: OdbcVersion> CData<SQL_C_WCHAR, V> for OdbcStr<SQLWCHAR> {}

#[expect(non_camel_case_types)]
pub struct SQL_C_SSHORT;
impl Ident for SQL_C_SSHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_SIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_SSHORT, V> for SQLSMALLINT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_USHORT;
impl Ident for SQL_C_USHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_UNSIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_USHORT, V> for SQLUSMALLINT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_SLONG;
impl Ident for SQL_C_SLONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_SIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_SLONG, V> for SQLINTEGER {}

#[expect(non_camel_case_types)]
pub struct SQL_C_ULONG;
impl Ident for SQL_C_ULONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_UNSIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_ULONG, V> for SQLUINTEGER {}

#[expect(non_camel_case_types)]
pub struct SQL_C_FLOAT;
impl Ident for SQL_C_FLOAT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_REAL);
}
impl<V: OdbcVersion> CData<SQL_C_FLOAT, V> for SQLREAL {}

#[expect(non_camel_case_types)]
pub struct SQL_C_DOUBLE;
impl Ident for SQL_C_DOUBLE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_DOUBLE);
}
impl<V: OdbcVersion> CData<SQL_C_DOUBLE, V> for SQLDOUBLE {}

#[expect(non_camel_case_types)]
pub struct SQL_C_BIT;
impl Ident for SQL_C_BIT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_BIT);
}
impl<V: OdbcVersion> CData<SQL_C_BIT, V> for SQLCHAR {}

#[expect(non_camel_case_types)]
pub struct SQL_C_STINYINT;
impl Ident for SQL_C_STINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_SIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_STINYINT, V> for SQLSCHAR {}

#[expect(non_camel_case_types)]
pub struct SQL_C_UTINYINT;
impl Ident for SQL_C_UTINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_UNSIGNED_OFFSET;
}
impl<V: OdbcVersion> CData<SQL_C_UTINYINT, V> for SQLCHAR {}

#[expect(non_camel_case_types)]
pub struct SQL_C_SBIGINT;
impl Ident for SQL_C_SBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_BIGINT);
}
impl<V: OdbcVersion> CData<SQL_C_SBIGINT, V> for SQLBIGINT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_UBIGINT;
impl Ident for SQL_C_UBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_BIGINT);
}
impl<V: OdbcVersion> CData<SQL_C_UBIGINT, V> for SQLUBIGINT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_BINARY;
impl Ident for SQL_C_BINARY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_BINARY);
}
impl<V: OdbcVersion> CData<SQL_C_BINARY, V> for OdbcStr<SQLCHAR> {}

// TODO: Weird?
pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

#[expect(non_camel_case_types)]
pub struct SQL_C_NUMERIC;
impl Ident for SQL_C_NUMERIC {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_NUMERIC);
}
impl<V: OdbcVersion> CData<SQL_C_NUMERIC, V> for SQL_NUMERIC_STRUCT {}

// TODO: This is 3.5
#[expect(non_camel_case_types)]
pub struct SQL_C_GUID;
impl Ident for SQL_C_GUID {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3_8::identifier(&SQL_GUID);
}
impl CData<SQL_C_GUID, SQL_OV_ODBC3_80> for SQLGUID {}

#[expect(non_camel_case_types)]
pub struct SQL_C_TYPE_DATE;
impl Ident for SQL_C_TYPE_DATE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_TYPE_DATE);
}
impl<V: OdbcVersion> CData<SQL_C_TYPE_DATE, V> for SQL_DATE_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME;
impl Ident for SQL_C_TYPE_TIME {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_TYPE_TIME);
}
impl<V: OdbcVersion> CData<SQL_C_TYPE_TIME, V> for SQL_TIME_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP;
impl Ident for SQL_C_TYPE_TIMESTAMP {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_TYPE_TIMESTAMP);
}
impl<V: OdbcVersion> CData<SQL_C_TYPE_TIMESTAMP, V> for SQL_TIMESTAMP_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME_WITH_TIMEZONE;
impl Ident for SQL_C_TYPE_TIME_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SqlTypeV4::identifier(&SQL_TYPE_TIME_WITH_TIMEZONE);
}
impl CData<SQL_C_TYPE_TIME_WITH_TIMEZONE, SQL_OV_ODBC4> for SQL_TIME_WITH_TIMEZONE_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE;
impl Ident for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SqlTypeV4::identifier(&SQL_TYPE_TIMESTAMP_WITH_TIMEZONE);
}
impl CData<SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE, SQL_OV_ODBC4>
    for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT
{
}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR;
impl Ident for SQL_C_INTERVAL_YEAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_YEAR);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_YEAR, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MONTH;
impl Ident for SQL_C_INTERVAL_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_MONTH);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_MONTH, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY;
impl Ident for SQL_C_INTERVAL_DAY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_DAY);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_DAY, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR;
impl Ident for SQL_C_INTERVAL_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_HOUR);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_HOUR, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE;
impl Ident for SQL_C_INTERVAL_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_MINUTE);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_MINUTE, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_SECOND;
impl Ident for SQL_C_INTERVAL_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_SECOND);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_SECOND, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR_TO_MONTH;
impl Ident for SQL_C_INTERVAL_YEAR_TO_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_YEAR_TO_MONTH);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_YEAR_TO_MONTH, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_HOUR;
impl Ident for SQL_C_INTERVAL_DAY_TO_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_DAY_TO_HOUR);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_DAY_TO_HOUR, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_DAY_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_DAY_TO_MINUTE);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_DAY_TO_MINUTE, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_SECOND;
impl Ident for SQL_C_INTERVAL_DAY_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_DAY_TO_SECOND);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_DAY_TO_SECOND, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_HOUR_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_HOUR_TO_MINUTE);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_HOUR_TO_MINUTE, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_SECOND;
impl Ident for SQL_C_INTERVAL_HOUR_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_HOUR_TO_SECOND);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_HOUR_TO_SECOND, V> for SQL_INTERVAL_STRUCT {}

#[expect(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE_TO_SECOND;
impl Ident for SQL_C_INTERVAL_MINUTE_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_INTERVAL_MINUTE_TO_SECOND);
}
impl<V: OdbcVersion> CData<SQL_C_INTERVAL_MINUTE_TO_SECOND, V> for SQL_INTERVAL_STRUCT {}

// TODO: Test if these types are required or user can achieve the same goal via some other way
// If SQL_ARD_TYPE and SQL_APD_TYPE are allowed, SQLGetData would have to be unsafe
// Also, these types can only be used for SQLGetData so be careful to implement only for CData<TT>
// because it'll get imeplemented for DeferredBuf automatically. In this case some restructuring
// would be required but it would be transparent to the users of the library
//#[expect(non_camel_case_types)]
//pub struct SQL_ARD_TYPE;
//impl Ident for SQL_ARD_TYPE {
//    type Type = SQLSMALLINT;
//    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_ARD_TYPE);
//}
//
//#[expect(non_camel_case_types)]
//pub struct SQL_APD_TYPE;
//impl Ident for SQL_APD_TYPE {
//    type Type = SQLSMALLINT;
//    const IDENTIFIER: Self::Type = SqlTypeV3::identifier(&SQL_APD_TYPE);
//}

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
#[expect(non_snake_case)]
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
    fn eq(&self, _: &SQL_INTERVAL_STRUCT) -> bool {
        unimplemented!()
    }
}
impl core::fmt::Debug for SQL_INTERVAL_STRUCT {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        unimplemented!()
    }
}

// TODO: Probably shouldn't use an enum with C FFI
// But it's literally defined as enum in ODBC
#[repr(C)]
#[expect(non_camel_case_types)]
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
// Maybe it would be ok in nightly
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
struct SQL_YEAR_MONTH_STRUCT {
    pub year: SQLUINTEGER,
    pub month: SQLUINTEGER,
}

// TODO: Must be copy because it's used in uinon
// Maybe it would be ok in nightly
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

/// CScalar must be repr(C) and have
/// the same representation as SQLPOINTER
pub unsafe trait CScalar {}

unsafe impl CScalar for SQLSMALLINT {}
unsafe impl CScalar for SQLUSMALLINT {}
unsafe impl CScalar for SQLUINTEGER {}
unsafe impl CScalar for SQLINTEGER {}
unsafe impl CScalar for SQLREAL {}
unsafe impl CScalar for SQLDOUBLE {}
unsafe impl CScalar for SQLCHAR {}
unsafe impl CScalar for SQLSCHAR {}
unsafe impl CScalar for SQLBIGINT {}
unsafe impl CScalar for SQLUBIGINT {}
unsafe impl CScalar for SQLGUID {}
unsafe impl CScalar for SQL_NUMERIC_STRUCT {}
unsafe impl CScalar for SQL_INTERVAL_STRUCT {}
unsafe impl CScalar for SQL_DATE_STRUCT {}
unsafe impl CScalar for SQL_TIME_STRUCT {}
unsafe impl CScalar for SQL_TIMESTAMP_STRUCT {}
unsafe impl CScalar for SQL_TIME_WITH_TIMEZONE_STRUCT {}
unsafe impl CScalar for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {}

pub trait CDataLen {
    fn len(&self) -> SQLLEN;
}

impl<T: CScalar> CDataLen for T {
    fn len(&self) -> SQLLEN {
        0
    }
}
impl<T: CScalar> CDataLen for UnsafeCell<T> {
    fn len(&self) -> SQLLEN {
        0
    }
}
impl<T: CScalar> CDataLen for MaybeUninit<T> {
    fn len(&self) -> SQLLEN {
        0
    }
}
// TODO:
//impl<T: CScalar> CDataLen for UnsafeCell<MaybeUninit<T>> {
//    fn len(&self) -> SQLLEN {
//        0
//    }
//}
impl<T> CDataLen for [T] {
    fn len(&self) -> SQLLEN {
        self.len()
            .try_into()
            .expect("Buffer length greater than SQLLEN max")
    }
}
impl<T> CDataLen for OdbcStr<T> {
    fn len(&self) -> SQLLEN {
        <[T] as CDataLen>::len(self)
    }
}
impl CDataLen for (SQLPOINTER, SQLLEN) {
    fn len(&self) -> SQLLEN {
        self.1
    }
}

//impl<T> ParameterDir<SQL_PARAM_INPUT> for [T] where [T]: DeferredBuf {}
//impl<T> ParameterDir<SQL_PARAM_OUTPUT> for [MaybeUninit<T>] where [T]: DeferredBuf {}
//impl<T> ParameterDir<SQL_PARAM_INPUT_OUTPUT> for [MaybeUninit<T>] where
//    [T]: ParameterDir<SQL_PARAM_INPUT> {}
