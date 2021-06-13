use crate::{
    handle::SQLHDBC, info::TxnIsolation, stmt::StmtAttr, Attr, AttrLen, AttrRead, AttrWrite, Ident,
    OdbcBool, OdbcDefined, True, SQLCHAR, SQLINTEGER, SQLUINTEGER, SQLWCHAR,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait ConnAttr<A: Ident>:
    Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>
{
    // TODO: Track active statements in debug mode because SQL_ATTR_ASYNC_ENABLE
    // can only be set when there are no active statements

    #[inline]
    // TODO: Not really sure whether attributes should be checked when getting them
    // with SQLGetConnectAttr. Currently only SQLSetConnectAttr uses this
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {}
}

impl<A: Ident> ConnAttr<A> for &[SQLCHAR] where [SQLCHAR]: ConnAttr<A> {}
impl<A: Ident> ConnAttr<A> for &[SQLWCHAR] where [SQLWCHAR]: ConnAttr<A> {}

impl<A: Ident> ConnAttr<A> for [SQLWCHAR]
where
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
    [SQLCHAR]: ConnAttr<A, NonBinary = True>,
{
}

impl<A: Ident, T: Ident> ConnAttr<A> for MaybeUninit<T>
where
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
    T: ConnAttr<A>,
{
}

impl<A: Ident> ConnAttr<A> for [MaybeUninit<SQLCHAR>]
where
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
    [SQLCHAR]: ConnAttr<A>,
{
}

impl<A: Ident> ConnAttr<A> for [MaybeUninit<SQLWCHAR>]
where
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
    [SQLWCHAR]: ConnAttr<A>,
{
}

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
impl ConnAttr<SQL_ATTR_ACCESS_MODE> for AccessMode {}
unsafe impl AttrRead<SQL_ATTR_ACCESS_MODE> for AccessMode {}
unsafe impl AttrWrite<SQL_ATTR_ACCESS_MODE> for AccessMode {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_AUTOCOMMIT;
unsafe impl Attr<SQL_ATTR_AUTOCOMMIT> for AutoCommit {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}
unsafe impl AttrRead<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}
unsafe impl AttrWrite<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 113)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_TIMEOUT;
unsafe impl Attr<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrRead<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrWrite<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 109)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURRENT_CATALOG;
unsafe impl Attr<SQL_ATTR_CURRENT_CATALOG> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_CURRENT_CATALOG> for [SQLCHAR] {}
unsafe impl AttrRead<SQL_ATTR_CURRENT_CATALOG> for [SQLCHAR] {}
unsafe impl AttrWrite<SQL_ATTR_CURRENT_CATALOG> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 103)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_LOGIN_TIMEOUT;
unsafe impl Attr<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_not_connected();
    }
}
unsafe impl AttrRead<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrWrite<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 112)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_PACKET_SIZE;
unsafe impl Attr<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_not_connected();
    }
}
unsafe impl AttrRead<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}
unsafe impl AttrWrite<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACE;
unsafe impl Attr<SQL_ATTR_TRACE> for Trace {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRACE> for Trace {}
unsafe impl AttrRead<SQL_ATTR_TRACE> for Trace {}
unsafe impl AttrWrite<SQL_ATTR_TRACE> for Trace {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACEFILE;
unsafe impl Attr<SQL_ATTR_TRACEFILE> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRACEFILE> for [SQLCHAR] {}
unsafe impl AttrRead<SQL_ATTR_TRACEFILE> for [SQLCHAR] {}
unsafe impl AttrWrite<SQL_ATTR_TRACEFILE> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRANSLATE_LIB;
unsafe impl Attr<SQL_ATTR_TRANSLATE_LIB> for [SQLCHAR] {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TRANSLATE_LIB> for [SQLCHAR] {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
unsafe impl AttrRead<SQL_ATTR_TRANSLATE_LIB> for [SQLCHAR] {}
unsafe impl AttrWrite<SQL_ATTR_TRANSLATE_LIB> for &[SQLCHAR] {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10001)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_AUTO_IPD;
unsafe impl Attr<SQL_ATTR_AUTO_IPD> for OdbcBool {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_AUTO_IPD> for OdbcBool {}
unsafe impl AttrRead<SQL_ATTR_AUTO_IPD> for OdbcBool {}

#[cfg(feature = "v3_8")]
#[derive(Ident)]
#[identifier(SQLINTEGER, 117)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
#[cfg(feature = "v3_8")]
unsafe impl Attr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
#[cfg(feature = "v3_8")]
impl ConnAttr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {}
#[cfg(feature = "v3_8")]
unsafe impl AttrRead<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {}
#[cfg(feature = "v3_8")]
unsafe impl AttrWrite<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {}

// TODO: It is not 3.5 in implementation ???
// but it says that drivers conforming to earlier versions can support this field
#[cfg(feature = "v3_5")]
#[derive(Ident)]
#[identifier(SQLINTEGER, 1209)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_CONNECTION_DEAD;
#[cfg(feature = "v3_5")]
unsafe impl Attr<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
#[cfg(feature = "v3_5")]
impl ConnAttr<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
#[cfg(feature = "v3_5")]
unsafe impl AttrRead<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 108)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TXN_ISOLATION;
unsafe impl Attr<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl ConnAttr<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {
    #[cfg(feature = "odbc_debug")]
    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
        ConnectionHandle.assert_connected();
    }
}
unsafe impl AttrRead<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}
unsafe impl AttrWrite<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 107)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//unsafe impl Attr<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//impl ConnAttr<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
//        ConnectionHandle.assert_connected();
//    }
//}
//unsafe impl AttrRead<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}
//unsafe impl AttrWrite<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}

//#[cfg(feature = "v3_8")]
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 118)]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//impl ConnAttr<SQL_ATTR_DBC_INFO_TOKEN> for SQLPOINTER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC) {
//        assert_connected(ConnectionHandle);
//    }
//}
//impl AttrWrite<SQL_ATTR_DBC_INFO_TOKEN> for SQLPOINTER {}

//#[cfg(feature = "v3_8")]
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 119)]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//// TODO: It's an Event handle. Should probably implement event handle
//impl AttrRead<SQL_ATTR_ASYNC_DBC_EVENT> for SQLPOINTER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 111)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;
//impl ConnAttr<SQL_ATTR_QUIET_MODE> for SQLHWND {}

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
//#[cfg(feature = "v3_51")]
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 115)]
//pub struct SQL_ATTR_ANSI_APP;

//#[cfg(feature = "v3_51")]
//pub enum AnsiApp {
//    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//}

//#[cfg(feature = "v3_8")]
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 116)]
//pub struct SQL_ATTR_RESET_CONNECTION;
//impl AttrRead<SQLUINTEGER>> for SQL_ATTR_RESET_CONNECTION {}
//impl AttrWrite<ResetConnection> for SQL_ATTR_RESET_CONNECTION {}

//#[cfg(feature = "v3_8")]
//#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//pub enum ResetConnection {
//    SQL_RESET_CONNECTION_YES = 1,
//}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 122)]
//#[cfg(feature = "v4")]
//pub struct SQL_ATTR_CREDENTIALS;
//#[cfg(feature = "v4")]
//unsafe impl Attr<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {
//    type DefinedBy = OdbcDefined;
//    type NonBinary = True;
//}
//#[cfg(feature = "v4")]
//unsafe impl AttrRead<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {}
//#[cfg(feature = "v4")]
//unsafe impl AttrWrite<SQL_ATTR_CREDENTIALS> for [SQLCHAR] {}

#[cfg(feature = "v4")]
#[derive(Ident)]
#[identifier(SQLINTEGER, 123)]
pub struct SQL_ATTR_REFRESH_CONNECTION;
#[cfg(feature = "v4")]
unsafe impl Attr<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
#[cfg(feature = "v4")]
unsafe impl AttrRead<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}
#[cfg(feature = "v4")]
unsafe impl AttrWrite<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}

// Re-exported as connection attribute
pub use crate::stmt::SQL_ATTR_ASYNC_ENABLE;
impl<'stmt, 'buf, T: Ident> ConnAttr<SQL_ATTR_ASYNC_ENABLE> for T where
    T: StmtAttr<'stmt, 'buf, SQL_ATTR_ASYNC_ENABLE>
{
}
impl<'stmt, 'buf, T> ConnAttr<SQL_ATTR_ASYNC_ENABLE> for [T] where
    [T]: StmtAttr<'stmt, 'buf, SQL_ATTR_ASYNC_ENABLE>
{
}

pub use crate::stmt::SQL_ATTR_METADATA_ID;
impl<'stmt, 'buf, T: Ident> ConnAttr<SQL_ATTR_METADATA_ID> for T where
    T: StmtAttr<'stmt, 'buf, SQL_ATTR_METADATA_ID>
{
}
impl<'stmt, 'buf, T> ConnAttr<SQL_ATTR_METADATA_ID> for [T] where
    [T]: StmtAttr<'stmt, 'buf, SQL_ATTR_METADATA_ID>
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

#[cfg(feature = "v3_8")]
#[odbc_type(SQLUINTEGER)]
pub struct AsyncDbcFunctionsEnable;
#[cfg(feature = "v3_8")]
pub const SQL_ASYNC_DBC_ENABLE_OFF: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(0);
#[cfg(feature = "v3_8")]
pub const SQL_ASYNC_DBC_ENABLE_ON: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(1);

#[cfg(feature = "v3_5")]
#[odbc_type(SQLUINTEGER)]
pub struct ConnectionDead;
#[cfg(feature = "v3_5")]
pub const SQL_CD_FALSE: ConnectionDead = ConnectionDead(0);
#[cfg(feature = "v3_5")]
pub const SQL_CD_TRUE: ConnectionDead = ConnectionDead(1);

#[cfg(feature = "v4")]
pub enum RefreshConnection {
    SQL_REFRESH_NOW = -1,
    SQL_REFRESH_AUTO = 0,
    SQL_REFRESH_MANUAL = 1,
}
