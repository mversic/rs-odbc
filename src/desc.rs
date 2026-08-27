use core::{cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit};

use co3::{ReprC, Tag};
use rust_spec::RustSpec;

use crate::{
    Defined, OdbcDefined,
    attr::inherit_attr,
    env::{OV_ODBC3, OV_ODBC3_80, OV_ODBC4, OdbcVersion},
    str::{OdbcChar, OdbcStr},
};

#[sealed::sealed]
pub trait DescType {}

macro_rules! define_desc_attributes {
    ($($(#[$attribute:meta])* $name:ident = $id:expr;)+) => {$(
        $(#[$attribute])*
        #[derive(Tag)]
        #[tag(i16, unsafe($id))]
        #[allow(non_camel_case_types)]
        pub enum $name {}

    )+};
}

define_desc_attributes! {
    COUNT = 1001;
    CONCISE_TYPE = 2;
    DISPLAY_SIZE = 6;
    UNSIGNED = 8;
    FIXED_PREC_SCALE = 9;
    UPDATABLE = 10;
    AUTO_UNIQUE_VALUE = 11;
    CASE_SENSITIVE = 12;
    SEARCHABLE = 13;
    TYPE_NAME = 14;
    TABLE_NAME = 15;
    SCHEMA_NAME = 16;
    CATALOG_NAME = 17;
    LABEL = 18;
    BASE_COLUMN_NAME = 22;
    BASE_TABLE_NAME = 23;
    LITERAL_PREFIX = 27;
    LITERAL_SUFFIX = 28;
    LOCAL_TYPE_NAME = 29;
    NUM_PREC_RADIX = 32;
    TYPE = 1002;
    LENGTH = 1003;
    PRECISION = 1005;
    SCALE = 1006;
    NULLABLE = 1008;
    NAME = 1011;
    UNNAMED = 1012;
    OCTET_LENGTH = 1013;
    ALLOC_TYPE = 1099;
    ARRAY_SIZE = 20;
    BIND_TYPE = 25;
    ROWS_PROCESSED_PTR = 34;
    DATA_PTR = 1010;
    OCTET_LENGTH_PTR = 1004;
    PARAMETER_TYPE = 33;
    // TODO: Defined in 3.5
    ROWVER = 35;
}

/// Marks a descriptor field whose value can be retrieved from descriptor type `DT`.
///
/// # Safety
/// `Buffer` must have the representation and initialization requirements prescribed by ODBC or
/// by the driver specification.
pub unsafe trait DescFieldGet<V: OdbcVersion, DT: DescType>: Defined {
    type Buffer<C: OdbcChar>: ?Sized;
}

/// Marks a descriptor field whose value can be supplied to descriptor type `DT`.
///
/// # Safety
/// `Value` must lower to the representation prescribed by ODBC or by the driver specification.
pub unsafe trait DescFieldSet<V: OdbcVersion, DT: DescType>: Defined {
    type Value<C: OdbcChar>;
}

inherit_attr!(get DescFieldGet, OV_ODBC3 => OV_ODBC3_80, desc);
inherit_attr!(set DescFieldSet, OV_ODBC3 => OV_ODBC3_80, desc);

inherit_attr!(get DescFieldGet, OV_ODBC3_80 => OV_ODBC4, desc);
inherit_attr!(set DescFieldSet, OV_ODBC3_80 => OV_ODBC4, desc);

#[derive(Debug)]
pub struct AppDesc<'buf> {
    pub(crate) rows_processed: PhantomData<&'buf ()>,
    pub(crate) data_ptrs: PhantomData<&'buf ()>,
}

#[derive(Debug)]
pub enum IRD {}
#[derive(Debug)]
pub enum IPD {}

#[sealed::sealed]
impl DescType for AppDesc<'_> {}
#[sealed::sealed]
impl DescType for IRD {}
#[sealed::sealed]
impl DescType for IPD {}

macro_rules! desc_fields {
    ($($field:ty {
        get $get_scope:tt => $buffer:tt;
        $(set $set_scope:tt => $value:ty;)?
    })+) => {$(
        desc_fields!(@get $field, $get_scope => $buffer);
        $(desc_fields!(@set $field, $set_scope => $value);)?
    )+};

    (@get $field:ty, all => $buffer:ty) => {
        unsafe impl<DT: DescType> DescFieldGet<OV_ODBC3, DT> for $field {
            type Buffer<C: OdbcChar> = MaybeUninit<$buffer>;
        }
    };
    (@get $field:ty, [$($descriptor:ty),+ $(,)?] => string) => {$(
        unsafe impl DescFieldGet<OV_ODBC3, $descriptor> for $field {
            type Buffer<C: OdbcChar> = OdbcStr<MaybeUninit<C>>;
        }
    )+};
    (@get $field:ty, [$($descriptor:ty),+ $(,)?] => $buffer:ty) => {$(
        unsafe impl DescFieldGet<OV_ODBC3, $descriptor> for $field {
            type Buffer<C: OdbcChar> = MaybeUninit<$buffer>;
        }
    )+};

    (@set $field:ty, all => $value:ty) => {
        unsafe impl<DT: DescType> DescFieldSet<OV_ODBC3, DT> for $field {
            type Value<C: OdbcChar> = $value;
        }
    };
    (@set $field:ty, [$($descriptor:ty),+ $(,)?] => $value:ty) => {$(
        unsafe impl DescFieldSet<OV_ODBC3, $descriptor> for $field {
            type Value<C: OdbcChar> = $value;
        }
    )+};
}

type RowsProcessedPtr = *mut usize;

desc_fields! {
    ALLOC_TYPE {
        get all => AllocType;
    }
    ARRAY_SIZE {
        get [AppDesc<'_>] => usize;
        set [AppDesc<'_>] => usize;
    }
    BIND_TYPE {
        get [AppDesc<'_>] => BindType;
        set [AppDesc<'_>] => BindType;
    }
    COUNT {
        get all => i16;
        set all => i16;
    }
    ROWS_PROCESSED_PTR {
        get [IRD, IPD] => RowsProcessedPtr;
    }
    AUTO_UNIQUE_VALUE {
        get [IRD] => AutoUniqueValue;
    }
    BASE_COLUMN_NAME {
        get [IRD] => string;
    }
    BASE_TABLE_NAME {
        get [IRD] => string;
    }
    CASE_SENSITIVE {
        get [IRD] => CaseSensitive;
    }
    CATALOG_NAME {
        get [IRD] => string;
    }
    DISPLAY_SIZE {
        get [IRD] => i32;
    }
    FIXED_PREC_SCALE {
        get [IRD, IPD] => FixedPrecScale;
    }
    OCTET_LENGTH {
        get all => isize;
        set all => isize;
    }
}

// Deferred fields retain these addresses beyond SQLSetDescField. Deriving the
// reference lifetime from AppDesc makes the descriptor borrow their storage.
unsafe impl<'buf> DescFieldSet<OV_ODBC3, AppDesc<'buf>> for OCTET_LENGTH_PTR {
    type Value<C: OdbcChar> = Option<&'buf UnsafeCell<MaybeUninit<isize>>>;
}

macro_rules! odbc_defined_desc_fields {
    ($($field:ty),+ $(,)?) => {$(
        impl Defined for $field {
            type By = OdbcDefined;
        }
    )+};
}

odbc_defined_desc_fields!(
    ALLOC_TYPE,
    ARRAY_SIZE,
    BIND_TYPE,
    ROWS_PROCESSED_PTR,
    DATA_PTR,
    OCTET_LENGTH_PTR,
    PARAMETER_TYPE,
);

// TODO: Defined in 3.5
odbc_defined_desc_fields!(ROWVER);

macro_rules! desc_bool {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
        #[repr(u32)]
        pub enum $name {
            FALSE = 0,
            TRUE = 1,
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                if value { Self::TRUE } else { Self::FALSE }
            }
        }

        impl From<$name> for bool {
            fn from(value: $name) -> Self {
                value == $name::TRUE
            }
        }

    )+};
}

desc_bool!(AutoUniqueValue, CaseSensitive, FixedPrecScale);

//#if (ODBCVER >= 0x0300)
//#define ARRAY_SIZE                     20
//#define AUTO_UNIQUE_VALUE              SQL_COLUMN_AUTO_INCREMENT
//#define BASE_COLUMN_NAME               22
//#define BASE_TABLE_NAME                23
//#define BIND_OFFSET_PTR                24
//#define BIND_TYPE                      25
//#define CASE_SENSITIVE                 SQL_COLUMN_CASE_SENSITIVE
//#define CATALOG_NAME                   SQL_COLUMN_QUALIFIER_NAME
//#define CONCISE_TYPE                   SQL_COLUMN_TYPE
//#define DATETIME_INTERVAL_PRECISION    26
//#define DISPLAY_SIZE                   SQL_COLUMN_DISPLAY_SIZE
//#define FIXED_PREC_SCALE               SQL_COLUMN_MONEY
//#define LABEL                          SQL_COLUMN_LABEL
//#define LITERAL_PREFIX                 27
//#define LITERAL_SUFFIX                 28
//#define LOCAL_TYPE_NAME                29
//#define MAXIMUM_SCALE                  30
//#define MINIMUM_SCALE                  31
//#define NUM_PREC_RADIX                 32
//#define PARAMETER_TYPE                 33
//#define ROWS_PROCESSED_PTR             34
//#if (ODBCVER >= 0x0350)
//#define ROWVER                         35
//#endif /* ODBCVER >= 0x0350 */
//#define SCHEMA_NAME                    SQL_COLUMN_OWNER_NAME
//#define SEARCHABLE                     SQL_COLUMN_SEARCHABLE
//#define TYPE_NAME                      SQL_COLUMN_TYPE_NAME
//#define TABLE_NAME                     SQL_COLUMN_TABLE_NAME
//#define UNSIGNED                       SQL_COLUMN_UNSIGNED
//#define UPDATABLE                      SQL_COLUMN_UPDATABLE
//#endif /* ODBCVER >= 0x0300 */
/// TODO: Not mentioned anywhere in the documentation
/// MAXIMUM_SCALE = 30,
/// MINIMUM_SCALE = 31,
//#[cfg(feature = "v4")]
//CHARACTER_SET_CATALOG = 1018,
//#[cfg(feature = "v4")]
//CHARACTER_SET_SCHEMA = 1019,
//#[cfg(feature = "v4")]
//CHARACTER_SET_NAME = 1020,
//#[cfg(feature = "v4")]
//COLLATION_CATALOG = 1015,
//#[cfg(feature = "v4")]
//COLLATION_SCHEMA = 1016,
//#[cfg(feature = "v4")]
//COLLATION_NAME = 1017,
//#[cfg(feature = "v4")]
//USER_DEFINED_TYPE_CATALOG = 1026,
//#[cfg(feature = "v4")]
//USER_DEFINED_TYPE_SCHEMA = 1027,
//#[cfg(feature = "v4")]
//USER_DEFINED_TYPE_NAME = 1028,
//#[cfg(feature = "v4")]
//MIME_TYPE = 36,

//    pub enum ARRAY_STATUS_PTR {
//        SQL_PARAM_SUCCESS = 0,
//        SQL_PARAM_SUCCESS_WITH_INFO = 6,
//        SQL_PARAM_ERROR = 5,
//        SQL_PARAM_UNUSED = 7,
//        SQL_PARAM_DIAG_UNAVAILABLE = 1,
//        // TODO: What are these?
//        //SQL_PARAM_PROCEED = 0,
//        //SQL_PARAM_IGNORE = 1,
//    }

//=====================================================================================//

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum AllocType {
    ALLOC_AUTO = 1,
    ALLOC_USER = 2,
}

// TODO: May be i32?
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct BindType(pub u32);

pub const BIND_BY_COLUMN: BindType = BindType(0);

impl From<u32> for BindType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
