use crate::env::OdbcVersion;
use crate::handle::{AppDesc, ImplDesc, ParamDesc, RowDesc};
use crate::{
    handle::SQLHDESC, Attr, AttrGet, AttrLen, AttrSet, Ident, IntoSQLPOINTER, OdbcBool,
    OdbcDefined, OdbcStr, SQLCHAR, SQLINTEGER, SQLLEN, SQLSMALLINT, SQLUINTEGER, SQLULEN,
    SQLUSMALLINT,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::cell::UnsafeCell;

// TODO: The statement attribute SQL_ATTR_USE_BOOKMARKS should always be set before calling SQLSetDescField to set bookmark fields. While this is not mandatory, it is strongly recommended.
pub trait DescField<A: crate::Ident, DT>: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {
    // TODO: Implement for buffers to bind their lifetimes
    fn update_handle<V: OdbcVersion>(&self, _: &SQLHDESC<DT, V>)
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

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Header fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1099)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ALLOC_TYPE;
//// This is read-only attribute
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
//unsafe impl Attr<SQL_DESC_ARRAY_STATUS_PTR> for [UnsafeCell<SQLUSMALLINT>] {
//    type DefinedBy = OdbcDefined;
//}
//impl<DT> DescField<SQL_DESC_ARRAY_STATUS_PTR, DT> for [UnsafeCell<SQLUSMALLINT>] {}
//unsafe impl AttrGet<SQL_DESC_ARRAY_STATUS_PTR> for [UnsafeCell<SQLUSMALLINT>] {}
//unsafe impl AttrSet<SQL_DESC_ARRAY_STATUS_PTR> for &[UnsafeCell<SQLUSMALLINT>] {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 24)]
#[allow(non_camel_case_types)]
#[cfg(feature = "raw_api")]
pub struct SQL_DESC_BIND_OFFSET_PTR;

#[cfg(feature = "raw_api")]
unsafe impl Attr<SQL_DESC_BIND_OFFSET_PTR> for UnsafeCell<SQLLEN> {
    type DefinedBy = OdbcDefined;
}

#[cfg(feature = "raw_api")]
impl DescField<SQL_DESC_BIND_OFFSET_PTR, AppDesc<'_>> for UnsafeCell<SQLLEN> {}

#[cfg(feature = "raw_api")]
unsafe impl AttrGet<SQL_DESC_BIND_OFFSET_PTR> for UnsafeCell<SQLLEN> {}

#[cfg(feature = "raw_api")]
unsafe impl AttrSet<SQL_DESC_BIND_OFFSET_PTR> for UnsafeCell<SQLLEN> {}

// TODO: This is actually integer type
//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 25)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_BIND_TYPE;
//unsafe impl Attr<SQL_DESC_BIND_TYPE> for BindType {
//    type DefinedBy = OdbcDefined;
//}
//impl DescField<SQL_DESC_BIND_TYPE, AppDesc<'_>> for BindType {}
//unsafe impl AttrGet<SQL_DESC_BIND_TYPE> for BindType {}
//unsafe impl AttrSet<SQL_DESC_BIND_TYPE> for BindType {}

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

//#[derive(Ident)]
//#[identifier(SQLSMALLINT, 34)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_ROWS_PROCESSED_PTR;
//unsafe impl Attr<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLUINTEGER>] {
//    type DefinedBy = OdbcDefined;
//}
//unsafe impl Attr<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLULEN>] {
//    type DefinedBy = OdbcDefined;
//}
//impl DescField<SQL_DESC_ROWS_PROCESSED_PTR, ImplDesc<ParamDesc>> for [UnsafeCell<SQLUINTEGER>] {}
//impl DescField<SQL_DESC_ROWS_PROCESSED_PTR, ImplDesc<RowDesc>> for [UnsafeCell<SQLULEN>] {}
//unsafe impl AttrGet<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLUINTEGER>] {}
//unsafe impl AttrGet<SQL_DESC_ROWS_PROCESSED_PTR> for [UnsafeCell<SQLULEN>] {}
//unsafe impl AttrSet<SQL_DESC_ROWS_PROCESSED_PTR> for &[UnsafeCell<SQLUINTEGER>] {}
//unsafe impl AttrSet<SQL_DESC_ROWS_PROCESSED_PTR> for &[UnsafeCell<SQLULEN>] {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Record fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, 11)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;
unsafe impl Attr<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_AUTO_UNIQUE_VALUE, ImplDesc<RowDesc>> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 22)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_BASE_COLUMN_NAME;
unsafe impl Attr<SQL_DESC_BASE_COLUMN_NAME> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_BASE_COLUMN_NAME, ImplDesc<RowDesc>> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DESC_BASE_COLUMN_NAME> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 23)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_BASE_TABLE_NAME;
unsafe impl Attr<SQL_DESC_BASE_TABLE_NAME> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_BASE_TABLE_NAME, ImplDesc<RowDesc>> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DESC_BASE_TABLE_NAME> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 12)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_CASE_SENSITIVE;
unsafe impl Attr<SQL_DESC_CASE_SENSITIVE> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_CASE_SENSITIVE, ImplDesc<RowDesc>> for OdbcBool {}
unsafe impl AttrGet<SQL_DESC_CASE_SENSITIVE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 17)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_CATALOG_NAME;
unsafe impl Attr<SQL_DESC_CATALOG_NAME> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl DescField<SQL_DESC_CATALOG_NAME, ImplDesc<RowDesc>> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DESC_CATALOG_NAME> for OdbcStr<SQLCHAR> {}

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
//#[identifier(SQLSMALLINT, 1010)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_DATA_PTR;
//unsafe impl Attr<SQL_DESC_DATA_PTR> for  {
//    type DefinedBy = OdbcDefined;
//}
//impl<DT> DescField<SQL_DESC_DATA_PTR, DT> for  {}
//unsafe impl AttrGet<SQL_DESC_DATA_PTR> for SQLPOINTER {}
//unsafe impl AttrSet<SQL_DESC_DATA_PTR> for  {}

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

#[derive(Ident)]
#[identifier(SQLSMALLINT, 26)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DATETIME_INTERVAL_PRECISION;

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1009)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_INDICATOR_PTR;

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
pub struct nQL_DESC_UNNAMED;

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

//    pub enum SQL_DESC_ALLOC_TYPE {
//        SQL_DESC_ALLOC_AUTO = 1,
//        SQL_DESC_ALLOC_USER = 2,
//    }
//
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

#[odbc_type(SQLUINTEGER)]
pub struct BindType;
pub const SQL_BIND_BY_COLUMN: BindType = BindType(1);
pub use SQL_BIND_BY_COLUMN as SQL_BIND_TYPE_DEFAULT;
