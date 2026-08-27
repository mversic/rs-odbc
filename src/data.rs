use core::ffi::c_void;

use co3::ReprC;
use rust_spec::RustSpec;

// TODO: Add support for mingw-x64 on x86 platform

macro_rules! odbc_integer {
    ($name:ident, $rust:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, RustSpec, ReprC)]
        #[reprC(identity)]
        #[repr(transparent)]
        pub struct $name($rust);

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn new(value: $rust) -> Self {
                Self(value)
            }

            #[allow(dead_code)]
            pub(crate) const fn get(self) -> $rust {
                self.0
            }
        }

        impl From<$rust> for $name {
            fn from(value: $rust) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $rust {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

odbc_integer!(SMALLINT, i16);
odbc_integer!(USMALLINT, u16);
odbc_integer!(INTEGER, i32);
odbc_integer!(UINTEGER, u32);
odbc_integer!(BIGINT, i64);
odbc_integer!(UBIGINT, u64);
odbc_integer!(LEN, isize);
odbc_integer!(ULEN, usize);
odbc_integer!(RETCODE, i16);

impl TryFrom<usize> for SMALLINT {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(i16::try_from(value)?))
    }
}

impl TryFrom<usize> for INTEGER {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self(i32::try_from(value)?))
    }
}

macro_rules! odbc_float {
    ($name:ident, $rust:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, RustSpec, ReprC)]
        #[reprC(identity)]
        #[repr(transparent)]
        pub struct $name($rust);

        impl $name {
            pub const fn new(value: $rust) -> Self {
                Self(value)
            }

            pub const fn get(self) -> $rust {
                self.0
            }
        }

        impl From<$rust> for $name {
            fn from(value: $rust) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $rust {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

odbc_float!(REAL, f32);
odbc_float!(DOUBLE, f64);
pub use DOUBLE as FLOAT;

/// ASCII encoded character
pub type CHAR = u8;
/// UCS-2 encoded character
pub type WCHAR = u16;

#[cfg(target_pointer_width = "32")]
pub type SETPOSIROW = u16;
#[cfg(target_pointer_width = "64")]
pub type SETPOSIROW = u64;

// TODO: Is this type required?
//type UWORD = u16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct POINTER(*mut c_void);

impl POINTER {
    pub(crate) const NULL: Self = Self(core::ptr::null_mut());

    const fn from_usize(value: usize) -> Self {
        Self(value as *mut c_void)
    }
}

impl<T> From<*mut T> for POINTER {
    fn from(value: *mut T) -> Self {
        Self(value.cast())
    }
}

impl<T> From<*const T> for POINTER {
    fn from(value: *const T) -> Self {
        Self(value.cast_mut().cast())
    }
}

macro_rules! impl_sqlpointer_from_integer {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl From<$ty> for POINTER {
                fn from(value: $ty) -> Self {
                    Self::from_usize(value as usize)
                }
            }
        )+
    };
}

impl_sqlpointer_from_integer! {
    u16,
    i16,
    u32,
    i32,
    usize,
    isize,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct StrLenOrInd(pub(crate) isize);

impl StrLenOrInd {
    /// # Safety
    ///
    /// The caller must ensure that the length is valid for the associated buffer.
    pub unsafe fn set_len(&mut self, len: isize) {
        if len < 0 {
            panic!("len must be non-negative: {}", len);
        }

        self.0 = len;
    }
}

pub const NULL_DATA: StrLenOrInd = StrLenOrInd(-1);
// Output constants
pub const NO_TOTAL: StrLenOrInd = StrLenOrInd(-4);
// Input constants
pub const NTS: StrLenOrInd = StrLenOrInd(-3);
pub const DATA_AT_EXEC: StrLenOrInd = StrLenOrInd(-2);
pub const COLUMN_IGNORE: StrLenOrInd = StrLenOrInd(-6);

// TODO: Do something
// pub fn SQL_LEN_DATA_AT_EXEC(length: isize) -> isize {
//    const SQL_LEN_DATA_AT_EXEC_OFFSET: isize = -100;
//    (-length).checked_add(SQL_LEN_DATA_AT_EXEC_OFFSET).expect()
//}

// SQLBindParameter only
pub const DEFAULT_PARAM: StrLenOrInd = StrLenOrInd(-5);

pub const MAX_NUMERIC_LEN: usize = 16;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct NUMERIC_STRUCT {
    pub precision: u8,
    pub scale: i8,
    /// The sign field is 1 if positive, 0 if negative.
    pub sign: u8,
    pub val: [u8; MAX_NUMERIC_LEN],
}

#[expect(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct DATE_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct TIME_STRUCT {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct TIMESTAMP_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    pub fraction: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct TIME_WITH_TIMEZONE_STRUCT {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub timezone_hours: i16,
    pub timezone_minutes: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct TIMESTAMP_WITH_TIMEZONE_STRUCT {
    pub year: i16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    /// Number of billionths of a second and ranges from 0 through 999,999,999
    pub fraction: u32,
    pub timezone_hours: i16,
    pub timezone_minutes: u16,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct INTERVAL_STRUCT {
    pub interval_type: INTERVAL,
    pub interval_sign: i16,
    // TODO: Make public
    interval: IntervalUnion,
}

impl Eq for INTERVAL_STRUCT {}

impl PartialEq<INTERVAL_STRUCT> for INTERVAL_STRUCT {
    fn eq(&self, _: &INTERVAL_STRUCT) -> bool {
        unimplemented!()
    }
}

impl core::fmt::Debug for INTERVAL_STRUCT {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        unimplemented!()
    }
}

// TODO: Probably shouldn't use an enum with C FFI
// But it's literally defined as enum in ODBC
#[expect(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum INTERVAL {
    IS_YEAR = 1,
    IS_MONTH = 2,
    IS_DAY = 3,
    IS_HOUR = 4,
    IS_MINUTE = 5,
    IS_SECOND = 6,
    IS_YEAR_TO_MONTH = 7,
    IS_DAY_TO_HOUR = 8,
    IS_DAY_TO_MINUTE = 9,
    IS_DAY_TO_SECOND = 10,
    IS_HOUR_TO_MINUTE = 11,
    IS_HOUR_TO_SECOND = 12,
    IS_MINUTE_TO_SECOND = 13,
}

#[derive(Clone, Copy)]
#[repr(C)]
union IntervalUnion {
    pub year_month: YEAR_MONTH_STRUCT,
    pub day_second: DAY_SECOND_STRUCT,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
struct YEAR_MONTH_STRUCT {
    pub year: u32,
    pub month: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(C)]
struct DAY_SECOND_STRUCT {
    pub day: u32,
    pub month: u32,
    pub minute: u32,
    pub second: u32,
    pub fraction: u32,
}
