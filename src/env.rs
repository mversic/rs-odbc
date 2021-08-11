use crate::attr::{Attr, AttrGet, AttrLen, AttrSet};
use crate::str::{OdbcChar, OdbcStr};
use crate::{Ident, OdbcDefined, SQLCHAR, SQLINTEGER, SQLUINTEGER, SQLWCHAR, Scalar};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait EnvAttr<A: Ident, V: OdbcVersion>:
    Attr<A, DefinedBy = OdbcDefined> + AttrLen<OdbcDefined, SQLINTEGER>
{
}

// Implement EnvAttr for all versions of environment attributes
impl<A: Ident, T: Scalar> EnvAttr<A, SQL_OV_ODBC3_80> for T where
    T: EnvAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> EnvAttr<A, SQL_OV_ODBC4> for T where
    T: EnvAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> EnvAttr<A, SQL_OV_ODBC3_80> for [T] where
    [T]: EnvAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, T: Scalar> EnvAttr<A, SQL_OV_ODBC4> for [T] where
    [T]: EnvAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, CH: OdbcChar> EnvAttr<A, SQL_OV_ODBC3_80> for OdbcStr<CH> where
    OdbcStr<CH>: EnvAttr<A, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>
{
}
impl<A: Ident, CH: OdbcChar> EnvAttr<A, SQL_OV_ODBC4> for OdbcStr<CH> where
    OdbcStr<CH>: EnvAttr<A, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>
{
}

// Implement EnvAttr for uninitialized environment attributes
impl<A: Ident, T: Scalar, V: OdbcVersion> EnvAttr<A, V> for MaybeUninit<T>
where
    T: EnvAttr<A, V> + AttrGet<A>,
    Self: AttrLen<OdbcDefined, SQLINTEGER>,
{
}
impl<A: Ident, T: Scalar, V: OdbcVersion> EnvAttr<A, V> for [MaybeUninit<T>]
where
    [T]: EnvAttr<A, V> + AttrGet<A>,
    Self: AttrLen<OdbcDefined, SQLINTEGER>,
{
}
impl<A: Ident, V: OdbcVersion> EnvAttr<A, V> for OdbcStr<MaybeUninit<SQLCHAR>> where
    OdbcStr<SQLCHAR>: EnvAttr<A, V> + AttrGet<A>
{
}
impl<A: Ident, V: OdbcVersion> EnvAttr<A, V> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: EnvAttr<A, V> + AttrGet<A>
{
}

// Implement EnvAttr for references to environment attributes (used by AttrSet)
// TODO: Is it possible to implement for all reference types? Check other attributes as well
impl<A: Ident, T: Scalar> EnvAttr<A, SQL_OV_ODBC3> for &T
where
    T: EnvAttr<A, SQL_OV_ODBC3>,
    Self: AttrSet<A>,
{
}
impl<A: Ident, T: Scalar, V: OdbcVersion> EnvAttr<A, V> for &[T]
where
    [T]: EnvAttr<A, V>,
    Self: AttrSet<A>,
{
}
impl<A: Ident, CH: OdbcChar, V: OdbcVersion> EnvAttr<A, V> for &OdbcStr<CH>
where
    OdbcStr<CH>: EnvAttr<A, V>,
    Self: AttrSet<A>,
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// TODO: Consider using const generics for OdbcVersion once it's available on stable,
// otherwise don't expose this attribute unless there is a valid use-case
#[derive(Ident)]
#[identifier(SQLINTEGER, 200)]
#[allow(non_camel_case_types)]
// This is read-only attribute because
// it's handled by the type system
pub(crate) struct SQL_ATTR_ODBC_VERSION;
//unsafe impl Attr<SQL_ATTR_ODBC_VERSION> for OdbcVersion {
//    type DefinedBy = OdbcDefined;
//}
//impl EnvAttr<SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3> for OdbcVersion {}
//unsafe impl AttrGet<SQL_ATTR_ODBC_VERSION> for OdbcVersion {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 202)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CP_MATCH;
unsafe impl Attr<SQL_ATTR_CP_MATCH> for CpMatch {
    type DefinedBy = OdbcDefined;
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
}
impl EnvAttr<SQL_ATTR_CONNECTION_POOLING, SQL_OV_ODBC3_80> for ConnectionPooling {}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}
unsafe impl AttrSet<SQL_ATTR_CONNECTION_POOLING> for ConnectionPooling {}

//=====================================================================================//

pub trait OdbcVersion {
    type PrevVersion: OdbcVersion;
    const IDENTIFIER: SQLUINTEGER;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC3 {}
impl OdbcVersion for SQL_OV_ODBC3 {
    type PrevVersion = SQL_OV_ODBC3;
    const IDENTIFIER: SQLUINTEGER = 3;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC3_80 {}
impl OdbcVersion for SQL_OV_ODBC3_80 {
    type PrevVersion = SQL_OV_ODBC3;
    const IDENTIFIER: SQLUINTEGER = 380;
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SQL_OV_ODBC4 {}
impl OdbcVersion for SQL_OV_ODBC4 {
    type PrevVersion = SQL_OV_ODBC3_80;
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
