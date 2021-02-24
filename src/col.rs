use crate::{ReadAttr, SQLSMALLINT, SQLUSMALLINT};
use rs_odbc_derive::{ColAttr, Identifier};
use std::mem::MaybeUninit;

pub trait ColAttr: crate::Identifier<IdentType = SQLUSMALLINT> {
    type AttrType;
}

// TODO: These seem to be from v2.0
//#[deprecated]
//#[allow(non_camel_case_types)]
//enum SQLColAttrs {
//    SQL_COLUMN_COUNT = 0,
//    SQL_COLUMN_NAME = 1,
//    SQL_COLUMN_LENGTH = 3,
//    SQL_COLUMN_PRECISION = 4,
//    SQL_COLUMN_SCALE = 5,
//    SQL_COLUMN_NULLABLE = 7,
//}

// TODO: These constants are not found in the documentation
//use SQLColAttrs::SQL_COLUMN_COUNT as SQL_COLATT_OPT_MIN;
//use SQLColAttrs::SQL_COLUMN_LABEL as SQL_COLATT_OPT_MAX;

// This is the only header field, others are record fields
#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1001)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_COUNT;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CONCISE_TYPE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DISPLAY_SIZE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNSIGNED;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 9)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_FIXED_PREC_SCALE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UPDATABLE;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLSMALLINT>> for SQL_DESC_UPDATABLE {}

///// Describes the updatability of the column in the result set, not the column in the base table.
//#[repr(SQLSMALLINT)]
//pub enum DescUpdatable {
//    SQL_ATTR_READONLY = 0,
//    SQL_ATTR_WRITE = 1,
//    /// It is unclear whether a column is updatable
//    SQL_ATTR_READWRITE_UNKNOWN = 2,
//}

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CASE_SENSITIVE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 13)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SEARCHABLE;
// TODO:
// SQLIdentifier, ColAttrs subdefines for SQL_COLUMN_SEARCHABLE These are also used by SQLGetInfo
//pub enum SQL_COLUMN_SEARCHABLE {
//    SQL_UNSEARCHABLE = 0,
//    SQL_LIKE_ONLY = 1,
//    SQL_ALL_EXCEPT_LIKE = 2,
//    SQL_SEARCHABLE = 3,
//}

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 14)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 15)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TABLE_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 16)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCHEMA_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 17)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CATALOG_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 18)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LABEL;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 22)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_COLUMN_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 23)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_TABLE_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 27)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_PREFIX;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 28)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_SUFFIX;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 29)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LOCAL_TYPE_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 32)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NUM_PREC_RADIX;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1002)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1003)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LENGTH;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1005)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PRECISION;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1006)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCALE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1008)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NULLABLE;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1011)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NAME;

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1012)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNNAMED;
unsafe impl<C> ReadAttr<C, MaybeUninit<SQLSMALLINT>> for SQL_DESC_UNNAMED {}

//#[repr(SQLSMALLINT)]
//pub enum DescUnnamed {
//    /// SQL_DESC_NAME field of the IRD contains a column alias or a column name
//    SQL_NAMED = 0,
//    /// There is no column name or column alias
//    SQL_UNNAMED = 1,
//}

#[derive(Identifier, ColAttr)]
#[identifier(SQLUSMALLINT, 1013)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH;

// TODO: These are unknown, find their values
// SQL_DESC_NUM_PREC_RADIX, SQL_DESC_CONCISE_TYPE, SQL_DESC_TYPE
