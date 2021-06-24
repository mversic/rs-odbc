use crate::{
    handle::SQLHDBC, info::TxnIsolation, stmt::StmtAttr, Attr, AttrLen, AttrGet, AttrSet, Ident,
    OdbcBool, OdbcDefined, True, SQLCHAR, SQLINTEGER, SQLUINTEGER, SQLWCHAR, V3, V3_8, V4, Version
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait ConnAttr<A: Ident, V: Version>:
    Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>
{
    // TODO: Track active statements in debug mode because SQL_ATTR_ASYNC_ENABLE
    // can only be set when there are no active statements

    #[inline]
    // TODO: Not really sure whether attributes should be checked when getting them
    // with SQLGetConnectAttr. Currently only SQLSetConnectAttr uses this
    fn check_attr(&self, ConnectionHandle: &SQLHDBC<V>) {}
}

// Implement ConnAttr for all versions of connection attributes
impl<A: Ident, T: Ident> ConnAttr<A, V3_8> for T where T: ConnAttr<A, V3> {}
impl<A: Ident, T: Ident> ConnAttr<A, V4> for T where T: ConnAttr<A, V3_8> {}
impl<A: Ident> ConnAttr<A, V3_8> for [SQLCHAR] where [SQLCHAR]: ConnAttr<A, V3> {}
impl<A: Ident> ConnAttr<A, V4> for [SQLCHAR] where [SQLCHAR]: ConnAttr<A, V3_8> {}

// Implement ConnAttr for unicode character connection attributes
impl<V: Version, A: Ident> ConnAttr<A, V> for [SQLWCHAR]
where
    [SQLCHAR]: ConnAttr<A, V, NonBinary = True>,
{
}

// Implement ConnAttr for uninitialized connection attributes
impl<V: Version, A: Ident, T: Ident> ConnAttr<A, V> for MaybeUninit<T>
where
    T: ConnAttr<A, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
impl<V: Version, A: Ident> ConnAttr<A, V> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: ConnAttr<A, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
impl<V: Version, A: Ident> ConnAttr<A, V> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: ConnAttr<A, V>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}

// Implement ConnAttr for references to character connection attributes (used by AttrSet)
impl<V: Version, A: Ident> ConnAttr<A, V> for &[SQLCHAR] where [SQLCHAR]: ConnAttr<A, V> {}
impl<V: Version, A: Ident> ConnAttr<A, V> for &[SQLWCHAR] where [SQLWCHAR]: ConnAttr<A, V> {}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ACCESS_MODE;
unsafe impl Attr<SQL_ATTR_ACCESS_MODE> for AccessMode {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_ACCESS_MODE, V3> for AccessMode {}
unsafe impl AttrGet<SQL_ATTR_ACCESS_MODE> for AccessMode {}
unsafe impl AttrSet<SQL_ATTR_ACCESS_MODE> for AccessMode {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_AUTOCOMMIT;
unsafe impl Attr<SQL_ATTR_AUTOCOMMIT> for AutoCommit {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_AUTOCOMMIT, V3> for AutoCommit {}
unsafe impl AttrGet<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}
unsafe impl AttrSet<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 113)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_TIMEOUT;
unsafe impl Attr<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_CONNECTION_TIMEOUT, V3> for SQLUINTEGER {}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 109)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURRENT_CATALOG;
unsafe impl Attr<SQL_ATTR_CURRENT_CATALOG> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_CURRENT_CATALOG, V3> for [SQLCHAR] {}
unsafe impl AttrGet<SQL_ATTR_CURRENT_CATALOG> for [SQLCHAR] {}
unsafe impl AttrSet<SQL_ATTR_CURRENT_CATALOG> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 103)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_LOGIN_TIMEOUT;
unsafe impl Attr<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_LOGIN_TIMEOUT, V3> for SQLUINTEGER {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_not_connected();
    }
}
unsafe impl AttrGet<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 112)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_PACKET_SIZE;
unsafe impl Attr<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_PACKET_SIZE, V3> for SQLUINTEGER {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_not_connected();
    }
}
unsafe impl AttrGet<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACE;
unsafe impl Attr<SQL_ATTR_TRACE> for Trace {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRACE, V3> for Trace {}
unsafe impl AttrGet<SQL_ATTR_TRACE> for Trace {}
unsafe impl AttrSet<SQL_ATTR_TRACE> for Trace {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACEFILE;
unsafe impl Attr<SQL_ATTR_TRACEFILE> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRACEFILE, V3> for [SQLCHAR] {}
unsafe impl AttrGet<SQL_ATTR_TRACEFILE> for [SQLCHAR] {}
unsafe impl AttrSet<SQL_ATTR_TRACEFILE> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRANSLATE_LIB;
unsafe impl Attr<SQL_ATTR_TRANSLATE_LIB> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRANSLATE_LIB, V3> for [SQLCHAR] {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
unsafe impl AttrGet<SQL_ATTR_TRANSLATE_LIB> for [SQLCHAR] {}
unsafe impl AttrSet<SQL_ATTR_TRANSLATE_LIB> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10001)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_AUTO_IPD;
unsafe impl Attr<SQL_ATTR_AUTO_IPD> for OdbcBool {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_AUTO_IPD, V3> for OdbcBool {}
unsafe impl AttrGet<SQL_ATTR_AUTO_IPD> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 117)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
unsafe impl Attr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE, V3_8> for AsyncDbcFunctionsEnable {}
unsafe impl AttrGet<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {}
unsafe impl AttrSet<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {}


// TODO: Spec says this is 3.5, but it is not 3.5 in implementation ???
// but it says that drivers conforming to earlier versions can support this field
#[derive(Ident)]
#[identifier(SQLINTEGER, 1209)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_CONNECTION_DEAD;
unsafe impl Attr<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_CONNECTION_DEAD, V3_8> for ConnectionDead {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 108)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TXN_ISOLATION;
unsafe impl Attr<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TXN_ISOLATION, V3> for TxnIsolation {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
unsafe impl AttrGet<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}
unsafe impl AttrSet<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 107)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//unsafe impl Attr<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl ConnAttr<SQL_ATTR_TRANSLATE_OPTION, V3> for SQLUINTEGER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
//        ConnectionHandle.assert_connected();
//    }
//}
//unsafe impl AttrGet<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}
//unsafe impl AttrSet<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 118)]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//impl ConnAttr<SQL_ATTR_DBC_INFO_TOKEN, V3_8> for SQLPOINTER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
//        assert_connected(ConnectionHandle);
//    }
//}
//impl AttrSet<SQL_ATTR_DBC_INFO_TOKEN> for SQLPOINTER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 119)]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//impl ConnAttr<SQL_ATTR_ASYNC_DBC_EVENT, V3_8> for SQLPOINTER {}
//// TODO: It's an Event handle. Should probably implement event handle
//impl AttrGet<SQL_ATTR_ASYNC_DBC_EVENT> for SQLPOINTER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 111)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;
//impl ConnAttr<SQL_ATTR_QUIET_MODE, V3> for SQLHWND {}

// TODO: Not found in documentation, only in implementation
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 114)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;

//#[derive(odbc_type)]
//#[allow(non_camel_case_types)]
//pub struct DisconnectBehavior;
//pub const SQL_DB_RETURN_TO_POOL: DisconnectBehavior = DisconnectBehavior(0);
//pub const SQL_DB_DISCONNECT: DisconnectBehavior = DisconnectBehavior(1);

//*  ODBC Driver Manager sets this connection attribute to a unicode driver
//    (which supports SQLConnectW) when the application is an ANSI application
//    (which calls SQLConnect, SQLDriverConnect, or SQLBrowseConnect).
//    This is SetConnectAttr only and application does not set this attribute
//    This attribute was introduced because some unicode driver's some APIs may
//    need to behave differently on ANSI or Unicode applications. A unicode
//    driver, which  has same behavior for both ANSI or Unicode applications,
//    should return SQL_ERROR when the driver manager sets this connection
//    attribute. When a unicode driver returns SQL_SUCCESS on this attribute,
//    the driver manager treates ANSI and Unicode connections differently in
//    connection pooling.
//*/
//// TODO: These 4 are not in Documentation??
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 115)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ANSI_APP;
//impl ConnAttr<SQL_ATTR_ANSI_APP, V3_51> for AnsiApp {}
//impl AttrGet<SQL_ATTR_ANSI_APP>> for AnsiApp {}
//impl AttrSet<SQL_ATTR_ANSI_APP> for AnsiApp {}

//pub enum AnsiApp {
//    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 116)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_RESET_CONNECTION;
//impl ConnAttr<SQL_ATTR_RESET_CONNECTION, V3_8> for ResetConnection {}
//impl AttrGet<SQL_ATTR_RESET_CONNECTION>> for ResetConnection {}
//impl AttrSet<SQL_ATTR_RESET_CONNECTION> for ResetConnection {}

//#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//pub enum ResetConnection {
//    SQL_RESET_CONNECTION_YES = 1,
//}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 122)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CREDENTIALS;
//unsafe impl Attr<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl ConnAttr<SQL_ATTR_CREDENTIALS, V4> for [SQLCHAR] {}
//unsafe impl AttrGet<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {}
//unsafe impl AttrSet<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 123)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_REFRESH_CONNECTION;
unsafe impl Attr<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_REFRESH_CONNECTION, V4> for RefreshConnection {}
unsafe impl AttrGet<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}
unsafe impl AttrSet<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}

// Re-exported as connection attribute
pub use crate::stmt::SQL_ATTR_ASYNC_ENABLE;
impl<'stmt, 'buf, T: Ident> ConnAttr<SQL_ATTR_ASYNC_ENABLE, V3> for T where
    T: StmtAttr<'stmt, 'buf, SQL_ATTR_ASYNC_ENABLE, V3>
{
}
impl<'stmt, 'buf, T> ConnAttr<SQL_ATTR_ASYNC_ENABLE, V3> for [T] where
    [T]: StmtAttr<'stmt, 'buf, SQL_ATTR_ASYNC_ENABLE, V3>
{
}

pub use crate::stmt::SQL_ATTR_METADATA_ID;
impl<'stmt, 'buf, T: Ident> ConnAttr<SQL_ATTR_METADATA_ID, V3> for T where
    T: StmtAttr<'stmt, 'buf, SQL_ATTR_METADATA_ID, V3>
{
}
impl<'stmt, 'buf, T> ConnAttr<SQL_ATTR_METADATA_ID, V3> for [T] where
    [T]: StmtAttr<'stmt, 'buf, SQL_ATTR_METADATA_ID, V3>
{
}

//=====================================================================================//

#[odbc_type(SQLUINTEGER)]
pub struct AccessMode;
pub const SQL_MODE_READ_WRITE: AccessMode = AccessMode(0);
pub const SQL_MODE_READ_ONLY: AccessMode = AccessMode(1);

#[odbc_type(SQLUINTEGER)]
pub struct AutoCommit;
pub const SQL_AUTOCOMMIT_OFF: AutoCommit = AutoCommit(0);
pub const SQL_AUTOCOMMIT_ON: AutoCommit = AutoCommit(1);

#[odbc_type(SQLUINTEGER)]
pub struct Trace;
pub const SQL_OPT_TRACE_OFF: Trace = Trace(0);
pub const SQL_OPT_TRACE_ON: Trace = Trace(1);

#[odbc_type(SQLUINTEGER)]
pub struct AsyncDbcFunctionsEnable;
pub const SQL_ASYNC_DBC_ENABLE_OFF: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(0);
pub const SQL_ASYNC_DBC_ENABLE_ON: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(1);

#[odbc_type(SQLUINTEGER)]
pub struct ConnectionDead;
pub const SQL_CD_FALSE: ConnectionDead = ConnectionDead(0);
pub const SQL_CD_TRUE: ConnectionDead = ConnectionDead(1);

#[odbc_type(SQLINTEGER)]
#[allow(non_camel_case_types)]
pub struct RefreshConnection;
pub const SQL_REFRESH_NOW: RefreshConnection = RefreshConnection(-1);
pub const SQL_REFRESH_AUTO: RefreshConnection = RefreshConnection(0);
pub const SQL_REFRESH_MANUAL: RefreshConnection = RefreshConnection(1);
