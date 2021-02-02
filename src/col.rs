use crate::{GetAttr, SQLSMALLINT, SQLUSMALLINT};
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
#[identifier(SQLUSMALLINT, 1001)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_COUNT;

#[identifier(SQLUSMALLINT, 2)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CONCISE_TYPE;

#[identifier(SQLUSMALLINT, 6)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DISPLAY_SIZE;

#[identifier(SQLUSMALLINT, 8)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNSIGNED;

#[identifier(SQLUSMALLINT, 9)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_FIXED_PREC_SCALE;

#[identifier(SQLUSMALLINT, 10)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UPDATABLE;
impl<C> GetAttr<C, MaybeUninit<SQLSMALLINT>> for SQL_DESC_UPDATABLE {}

///// Describes the updatability of the column in the result set, not the column in the base table.
//#[repr(SQLSMALLINT)]
//pub enum DescUpdatable {
//    SQL_ATTR_READONLY = 0,
//    SQL_ATTR_WRITE = 1,
//    /// It is unclear whether a column is updatable
//    SQL_ATTR_READWRITE_UNKNOWN = 2,
//}

#[identifier(SQLUSMALLINT, 11)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;

#[identifier(SQLUSMALLINT, 12)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CASE_SENSITIVE;

#[identifier(SQLUSMALLINT, 13)]
#[derive(Identifier, ColAttr)]
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

#[identifier(SQLUSMALLINT, 14)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE_NAME;

#[identifier(SQLUSMALLINT, 15)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TABLE_NAME;

#[identifier(SQLUSMALLINT, 16)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCHEMA_NAME;

#[identifier(SQLUSMALLINT, 17)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CATALOG_NAME;

#[identifier(SQLUSMALLINT, 18)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LABEL;

#[identifier(SQLUSMALLINT, 22)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_COLUMN_NAME;

#[identifier(SQLUSMALLINT, 23)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_TABLE_NAME;

#[identifier(SQLUSMALLINT, 27)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_PREFIX;

#[identifier(SQLUSMALLINT, 28)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_SUFFIX;

#[identifier(SQLUSMALLINT, 29)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LOCAL_TYPE_NAME;

#[identifier(SQLUSMALLINT, 32)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NUM_PREC_RADIX;

#[identifier(SQLUSMALLINT, 1002)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE;

#[identifier(SQLUSMALLINT, 1003)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LENGTH;

#[identifier(SQLUSMALLINT, 1005)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PRECISION;

#[identifier(SQLUSMALLINT, 1006)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCALE;

#[identifier(SQLUSMALLINT, 1008)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NULLABLE;

#[identifier(SQLUSMALLINT, 1011)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NAME;

#[identifier(SQLUSMALLINT, 1012)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNNAMED;
impl<C> GetAttr<C, MaybeUninit<SQLSMALLINT>> for SQL_DESC_UNNAMED {}

//#[repr(SQLSMALLINT)]
//pub enum DescUnnamed {
//    /// SQL_DESC_NAME field of the IRD contains a column alias or a column name
//    SQL_NAMED = 0,
//    /// There is no column name or column alias
//    SQL_UNNAMED = 1,
//}

#[identifier(SQLUSMALLINT, 1013)]
#[derive(Identifier, ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH;

// TODO: These are unknown, find their values
// SQL_DESC_NUM_PREC_RADIX, SQL_DESC_CONCISE_TYPE, SQL_DESC_TYPE
