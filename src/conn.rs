use crate::{
    AnsiType, AsMutRawSlice, AsRawSlice, AsSQLPOINTER, GetAttr, Len, OdbcAttr, SetAttr,
    UnicodeType, SQLCHAR, SQLINTEGER, SQLPOINTER, SQLUINTEGER, SQLWCHAR,
};
use rs_odbc_derive::{ConnAttr, EqSQLUINTEGER, Identifier};
use std::mem::MaybeUninit;

// Re-exported as connection attributes
pub use crate::stmt::SQL_ATTR_ASYNC_ENABLE;
pub use crate::stmt::SQL_ATTR_METADATA_ID;

pub trait ConnAttr: crate::Identifier<IdentType = SQLINTEGER> {
    type AttrType;
}

#[identifier(SQLINTEGER, 101)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ACCESS_MODE;
pub use AccessMode::SQL_MODE_READ_WRITE as SQL_MODE_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_ACCESS_MODE {}
impl<C> SetAttr<C, AccessMode> for SQL_ATTR_ACCESS_MODE {}

#[identifier(SQLINTEGER, 102)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_AUTOCOMMIT;
pub use AutoCommit::SQL_AUTOCOMMIT_ON as SQL_AUTOCOMMIT_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_AUTOCOMMIT {}
impl<C> SetAttr<C, AutoCommit> for SQL_ATTR_AUTOCOMMIT {}

#[identifier(SQLINTEGER, 113)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONNECTION_TIMEOUT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_CONNECTION_TIMEOUT {}
impl<C> SetAttr<C, SQLUINTEGER> for SQL_ATTR_CONNECTION_TIMEOUT {}

#[identifier(SQLINTEGER, 109)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURRENT_CATALOG;
impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> GetAttr<AnsiType, T> for SQL_ATTR_CURRENT_CATALOG {}
impl<T: AsMutRawSlice<SQLWCHAR, SQLINTEGER>> GetAttr<UnicodeType, T> for SQL_ATTR_CURRENT_CATALOG {}
impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> SetAttr<AnsiType, T> for SQL_ATTR_CURRENT_CATALOG {}
impl<T: AsRawSlice<SQLWCHAR, SQLINTEGER>> SetAttr<UnicodeType, T> for SQL_ATTR_CURRENT_CATALOG {}

// TODO: Not found in documentation, only in implementation
//#[identifier(SQLINTEGER, 114)]
//#[derive(Identifier, ConnAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;

//#[allow(non_camel_case_types)]
//#[derive(AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//pub enum DisconnectBehavior {
//    SQL_DB_RETURN_TO_POOL = 0,
//    SQL_DB_DISCONNECT = 1,
//}
//pub use DisconnectBehavior::SQL_DB_RETURN_TO_POOL as SQL_DB_DEFAULT;
//impl AsSQLPOINTER<SQLPOINTER, SQLINTEGER> for DisconnectBehavior {
//    fn as_SQLPOINTER(&self) -> (SQLPOINTER, SQLINTEGER) {
//        (*self as something as SQLPOINTER, 0)
//    }
//}

// TODO: Seems to be Microsoft related
//#[identifier(SQLINTEGER, 1207)]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_ENLIST_IN_DTC;
//impl<C> GetAttr<C, SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}
//impl<C> SetAttr<C, SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}

//pub enum EnlistInDtc {
//    SQL_DTC_DONE = 0,
//}
//impl AsSQLPOINTER<T, SQLINTEGER> for EnlistInDtc {
//    fn as_SQLPOINTER(&self) -> (SQLPOINTER, SQLINTEGER) {
//        (*self as something as SQLPOINTER, 0)
//    }
//}

// TODO: Unknown
//#[identifier(SQLINTEGER, 1208)]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_ENLIST_IN_XA;

#[identifier(SQLINTEGER, 103)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_LOGIN_TIMEOUT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_LOGIN_TIMEOUT {}
impl<C> SetAttr<C, SQLUINTEGER> for SQL_ATTR_LOGIN_TIMEOUT {}

// TODO: Seems to be deprecated
//#[identifier(SQLINTEGER, 110)]
//#[derive(Identifier, ConnAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ODBC_CURSORS;
//impl GetAttr<MaybeUninit<SQLULEN>> for SQL_ATTR_ODBC_CURSORS {}
//impl SetAttr<OdbcCursors> for SQL_ATTR_ODBC_CURSORS {}

//#[allow(non_camel_case_types)]
//#[derive(EqSQLULEN, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//pub enum OdbcCursors {
//    SQL_CUR_USE_IF_NEEDED = 0,
//    SQL_CUR_USE_ODBC = 1,
//    SQL_CUR_USE_DRIVER = 2,
//}
//pub use OdbcCursors::SQL_CUR_USE_DRIVER as SQL_CUR_DEFAULT;
//impl AsSQLPOINTER<SQLPOINTER, SQLINTEGER> for OdbcCursors {
//    fn as_SQLPOINTER(&self) -> (SQLPOINTER, SQLINTEGER) {
//        (*self as SQLULEN as SQLPOINTER, 0)
//    }
//}

#[identifier(SQLINTEGER, 112)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_PACKET_SIZE;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_PACKET_SIZE {}
impl<C> SetAttr<C, SQLUINTEGER> for SQL_ATTR_PACKET_SIZE {}

//#[identifier(SQLINTEGER, 111)]
//#[derive(Identifier, ConnAttr)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;
//impl GetAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}
//impl SetAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}

#[identifier(SQLINTEGER, 104)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACE;
pub use Trace::SQL_OPT_TRACE_OFF as SQL_OPT_TRACE_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_TRACE {}
impl<C> SetAttr<C, Trace> for SQL_ATTR_TRACE {}

#[identifier(SQLINTEGER, 105)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRACEFILE;
// TODO: Is this default really?
//pub const SQL_OPT_TRACE_FILE_DEFAULT = "\\SQL.LOG";

// TODO: Has to be null-terminated
impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> GetAttr<AnsiType, T> for SQL_ATTR_TRACEFILE {}
impl<T: AsMutRawSlice<SQLWCHAR, SQLINTEGER>> GetAttr<UnicodeType, T> for SQL_ATTR_TRACEFILE {}
impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> SetAttr<AnsiType, T> for SQL_ATTR_TRACEFILE {}
impl<T: AsRawSlice<SQLWCHAR, SQLINTEGER>> SetAttr<UnicodeType, T> for SQL_ATTR_TRACEFILE {}

#[identifier(SQLINTEGER, 106)]
#[derive(Identifier, ConnAttr)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_TRANSLATE_LIB;

// TODO: Has to be null-terminated
impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> GetAttr<AnsiType, T> for SQL_ATTR_TRANSLATE_LIB {}
impl<T: AsMutRawSlice<SQLCHAR, SQLINTEGER>> GetAttr<UnicodeType, T> for SQL_ATTR_TRANSLATE_LIB {}
impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> SetAttr<AnsiType, T> for SQL_ATTR_TRANSLATE_LIB {}
impl<T: AsRawSlice<SQLCHAR, SQLINTEGER>> SetAttr<UnicodeType, T> for SQL_ATTR_TRANSLATE_LIB {}

// TODO: Investigate this
//#[identifier(SQLINTEGER, 107)]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//impl GetAttr<MaybeUninit<SQLUINTEGER>> for SQL_ATTR_TRANSLATE_OPTION {}
//impl SetAttr<SQLUINTEGER> for SQL_ATTR_TRANSLATE_OPTION {}

// TODO: Uncertain
//#[identifier(SQLINTEGER, 108)]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_TXN_ISOLATION;

#[identifier(SQLINTEGER, 10001)]
#[derive(Identifier, ConnAttr)]
// This is read-only attribute
pub struct SQL_ATTR_AUTO_IPD;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_AUTO_IPD {}

#[identifier(SQLINTEGER, 117)]
#[cfg(feature = "v3_8")]
#[derive(Identifier, ConnAttr)]
pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
pub use AsyncDbcFunctionsEnable::SQL_ASYNC_DBC_ENABLE_OFF as SQL_ASYNC_DBC_ENABLE_DEFAULT;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}
impl<C> SetAttr<C, AsyncDbcFunctionsEnable> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}

//#[identifier(SQLINTEGER, 118)]
//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//impl<C> SetAttr<C, SQLPOINTER> for SQL_ATTR_DBC_INFO_TOKEN {}

//#[identifier(SQLINTEGER, 119)]
//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//// TODO: It's an Event handle. Should probably implement event handle
//impl<C> GetAttr<C, SQLPOINTER> for SQL_ATTR_ASYNC_DBC_EVENT {}

// TODO: It is not 3.5 in implementation ???
// but it says that drivers conforming to earlier versions can support this field. HMMMMMMMMMMM
#[identifier(SQLINTEGER, 1209)]
#[cfg(feature = "v3_5")]
#[derive(Identifier, ConnAttr)]
// This is read-only attribute
pub struct SQL_ATTR_CONNECTION_DEAD;
impl<C> GetAttr<C, MaybeUninit<SQLUINTEGER>> for SQL_ATTR_CONNECTION_DEAD {}

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
//#[identifier(SQLINTEGER, 115)]
//#[cfg(feature = "v3_51")]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_ANSI_APP;

//#[cfg(feature = "v3_51")]
//pub enum AnsiApp {
//    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//}

//#[identifier(SQLINTEGER, 116)]
//#[cfg(feature = "v3_8")]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_RESET_CONNECTION;
//impl GetAttr<MaybeUninit<SQLUINTEGER>> for SQL_ATTR_RESET_CONNECTION {}
//impl SetAttr<ResetConnection> for SQL_ATTR_RESET_CONNECTION {}

//#[cfg(feature = "v3_8")]
//#[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
//pub enum ResetConnection {
//    SQL_RESET_CONNECTION_YES = 1,
//}

//#[identifier(SQLINTEGER, 122)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, ConnAttr)]
//pub struct SQL_ATTR_CREDENTIALS;

//#[identifier(SQLINTEGER, 123)]
//#[cfg(feature = "v4")]
//#[derive(Identifier, ConnAttr)]
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

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessMode {
    SQL_MODE_READ_WRITE = 0,
    SQL_MODE_READ_ONLY = 1,
}
impl AsSQLPOINTER for AccessMode {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for AccessMode {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AutoCommit {
    SQL_AUTOCOMMIT_OFF = 0,
    SQL_AUTOCOMMIT_ON = 1,
}
impl AsSQLPOINTER for AutoCommit {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for AutoCommit {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[allow(non_camel_case_types)]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Trace {
    SQL_OPT_TRACE_OFF = 0,
    SQL_OPT_TRACE_ON = 1,
}
impl AsSQLPOINTER for Trace {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for Trace {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[cfg(feature = "v3_8")]
#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsyncDbcFunctionsEnable {
    SQL_ASYNC_DBC_ENABLE_OFF = 0,
    SQL_ASYNC_DBC_ENABLE_ON = 1,
}
impl AsSQLPOINTER for AsyncDbcFunctionsEnable {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for AsyncDbcFunctionsEnable {
    fn len(&self) -> LEN {
        Default::default()
    }
}

#[derive(EqSQLUINTEGER, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionDead {
    SQL_CD_TRUE = 1,
    SQL_CD_FALSE = 0,
}
impl AsSQLPOINTER for ConnectionDead {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        *self as SQLUINTEGER as SQLPOINTER
    }
}
impl<LEN: Default> Len<OdbcAttr, LEN> for ConnectionDead {
    fn len(&self) -> LEN {
        Default::default()
    }
}
