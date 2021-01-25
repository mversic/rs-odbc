use crate::{AsRawParts, Attribute, GetAttr, OdbcAttr, OdbcBool, SetAttr};
use crate::{SQLINTEGER, SQLPOINTER, SQLUINTEGER};
use rs_odbc_derive::{AnsiType, EnvAttr, EqSQLUINTEGER};

pub trait EnvAttr: Attribute<IdentType = SQLINTEGER> {}

#[identifier(200)]
#[derive(EnvAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ODBC_VERSION;
impl GetAttr<SQLUINTEGER> for SQL_ATTR_ODBC_VERSION {}
impl SetAttr<OdbcVersion> for SQL_ATTR_ODBC_VERSION {}

#[identifier(201)]
#[derive(EnvAttr)]
#[cfg(feature = "v3_8")]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_POOLING;
#[cfg(feature = "v3_8")]
pub use ConnectionPooling::SQL_CP_OFF as SQL_CP_DEFAULT;
#[cfg(feature = "v3_8")]
impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_POOLING {}
#[cfg(feature = "v3_8")]
impl SetAttr<ConnectionPooling> for SQL_ATTR_CONNECTION_POOLING {}

#[identifier(202)]
#[derive(EnvAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
pub use CpMatch::SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;
impl GetAttr<SQLUINTEGER> for SQL_ATTR_CP_MATCH {}
impl SetAttr<CpMatch> for SQL_ATTR_CP_MATCH {}

// TODO:
// For private driver manager
// #[identifier(203)]
// #[derive(EnvAttr)]
// pub struct SQL_ATTR_APPLICATION_KEY;

#[identifier(1001)]
#[derive(EnvAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_OUTPUT_NTS;
impl GetAttr<SQLUINTEGER> for SQL_ATTR_OUTPUT_NTS {}
impl SetAttr<OdbcBool> for SQL_ATTR_OUTPUT_NTS {}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcVersion {
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "v3_8")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "v4")]
    SQL_OV_ODBC4 = 400,
}
impl AsRawParts<OdbcAttr, SQLINTEGER> for OdbcVersion {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLUINTEGER as SQLPOINTER, 0)
    }
}

#[cfg(feature = "v3_8")]
#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionPooling {
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
    SQL_CP_DRIVER_AWARE = 3,
}

#[cfg(feature = "v3_8")]
impl AsRawParts<OdbcAttr, SQLINTEGER> for ConnectionPooling {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLUINTEGER as SQLPOINTER, 0)
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpMatch {
    SQL_CP_STRICT_MATCH = 0,
    SQL_CP_RELAXED_MATCH = 1,
}

impl AsRawParts<OdbcAttr, SQLINTEGER> for CpMatch {
    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
        (*self as SQLUINTEGER as SQLPOINTER, 0)
    }
}
