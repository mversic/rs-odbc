use crate::{
    AsSQLPOINTER, GetAttr, Len, OdbcAttr, SetAttr, SQLINTEGER, SQLPOINTER, SQLUINTEGER,
};
use rs_odbc_derive::{EnvAttr, EqSQLUINTEGER, Identifier};
use std::mem::MaybeUninit;

pub trait EnvAttr: crate::Identifier<IdentType = SQLINTEGER> {}

#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 200)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ODBC_VERSION;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_ODBC_VERSION {}
impl<C> SetAttr<C, OdbcVersion> for SQL_ATTR_ODBC_VERSION {}

#[cfg(feature = "v3_8")]
#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 201)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_POOLING;
#[cfg(feature = "v3_8")]
pub use ConnectionPooling::SQL_CP_OFF as SQL_CP_DEFAULT;
#[cfg(feature = "v3_8")]
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_CONNECTION_POOLING {}
#[cfg(feature = "v3_8")]
impl<C> SetAttr<C, ConnectionPooling> for SQL_ATTR_CONNECTION_POOLING {}

#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 202)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
pub use CpMatch::SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_CP_MATCH {}
impl<C> SetAttr<C, CpMatch> for SQL_ATTR_CP_MATCH {}

// TODO:
// For private driver manager
// #[derive(Identifier, EnvAttr)]
// #[identifier(SQLINTEGER, 203)]
// #[allow(non_camel_case_types)]
// pub struct SQL_ATTR_APPLICATION_KEY;

// TODO: Is this used in V3.x?
//#[derive(Identifier, EnvAttr)]
//#[identifier(SQLINTEGER, 1001)]
// #[allow(non_camel_case_types)]
//pub struct SQL_ATTR_OUTPUT_NTS;
//impl GetAttr<MaybeUninit<SQLUINTEGER>> for SQL_ATTR_OUTPUT_NTS {}
//impl SetAttr<OdbcBool> for SQL_ATTR_OUTPUT_NTS {}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcVersion {
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "v3_8")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "v4")]
    SQL_OV_ODBC4 = 400,
}
impl AsSQLPOINTER for OdbcVersion {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for OdbcVersion {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[cfg(feature = "v3_8")]
#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionPooling {
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
    SQL_CP_DRIVER_AWARE = 3,
}

#[cfg(feature = "v3_8")]
impl AsSQLPOINTER for ConnectionPooling {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for ConnectionPooling {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpMatch {
    SQL_CP_STRICT_MATCH = 0,
    SQL_CP_RELAXED_MATCH = 1,
}

impl AsSQLPOINTER for CpMatch {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for CpMatch {
    fn len(&self) -> LEN {
        Default::default()
    }
}
