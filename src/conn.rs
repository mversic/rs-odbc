use crate::{
    AnsiType, AsMutRawSlice, AsRawSlice, ReadAttr, WriteAttr, UnicodeType, SQLCHAR, SQLINTEGER,
    SQLUINTEGER, SQLWCHAR, OdbcBool
};
use rs_odbc_derive::{odbc_type, ConnAttr, Identifier};

// Re-exported as connection attributes
pub use crate::stmt::SQL_ATTR_ASYNC_ENABLE;
pub use crate::stmt::SQL_ATTR_METADATA_ID;

pub trait ConnAttr: crate::Identifier<IdentType = SQLINTEGER> {
    type AttrType;
}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 101)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ACCESS_MODE;
unsafe impl<C> ReadAttr<AccessMode, C> for SQL_ATTR_ACCESS_MODE {}
unsafe impl<C> WriteAttr<AccessMode, C> for SQL_ATTR_ACCESS_MODE {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 102)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_AUTOCOMMIT;
unsafe impl<C> ReadAttr<AutoCommit, C> for SQL_ATTR_AUTOCOMMIT {}
unsafe impl<C> WriteAttr<AutoCommit, C> for SQL_ATTR_AUTOCOMMIT {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 113)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_TIMEOUT;
unsafe impl<C> ReadAttr<SQLUINTEGER, C> for SQL_ATTR_CONNECTION_TIMEOUT {}
unsafe impl<C> WriteAttr<SQLUINTEGER, C> for SQL_ATTR_CONNECTION_TIMEOUT {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 109)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURRENT_CATALOG;
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> ReadAttr<T, AnsiType> for SQL_ATTR_CURRENT_CATALOG {}
unsafe impl<T: AsMutRawSlice<SQLWCHAR, SQLINTEGER>> ReadAttr<T, UnicodeType> for SQL_ATTR_CURRENT_CATALOG {}
unsafe impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> WriteAttr<T, AnsiType> for SQL_ATTR_CURRENT_CATALOG {}
unsafe impl<T: AsRawSlice<SQLWCHAR, SQLINTEGER>> WriteAttr<T, UnicodeType> for SQL_ATTR_CURRENT_CATALOG {}

// TODO: Not found in documentation, only in implementation
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 114)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;

//#[derive(AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//#[allow(non_camel_case_types)]
//pub struct DisconnectBehavior;
//pub const SQL_DB_RETURN_TO_POOL: DisconnectBehavior = DisconnectBehavior(0);
//pub const SQL_DB_DISCONNECT: DisconnectBehavior = DisconnectBehavior(1);

// TODO: Seems to be Microsoft related
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 1207)]
//pub struct SQL_ATTR_ENLIST_IN_DTC;
//impl<C> ReadAttr<SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}
//impl<C> WriteAttr<SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}

//pub struct EnlistInDtc;
//pub const SQL_DTC_DONE: = EnlistInDtc(0);

// TODO: Unknown
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 1208)]
//pub struct SQL_ATTR_ENLIST_IN_XA;

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 103)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_LOGIN_TIMEOUT;
unsafe impl<C> ReadAttr<SQLUINTEGER, C> for SQL_ATTR_LOGIN_TIMEOUT {}
unsafe impl<C> WriteAttr<SQLUINTEGER, C> for SQL_ATTR_LOGIN_TIMEOUT {}

// TODO: Seems to be deprecated
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 110)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ODBC_CURSORS;
//impl ReadAttr<SQLULEN>> for SQL_ATTR_ODBC_CURSORS {}
//impl WriteAttr<OdbcCursors> for SQL_ATTR_ODBC_CURSORS {}

//#[allow(non_camel_case_types)]
//#[derive(EqSQLULEN, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//pub const OdbcCursors;
//pub const SQL_CUR_USE_IF_NEEDED: SQLULEN = OdbcCursors(0);
//pub const SQL_CUR_USE_ODBC: SQLULEN = OdbcCursors(1);
//pub const SQL_CUR_USE_DRIVER: SQLULEN = OdbcCursors(2);
//pub use SQL_CUR_USE_DRIVER as SQL_CUR_DEFAULT;

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 112)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_PACKET_SIZE;
unsafe impl<C> ReadAttr<SQLUINTEGER, C> for SQL_ATTR_PACKET_SIZE {}
unsafe impl<C> WriteAttr<SQLUINTEGER, C> for SQL_ATTR_PACKET_SIZE {}

//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 111)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;
//impl ReadAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}
//impl WriteAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 104)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACE;
unsafe impl<C> ReadAttr<Trace, C> for SQL_ATTR_TRACE {}
unsafe impl<C> WriteAttr<Trace, C> for SQL_ATTR_TRACE {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 105)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACEFILE;
// TODO: Is this default really?
//pub const SQL_OPT_TRACE_FILE_DEFAULT = "\\SQL.LOG";

// TODO: Has to be null-terminated
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> ReadAttr<T, AnsiType> for SQL_ATTR_TRACEFILE {}
unsafe impl<T: AsMutRawSlice<SQLWCHAR, SQLINTEGER>> ReadAttr<T, UnicodeType> for SQL_ATTR_TRACEFILE {}
unsafe impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> WriteAttr<T, AnsiType> for SQL_ATTR_TRACEFILE {}
unsafe impl<T: AsRawSlice<SQLWCHAR, SQLINTEGER>> WriteAttr<T, UnicodeType> for SQL_ATTR_TRACEFILE {}

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 106)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRANSLATE_LIB;

// TODO: Has to be null-terminated
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> ReadAttr<T, AnsiType> for SQL_ATTR_TRANSLATE_LIB {}
unsafe impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> ReadAttr<T, UnicodeType> for SQL_ATTR_TRANSLATE_LIB {}
unsafe impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> WriteAttr<T, AnsiType> for SQL_ATTR_TRANSLATE_LIB {}
unsafe impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> WriteAttr<T, UnicodeType> for SQL_ATTR_TRANSLATE_LIB {}

// TODO: Investigate this
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 107)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//impl ReadAttr<SQLUINTEGER>> for SQL_ATTR_TRANSLATE_OPTION {}
//impl WriteAttr<SQLUINTEGER> for SQL_ATTR_TRANSLATE_OPTION {}

// TODO: Uncertain
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 108)]
//pub struct SQL_ATTR_TXN_ISOLATION;

#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 10001)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_AUTO_IPD;
unsafe impl<C> ReadAttr<OdbcBool, C> for SQL_ATTR_AUTO_IPD {}

#[cfg(feature = "v3_8")]
#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 117)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
#[cfg(feature = "v3_8")]
unsafe impl<C> ReadAttr<AsyncDbcFunctionsEnable, C> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}
#[cfg(feature = "v3_8")]
unsafe impl<C> WriteAttr<AsyncDbcFunctionsEnable, C> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}

//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 118)]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//impl<C> WriteAttr<SQLPOINTER, C> for SQL_ATTR_DBC_INFO_TOKEN {}

//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 119)]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//// TODO: It's an Event handle. Should probably implement event handle
//impl<C> ReadAttr<SQLPOINTER, C> for SQL_ATTR_ASYNC_DBC_EVENT {}

// TODO: It is not 3.5 in implementation ???
// but it says that drivers conforming to earlier versions can support this field. HMMMMMMMMMMM
#[cfg(feature = "v3_5")]
#[derive(Identifier, ConnAttr)]
#[identifier(SQLINTEGER, 1209)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_CONNECTION_DEAD;
unsafe impl<C> ReadAttr<ConnectionDead, C> for SQL_ATTR_CONNECTION_DEAD {}

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
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 115)]
//pub struct SQL_ATTR_ANSI_APP;

//#[cfg(feature = "v3_51")]
//pub enum AnsiApp {
//    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//}

//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 116)]
//pub struct SQL_ATTR_RESET_CONNECTION;
//impl ReadAttr<SQLUINTEGER>> for SQL_ATTR_RESET_CONNECTION {}
//impl WriteAttr<ResetConnection> for SQL_ATTR_RESET_CONNECTION {}

//#[cfg(feature = "v3_8")]
//#[derive(AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//pub enum ResetConnection {
//    SQL_RESET_CONNECTION_YES = 1,
//}

//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 122)]
//#[cfg(feature = "v4")]
//pub struct SQL_ATTR_CREDENTIALS;

//#[derive(Identifier, ConnAttr)]
//#[identifier(SQLINTEGER, 123)]
//#[cfg(feature = "v4")]
//pub struct SQL_ATTR_REFRESH_CONNECTION;

//#[cfg(feature = "v4")]
//pub enum RefreshConnection {
//    SQL_REFRESH_NOW = -1,
//    SQL_REFRESH_AUTO = 0,
//    SQL_REFRESH_MANUAL = 1,
//}

// TODO: Reexport these in conn module
// TODO: Or derive them, but still export?
//impl ConnAttr for crate::stmt::SQL_ATTR_METADATA_ID {}
//impl ConnAttr for crate::stmt::SQL_ATTR_ASYNC_ENABLE {}

#[odbc_type(SQLUINTEGER)]
pub struct AccessMode;
pub const SQL_MODE_READ_WRITE: AccessMode = AccessMode(0);
pub const SQL_MODE_READ_ONLY: AccessMode = AccessMode(1);
pub use SQL_MODE_READ_WRITE as SQL_MODE_DEFAULT;

#[odbc_type(SQLUINTEGER)]
pub struct AutoCommit;
pub const SQL_AUTOCOMMIT_OFF: AutoCommit = AutoCommit(0);
pub const SQL_AUTOCOMMIT_ON: AutoCommit = AutoCommit(1);
pub use SQL_AUTOCOMMIT_ON as SQL_AUTOCOMMIT_DEFAULT;

#[odbc_type(SQLUINTEGER)]
pub struct Trace;
pub const SQL_OPT_TRACE_OFF: Trace = Trace(0);
pub const SQL_OPT_TRACE_ON: Trace = Trace(1);
pub use SQL_OPT_TRACE_OFF as SQL_OPT_TRACE_DEFAULT;

#[odbc_type(SQLUINTEGER)]
#[cfg(feature = "v3_8")]
pub struct AsyncDbcFunctionsEnable;
#[cfg(feature = "v3_8")]
pub const SQL_ASYNC_DBC_ENABLE_OFF: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(0);
#[cfg(feature = "v3_8")]
pub const SQL_ASYNC_DBC_ENABLE_ON: AsyncDbcFunctionsEnable = AsyncDbcFunctionsEnable(1);
#[cfg(feature = "v3_8")]
pub use SQL_ASYNC_DBC_ENABLE_OFF as SQL_ASYNC_DBC_ENABLE_DEFAULT;

#[odbc_type(SQLUINTEGER)]
pub struct ConnectionDead;
pub const SQL_CD_FALSE: ConnectionDead = ConnectionDead(0);
pub const SQL_CD_TRUE: ConnectionDead = ConnectionDead(1);
