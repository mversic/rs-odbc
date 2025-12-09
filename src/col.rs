use crate::attr::{Attr, AttrGet, AttrLen, AttrSet};
use crate::env::{OdbcVersion, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::str::{OdbcChar, OdbcStr};
use crate::{Ident, OdbcDefined, SQLCHAR, SQLLEN, SQLSMALLINT, SQLWCHAR, Scalar};
use core::mem::MaybeUninit;
use rs_odbc_derive::Ident;

pub trait ColAttr<A: Ident, V: OdbcVersion>:
    Attr<A> + AttrLen<Self::DefinedBy, SQLSMALLINT>
{
}

// Implement ColAttr for all versions of column attributes
impl<A: Ident, T: Scalar> ColAttr<A, SQL_OV_ODBC3_80> for T where
    T: ColAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> ColAttr<A, SQL_OV_ODBC4> for T where
    T: ColAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> ColAttr<A, SQL_OV_ODBC3_80> for [T] where
    [T]: ColAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> ColAttr<A, SQL_OV_ODBC4> for [T] where
    [T]: ColAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, CH: OdbcChar> ColAttr<A, SQL_OV_ODBC3_80> for OdbcStr<CH> where
    OdbcStr<CH>: ColAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, CH: OdbcChar> ColAttr<A, SQL_OV_ODBC4> for OdbcStr<CH> where
    OdbcStr<CH>: ColAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}

// Implement ColAttr for uninitialized column attributes
impl<A: Ident, T: Scalar, V: OdbcVersion> ColAttr<A, V> for MaybeUninit<T>
where
    T: ColAttr<A, V> + AttrGet<A>,
    Self: AttrLen<Self::DefinedBy, SQLSMALLINT>,
{
}
impl<A: Ident, T: Scalar, V: OdbcVersion> ColAttr<A, V> for [MaybeUninit<T>]
where
    [T]: ColAttr<A, V> + AttrGet<A>,
    Self: AttrLen<Self::DefinedBy, SQLSMALLINT>,
{
}
impl<A: Ident, V: OdbcVersion> ColAttr<A, V> for OdbcStr<MaybeUninit<SQLCHAR>> where
    OdbcStr<SQLCHAR>: ColAttr<A, V> + AttrGet<A>
{
}
impl<A: Ident, V: OdbcVersion> ColAttr<A, V> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: ColAttr<A, V> + AttrGet<A>
{
}

// Implement ColAttr for references to unsized (used by AttrSet)
impl<A: Ident, T: Scalar, V: OdbcVersion> ColAttr<A, V> for &[T]
where
    [T]: ColAttr<A, V>,
    Self: AttrSet<A>,
{
}
impl<A: Ident, CH: OdbcChar, V: OdbcVersion> ColAttr<A, V> for &OdbcStr<CH>
where
    OdbcStr<CH>: ColAttr<A, V>,
    Self: AttrSet<A>,
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// TODO: These seem to be from v2.0
//#[deprecated]
//#[expect(non_camel_case_types)]
//enum SQLColAttrIdents {
//    SQL_COLUMN_COUNT = 0,
//    SQL_COLUMN_NAME = 1,
//    SQL_COLUMN_LENGTH = 3,
//    SQL_COLUMN_PRECISION = 4,
//    SQL_COLUMN_SCALE = 5,
//    SQL_COLUMN_NULLABLE = 7,
//}

// TODO: These constants are not found in the documentation
//use SQLColAttrIdents::SQL_COLUMN_COUNT as SQL_COLATT_OPT_MIN;
//use SQLColAttrIdents::SQL_COLUMN_LABEL as SQL_COLATT_OPT_MAX;

// This is the only header field, others are record fields
#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1001)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_COUNT;
unsafe impl Attr<SQL_DESC_COUNT> for SQLLEN {
    type DefinedBy = OdbcDefined;
}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 2)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_CONCISE_TYPE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 6)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_DISPLAY_SIZE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 8)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_UNSIGNED;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 9)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_FIXED_PREC_SCALE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 10)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_UPDATABLE;

///// Describes the updatability of the column in the result set, not the column in the base table.
//#[repr(SQLSMALLINT)]
//pub enum DescUpdatable {
//    SQL_ATTR_READONLY = 0,
//    SQL_ATTR_WRITE = 1,
//    /// It is unclear whether a column is updatable
//    SQL_ATTR_READWRITE_UNKNOWN = 2,
//}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 11)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 12)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_CASE_SENSITIVE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 13)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_SEARCHABLE;
// TODO:
// SQLIdents subdefines for SQL_COLUMN_SEARCHABLE These are also used by SQLGetInfo
//pub enum SQL_COLUMN_SEARCHABLE {
//    SQL_UNSEARCHABLE = 0,
//    SQL_LIKE_ONLY = 1,
//    SQL_ALL_EXCEPT_LIKE = 2,
//    SQL_SEARCHABLE = 3,
//}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 14)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_TYPE_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 15)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_TABLE_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 16)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_SCHEMA_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 17)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_CATALOG_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 18)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_LABEL;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 22)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_BASE_COLUMN_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 23)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_BASE_TABLE_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 27)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_PREFIX;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 28)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_SUFFIX;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 29)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_LOCAL_TYPE_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 32)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_NUM_PREC_RADIX;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1002)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_TYPE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1003)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_LENGTH;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1005)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_PRECISION;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1006)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_SCALE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1008)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_NULLABLE;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1011)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_NAME;

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1012)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_UNNAMED;

//#[repr(SQLSMALLINT)]
//pub enum DescUnnamed {
//    /// SQL_DESC_NAME field of the IRD contains a column alias or a column name
//    SQL_NAMED = 0,
//    /// There is no column name or column alias
//    SQL_UNNAMED = 1,
//}

#[derive(Ident)]
#[identifier(SQLUSMALLINT, 1013)]
#[expect(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH;

// TODO: These are unknown, find their values
// SQL_DESC_NUM_PREC_RADIX, SQL_DESC_CONCISE_TYPE, SQL_DESC_TYPE
