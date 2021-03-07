use crate::{ReadAttr, WriteAttr, SQLINTEGER, SQLUINTEGER};
use rs_odbc_derive::{odbc_type, EnvAttr, Identifier};
use std::mem::MaybeUninit;

pub trait EnvAttr: crate::Identifier<IdentType = SQLINTEGER> {}

#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 200)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ODBC_VERSION;
unsafe impl<C> ReadAttr<MaybeUninit<OdbcVersion>, C> for SQL_ATTR_ODBC_VERSION {}
unsafe impl<C> WriteAttr<OdbcVersion, C> for SQL_ATTR_ODBC_VERSION {}

#[cfg(feature = "v3_8")]
#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 201)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_POOLING;
#[cfg(feature = "v3_8")]
unsafe impl<C> ReadAttr<MaybeUninit<ConnectionPooling>, C> for SQL_ATTR_CONNECTION_POOLING {}
#[cfg(feature = "v3_8")]
unsafe impl<C> WriteAttr<ConnectionPooling, C> for SQL_ATTR_CONNECTION_POOLING {}

#[derive(Identifier, EnvAttr)]
#[identifier(SQLINTEGER, 202)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
unsafe impl<C> ReadAttr<MaybeUninit<CpMatch>, C> for SQL_ATTR_CP_MATCH {}
unsafe impl<C> WriteAttr<CpMatch, C> for SQL_ATTR_CP_MATCH {}

// TODO:
//For private driver manager
//#[derive(Identifier, EnvAttr)]
//#[identifier(SQLINTEGER, 203)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_APPLICATION_KEY;

// TODO: Is this used in V3.x?
//#[derive(Identifier, EnvAttr)]
//#[identifier(SQLINTEGER, 1001)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_OUTPUT_NTS;
//impl ReadAttr<MaybeUninit<OdbcBool>> for SQL_ATTR_OUTPUT_NTS {}
//impl WriteAttr<OdbcBool> for SQL_ATTR_OUTPUT_NTS {}

#[odbc_type(SQLUINTEGER)]
pub struct OdbcVersion;
pub const SQL_OV_ODBC3: OdbcVersion = OdbcVersion(3);
#[cfg(feature = "v3_8")]
pub const SQL_OV_ODBC3_80: OdbcVersion = OdbcVersion(380);

#[cfg(feature = "v3_8")]
#[odbc_type(SQLUINTEGER)]
pub struct ConnectionPooling;
#[cfg(feature = "v3_8")]
pub const SQL_CP_OFF: ConnectionPooling = ConnectionPooling(0);
#[cfg(feature = "v3_8")]
pub const SQL_CP_ONE_PER_DRIVER: ConnectionPooling = ConnectionPooling(1);
#[cfg(feature = "v3_8")]
pub const SQL_CP_ONE_PER_HENV: ConnectionPooling = ConnectionPooling(2);
#[cfg(feature = "v3_8")]
pub const SQL_CP_DRIVER_AWARE: ConnectionPooling = ConnectionPooling(3);
#[cfg(feature = "v3_8")]
pub use SQL_CP_OFF as SQL_CP_DEFAULT;

#[odbc_type(SQLUINTEGER)]
pub struct CpMatch;
pub const SQL_CP_STRICT_MATCH: CpMatch = CpMatch(0);
pub const SQL_CP_RELAXED_MATCH: CpMatch = CpMatch(1);
pub use SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;
