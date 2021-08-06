use crate::attr::{Attr, AttrGet, AttrLen, AttrSet};
use crate::env::{OdbcVersion, SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::handle::SQLHDBC;
use crate::str::{OdbcChar, OdbcStr};
use crate::{
    info::TxnIsolation, stmt::StmtAttr, Ident, OdbcBool, OdbcDefined, SQLCHAR, SQLINTEGER,
    SQLUINTEGER, SQLWCHAR,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub trait ConnState: private::ConnState {}

/// C3 is not a valid state for setting or getting attributes
pub trait ConnAttr<A: Ident, C: ConnState, V: OdbcVersion>:
    Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER>
{
    // TODO: Attributes for which the value wasn't set with SQLSetConnectAttr
    // cannot be used in SQLGetConnectAttr except for:
    // SQL_ATTR_ACCESS_MODE, SQL_ATTR_AUTOCOMMIT, SQL_ATTR_LOGIN_TIMEOUT,
    // SQL_ATTR_ODBC_CURSORS, SQL_ATTR_TRACE, or SQL_ATTR_TRACEFILE
    // which have defined default values by the ODBC specification
    // Check: https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/connection-transitions?view=sql-server-ver15#sqlbrowseconnect

    // TODO: Track active statements in debug mode because SQL_ATTR_ASYNC_ENABLE
    // can only be set when there are no active statements
}

// TODO: Where to keep these two traits? here in api.rs or handle.rs?
pub trait BrowseConnect {}
pub trait Disconnect {}

/// Allocated
#[derive(Debug)]
pub enum C2 {}

/// Need data
#[derive(Debug)]
pub enum C3 {}

/// Connected
#[derive(Debug)]
pub enum C4 {}

impl ConnState for C2 {}
impl ConnState for C3 {}
impl ConnState for C4 {}

// Implement ConnAttr for all versions of connection attributes
impl<A: Ident, T: Ident, C: ConnState> ConnAttr<A, C, SQL_OV_ODBC3_80> for T where
    T: ConnAttr<A, C, SQL_OV_ODBC3>
{
}
impl<A: Ident, T: Ident, C: ConnState> ConnAttr<A, C, SQL_OV_ODBC4> for T where
    T: ConnAttr<A, C, SQL_OV_ODBC3_80>
{
}
impl<A: Ident, CH: OdbcChar, C: ConnState> ConnAttr<A, C, SQL_OV_ODBC3_80> for OdbcStr<CH> where
    OdbcStr<CH>: ConnAttr<A, C, SQL_OV_ODBC3>
{
}
impl<A: Ident, CH: OdbcChar, C: ConnState> ConnAttr<A, C, SQL_OV_ODBC4> for OdbcStr<CH> where
    OdbcStr<CH>: ConnAttr<A, C, SQL_OV_ODBC3_80>
{
}

// Implement ConnAttr for uninitialized connection attributes
impl<A: Ident, T: Ident, C: ConnState, V: OdbcVersion> ConnAttr<A, C, V> for MaybeUninit<T>
where
    T: ConnAttr<A, C, V>,
    Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<A: Ident, C: ConnState, V: OdbcVersion> ConnAttr<A, C, V> for OdbcStr<MaybeUninit<SQLCHAR>> where
    OdbcStr<SQLCHAR>: ConnAttr<A, C, V>
{
}
impl<A: Ident, C: ConnState, V: OdbcVersion> ConnAttr<A, C, V> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: ConnAttr<A, C, V>
{
}

// Implement ConnAttr for references to character connection attributes (used by AttrSet)
impl<A: Ident, C: ConnState, CH: OdbcChar, V: OdbcVersion> ConnAttr<A, C, V> for &OdbcStr<CH>
where
    OdbcStr<CH>: ConnAttr<A, C, V>,
    Self: AttrSet<A>,
{
}

impl<V: OdbcVersion> BrowseConnect for SQLHDBC<'_, C2, V> {}
impl<V: OdbcVersion> BrowseConnect for SQLHDBC<'_, C3, V> {}
impl<V: OdbcVersion> Disconnect for SQLHDBC<'_, C3, V> {}
impl<V: OdbcVersion> Disconnect for SQLHDBC<'_, C4, V> {}

mod private {
    use super::{C2, C3, C4};
    #[double]
    use crate::api::ffi;
    use crate::convert::AsSQLHANDLE;
    use crate::handle::SQLHDBC;
    use crate::{env, sqlreturn};
    use mockall_double::double;
    use std::{any, thread};

    pub trait ConnState {
        // TODO: If drop impl specialization is allowed this fn will not be required
        // Related to https://github.com/rust-lang/rust/issues/20400
        fn disconnect<V: env::OdbcVersion>(handle: &mut SQLHDBC<Self, V>)
        where
            Self: super::ConnState + Sized,
        {
            let sql_return = unsafe { ffi::SQLDisconnect(handle.as_SQLHANDLE()) };

            if sql_return != sqlreturn::SQL_SUCCESS && !thread::panicking() {
                panic!(
                    "{}: SQLDisconnect returned {:?}",
                    any::type_name::<Self>(),
                    sql_return
                )
            }
        }
    }

    impl ConnState for C2 {
        fn disconnect<V: env::OdbcVersion>(_: &mut SQLHDBC<Self, V>) {}
    }
    impl ConnState for C3 {}
    impl ConnState for C4 {}
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ACCESS_MODE;
unsafe impl Attr<SQL_ATTR_ACCESS_MODE> for AccessMode {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_ACCESS_MODE, C2, SQL_OV_ODBC3> for AccessMode {}
impl ConnAttr<SQL_ATTR_ACCESS_MODE, C4, SQL_OV_ODBC3> for AccessMode {}
unsafe impl AttrGet<SQL_ATTR_ACCESS_MODE> for AccessMode {}
unsafe impl AttrSet<SQL_ATTR_ACCESS_MODE> for AccessMode {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 102)]
#[allow(non_camel_case_types)]
// TODO: Implement in type system
pub struct SQL_ATTR_AUTOCOMMIT;
unsafe impl Attr<SQL_ATTR_AUTOCOMMIT> for AutoCommit {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_AUTOCOMMIT, C2, SQL_OV_ODBC3> for AutoCommit {}
impl ConnAttr<SQL_ATTR_AUTOCOMMIT, C4, SQL_OV_ODBC3> for AutoCommit {}
unsafe impl AttrGet<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}
unsafe impl AttrSet<SQL_ATTR_AUTOCOMMIT> for AutoCommit {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 113)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_TIMEOUT;
unsafe impl Attr<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_CONNECTION_TIMEOUT, C2, SQL_OV_ODBC3> for SQLUINTEGER {}
impl ConnAttr<SQL_ATTR_CONNECTION_TIMEOUT, C4, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_CONNECTION_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 109)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURRENT_CATALOG;
unsafe impl<CH: OdbcChar> Attr<SQL_ATTR_CURRENT_CATALOG> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> ConnAttr<SQL_ATTR_CURRENT_CATALOG, C2, SQL_OV_ODBC3> for OdbcStr<CH> {}
impl<CH: OdbcChar> ConnAttr<SQL_ATTR_CURRENT_CATALOG, C4, SQL_OV_ODBC3> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_ATTR_CURRENT_CATALOG> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrSet<SQL_ATTR_CURRENT_CATALOG> for &OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 103)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_LOGIN_TIMEOUT;
unsafe impl Attr<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_LOGIN_TIMEOUT, C2, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl AttrGet<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_LOGIN_TIMEOUT> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 112)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_PACKET_SIZE;
unsafe impl Attr<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_PACKET_SIZE, C2, SQL_OV_ODBC3> for SQLUINTEGER {}
unsafe impl AttrGet<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}
unsafe impl AttrSet<SQL_ATTR_PACKET_SIZE> for SQLUINTEGER {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACE;
unsafe impl Attr<SQL_ATTR_TRACE> for Trace {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_TRACE, C2, SQL_OV_ODBC3> for Trace {}
impl ConnAttr<SQL_ATTR_TRACE, C4, SQL_OV_ODBC3> for Trace {}
unsafe impl AttrGet<SQL_ATTR_TRACE> for Trace {}
unsafe impl AttrSet<SQL_ATTR_TRACE> for Trace {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACEFILE;
unsafe impl<CH: OdbcChar> Attr<SQL_ATTR_TRACEFILE> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> ConnAttr<SQL_ATTR_TRACEFILE, C2, SQL_OV_ODBC3> for OdbcStr<CH> {}
impl<CH: OdbcChar> ConnAttr<SQL_ATTR_TRACEFILE, C4, SQL_OV_ODBC3> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_ATTR_TRACEFILE> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrSet<SQL_ATTR_TRACEFILE> for &OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRANSLATE_LIB;
unsafe impl<CH: OdbcChar> Attr<SQL_ATTR_TRANSLATE_LIB> for OdbcStr<CH> {
    type DefinedBy = OdbcDefined;
}
impl<CH: OdbcChar> ConnAttr<SQL_ATTR_TRANSLATE_LIB, C4, SQL_OV_ODBC3> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrGet<SQL_ATTR_TRANSLATE_LIB> for OdbcStr<CH> {}
unsafe impl<CH: OdbcChar> AttrSet<SQL_ATTR_TRANSLATE_LIB> for &OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10001)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_AUTO_IPD;
unsafe impl Attr<SQL_ATTR_AUTO_IPD> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_AUTO_IPD, C2, SQL_OV_ODBC3> for OdbcBool {}
impl ConnAttr<SQL_ATTR_AUTO_IPD, C4, SQL_OV_ODBC3> for OdbcBool {}
unsafe impl AttrGet<SQL_ATTR_AUTO_IPD> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 117)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
unsafe impl Attr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE> for AsyncDbcFunctionsEnable {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE, C2, SQL_OV_ODBC3_80>
    for AsyncDbcFunctionsEnable
{
}
impl ConnAttr<SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE, C4, SQL_OV_ODBC3_80>
    for AsyncDbcFunctionsEnable
{
}
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
}
impl ConnAttr<SQL_ATTR_CONNECTION_DEAD, C4, SQL_OV_ODBC3_80> for ConnectionDead {}
unsafe impl AttrGet<SQL_ATTR_CONNECTION_DEAD> for ConnectionDead {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 108)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TXN_ISOLATION;
unsafe impl Attr<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {
    type DefinedBy = OdbcDefined;
}
// TODO: Check for open transaction
impl ConnAttr<SQL_ATTR_TXN_ISOLATION, C2, SQL_OV_ODBC3> for TxnIsolation {}
impl ConnAttr<SQL_ATTR_TXN_ISOLATION, C4, SQL_OV_ODBC3> for TxnIsolation {}
unsafe impl AttrGet<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}
unsafe impl AttrSet<SQL_ATTR_TXN_ISOLATION> for TxnIsolation {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 107)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//unsafe impl Attr<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {
//    type DefinedBy = OdbcDefined;
//}
//impl ConnAttr<SQL_ATTR_TRANSLATE_OPTION, C, SQL_OV_ODBC3> for SQLUINTEGER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC<SQL_OV_ODBC3>) {
//        ConnectionHandle.assert_connected();
//    }
//}
//unsafe impl AttrGet<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}
//unsafe impl AttrSet<SQL_ATTR_TRANSLATE_OPTION> for SQLUINTEGER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 118)]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//unsafe impl Attr<SQL_ATTR_DBC_INFO_TOKEN> for SQLPOINTER {
//    type DefinedBy = OdbcDefined;
//}
//impl ConnAttr<SQL_ATTR_DBC_INFO_TOKEN, C, SQL_OV_ODBC3_80> for SQLPOINTER {
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &SQLHDBC<SQL_OV_ODBC3_80>) {
//        assert_connected(ConnectionHandle);
//    }
//impl AttrSet<SQL_ATTR_DBC_INFO_TOKEN> for SQLPOINTER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 119)]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//// TODO: It's an Event handle. Should probably implement event handle
//unsafe impl Attr<SQL_ATTR_ASYNC_DBC_EVENT> for SQLPOINTER {
//    type DefinedBy = OdbcDefined;
//}
//impl ConnAttr<SQL_ATTR_ASYNC_DBC_EVENT, C, SQL_OV_ODBC3_80> for SQLPOINTER {}
//impl AttrGet<SQL_ATTR_ASYNC_DBC_EVENT> for SQLPOINTER {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 111)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;
//impl ConnAttr<SQL_ATTR_QUIET_MODE, C, SQL_OV_ODBC3> for {}

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
//impl ConnAttr<SQL_ATTR_ANSI_APP, SQL_OV_ODBC3_51> for AnsiApp {}
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
//impl ConnAttr<SQL_ATTR_RESET_CONNECTION, SQL_OV_ODBC3_80> for ResetConnection {}
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
//unsafe impl<CH: OdbcChar> Attr<SQL_ATTR_CREDENTIALS> for OdbcStr<CH> {
//    type DefinedBy = OdbcDefined;
//}
//impl<CH: OdbcChar> ConnAttr<SQL_ATTR_CREDENTIALS, SQL_OV_ODBC4> for OdbcStr<CH> {}
//unsafe impl<CH: OdbcChar> AttrGet<SQL_ATTR_CREDENTIALS> for OdbcStr<CH> {}
//unsafe impl<CH: OdbcChar> AttrSet<SQL_ATTR_CREDENTIALS> for OdbcStr<CH> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 123)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_REFRESH_CONNECTION;
unsafe impl Attr<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {
    type DefinedBy = OdbcDefined;
}
impl ConnAttr<SQL_ATTR_REFRESH_CONNECTION, C2, SQL_OV_ODBC4> for RefreshConnection {}
impl ConnAttr<SQL_ATTR_REFRESH_CONNECTION, C4, SQL_OV_ODBC4> for RefreshConnection {}
unsafe impl AttrGet<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}
unsafe impl AttrSet<SQL_ATTR_REFRESH_CONNECTION> for RefreshConnection {}

// Re-exported as connection attribute
pub use crate::stmt::SQL_ATTR_ASYNC_ENABLE;
impl<'desc, 'buf, T: Ident> ConnAttr<SQL_ATTR_ASYNC_ENABLE, C2, SQL_OV_ODBC3> for T where
    T: StmtAttr<'desc, 'buf, SQL_ATTR_ASYNC_ENABLE, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T: Ident> ConnAttr<SQL_ATTR_ASYNC_ENABLE, C4, SQL_OV_ODBC3> for T where
    T: StmtAttr<'desc, 'buf, SQL_ATTR_ASYNC_ENABLE, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T> ConnAttr<SQL_ATTR_ASYNC_ENABLE, C2, SQL_OV_ODBC3> for [T] where
    [T]: StmtAttr<'desc, 'buf, SQL_ATTR_ASYNC_ENABLE, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T> ConnAttr<SQL_ATTR_ASYNC_ENABLE, C4, SQL_OV_ODBC3> for [T] where
    [T]: StmtAttr<'desc, 'buf, SQL_ATTR_ASYNC_ENABLE, SQL_OV_ODBC3>
{
}

pub use crate::stmt::SQL_ATTR_METADATA_ID;
impl<'desc, 'buf, T: Ident> ConnAttr<SQL_ATTR_METADATA_ID, C2, SQL_OV_ODBC3> for T where
    T: StmtAttr<'desc, 'buf, SQL_ATTR_METADATA_ID, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T: Ident> ConnAttr<SQL_ATTR_METADATA_ID, C4, SQL_OV_ODBC3> for T where
    T: StmtAttr<'desc, 'buf, SQL_ATTR_METADATA_ID, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T> ConnAttr<SQL_ATTR_METADATA_ID, C2, SQL_OV_ODBC3> for [T] where
    [T]: StmtAttr<'desc, 'buf, SQL_ATTR_METADATA_ID, SQL_OV_ODBC3>
{
}
impl<'desc, 'buf, T> ConnAttr<SQL_ATTR_METADATA_ID, C4, SQL_OV_ODBC3> for [T] where
    [T]: StmtAttr<'desc, 'buf, SQL_ATTR_METADATA_ID, SQL_OV_ODBC3>
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
