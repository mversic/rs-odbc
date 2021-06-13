use crate::handle::{AppDesc, ImplDesc, RowDesc};
use crate::{
    handle::SQLHDESC, Attr, AttrLen, AttrRead, AttrWrite, Ident, IntoSQLPOINTER, OdbcBool,
    OdbcDefined, True, SQLCHAR, SQLINTEGER, SQLLEN, SQLSMALLINT, SQLULEN, SQLUSMALLINT,
};
use rs_odbc_derive::{odbc_type, Ident};

pub trait WriteDescField<DT, A: Ident>:
    Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER> + IntoSQLPOINTER
{
    // TODO: Implement for buffers to bind their lifetimes
    fn update_handle(&self, _: &SQLHDESC<DT>) {
        // TODO: If an application calls SQLSetDescField to set any field other than SQL_DESC_COUNT
        // or the deferred fields SQL_DESC_DATA_PTR, SQL_DESC_OCTET_LENGTH_PTR, or SQL_DESC_INDICATOR_PTR,
        // the record becomes unbound.

        // When setting descriptor fields by calling SQLSetDescField, the application must follow a specific sequence:
        //  The application must first set the SQL_DESC_TYPE, SQL_DESC_CONCISE_TYPE, or SQL_DESC_DATETIME_INTERVAL_CODE field.
        //  After one of these fields has been set, the application can set an attribute of a data type, and the driver sets data type attribute fields to the appropriate default values for the data type. Automatic defaulting of type attribute fields ensures that the descriptor is always ready to use once the application has specified a data type. If the application explicitly sets a data type attribute, it is overriding the default attribute.
        //  After one of the fields listed in step 1 has been set, and data type attributes have been set, the application can set SQL_DESC_DATA_PTR. This prompts a consistency check of descriptor fields. If the application changes the data type or attributes after setting the SQL_DESC_DATA_PTR field, the driver sets SQL_DESC_DATA_PTR to a null pointer, unbinding the record. This forces the application to complete the proper steps in sequence, before the descriptor record is usable.
    }
}

// TODO: The statement attribute SQL_ATTR_USE_BOOKMARKS should always be set before calling SQLSetDescField to set bookmark fields. While this is not mandatory, it is strongly recommended.
pub trait DescField<DT, A: crate::Ident>:
    Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>
{
}

// TODO:
//impl<'a, H: Handle, D: Ident> DescField<H, D> for &'a [SQLWCHAR]
//where
//    &'a [SQLCHAR]: DescField<H, D>,
//    // TODO: This seems superflous
//    &'a [SQLWCHAR]: AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
//{
//}
//
//impl<H: Handle, D: Ident, T: Ident> DescField<H, D> for std::mem::MaybeUninit<T>
//where
//    T: DescField<H, D>,
//    Self: Attr<D> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
//{
//}
//impl<'a, H: Handle, D: Ident, T> DescField<H, D> for &'a [std::mem::MaybeUninit<T>]
//where
//    &'a [T]: DescField<H, D>,
//    Self: Attr<D> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
//{
//}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Header fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLINTEGER, 1099)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ALLOC_TYPE;
//// This is read-only attribute
unsafe impl Attr<SQL_DESC_ALLOC_TYPE> for AllocType {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<DT> DescField<DT, SQL_DESC_ALLOC_TYPE> for AllocType {}
unsafe impl AttrRead<SQL_DESC_ALLOC_TYPE> for AllocType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 20)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ARRAY_SIZE;
unsafe impl Attr<SQL_DESC_ARRAY_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<AppDesc<'_>, SQL_DESC_ARRAY_SIZE> for SQLULEN {}
unsafe impl AttrRead<SQL_DESC_ARRAY_SIZE> for SQLULEN {}
unsafe impl AttrWrite<SQL_DESC_ARRAY_SIZE> for SQLULEN {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 21)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_ARRAY_STATUS_PTR;
//unsafe impl<DT> Attr<DT, SQL_DESC_ARRAY_STATUS_PTR> for [SQLUSMALLINT] {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl DescField<SQL_DESC_ARRAY_STATUS_PTR> for [SQLUSMALLINT] {}
// TODO: This field can be made set-only
//unsafe impl AttrRead<SQL_DESC_ARRAY_STATUS_PTR> for [SQLUSMALLINT] {}
//unsafe impl AttrWrite<SQL_DESC_ARRAY_STATUS_PTR> for [SQLUSMALLINT] {}

// TODO: How can I support this. This is very unsafe
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 24)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_BIND_OFFSET_PTR;
//unsafe impl Attr<AppDesc<'_>, SQL_DESC_BIND_OFFSET_PTR> for [SQLLEN] {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl<DT> DescField<DT, SQL_DESC_BIND_OFFSET_PTR> for [SQLLEN] {}
//unsafe impl AttrRead<SQL_DESC_BIND_OFFSET_PTR> for [SQLLEN] {}
//unsafe impl AttrWrite<SQL_DESC_BIND_OFFSET_PTR> for [SQLLEN] {}

// TODO: This is actually integer type
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 25)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_BIND_TYPE;
//unsafe impl Attr<SQL_DESC_BIND_TYPE> for BindType {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl DescField<AppDesc<'_>, SQL_DESC_BIND_TYPE> for BindType {}
//unsafe impl AttrRead<SQL_DESC_BIND_TYPE> for BindType {}
//unsafe impl AttrWrite<SQL_DESC_BIND_TYPE> for BindType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 1001)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_COUNT;
unsafe impl Attr<SQL_DESC_COUNT> for SQLSMALLINT {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<DT> DescField<DT, SQL_DESC_COUNT> for SQLSMALLINT {}
unsafe impl AttrRead<SQL_DESC_COUNT> for SQLSMALLINT {}
unsafe impl AttrWrite<SQL_DESC_COUNT> for SQLSMALLINT {}

// TODO: Can be both *SQLUINTEGER or *SQLULEN
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 34)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_ROWS_PROCESSED_PTR;
//unsafe impl Attr<SQL_DESC_ROWS_PROCESSED_PTR> for  {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl DescField<ImplDesc, SQL_DESC_ROWS_PROCESSED_PTR> for  {}
//unsafe impl AttrRead<SQL_DESC_ROWS_PROCESSED_PTR> for  {}
//unsafe impl AttrWrite<SQL_DESC_ROWS_PROCESSED_PTR> for  {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Record fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;
unsafe impl Attr<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<ImplDesc<RowDesc>, SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {}
unsafe impl AttrRead<SQL_DESC_AUTO_UNIQUE_VALUE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 22)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_BASE_COLUMN_NAME;
unsafe impl Attr<SQL_DESC_BASE_COLUMN_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<ImplDesc<RowDesc>, SQL_DESC_BASE_COLUMN_NAME> for [SQLCHAR] {}
unsafe impl AttrRead<SQL_DESC_BASE_COLUMN_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 23)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_BASE_TABLE_NAME;
unsafe impl Attr<SQL_DESC_BASE_TABLE_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<ImplDesc<RowDesc>, SQL_DESC_BASE_TABLE_NAME> for [SQLCHAR] {}
unsafe impl AttrRead<SQL_DESC_BASE_TABLE_NAME> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_CASE_SENSITIVE;
unsafe impl Attr<SQL_DESC_CASE_SENSITIVE> for OdbcBool {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<ImplDesc<RowDesc>, SQL_DESC_CASE_SENSITIVE> for OdbcBool {}
unsafe impl AttrRead<SQL_DESC_CASE_SENSITIVE> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 17)]
#[allow(non_camel_case_types)]
//// This is read-only attribute
pub struct SQL_DESC_CATALOG_NAME;
unsafe impl Attr<SQL_DESC_CATALOG_NAME> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl DescField<ImplDesc<RowDesc>, SQL_DESC_CATALOG_NAME> for [SQLCHAR] {}
unsafe impl AttrRead<SQL_DESC_CATALOG_NAME> for [SQLCHAR] {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 2)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_CONCISE_TYPE;
//unsafe impl Attr<SQL_DESC_CONCISE_TYPE> for SqlType {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl<DT> DescField<DT, SQL_DESC_CONCISE_TYPE> for SqlType {}
//unsafe impl AttrRead<SQL_DESC_CONCISE_TYPE> for SqlType {}
//unsafe impl AttrWrite<SQL_DESC_CONCISE_TYPE> for SqlType {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 1010)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_DATA_PTR;
//unsafe impl Attr<SQL_DESC_DATA_PTR> for  {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl<DT> DescField<DT, SQL_DESC_DATA_PTR> for  {}
//unsafe impl AttrRead<SQL_DESC_DATA_PTR> for SQLPOINTER {}
//unsafe impl AttrWrite<SQL_DESC_DATA_PTR> for  {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 1007)]
//#[allow(non_camel_case_types)]
//pub struct SQL_DESC_DATETIME_INTERVAL_CODE;
//unsafe impl Attr<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl<DT> DescField<DT, SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {}
//unsafe impl AttrRead<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {}
//unsafe impl AttrWrite<SQL_DESC_DATETIME_INTERVAL_CODE> for DatetimeIntervalCode {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 26)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DATETIME_INTERVAL_PRECISION;

#[derive(Ident)]
#[identifier(SQLINTEGER, 1009)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_INDICATOR_PTR;

#[derive(Ident)]
#[identifier(SQLINTEGER, 1004)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH_PTR;

#[derive(Ident)]
#[identifier(SQLINTEGER, 33)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PARAMETER_TYPE;

#[cfg(feature = "v3_5")]
#[derive(Ident)]
#[identifier(SQLINTEGER, 35)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_ROWVER;

#[derive(Ident)]
#[identifier(SQLINTEGER, 1012)]
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
