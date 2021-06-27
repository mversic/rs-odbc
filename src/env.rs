use crate::{
    Attr, AttrLen, AttrGet, AttrSet, Ident, OdbcDefined, True, SQLCHAR, SQLINTEGER,
    SQLWCHAR, SQLUINTEGER,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait EnvAttr<A: Ident, V>:
    Attr<A, DefinedBy = OdbcDefined> + AttrLen<OdbcDefined, Self::NonBinary, SQLINTEGER>
{
}

// Implement EnvAttr for all versions of environment attributes
impl<A: Ident, T: Ident> EnvAttr<A, SQL_OV_ODBC3_80> for T where T: EnvAttr<A, SQL_OV_ODBC3> {}
impl<A: Ident, T: Ident> EnvAttr<A, SQL_OV_ODBC4> for T where T: EnvAttr<A, SQL_OV_ODBC3_80> {}
impl<A: Ident> EnvAttr<A, SQL_OV_ODBC3_80> for [SQLCHAR] where [SQLCHAR]: EnvAttr<A, SQL_OV_ODBC3> {}
impl<A: Ident> EnvAttr<A, SQL_OV_ODBC4> for [SQLCHAR] where [SQLCHAR]: EnvAttr<A, SQL_OV_ODBC3_80> {}

// Implement EnvAttr for unicode character environment attributes
impl<V, A: Ident> EnvAttr<A, V> for [SQLWCHAR] where [SQLCHAR]: EnvAttr<A, V, NonBinary = True> {}

// Implement EnvAttr for uninitialized environment attributes
impl<V, A: Ident, T: Ident> EnvAttr<A, V> for MaybeUninit<T> where T: EnvAttr<A, V> {}
impl<V, A: Ident> EnvAttr<A, V> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: EnvAttr<A, V>,
    Self: AttrLen<OdbcDefined, Self::NonBinary, SQLINTEGER>,
{
}
impl<V, A: Ident> EnvAttr<A, V> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: EnvAttr<A, V>,
    Self: AttrLen<OdbcDefined, Self::NonBinary, SQLINTEGER>,
{
}

// Implement EnvAttr for references to character environment attributes (used by AttrSet)
impl<V, A: Ident> EnvAttr<A, V> for &[SQLCHAR] where [SQLCHAR]: EnvAttr<A, V> {}
impl<V, A: Ident> EnvAttr<A, V> for &[SQLWCHAR] where [SQLWCHAR]: EnvAttr<A, V> {}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 200)]
#[allow(non_camel_case_types)]
// This is read-only attribute becaues it's handled by type system
pub struct SQL_ATTR_ODBC_VERSION;
// TODO: When const generics are implemented restore this attribute
//unsafe impl Attr<SQL_ATTR_ODBC_VERSION> for OdbcVersion {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl EnvAttr<SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3> for OdbcVersion {}
//unsafe impl AttrGet<SQL_ATTR_ODBC_VERSION> for OdbcVersion {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 202)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
unsafe impl Attr<SQL_ATTR_CP_MATCH> for CpMatch {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl EnvAttr<SQL_ATTR_CP_MATCH, SQL_OV_ODBC3> for CpMatch {}
unsafe impl AttrGet<SQL_ATTR_CP_MATCH> for CpMatch {}
unsafe impl AttrSet<SQL_ATTR_CP_MATCH> for CpMatch {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 201)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_POOLING;
unsafe impl Attr<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl EnvAttr<SQL_ATTR_CONNECTION_POOLING, SQL_OV_ODBC3_80> for ConnectionPooling {}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}
unsafe impl AttrSet<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}

//=====================================================================================//

pub trait OdbcVersion {
    const IDENTIFIER: SQLUINTEGER;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC3 {}
impl OdbcVersion for SQL_OV_ODBC3 {
    const IDENTIFIER: SQLUINTEGER = 3;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC3_80 {
}
impl OdbcVersion for SQL_OV_ODBC3_80 {
    const IDENTIFIER: SQLUINTEGER = 380;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC4 {}
impl OdbcVersion for SQL_OV_ODBC4 {
    const IDENTIFIER: SQLUINTEGER = 400;
}

#[odbc_type(SQLUINTEGER)]
pub struct CpMatch;
pub const SQL_CP_STRICT_MATCH: CpMatch = CpMatch(0);
pub const SQL_CP_RELAXED_MATCH: CpMatch = CpMatch(1);

#[odbc_type(SQLUINTEGER)]
pub struct ConnectionPooling;
pub const SQL_CP_OFF: ConnectionPooling = ConnectionPooling(0);
pub const SQL_CP_ONE_PER_DRIVER: ConnectionPooling = ConnectionPooling(1);
pub const SQL_CP_ONE_PER_HENV: ConnectionPooling = ConnectionPooling(2);
pub const SQL_CP_DRIVER_AWARE: ConnectionPooling = ConnectionPooling(3);
