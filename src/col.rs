use crate::{Attribute, GetAttr, SQLSMALLINT, SQLUSMALLINT};
use rs_odbc_derive::ColAttr;

pub trait ColAttr: Attribute<IdentType = SQLUSMALLINT> {}

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
#[identifier(1001)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_COUNT;

#[identifier(2)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CONCISE_TYPE;

#[identifier(6)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_DISPLAY_SIZE;

#[identifier(8)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNSIGNED;

#[identifier(9)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_FIXED_PREC_SCALE;

#[identifier(10)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UPDATABLE;
impl GetAttr<SQLSMALLINT> for SQL_DESC_UPDATABLE {}

///// Describes the updatability of the column in the result set, not the column in the base table.
//#[repr(SQLSMALLINT)]
//pub enum DescUpdatable {
//    SQL_ATTR_READONLY = 0,
//    SQL_ATTR_WRITE = 1,
//    /// It is unclear whether a column is updatable
//    SQL_ATTR_READWRITE_UNKNOWN = 2,
//}

#[identifier(11)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_AUTO_UNIQUE_VALUE;

#[identifier(12)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CASE_SENSITIVE;

#[identifier(13)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SEARCHABLE;
// TODO:
// SQLColAttrs subdefines for SQL_COLUMN_SEARCHABLE These are also used by SQLGetInfo
//pub enum SQL_COLUMN_SEARCHABLE {
//    SQL_UNSEARCHABLE = 0,
//    SQL_LIKE_ONLY = 1,
//    SQL_ALL_EXCEPT_LIKE = 2,
//    SQL_SEARCHABLE = 3,
//}

#[identifier(14)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE_NAME;

#[identifier(15)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TABLE_NAME;

#[identifier(16)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCHEMA_NAME;

#[identifier(17)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_CATALOG_NAME;

#[identifier(18)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LABEL;

#[identifier(22)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_COLUMN_NAME;

#[identifier(23)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_BASE_TABLE_NAME;

#[identifier(27)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_PREFIX;

#[identifier(28)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LITERAL_SUFFIX;

#[identifier(29)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LOCAL_TYPE_NAME;

#[identifier(32)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NUM_PREC_RADIX;

#[identifier(1002)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_TYPE;

#[identifier(1003)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_LENGTH;

#[identifier(1005)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_PRECISION;

#[identifier(1006)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_SCALE;

#[identifier(1008)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NULLABLE;

#[identifier(1011)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_NAME;

#[identifier(1012)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_UNNAMED;
impl GetAttr<SQLSMALLINT> for SQL_DESC_UNNAMED {}

//#[repr(SQLSMALLINT)]
//pub enum DescUnnamed {
//    /// SQL_DESC_NAME field of the IRD contains a column alias or a column name
//    SQL_NAMED = 0,
//    /// There is no column name or column alias
//    SQL_UNNAMED = 1,
//}

#[identifier(1013)]
#[derive(ColAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_DESC_OCTET_LENGTH;

// TODO: These are unknown, find their values
// SQL_DESC_NUM_PREC_RADIX, SQL_DESC_CONCISE_TYPE, SQL_DESC_TYPE
