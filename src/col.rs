use co3::ReprC;
use rust_spec::RustSpec;

use crate::{
    Defined,
    attr::{impl_attr, inherit_attr},
    desc::*,
    env::{OV_ODBC3, OV_ODBC3_80, OV_ODBC4, OdbcVersion},
};

/// Marks a column attribute whose value can be retrieved.
///
/// # Safety
/// `Buffer` must have the representation and initialization requirements prescribed by ODBC or
/// by the driver specification.
pub unsafe trait ColAttrGet<V: OdbcVersion>: Defined {
    type CharacterBuffer<C: crate::str::OdbcChar>: ?Sized;
    type NumericBuffer;
}

/// Marks an unavailable output channel while retaining the ABI layout of `T`.
///
/// Its private field prevents safe callers from constructing a value for that channel.
#[doc(hidden)]
#[derive(RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct Unavailable<T>(T);

inherit_attr!(get ColAttrGet, OV_ODBC3 => OV_ODBC3_80, col);
inherit_attr!(get ColAttrGet, OV_ODBC3_80 => OV_ODBC4, col);

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(isize)]
pub enum Updatable {
    ATTR_READONLY = 0,
    ATTR_WRITE = 1,
    ATTR_READWRITE_UNKNOWN = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(isize)]
pub enum Searchable {
    PRED_NONE = 0,
    PRED_CHAR = 1,
    PRED_BASIC = 2,
    PRED_SEARCHABLE = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(isize)]
pub enum Unnamed {
    NAMED = 0,
    UNNAMED = 1,
}

impl_attr!(Col, get, OV_ODBC3, COUNT => isize);
impl_attr!(Col, get, OV_ODBC3, CONCISE_TYPE => isize);
impl_attr!(Col, get, OV_ODBC3, DISPLAY_SIZE => isize);
impl_attr!(Col, get, OV_ODBC3, UNSIGNED => isize);
impl_attr!(Col, get, OV_ODBC3, FIXED_PREC_SCALE => isize);
impl_attr!(Col, get, OV_ODBC3, UPDATABLE => Updatable);
impl_attr!(Col, get, OV_ODBC3, AUTO_UNIQUE_VALUE => isize);
impl_attr!(Col, get, OV_ODBC3, CASE_SENSITIVE => isize);
impl_attr!(Col, get, OV_ODBC3, SEARCHABLE => Searchable);
impl_attr!(Col, get, OV_ODBC3, TYPE_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, TABLE_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, SCHEMA_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, CATALOG_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, LABEL => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, BASE_COLUMN_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, BASE_TABLE_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, LITERAL_PREFIX => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, LITERAL_SUFFIX => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, LOCAL_TYPE_NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, NUM_PREC_RADIX => isize);
impl_attr!(Col, get, OV_ODBC3, TYPE => isize);
impl_attr!(Col, get, OV_ODBC3, LENGTH => isize);
impl_attr!(Col, get, OV_ODBC3, PRECISION => isize);
impl_attr!(Col, get, OV_ODBC3, SCALE => isize);
impl_attr!(Col, get, OV_ODBC3, NULLABLE => isize);
impl_attr!(Col, get, OV_ODBC3, NAME => &'a OdbcStr<C>);
impl_attr!(Col, get, OV_ODBC3, UNNAMED => Unnamed);
impl_attr!(Col, get, OV_ODBC3, OCTET_LENGTH => isize);
