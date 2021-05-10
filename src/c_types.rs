use crate::sql_types::*;
use crate::Ident;
use crate::{
    AsMutSQLPOINTER, AsSQLPOINTER, SQLBIGINT, SQLCHAR, SQLDOUBLE, SQLINTEGER, SQLLEN, SQLPOINTER,
    SQLREAL, SQLSCHAR, SQLSMALLINT, SQLUBIGINT, SQLUINTEGER, SQLUSMALLINT, SQLWCHAR,
};
use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::mem::MaybeUninit;

pub trait Buf<TT: Ident>: BufLen {}
// TODO: If mutable reference is coerced to shared and then to SQLPOINTER this is WRONG!
// This could easily be the case
pub trait OutBuf<TT: Ident>: BufLen + AsMutSQLPOINTER {}

/// Care must be taken because references to DeferredBuf might be written to.
/// This means that DeferredBuf should be implemented on UnsafeCell<T>.
pub unsafe trait DeferredBuf<TT: Ident>: BufLen + AsSQLPOINTER {}

/// ScalarCType must have SQLPOINTER representation
pub unsafe trait ScalarCType {}

pub trait BufLen {
    // TODO: Return MaybeUninit? len is not used for scalar types
    fn len(&self) -> SQLLEN;
}

#[repr(transparent)]
pub struct StrLenOrInd(SQLLEN);
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
// pub fn SQL_LEN_DATA_AT_EXEC;

// SQLBindParameter only
pub const SQL_DEFAULT_PARAM: StrLenOrInd = StrLenOrInd(-5);

const SQL_UNSIGNED_OFFSET: SQLSMALLINT = -22;
const SQL_SIGNED_OFFSET: SQLSMALLINT = -20;
const SQL_C_SHORT: SQLSMALLINT = SQL_SMALLINT.identifier();
const SQL_C_LONG: SQLSMALLINT = SQL_INTEGER.identifier();
const SQL_C_TINYINT: SQLSMALLINT = SQL_TINYINT.identifier();

// TODO: This value is discouraged from being used
//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 99)]
//#[allow(non_camel_case_types)]
//struct SQL_C_DEFAULT;

#[allow(non_camel_case_types)]
pub struct SQL_C_CHAR;
impl Ident for SQL_C_CHAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_CHAR.identifier();
}
impl Buf<SQL_C_CHAR> for [SQLCHAR] {}

#[allow(non_camel_case_types)]
pub struct SQL_C_WCHAR;
impl Ident for SQL_C_WCHAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_WCHAR.identifier();
}
impl Buf<SQL_C_WCHAR> for [SQLWCHAR] {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SSHORT;
impl Ident for SQL_C_SSHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_SIGNED_OFFSET;
}
impl Buf<SQL_C_SSHORT> for SQLSMALLINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_USHORT;
impl Ident for SQL_C_USHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_UNSIGNED_OFFSET;
}
impl Buf<SQL_C_USHORT> for SQLUSMALLINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SLONG;
impl Ident for SQL_C_SLONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_SIGNED_OFFSET;
}
impl Buf<SQL_C_SLONG> for SQLINTEGER {}

#[allow(non_camel_case_types)]
pub struct SQL_C_ULONG;
impl Ident for SQL_C_ULONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_UNSIGNED_OFFSET;
}
impl Buf<SQL_C_ULONG> for SQLUINTEGER {}

#[allow(non_camel_case_types)]
pub struct SQL_C_FLOAT;
impl Ident for SQL_C_FLOAT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_REAL.identifier();
}
impl Buf<SQL_C_FLOAT> for SQLREAL {}

#[allow(non_camel_case_types)]
pub struct SQL_C_DOUBLE;
impl Ident for SQL_C_DOUBLE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_DOUBLE.identifier();
}
impl Buf<SQL_C_DOUBLE> for SQLDOUBLE {}

#[allow(non_camel_case_types)]
pub struct SQL_C_BIT;
impl Ident for SQL_C_BIT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIT.identifier();
}
impl Buf<SQL_C_BIT> for SQLCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_STINYINT;
impl Ident for SQL_C_STINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_SIGNED_OFFSET;
}
impl Buf<SQL_C_STINYINT> for SQLSCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_UTINYINT;
impl Ident for SQL_C_UTINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_UNSIGNED_OFFSET;
}
impl Buf<SQL_C_UTINYINT> for SQLCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SBIGINT;
impl Ident for SQL_C_SBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIGINT.identifier() + SQL_SIGNED_OFFSET;
}
impl Buf<SQL_C_SBIGINT> for SQLBIGINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_UBIGINT;
impl Ident for SQL_C_UBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIGINT.identifier() + SQL_UNSIGNED_OFFSET;
}
impl Buf<SQL_C_UBIGINT> for SQLUBIGINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_BINARY;
impl Ident for SQL_C_BINARY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BINARY.identifier();
}
impl Buf<SQL_C_BINARY> for [SQLCHAR] {}

// TODO: Weird?
pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

#[allow(non_camel_case_types)]
pub struct SQL_C_NUMERIC;
impl Ident for SQL_C_NUMERIC {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_NUMERIC.identifier();
}
impl Buf<SQL_C_NUMERIC> for SQL_NUMERIC_STRUCT {}

#[cfg(feature = "v3_5")]
#[allow(non_camel_case_types)]
pub struct SQL_C_GUID;
impl Ident for SQL_C_GUID {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_GUID.identifier();
}
impl Buf<SQL_C_GUID> for SQLGUID {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_DATE;
impl Ident for SQL_C_TYPE_DATE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_DATE.identifier();
}
impl Buf<SQL_C_TYPE_DATE> for SQL_DATE_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME;
impl Ident for SQL_C_TYPE_TIME {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_TIME.identifier();
}
impl Buf<SQL_C_TYPE_TIME> for SQL_TIME_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP;
impl Ident for SQL_C_TYPE_TIMESTAMP {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_TIMESTAMP.identifier();
}
impl Buf<SQL_C_TYPE_TIMESTAMP> for SQL_TIMESTAMP_STRUCT {}

#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Ident for SQL_C_TYPE_TIME_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIME_WITH_TIMEZONE.identifier();
}
#[cfg(feature = "v4")]
impl Buf<SQL_C_TYPE_TIME_WITH_TIMEZONE> for SQL_TIME_WITH_TIMEZONE_STRUCT {}

#[cfg(feature = "v4")]
#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE;
#[cfg(feature = "v4")]
impl Ident for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIMESTAMP_WITH_TIMEZONE.identifier();
}
#[cfg(feature = "v4")]
impl Buf<SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE> for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR;
impl Ident for SQL_C_INTERVAL_YEAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_YEAR.identifier();
}
impl Buf<SQL_C_INTERVAL_YEAR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MONTH;
impl Ident for SQL_C_INTERVAL_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MONTH.identifier();
}
impl Buf<SQL_C_INTERVAL_MONTH> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY;
impl Ident for SQL_C_INTERVAL_DAY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY.identifier();
}
impl Buf<SQL_C_INTERVAL_DAY> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR;
impl Ident for SQL_C_INTERVAL_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR.identifier();
}
impl Buf<SQL_C_INTERVAL_HOUR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE;
impl Ident for SQL_C_INTERVAL_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MINUTE.identifier();
}
impl Buf<SQL_C_INTERVAL_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_SECOND;
impl Ident for SQL_C_INTERVAL_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_SECOND.identifier();
}
impl Buf<SQL_C_INTERVAL_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR_TO_MONTH;
impl Ident for SQL_C_INTERVAL_YEAR_TO_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_YEAR_TO_MONTH.identifier();
}
impl Buf<SQL_C_INTERVAL_YEAR_TO_MONTH> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_HOUR;
impl Ident for SQL_C_INTERVAL_DAY_TO_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_HOUR.identifier();
}
impl Buf<SQL_C_INTERVAL_DAY_TO_HOUR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_DAY_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_MINUTE.identifier();
}
impl Buf<SQL_C_INTERVAL_DAY_TO_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_SECOND;
impl Ident for SQL_C_INTERVAL_DAY_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_SECOND.identifier();
}
impl Buf<SQL_C_INTERVAL_DAY_TO_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_HOUR_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR_TO_MINUTE.identifier();
}
impl Buf<SQL_C_INTERVAL_HOUR_TO_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_SECOND;
impl Ident for SQL_C_INTERVAL_HOUR_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR_TO_SECOND.identifier();
}
impl Buf<SQL_C_INTERVAL_HOUR_TO_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE_TO_SECOND;
impl Ident for SQL_C_INTERVAL_MINUTE_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MINUTE_TO_SECOND.identifier();
}
impl Buf<SQL_C_INTERVAL_MINUTE_TO_SECOND> for SQL_INTERVAL_STRUCT {}

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

unsafe impl ScalarCType for SQLSMALLINT {}
unsafe impl ScalarCType for SQLUSMALLINT {}
unsafe impl ScalarCType for SQLUINTEGER {}
unsafe impl ScalarCType for SQLINTEGER {}
unsafe impl ScalarCType for SQL_INTERVAL_STRUCT {}
unsafe impl ScalarCType for SQLREAL {}
unsafe impl ScalarCType for SQLDOUBLE {}
unsafe impl ScalarCType for SQLCHAR {}
unsafe impl ScalarCType for SQLSCHAR {}
unsafe impl ScalarCType for SQLBIGINT {}
unsafe impl ScalarCType for SQL_NUMERIC_STRUCT {}
unsafe impl ScalarCType for SQLGUID {}
unsafe impl ScalarCType for SQL_DATE_STRUCT {}
unsafe impl ScalarCType for SQL_TIME_STRUCT {}
unsafe impl ScalarCType for SQL_TIMESTAMP_STRUCT {}
unsafe impl ScalarCType for SQLUBIGINT {}
#[cfg(feature = "v4")]
unsafe impl ScalarCType for SQL_TIME_WITH_TIMEZONE_STRUCT {}
#[cfg(feature = "v4")]
unsafe impl ScalarCType for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {}

impl<T> BufLen for [T] {
    fn len(&self) -> SQLLEN {
        self.len()
            .try_into()
            .expect("Buffer length greater than SQLLEN max")
    }
}
impl<T: ScalarCType> BufLen for T {
    fn len(&self) -> SQLLEN {
        0
    }
}
impl<T: ScalarCType> BufLen for MaybeUninit<T> {
    fn len(&self) -> SQLLEN {
        0
    }
}
impl<T: ScalarCType> BufLen for UnsafeCell<T> {
    fn len(&self) -> SQLLEN {
        0
    }
}

unsafe impl<T: ScalarCType> AsSQLPOINTER for UnsafeCell<T> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // Transforming into reference can cause UB so it is avoided under the assumption
        // that the underlaying type T has the same representation as SQLPOINTER
        // which should hold true for any type implementing ScalarCType trait
        self.get().cast()
    }
}
unsafe impl<T: ScalarCType> AsSQLPOINTER for UnsafeCell<MaybeUninit<T>> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        self.get().cast()
    }
}

unsafe impl<T: ScalarCType> AsMutSQLPOINTER for T {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // It is assumed that ScalarCType is repr(transraprent) or repr(C)
        (self as *mut Self).cast()
    }
}
unsafe impl<T: ScalarCType> AsMutSQLPOINTER for MaybeUninit<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // It is assumed that ScalarCType is repr(transraprent) or repr(C)
        (self as *mut Self).cast()
    }
}

impl<TT: Ident, T> Buf<TT> for [MaybeUninit<T>] {}
impl<TT: Ident, T: ScalarCType> Buf<TT> for MaybeUninit<T> {}

impl<TT: Ident, T> OutBuf<TT> for [T] where [T]: Buf<TT> {}
impl<TT: Ident, T: ScalarCType> OutBuf<TT> for T where T: Buf<TT> {}

unsafe impl<TT: Ident, T> DeferredBuf<TT> for [UnsafeCell<T>] where [T]: Buf<TT> {}
unsafe impl<TT: Ident, T: ScalarCType> DeferredBuf<TT> for UnsafeCell<T> where T: Buf<TT> {}

// TODO: Also, should these be implemented for SQLBindCol?
//unsafe impl AsSQLPOINTER for SQLINTEGER {
//    fn as_SQLPOINTER(&self) -> SQLPOINTER {
//        self as *const _ as SQLPOINTER
//    }
//}
//
//// TODO: Is this to be implemented for SQLLEN? Documentation states it should be implemented for 32-bit value
//// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/sending-long-data?view=sql-server-ver15
//unsafe impl<TT: Ident> DeferredBuf<TT> for SQLINTEGER {}
