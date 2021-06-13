use crate::{Attr, AttrLen, AttrRead, AttrWrite, Ident, OdbcDefined, True, SQLCHAR, SQLINTEGER};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait EnvAttr<A: Ident>:
    Attr<A, DefinedBy = OdbcDefined> + AttrLen<OdbcDefined, Self::NonBinary, SQLINTEGER>
{
}

impl<A: Ident> EnvAttr<A> for &[SQLCHAR] where [SQLCHAR]: EnvAttr<A> {}

impl<A: Ident, T: Ident> EnvAttr<A> for MaybeUninit<T> where T: EnvAttr<A> {}

impl<A: Ident> EnvAttr<A> for [MaybeUninit<SQLCHAR>]
where
    Self: AttrLen<OdbcDefined, Self::NonBinary, SQLINTEGER>,
    [SQLCHAR]: EnvAttr<A>,
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 200)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ODBC_VERSION;
unsafe impl Attr<SQL_ATTR_ODBC_VERSION> for OdbcVersion {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl EnvAttr<SQL_ATTR_ODBC_VERSION> for OdbcVersion {}
unsafe impl AttrRead<SQL_ATTR_ODBC_VERSION> for OdbcVersion {}
unsafe impl AttrWrite<SQL_ATTR_ODBC_VERSION> for OdbcVersion {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 202)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
unsafe impl Attr<SQL_ATTR_CP_MATCH> for CpMatch {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl EnvAttr<SQL_ATTR_CP_MATCH> for CpMatch {}
unsafe impl AttrRead<SQL_ATTR_CP_MATCH> for CpMatch {}
unsafe impl AttrWrite<SQL_ATTR_CP_MATCH> for CpMatch {}

#[cfg(feature = "v3_8")]
#[derive(Ident)]
#[identifier(SQLINTEGER, 201)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_POOLING;
#[cfg(feature = "v3_8")]
unsafe impl Attr<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
#[cfg(feature = "v3_8")]
impl EnvAttr<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}
#[cfg(feature = "v3_8")]
unsafe impl AttrRead<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}
#[cfg(feature = "v3_8")]
unsafe impl AttrWrite<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}

//=====================================================================================//

#[odbc_type(SQLUINTEGER)]
pub struct CpMatch;
pub const SQL_CP_STRICT_MATCH: CpMatch = CpMatch(0);
pub const SQL_CP_RELAXED_MATCH: CpMatch = CpMatch(1);

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
