use crate::attr::{Attr, AttrGet, AttrLen, AttrSet};
use crate::c_types::ScalarCType;
use crate::env::OdbcVersion;
use crate::str::{OdbcChar, OdbcStr};
use crate::SQLLEN;
use crate::{
    handle::UnsafeSQLHDESC, Ident, OdbcBool, OdbcDefined, SQLCHAR, SQLINTEGER, SQLSMALLINT, SQLUINTEGER,
    SQLULEN, SQLWCHAR,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

impl<'buf> DescType<'buf> for AppDesc<'buf> {}
impl<T> DescType<'_> for ImplDesc<T> {}

// TODO: The statement attribute SQL_ATTR_USE_BOOKMARKS should always be set before calling SQLSetDescField to set bookmark fields. While this is not mandatory, it is strongly recommended.
pub trait DescField<A: Ident, DT>: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {
    // TODO: Implement for buffers to bind their lifetimes
    fn update_handle<V: OdbcVersion>(&self, _: &UnsafeSQLHDESC<DT, V>)
    where
        Self: AttrSet<A>,
    {
        // TODO: If an application calls SQLSetDescField to set any field other than SQL_DESC_COUNT
        // or the deferred fields SQL_DESC_DATA_PTR, SQL_DESC_OCTET_LENGTH_PTR, or SQL_DESC_INDICATOR_PTR,
        // the record becomes unbound.

        // When setting descriptor fields by calling SQLSetDescField, the application must follow a specific sequence:
        //  The application must first set the SQL_DESC_TYPE, SQL_DESC_CONCISE_TYPE, or SQL_DESC_DATETIME_INTERVAL_CODE field.
        //  After one of these fields has been set, the application can set an attribute of a data type, and the driver sets data type attribute fields to the appropriate default values for the data type. Automatic defaulting of type attribute fields ensures that the descriptor is always ready to use once the application has specified a data type. If the application explicitly sets a data type attribute, it is overriding the default attribute.
        //  After one of the fields listed in step 1 has been set, and data type attributes have been set, the application can set SQL_DESC_DATA_PTR. This prompts a consistency check of descriptor fields. If the application changes the data type or attributes after setting the SQL_DESC_DATA_PTR field, the driver sets SQL_DESC_DATA_PTR to a null pointer, unbinding the record. This forces the application to complete the proper steps in sequence, before the descriptor record is usable.
    }
}

// TODO: It's unclear if this trait is required because
// of column lifetime binding or it can be removed
pub trait DescType<'buf> {}

#[derive(Debug)]
pub struct AppDesc<'buf> {
    pub(crate) rows_processed: PhantomData<&'buf ()>,
    pub(crate) data_ptrs: PhantomData<&'buf ()>,
}

#[derive(Debug)]
pub struct ImplDesc<T> {
    desc_type: PhantomData<T>,
}

#[derive(Debug)]
pub enum IRD {}
#[derive(Debug)]
pub enum IPD {}

// Implement DescField for uninitialized descriptor fields
impl<A: Ident, T: Ident, DT> DescField<A, DT> for MaybeUninit<T>
where
    T: DescField<A, DT>,
    Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<A: Ident, DT> DescField<A, DT> for OdbcStr<MaybeUninit<SQLCHAR>>
where
    OdbcStr<SQLCHAR>: DescField<A, DT>,
    Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<A: Ident, DT> DescField<A, DT> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: DescField<A, DT>
{
}

// Implement DescField for references to character descriptor fields (used by AttrSet)
impl<A: Ident, DT, CH: OdbcChar> DescField<A, DT> for &OdbcStr<CH> where
    OdbcStr<CH>: DescField<A, DT>
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Header fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1099)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ALLOC_TYPE;
// This is read-only attribute
unsafe impl Attr<SQL_DESC_ALLOC_TYPE> for AllocType {
    type DefinedBy = OdbcDefined;
}
impl<DT> DescField<SQL_DESC_ALLOC_TYPE, DT> for AllocType {}
unsafe impl AttrGet<SQL_DESC_ALLOC_TYPE> for AllocType {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 20)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ARRAY_SIZE;
unsafe impl Attr<SQL_DESC_ARRAY_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_ARRAY_SIZE, AppDesc<'_>> for SQLULEN {}
unsafe impl AttrGet<SQL_DESC_ARRAY_SIZE> for SQLULEN {}
unsafe impl AttrSet<SQL_DESC_ARRAY_SIZE> for SQLULEN {}

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 21)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_ARRAY_STATUS_PTR;
//unsafe impl Attr<SQL_DESC_ARRAY_STATUS_PTR> for [UnsafeCell<>] {
//    type DefinedBy = OdbcDefined;
//}
//impl<DT> DescField<SQL_DESC_ARRAY_STATUS_PTR, DT> for [UnsafeCell<>] {
//    fn update_handle<V: OdbcVersion>(&self, _: &UnsafeSQLHDESC<DT, V>) where Self: AttrSet<A> {
//        // TODO: Do something
//    }
//}
//unsafe impl AttrGet<SQL_DESC_ARRAY_STATUS_PTR> for [UnsafeCell<>] {}
//unsafe impl AttrSet<SQL_DESC_ARRAY_STATUS_PTR> for &[UnsafeCell<>] {}

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 24)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_BIND_OFFSET_PTR;
//unsafe impl Attr<SQL_DESC_BIND_OFFSET_PTR> for UnsafeCell<SQLLEN> {
//    type DefinedBy = OdbcDefined;
//}
//impl DescField<SQL_DESC_BIND_OFFSET_PTR, AppDesc<'_>> for UnsafeCell<SQLLEN> {
//    fn update_handle<V: OdbcVersion>(&self, DescriptorHandle: &UnsafeSQLHDESC<AppDesc<'_>, V>)
//    where
//        Self: AttrSet<A>,
//    {
//        handle.bind_offset.set(*self);
//    }
//}
//unsafe impl AttrGet<SQL_DESC_BIND_OFFSET_PTR> for UnsafeCell<SQLLEN> {}
//unsafe impl AttrSet<SQL_DESC_BIND_OFFSET_PTR> for &UnsafeCell<SQLLEN> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 25)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BIND_TYPE;
unsafe impl Attr<SQL_DESC_BIND_TYPE> for BindType {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_BIND_TYPE, AppDesc<'_>> for BindType {}
unsafe impl AttrGet<SQL_DESC_BIND_TYPE> for BindType {}
unsafe impl AttrSet<SQL_DESC_BIND_TYPE> for BindType {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1001)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_COUNT;
unsafe impl Attr<SQL_DESC_COUNT> for SQLSMALLINT {
    type DefinedBy = OdbcDefined;
}
impl<DT> DescField<SQL_DESC_COUNT, DT> for SQLSMALLINT {}
unsafe impl AttrGet<SQL_DESC_COUNT> for SQLSMALLINT {}
unsafe impl AttrSet<SQL_DESC_COUNT> for SQLSMALLINT {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 34)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ROWS_PROCESSED_PTR;
unsafe impl Attr<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLUINTEGER>] {
    type DefinedBy = OdbcDefined;
}
unsafe impl Attr<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLULEN>] {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_ROWS_PROCESSED_PTR, ImplDesc<IRD>> for [UnsafeCell<SQLULEN>] {
    #[cfg(feature = "odbc_debug")]
    fn update_handle<V: OdbcVersion>(&self, DescriptorHandle: &UnsafeSQLHDESC<ImplDesc<IRD>, V>) {
        unimplemented!()
    }
}
impl DescField<SQL_DESC_ROWS_PROCESSED_PTR, ImplDesc<IPD>> for [UnsafeCell<SQLUINTEGER>] {
    #[cfg(feature = "odbc_debug")]
    fn update_handle<V: OdbcVersion>(&self, DescriptorHandle: &UnsafeSQLHDESC<ImplDesc<IPD>, V>) {
        unimplemented!()
    }
}
unsafe impl AttrGet<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLUINTEGER>] {}
unsafe impl AttrGet<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLULEN>] {}
unsafe impl AttrSet<SQL_DESC_ROWS_PROCESSED_PTR> for &[UnsafeCell<SQLUINTEGER>] {}
unsafe impl AttrSet<SQL_DESC_ROWS_PROCESSED_PTR> for &[UnsafeCell<SQLULEN>] {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Record fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, 11)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;
unsafe impl Attr<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_AUTO_UNIQUE_VALUE, ImplDesc<IRD>> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 22)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_BASE_COLUMN_NAME;
unsafe impl<CH: OdbcChar> Attr<SQL_DESC_BASE_COLUMN_NAME> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> DescField<SQL_DESC_BASE_COLUMN_NAME, ImplDesc<IRD>> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_DESC_BASE_COLUMN_NAME> for OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 23)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_BASE_TABLE_NAME;
unsafe impl<CH: OdbcChar> Attr<SQL_DESC_BASE_TABLE_NAME> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> DescField<SQL_DESC_BASE_TABLE_NAME, ImplDesc<IRD>> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_DESC_BASE_TABLE_NAME> for OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 12)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_CASE_SENSITIVE;
unsafe impl Attr<SQL_DESC_CASE_SENSITIVE> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_CASE_SENSITIVE, ImplDesc<IRD>> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_CASE_SENSITIVE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 17)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_CATALOG_NAME;
unsafe impl<CH: OdbcChar> Attr<SQL_DESC_CATALOG_NAME> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> DescField<SQL_DESC_CATALOG_NAME, ImplDesc<IRD>> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_DESC_CATALOG_NAME> for OdbcStr<CH> {}

// TODO:
//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 2)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_CONCISE_TYPE;
//unsafe impl Attr<SQL_DESC_CONCISE_TYPE> for SqlType {
//    type DefinedBy = OdbcDefined;
//}
//impl<DT> DescField<SQL_DESC_CONCISE_TYPE, DT> for SqlType {}
//unsafe impl AttrGet<SQL_DESC_CONCISE_TYPE> for SqlType {}
//unsafe impl AttrSet<SQL_DESC_CONCISE_TYPE> for SqlType {}

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 1007)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_DATETIME_INTERVAL_CODE;
//unsafe impl Attr<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {
//    type DefinedBy = OdbcDefined;
//}
//impl<DT> DescField<SQL_DESC_DATETIME_INTERVAL_CODE, DT> for DatetimeIntervalCode {}
//unsafe impl AttrGet<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {}
//unsafe impl AttrSet<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {}

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 26)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_DATETIME_INTERVAL_PRECISION;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1010)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DATA_PTR;
unsafe impl<T> Attr<SQL_DESC_DATA_PTR> for UnsafeCell<T> {
    type DefinedBy = OdbcDefined;
}
impl<T: ScalarCType> DescField<SQL_DESC_DATA_PTR, ImplDesc<IPD>> for UnsafeCell<T> {}
impl<T: ScalarCType> DescField<SQL_DESC_DATA_PTR, AppDesc<'_>> for UnsafeCell<T> {}
unsafe impl<T: ScalarCType> AttrGet<SQL_DESC_DATA_PTR> for UnsafeCell<T> {}
unsafe impl<T: ScalarCType> AttrSet<SQL_DESC_DATA_PTR> for &UnsafeCell<T> {}

unsafe impl<T> Attr<SQL_DESC_DATA_PTR> for [UnsafeCell<T>] {
    type DefinedBy = OdbcDefined;
}
impl<T, DT> DescField<SQL_DESC_DATA_PTR, DT> for [UnsafeCell<T>] {}
unsafe impl<T> AttrGet<SQL_DESC_DATA_PTR> for [UnsafeCell<T>] {}
unsafe impl<T> AttrSet<SQL_DESC_DATA_PTR> for &[UnsafeCell<T>] {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 6)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_DISPLAY_SIZE;
unsafe impl Attr<SQL_DESC_DISPLAY_SIZE> for SQLINTEGER {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_DISPLAY_SIZE, ImplDesc<IRD>> for SQLINTEGER {}
unsafe impl AttrGet<SQL_DESC_DISPLAY_SIZE> for SQLINTEGER {}

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 1009)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_INDICATOR_PTR;
//unsafe impl Attr<SQL_DESC_DISPLAY_SIZE> for {
//    type DefinedBy = OdbcDefined;
//}
//impl DescField<SQL_DESC_DISPLAY_SIZE, AppDesc<'_>> for {}
//unsafe impl AttrGet<SQL_DESC_DISPLAY_SIZE> for {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 9)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_DESC_FIXED_PREC_SCALE;
unsafe impl Attr<SQL_DESC_FIXED_PREC_SCALE> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<T> DescField<SQL_DESC_FIXED_PREC_SCALE, ImplDesc<T>> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_FIXED_PREC_SCALE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1004)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH_PTR;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 33)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PARAMETER_TYPE;

#[cfg(feature = "v3_5")]
#[derive(Ident)]
#[identifier(SQLSMALLINT, 35)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ROWVER;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1012)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNNAMED;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1002)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1003)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LENGTH;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1005)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PRECISION;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1006)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCALE;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1008)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NULLABLE;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1011)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NAME;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1013)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH;
unsafe impl Attr<SQL_DESC_OCTET_LENGTH> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<DT> DescField<SQL_DESC_OCTET_LENGTH, DT> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_OCTET_LENGTH> for OdbcBool {}

//#if (ODBCVER >= 0x0300)
//#define SQL_DESC_ARRAY_SIZE                     20
//#define SQL_DESC_AUTO_UNIQUE_VALUE              SQL_COLUMN_AUTO_INCREMENT
//#define SQL_DESC_BASE_COLUMN_NAME               22
//#define SQL_DESC_BASE_TABLE_NAME                23
//#define SQL_DESC_BIND_OFFSET_PTR                24
//#define SQL_DESC_BIND_TYPE                      25
//#define SQL_DESC_CASE_SENSITIVE                 SQL_COLUMN_CASE_SENSITIVE
//#define SQL_DESC_CATALOG_NAME                   SQL_COLUMN_QUALIFIER_NAME
//#define SQL_DESC_CONCISE_TYPE                   SQL_COLUMN_TYPE
//#define SQL_DESC_DATETIME_INTERVAL_PRECISION    26
//#define SQL_DESC_DISPLAY_SIZE                   SQL_COLUMN_DISPLAY_SIZE
//#define SQL_DESC_FIXED_PREC_SCALE               SQL_COLUMN_MONEY
//#define SQL_DESC_LABEL                          SQL_COLUMN_LABEL
//#define SQL_DESC_LITERAL_PREFIX                 27
//#define SQL_DESC_LITERAL_SUFFIX                 28
//#define SQL_DESC_LOCAL_TYPE_NAME                29
//#define SQL_DESC_MAXIMUM_SCALE                  30
//#define SQL_DESC_MINIMUM_SCALE                  31
//#define SQL_DESC_NUM_PREC_RADIX                 32
//#define SQL_DESC_PARAMETER_TYPE                 33
//#define SQL_DESC_ROWS_PROCESSED_PTR             34
//#if (ODBCVER >= 0x0350)
//#define SQL_DESC_ROWVER                         35
//#endif /* ODBCVER >= 0x0350 */
//#define SQL_DESC_SCHEMA_NAME                    SQL_COLUMN_OWNER_NAME
//#define SQL_DESC_SEARCHABLE                     SQL_COLUMN_SEARCHABLE
//#define SQL_DESC_TYPE_NAME                      SQL_COLUMN_TYPE_NAME
//#define SQL_DESC_TABLE_NAME                     SQL_COLUMN_TABLE_NAME
//#define SQL_DESC_UNSIGNED                       SQL_COLUMN_UNSIGNED
//#define SQL_DESC_UPDATABLE                      SQL_COLUMN_UPDATABLE
//#endif /* ODBCVER >= 0x0300 */
//// TODO: Not mentioned anywhere in the documentation
//// SQL_DESC_MAXIMUM_SCALE = 30,
//// SQL_DESC_MINIMUM_SCALE = 31,
//#[cfg(feature = "v4")]
//SQL_DESC_CHARACTER_SET_CATALOG = 1018,
//#[cfg(feature = "v4")]
//SQL_DESC_CHARACTER_SET_SCHEMA = 1019,
//#[cfg(feature = "v4")]
//SQL_DESC_CHARACTER_SET_NAME = 1020,
//#[cfg(feature = "v4")]
//SQL_DESC_COLLATION_CATALOG = 1015,
//#[cfg(feature = "v4")]
//SQL_DESC_COLLATION_SCHEMA = 1016,
//#[cfg(feature = "v4")]
//SQL_DESC_COLLATION_NAME = 1017,
//#[cfg(feature = "v4")]
//SQL_DESC_USER_DEFINED_TYPE_CATALOG = 1026,
//#[cfg(feature = "v4")]
//SQL_DESC_USER_DEFINED_TYPE_SCHEMA = 1027,
//#[cfg(feature = "v4")]
//SQL_DESC_USER_DEFINED_TYPE_NAME = 1028,
//#[cfg(feature = "v4")]
//SQL_DESC_MIME_TYPE = 36,

//    pub enum SQL_DESC_ARRAY_STATUS_PTR {
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

#[odbc_type(SQLSMALLINT)]
pub struct AllocType;
pub const SQL_DESC_ALLOC_AUTO: AllocType = AllocType(1);
pub const SQL_DESC_ALLOC_USER: AllocType = AllocType(2);

// TODO: May be SQLINTEGER?
#[odbc_type(SQLUINTEGER)]
pub struct BindType;
pub const SQL_BIND_BY_COLUMN: BindType = BindType(1);
