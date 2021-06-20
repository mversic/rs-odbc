use crate::sql_types::*;
use crate::Ident;
use crate::{
    AsMutSQLPOINTER, AsSQLPOINTER, IntoSQLPOINTER, SQLBIGINT, SQLCHAR, SQLDOUBLE, SQLINTEGER,
    SQLLEN, SQLPOINTER, SQLREAL, SQLSCHAR, SQLSMALLINT, SQLUBIGINT, SQLUINTEGER, SQLUSMALLINT,
    SQLWCHAR, SQL_PARAM_INPUT, SQL_PARAM_INPUT_OUTPUT, SQL_PARAM_OUTPUT, V4, V3_5
};

use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub trait CDataLen {
    fn len(&self) -> SQLLEN;
}

pub trait CData<V, TT: Ident>: CDataLen {}

// TODO: Do I need to disambiguate between BindCol and BindParameters deferred buffers
/// Care must be taken because references to DeferredBuf might be written to. This usually
/// means that DeferredBuf should only be implemented on &UnsafeCell<T> or &[UnsafeCell<T>].
pub unsafe trait DeferredBuf<'buf, V, TT: Ident>: CDataLen + IntoSQLPOINTER {}

#[repr(transparent)]
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
impl<V> CData<V, SQL_C_CHAR> for [SQLCHAR] {}

#[allow(non_camel_case_types)]
pub struct SQL_C_WCHAR;
impl Ident for SQL_C_WCHAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_WCHAR.identifier();
}
impl<V> CData<V, SQL_C_WCHAR> for [SQLWCHAR] {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SSHORT;
impl Ident for SQL_C_SSHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_SIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_SSHORT> for SQLSMALLINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_USHORT;
impl Ident for SQL_C_USHORT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_SHORT + SQL_UNSIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_USHORT> for SQLUSMALLINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SLONG;
impl Ident for SQL_C_SLONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_SIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_SLONG> for SQLINTEGER {}

#[allow(non_camel_case_types)]
pub struct SQL_C_ULONG;
impl Ident for SQL_C_ULONG {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_LONG + SQL_UNSIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_ULONG> for SQLUINTEGER {}

#[allow(non_camel_case_types)]
pub struct SQL_C_FLOAT;
impl Ident for SQL_C_FLOAT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_REAL.identifier();
}
impl<V> CData<V, SQL_C_FLOAT> for SQLREAL {}

#[allow(non_camel_case_types)]
pub struct SQL_C_DOUBLE;
impl Ident for SQL_C_DOUBLE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_DOUBLE.identifier();
}
impl<V> CData<V, SQL_C_DOUBLE> for SQLDOUBLE {}

#[allow(non_camel_case_types)]
pub struct SQL_C_BIT;
impl Ident for SQL_C_BIT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIT.identifier();
}
impl<V> CData<V, SQL_C_BIT> for SQLCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_STINYINT;
impl Ident for SQL_C_STINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_SIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_STINYINT> for SQLSCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_UTINYINT;
impl Ident for SQL_C_UTINYINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_C_TINYINT + SQL_UNSIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_UTINYINT> for SQLCHAR {}

#[allow(non_camel_case_types)]
pub struct SQL_C_SBIGINT;
impl Ident for SQL_C_SBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIGINT.identifier() + SQL_SIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_SBIGINT> for SQLBIGINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_UBIGINT;
impl Ident for SQL_C_UBIGINT {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BIGINT.identifier() + SQL_UNSIGNED_OFFSET;
}
impl<V> CData<V, SQL_C_UBIGINT> for SQLUBIGINT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_BINARY;
impl Ident for SQL_C_BINARY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_BINARY.identifier();
}
impl<V> CData<V, SQL_C_BINARY> for [SQLCHAR] {}

// TODO: Weird?
pub use SQL_C_BINARY as SQL_C_VARBOOKMARK;

#[allow(non_camel_case_types)]
pub struct SQL_C_NUMERIC;
impl Ident for SQL_C_NUMERIC {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_NUMERIC.identifier();
}
impl<V> CData<V, SQL_C_NUMERIC> for SQL_NUMERIC_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_GUID;
impl Ident for SQL_C_GUID {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_GUID.identifier();
}
impl CData<V3_5, SQL_C_GUID> for SQLGUID {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_DATE;
impl Ident for SQL_C_TYPE_DATE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_DATE.identifier();
}
impl<V> CData<V, SQL_C_TYPE_DATE> for SQL_DATE_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME;
impl Ident for SQL_C_TYPE_TIME {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_TIME.identifier();
}
impl<V> CData<V, SQL_C_TYPE_TIME> for SQL_TIME_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP;
impl Ident for SQL_C_TYPE_TIMESTAMP {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_TYPE_TIMESTAMP.identifier();
}
impl<V> CData<V, SQL_C_TYPE_TIMESTAMP> for SQL_TIMESTAMP_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIME_WITH_TIMEZONE;
impl Ident for SQL_C_TYPE_TIME_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIME_WITH_TIMEZONE.identifier();
}
impl CData<V4, SQL_C_TYPE_TIME_WITH_TIMEZONE> for SQL_TIME_WITH_TIMEZONE_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE;
impl Ident for SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: SQLSMALLINT = SQL_TYPE_TIMESTAMP_WITH_TIMEZONE.identifier();
}
impl CData<V4, SQL_C_TYPE_TIMESTAMP_WITH_TIMEZONE> for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR;
impl Ident for SQL_C_INTERVAL_YEAR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_YEAR.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_YEAR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MONTH;
impl Ident for SQL_C_INTERVAL_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MONTH.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_MONTH> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY;
impl Ident for SQL_C_INTERVAL_DAY {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_DAY> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR;
impl Ident for SQL_C_INTERVAL_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_HOUR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE;
impl Ident for SQL_C_INTERVAL_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MINUTE.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_SECOND;
impl Ident for SQL_C_INTERVAL_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_SECOND.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_YEAR_TO_MONTH;
impl Ident for SQL_C_INTERVAL_YEAR_TO_MONTH {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_YEAR_TO_MONTH.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_YEAR_TO_MONTH> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_HOUR;
impl Ident for SQL_C_INTERVAL_DAY_TO_HOUR {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_HOUR.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_DAY_TO_HOUR> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_DAY_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_MINUTE.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_DAY_TO_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_DAY_TO_SECOND;
impl Ident for SQL_C_INTERVAL_DAY_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_DAY_TO_SECOND.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_DAY_TO_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_MINUTE;
impl Ident for SQL_C_INTERVAL_HOUR_TO_MINUTE {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR_TO_MINUTE.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_HOUR_TO_MINUTE> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_HOUR_TO_SECOND;
impl Ident for SQL_C_INTERVAL_HOUR_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_HOUR_TO_SECOND.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_HOUR_TO_SECOND> for SQL_INTERVAL_STRUCT {}

#[allow(non_camel_case_types)]
pub struct SQL_C_INTERVAL_MINUTE_TO_SECOND;
impl Ident for SQL_C_INTERVAL_MINUTE_TO_SECOND {
    type Type = SQLSMALLINT;
    const IDENTIFIER: Self::Type = SQL_INTERVAL_MINUTE_TO_SECOND.identifier();
}
impl<V> CData<V, SQL_C_INTERVAL_MINUTE_TO_SECOND> for SQL_INTERVAL_STRUCT {}

// TODO: Test if these types are required or user can achieve the same goal via some other way
// If SQL_ARD_TYPE and SQL_APD_TYPE are allowed, SQLGetData would have to be unsafe
// Also, these types can only be used for SQLGetData so be careful to implement only for CData<TT>
// because it'll get imeplemented for DeferredBuf automatically. In this case some restructuring
// would be required but it would be transparent to the users of the library
//#[allow(non_camel_case_types)]
//pub struct SQL_ARD_TYPE;
//impl Ident for SQL_ARD_TYPE {
//    type Type = SQLSMALLINT;
//    const IDENTIFIER: Self::Type = SQL_ARD_TYPE.identifier();
//}
//
//#[allow(non_camel_case_types)]
//pub struct SQL_APD_TYPE;
//impl Ident for SQL_APD_TYPE {
//    type Type = SQLSMALLINT;
//    const IDENTIFIER: Self::Type = SQL_APD_TYPE.identifier();
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

/// ScalarCType must be repr(C)
pub unsafe trait ScalarCType {}

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
unsafe impl ScalarCType for SQL_TIME_WITH_TIMEZONE_STRUCT {}
unsafe impl ScalarCType for SQL_TIMESTAMP_WITH_TIMEZONE_STRUCT {}

impl<T> CDataLen for [T] {
    fn len(&self) -> SQLLEN {
        self.len()
            .try_into()
            .expect("Buffer length greater than SQLLEN max")
    }
}
impl<T: ScalarCType> CDataLen for T {
    fn len(&self) -> SQLLEN {
        MaybeUninit::<T>::len(unsafe { std::mem::transmute(self) })
    }
}
impl<T: ScalarCType> CDataLen for MaybeUninit<T> {
    fn len(&self) -> SQLLEN {
        0
    }
}
impl<T> CDataLen for &[T] {
    fn len(&self) -> SQLLEN {
        CDataLen::len(*self)
    }
}
impl<T: ScalarCType> CDataLen for &UnsafeCell<T> {
    fn len(&self) -> SQLLEN {
        T::len(unsafe { std::mem::transmute(*self) })
    }
}
impl CDataLen for NonNull<c_void> {
    fn len(&self) -> SQLLEN {
        0
    }
}

unsafe impl<T: ScalarCType> IntoSQLPOINTER for &T {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as *const _ as SQLPOINTER
    }
}
unsafe impl<T: ScalarCType> IntoSQLPOINTER for &UnsafeCell<T> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.get().cast()
    }
}

unsafe impl<T: ScalarCType> AsSQLPOINTER for T {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        self as *const _ as SQLPOINTER
    }
}

unsafe impl<T: ScalarCType> AsMutSQLPOINTER for T {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        (self as *mut Self).cast()
    }
}
unsafe impl<T: ScalarCType> AsMutSQLPOINTER for MaybeUninit<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}

impl<V, TT: Ident, T> CData<V, TT> for [MaybeUninit<T>] where [T]: CData<V, TT> {}
impl<V, TT: Ident, T: ScalarCType> CData<V, TT> for MaybeUninit<T> where T: CData<V, TT> {}

unsafe impl<'buf, V, TT: Ident, T> DeferredBuf<'buf, V, TT> for &'buf [UnsafeCell<T>] where [T]: CData<V, TT> {}
unsafe impl<'buf, V, TT: Ident, T: ScalarCType> DeferredBuf<'buf, V, TT> for &'buf UnsafeCell<T> where
    T: CData<V, TT>
{
}

unsafe impl<'buf, V, TT: Ident> DeferredBuf<'buf, V, TT> for NonNull<c_void> {}

//impl<T> ParameterDir<SQL_PARAM_INPUT> for [T] where [T]: DeferredBuf {}
//impl<T> ParameterDir<SQL_PARAM_OUTPUT> for [MaybeUninit<T>] where [T]: DeferredBuf {}
//impl<T> ParameterDir<SQL_PARAM_INPUT_OUTPUT> for [MaybeUninit<T>] where
//    [T]: ParameterDir<SQL_PARAM_INPUT> {}
