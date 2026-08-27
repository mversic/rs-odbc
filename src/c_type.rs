use core::mem::MaybeUninit;

use co3::{ReprC, tag::Tagged};
use rust_spec::RustSpec;

use crate::{
    data::{
        DATE_STRUCT, GUID as GUID_DATA, INTERVAL_STRUCT, NUMERIC_STRUCT, TIME_STRUCT,
        TIME_WITH_TIMEZONE_STRUCT, TIMESTAMP_STRUCT, TIMESTAMP_WITH_TIMEZONE_STRUCT,
    },
    env::OdbcVersion,
    sql_type,
    str::OdbcStr,
};

/// A C data type identifier returned by an ODBC function.
///
/// Unlike the zero-sized C type markers, this stores a type selected at
/// runtime. The value is intentionally open-ended so extension identifiers
/// remain representable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct CTypeCode(i16);

impl CTypeCode {
    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn get(self) -> i16 {
        self.0
    }
}

/// A C data type identifier with a statically known Rust buffer representation.
///
/// This trait is open so a driver crate can implement it for a local marker type and the
/// ODBC versions for which its extended C type is available. APIs using the marker also
/// require it to implement [`Tagged`], which supplies the driver-specific tag.
///
/// # Safety
///
/// `Value` must have the buffer representation prescribed by the type identifier.
pub unsafe trait CType<V: OdbcVersion> {
    type Value: ?Sized;
}

const UNSIGNED_OFFSET: i16 = -22;
const SIGNED_OFFSET: i16 = -20;

const SHORT: i16 = <sql_type::SMALLINT as Tagged>::TAG;
const LONG: i16 = <sql_type::INTEGER as Tagged>::TAG;
const TINYINT: i16 = <sql_type::TINYINT as Tagged>::TAG;

macro_rules! c_types {
    ($($type:ident {
        id: $id:expr,
        value: $value:ty,
        sql_type: $sql_type:ty,
    })+) => {$(
        #[derive(Clone, Copy, co3::Tag)]
        #[tag(i16, unsafe($id))]
        #[allow(non_camel_case_types)]
        pub enum $type {}

        unsafe impl<V: OdbcVersion> CType<V> for $type
        where
            $sql_type: sql_type::SqlType<V>,
        {
            type Value = $value;
        }
    )+};
}

c_types! {
    CHAR {
        id: <sql_type::CHAR as Tagged>::TAG,
        value: OdbcStr<MaybeUninit<crate::data::CHAR>>,
        sql_type: sql_type::CHAR,
    }
    WCHAR {
        id: <sql_type::WCHAR as Tagged>::TAG,
        value: OdbcStr<MaybeUninit<crate::data::WCHAR>>,
        sql_type: sql_type::WCHAR,
    }
    SSHORT {
        id: SHORT + SIGNED_OFFSET,
        value: MaybeUninit<i16>,
        sql_type: sql_type::SMALLINT,
    }
    USHORT {
        id: SHORT + UNSIGNED_OFFSET,
        value: MaybeUninit<u16>,
        sql_type: sql_type::SMALLINT,
    }
    SLONG {
        id: LONG + SIGNED_OFFSET,
        value: MaybeUninit<i32>,
        sql_type: sql_type::INTEGER,
    }
    ULONG {
        id: LONG + UNSIGNED_OFFSET,
        value: MaybeUninit<u32>,
        sql_type: sql_type::INTEGER,
    }
    FLOAT {
        id: <sql_type::REAL as Tagged>::TAG,
        value: MaybeUninit<f32>,
        sql_type: sql_type::REAL,
    }
    DOUBLE {
        id: <sql_type::DOUBLE as Tagged>::TAG,
        value: MaybeUninit<f64>,
        sql_type: sql_type::DOUBLE,
    }
    BIT {
        id: <sql_type::BIT as Tagged>::TAG,
        value: MaybeUninit<u8>,
        sql_type: sql_type::BIT,
    }
    STINYINT {
        id: TINYINT + SIGNED_OFFSET,
        value: MaybeUninit<i8>,
        sql_type: sql_type::TINYINT,
    }
    UTINYINT {
        id: TINYINT + UNSIGNED_OFFSET,
        value: MaybeUninit<u8>,
        sql_type: sql_type::TINYINT,
    }
    SBIGINT {
        id: <sql_type::BIGINT as Tagged>::TAG,
        value: MaybeUninit<i64>,
        sql_type: sql_type::BIGINT,
    }
    UBIGINT {
        id: <sql_type::BIGINT as Tagged>::TAG,
        value: MaybeUninit<u64>,
        sql_type: sql_type::BIGINT,
    }
    BINARY {
        id: <sql_type::BINARY as Tagged>::TAG,
        value: [MaybeUninit<u8>],
        sql_type: sql_type::BINARY,
    }
    NUMERIC {
        id: <sql_type::NUMERIC as Tagged>::TAG,
        value: MaybeUninit<NUMERIC_STRUCT>,
        sql_type: sql_type::NUMERIC,
    }
    // TODO: This is 3.5
    GUID {
        id: <sql_type::GUID as Tagged>::TAG,
        value: MaybeUninit<GUID_DATA>,
        sql_type: sql_type::GUID,
    }
    TYPE_DATE {
        id: <sql_type::TYPE_DATE as Tagged>::TAG,
        value: MaybeUninit<DATE_STRUCT>,
        sql_type: sql_type::TYPE_DATE,
    }
    TYPE_TIME {
        id: <sql_type::TYPE_TIME as Tagged>::TAG,
        value: MaybeUninit<TIME_STRUCT>,
        sql_type: sql_type::TYPE_TIME,
    }
    TYPE_TIMESTAMP {
        id: <sql_type::TYPE_TIMESTAMP as Tagged>::TAG,
        value: MaybeUninit<TIMESTAMP_STRUCT>,
        sql_type: sql_type::TYPE_TIMESTAMP,
    }
    TYPE_TIME_WITH_TIMEZONE {
        id: <sql_type::TYPE_TIME_WITH_TIMEZONE as Tagged>::TAG,
        value: MaybeUninit<TIME_WITH_TIMEZONE_STRUCT>,
        sql_type: sql_type::TYPE_TIME_WITH_TIMEZONE,
    }
    TYPE_TIMESTAMP_WITH_TIMEZONE {
        id: <sql_type::TYPE_TIMESTAMP_WITH_TIMEZONE as Tagged>::TAG,
        value: MaybeUninit<TIMESTAMP_WITH_TIMEZONE_STRUCT>,
        sql_type: sql_type::TYPE_TIMESTAMP_WITH_TIMEZONE,
    }
    INTERVAL_YEAR {
        id: <sql_type::INTERVAL_YEAR as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_YEAR,
    }
    INTERVAL_MONTH {
        id: <sql_type::INTERVAL_MONTH as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_MONTH,
    }
    INTERVAL_DAY {
        id: <sql_type::INTERVAL_DAY as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_DAY,
    }
    INTERVAL_HOUR {
        id: <sql_type::INTERVAL_HOUR as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_HOUR,
    }
    INTERVAL_MINUTE {
        id: <sql_type::INTERVAL_MINUTE as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_MINUTE,
    }
    INTERVAL_SECOND {
        id: <sql_type::INTERVAL_SECOND as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_SECOND,
    }
    INTERVAL_YEAR_TO_MONTH {
        id: <sql_type::INTERVAL_YEAR_TO_MONTH as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_YEAR_TO_MONTH,
    }
    INTERVAL_DAY_TO_HOUR {
        id: <sql_type::INTERVAL_DAY_TO_HOUR as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_DAY_TO_HOUR,
    }
    INTERVAL_DAY_TO_MINUTE {
        id: <sql_type::INTERVAL_DAY_TO_MINUTE as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_DAY_TO_MINUTE,
    }
    INTERVAL_DAY_TO_SECOND {
        id: <sql_type::INTERVAL_DAY_TO_SECOND as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_DAY_TO_SECOND,
    }
    INTERVAL_HOUR_TO_MINUTE {
        id: <sql_type::INTERVAL_HOUR_TO_MINUTE as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_HOUR_TO_MINUTE,
    }
    INTERVAL_HOUR_TO_SECOND {
        id: <sql_type::INTERVAL_HOUR_TO_SECOND as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_HOUR_TO_SECOND,
    }
    INTERVAL_MINUTE_TO_SECOND {
        id: <sql_type::INTERVAL_MINUTE_TO_SECOND as Tagged>::TAG,
        value: MaybeUninit<INTERVAL_STRUCT>,
        sql_type: sql_type::INTERVAL_MINUTE_TO_SECOND,
    }
}

// TODO: Weird?
//pub use BINARY as VARBOOKMARK;

// TODO: Test if these types are required or user can achieve the same goal via some other way
// If SQL_ARD_TYPE and SQL_APD_TYPE are allowed, SQLGetData would have to be unsafe
// Also, these types can only be used for SQLGetData
//#[expect(non_camel_case_types)]
//pub enum SQL_ARD_TYPE {}
//impl_c_type_handle!(SQL_ARD_TYPE => <SQL_ARD_TYPE as Tagged>::TAG);
//#[expect(non_camel_case_types)]
//pub enum SQL_APD_TYPE {}
//impl_c_type_handle!(SQL_APD_TYPE => <SQL_APD_TYPE as Tagged>::TAG);

// TODO: This is discouraged
///// Requests the driver-defined default C representation for the SQL column.
/////
///// Because the concrete representation is selected at runtime, users must
///// ensure that the bound buffer is suitable for the column's SQL type.
//#[expect(non_camel_case_types)]
//pub enum DEFAULT {}
//impl_c_type_handle!(DEFAULT => 99);
