pub mod api;
pub mod c_types;
pub mod handle;
pub mod sql_types;
pub mod sqlchar_str;
pub mod sqlreturn;

use std::mem::MaybeUninit;

pub use conn::{
    AccessMode::*, AsyncDbcFunctionsEnable::*, AutoCommit::*, ConnectionDead::*, Trace::*,
    SQL_ASYNC_DBC_ENABLE_DEFAULT, SQL_ATTR_ACCESS_MODE, SQL_ATTR_ASYNC_DBC_EVENT,
    SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE, SQL_ATTR_AUTOCOMMIT, SQL_ATTR_AUTO_IPD,
    SQL_ATTR_CONNECTION_DEAD, SQL_ATTR_CONNECTION_TIMEOUT, SQL_ATTR_CURRENT_CATALOG,
    SQL_ATTR_DBC_INFO_TOKEN, SQL_ATTR_LOGIN_TIMEOUT, SQL_ATTR_PACKET_SIZE, SQL_ATTR_TRACE,
    SQL_ATTR_TRACEFILE, SQL_ATTR_TRANSLATE_LIB, SQL_AUTOCOMMIT_DEFAULT, SQL_MODE_DEFAULT,
    SQL_OPT_TRACE_DEFAULT,
};
pub use env::{
    ConnectionPooling::*, CpMatch::*, OdbcVersion::*, SQL_ATTR_CONNECTION_POOLING,
    SQL_ATTR_CP_MATCH, SQL_ATTR_ODBC_VERSION, SQL_ATTR_OUTPUT_NTS, SQL_CP_DEFAULT,
    SQL_CP_MATCH_DEFAULT,
};
pub use handle::{
    SQLHANDLE, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQL_HANDLE_DBC, SQL_HANDLE_DESC,
    SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
}; // TODO: SQLHWND
pub use sqlchar_str::SQLCHARString;
pub use DriverCompletion::*;
pub use {api::*, c_types::*, sql_types::*, sqlreturn::*};

type SQLPOINTER = *mut std::ffi::c_void;

pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}
// TODO: Is it possible to derive this trait?
pub trait AsRawParts<P, LEN> {
    // TODO: Is it possible to not have P type?
    fn as_raw_parts(&self) -> (SQLPOINTER, LEN);
}
pub trait AsMutRawSlice<P, LEN> {
    // TODO: Is it possible to not have P type?
    // TODO: Consider extracting StrLen to a separate trait
    type StrLen;
    fn as_mut_raw_slice(&mut self) -> (SQLPOINTER, LEN);
}

impl AsMutPtr<SQLINTEGER> for MaybeUninit<SQLINTEGER> {
    fn as_mut_ptr(&mut self) -> *mut SQLINTEGER {
        self.as_mut_ptr()
    }
}
impl<T> AsMutPtr<T> for MaybeUninit<()> {
    fn as_mut_ptr(&mut self) -> *mut T {
        // TODO: Is this dangling pointer of?
        // std::ptr::NonNull::dangling().as_ptr()
        std::ptr::null_mut()
    }
}

impl AnsiType for SQLUINTEGER {}
impl UnicodeType for SQLUINTEGER {}
impl<T> AsMutRawSlice<T, SQLINTEGER> for SQLUINTEGER {
    type StrLen = ();
    fn as_mut_raw_slice(&mut self) -> (SQLPOINTER, SQLINTEGER) {
        (self as *mut _ as SQLPOINTER, 0)
    }
}

pub trait Attribute {
    type AttributeType;
    type IdentifierType;

    fn identifier() -> Self::IdentifierType;
}
pub enum OdbcAttribute {}
pub enum DriverAttribute {}
pub trait GetAttr<T> {}
pub trait SetAttr<T> {}
pub trait AnsiType {}
pub trait UnicodeType {}

pub trait AsOdbcChar {}
pub trait AsAscii: AsOdbcChar {}
pub trait AsUnicode: AsOdbcChar {}

pub trait AsSQLCHARRawSlice<LEN> {
    #[allow(non_snake_case)]
    fn as_SQLCHAR_raw_slice(&self) -> (*const SQLCHAR, LEN);
}
pub trait AsSQLWCHARRawSlice<LEN> {
    #[allow(non_snake_case)]
    fn as_SQLCHAR_raw_slice(&self) -> (*const SQLCHAR, LEN);
}
pub trait AsMutSQLCHARRawSlice<LEN> {
    //type InitializedType;
    #[allow(non_snake_case)]
    fn as_mut_SQLCHAR_raw_slice(&mut self) -> (*mut SQLCHAR, LEN);
    //unsafe fn assume_init(self) -> Self::InitializedType;
}
pub trait AsMutSQLWCHARRawSlice<LEN> {
    //type InitializedType;
    #[allow(non_snake_case)]
    fn as_mut_SQLCHAR_raw_slice(&mut self) -> (*mut SQLCHAR, LEN);
    //unsafe fn assume_init(self) -> Self::InitializedType;
}
pub trait AsCharRawSlice<LEN>: AsSQLCHARRawSlice<LEN> + AsSQLWCHARRawSlice<LEN> {}
pub trait AsMutCharRawSlice<LEN>: AsMutSQLCHARRawSlice<LEN> + AsMutSQLWCHARRawSlice<LEN> {}

impl AsSQLCHARRawSlice<SQLSMALLINT> for str {
    #[allow(non_snake_case)]
    fn as_SQLCHAR_raw_slice(&self) -> (*const SQLCHAR, SQLSMALLINT) {
        (self.as_ptr(), self.len() as SQLSMALLINT)
    }
}

// TODO: Maybe implement something like this?
//impl<const M: usize> AsMutSQLCHARRawSlice<SQLSMALLINT> for [MaybeUninit<SQLCHAR>; M] {
//    type InitializedType = [SQLCHAR; M];
//
//    fn as_mut_SQLCHAR_raw_slice(&mut self) -> (*mut SQLCHAR, SQLSMALLINT) {
//        unimplemented!()
//    }
//    unsafe fn assume_init(self) -> Self::InitializedType {
//        let mut nul_mark_found = false;
//
//        self.iter_mut().for_each(|x| {
//            if nul_mark_found {
//                if *x.as_mut_ptr() == 0 {
//                    nul_mark_found = true;
//                }
//            } else {
//                std::ptr::write(x.as_mut_ptr(), 0);
//            }
//        });
//
//        std::mem::transmute::<_, Self::InitializedType>(self)
//    }
//}

// TODO: Comapare attribute types: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(u32, true), SQL_ATTR_AUTO_IPD(u32, _)
#[allow(non_camel_case_types)]
// TODO: Type equality should be derived such as EqSQLUINTEGER
#[derive(rs_odbc_derive::AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcBool {
    SQL_FALSE = 0,
    SQL_TRUE = 1,
}

// TODO
//pub use SQL_COLUMN_SEARCHABLE::SQL_SEARCHABLE as SQL_PRED_SEARCHABLE;
// Special return values for SQLGetData
// SQL_NO_TOTAL = -4,

#[allow(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

pub mod env {
    use crate::{AsRawParts, Attribute, GetAttr, OdbcAttribute, OdbcBool, SetAttr};
    use crate::{SQLINTEGER, SQLPOINTER, SQLUINTEGER};
    use rs_odbc_derive::{AnsiType, EnvAttribute, EqSQLUINTEGER};

    pub trait EnvAttribute: Attribute<IdentifierType = SQLINTEGER> {}

    #[identifier(200)]
    #[derive(EnvAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_ODBC_VERSION;
    impl SetAttr<OdbcVersion> for SQL_ATTR_ODBC_VERSION {}
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_ODBC_VERSION {}

    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum OdbcVersion {
        SQL_OV_ODBC3 = 3,
        #[cfg(feature = "v3_8")]
        SQL_OV_ODBC3_80 = 380,
        #[cfg(feature = "v4")]
        SQL_OV_ODBC4 = 400,
    }
    impl AsRawParts<OdbcAttribute, SQLINTEGER> for OdbcVersion {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(201)]
    #[derive(EnvAttribute)]
    #[cfg(feature = "v3_8")]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_CONNECTION_POOLING;
    #[cfg(feature = "v3_8")]
    impl SetAttr<ConnectionPooling> for SQL_ATTR_CONNECTION_POOLING {}
    #[cfg(feature = "v3_8")]
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_POOLING {}

    #[cfg(feature = "v3_8")]
    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ConnectionPooling {
        SQL_CP_OFF = 0,
        SQL_CP_ONE_PER_DRIVER = 1,
        SQL_CP_ONE_PER_HENV = 2,
        SQL_CP_DRIVER_AWARE = 3,
    }
    #[cfg(feature = "v3_8")]
    pub use ConnectionPooling::SQL_CP_OFF as SQL_CP_DEFAULT;

    #[cfg(feature = "v3_8")]
    impl AsRawParts<OdbcAttribute, SQLINTEGER> for ConnectionPooling {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(202)]
    #[derive(EnvAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_CP_MATCH;
    impl SetAttr<CpMatch> for SQL_ATTR_CP_MATCH {}
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CP_MATCH {}

    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum CpMatch {
        SQL_CP_STRICT_MATCH = 0,
        SQL_CP_RELAXED_MATCH = 1,
    }
    pub use CpMatch::SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;

    impl AsRawParts<OdbcAttribute, SQLINTEGER> for CpMatch {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    // TODO:
    // For private driver manager
    // #[identifier(203)]
    // #[derive(EnvAttribute)]
    // pub struct SQL_ATTR_APPLICATION_KEY;
    // impl Attribute for SQL_ATTR_APPLICATION_KEY {
    //     type AttributeType = OdbcAttribute;
    //     type IdentifierType = SQLINTEGER;
    //     fn identifier() -> Self::IdentifierType { 203 }
    // }

    #[identifier(1001)]
    #[derive(EnvAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_OUTPUT_NTS;
    impl SetAttr<OdbcBool> for SQL_ATTR_OUTPUT_NTS {}
    impl GetAttr<SQLINTEGER> for SQL_ATTR_OUTPUT_NTS {}
}

pub mod conn {
    use crate::{
        AsCharRawSlice, AsMutCharRawSlice, AsRawParts, Attribute, GetAttr, SetAttr, SQLINTEGER,
        SQLPOINTER, SQLUINTEGER,
    };
    use rs_odbc_derive::{AnsiType, ConnAttribute, EqSQLUINTEGER, UnicodeType};

    pub trait ConnAttribute: Attribute<IdentifierType = SQLINTEGER> {}

    #[identifier(101)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_ACCESS_MODE;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_ACCESS_MODE {}
    impl SetAttr<AccessMode> for SQL_ATTR_ACCESS_MODE {}

    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AccessMode {
        SQL_MODE_READ_WRITE = 0,
        SQL_MODE_READ_ONLY = 1,
    }
    pub use AccessMode::SQL_MODE_READ_WRITE as SQL_MODE_DEFAULT;
    impl<T> AsRawParts<T, SQLINTEGER> for AccessMode {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(102)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_AUTOCOMMIT;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_AUTOCOMMIT {}
    impl SetAttr<AutoCommit> for SQL_ATTR_AUTOCOMMIT {}

    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AutoCommit {
        SQL_AUTOCOMMIT_OFF = 0,
        SQL_AUTOCOMMIT_ON = 1,
    }
    pub use AutoCommit::SQL_AUTOCOMMIT_ON as SQL_AUTOCOMMIT_DEFAULT;
    impl<T> AsRawParts<T, SQLINTEGER> for AutoCommit {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(113)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_CONNECTION_TIMEOUT;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_TIMEOUT {}
    impl SetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_TIMEOUT {}

    #[identifier(109)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_CURRENT_CATALOG;
    impl<T: AsCharRawSlice<SQLINTEGER>> GetAttr<T> for SQL_ATTR_CURRENT_CATALOG {}
    impl<T: AsMutCharRawSlice<SQLINTEGER>> SetAttr<T> for SQL_ATTR_CURRENT_CATALOG {}

    // TODO: Not found in documentation, only in implementation
    //#[identifier(114)]
    //#[derive(ConnAttribute)]
    //#[allow(non_camel_case_types)]
    //pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;

    //#[allow(non_camel_case_types)]
    //#[derive(AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    //pub enum DisconnectBehavior {
    //    SQL_DB_RETURN_TO_POOL = 0,
    //    SQL_DB_DISCONNECT = 1,
    //}
    //pub use DisconnectBehavior::SQL_DB_RETURN_TO_POOL as SQL_DB_DEFAULT;
    //impl<T> AsRawParts<T, SQLINTEGER> for DisconnectBehavior {
    //    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
    //        (*self as something as SQLPOINTER, 0)
    //    }
    //}

    // TODO: Seems to be Microsoft related
    //#[identifier(1207)]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_ENLIST_IN_DTC;
    //impl GetAttr<SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}
    //impl SetAttr<SQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}

    //pub enum EnlistInDtc {
    //    SQL_DTC_DONE = 0,
    //}
    //impl<T> AsRawParts<T, SQLINTEGER> for EnlistInDtc {
    //    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
    //        (*self as something as SQLPOINTER, 0)
    //    }
    //}

    // TODO: Unknown
    //#[identifier(1208)]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_ENLIST_IN_XA;

    #[identifier(103)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_LOGIN_TIMEOUT;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_LOGIN_TIMEOUT {}
    impl SetAttr<SQLUINTEGER> for SQL_ATTR_LOGIN_TIMEOUT {}

    // TODO: Seems to be deprecated
    //#[identifier(110)]
    //#[derive(ConnAttribute)]
    //#[allow(non_camel_case_types)]
    //pub struct SQL_ATTR_ODBC_CURSORS;
    //impl GetAttr<SQLULEN> for SQL_ATTR_ODBC_CURSORS {}
    //impl SetAttr<OdbcCursors> for SQL_ATTR_ODBC_CURSORS {}

    //#[allow(non_camel_case_types)]
    //#[derive(EqSQLULEN, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    //pub enum OdbcCursors {
    //    SQL_CUR_USE_IF_NEEDED = 0,
    //    SQL_CUR_USE_ODBC = 1,
    //    SQL_CUR_USE_DRIVER = 2,
    //}
    //pub use OdbcCursors::SQL_CUR_USE_DRIVER as SQL_CUR_DEFAULT;
    //impl<T> AsRawParts<T, SQLINTEGER> for OdbcCursors {
    //    fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
    //        (*self as SQLULEN as SQLPOINTER, 0)
    //    }
    //}

    #[identifier(112)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_PACKET_SIZE;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_PACKET_SIZE {}
    impl SetAttr<SQLUINTEGER> for SQL_ATTR_PACKET_SIZE {}

    //#[identifier(111)]
    //#[derive(ConnAttribute)]
    //#[allow(non_camel_case_types)]
    //pub struct SQL_ATTR_QUIET_MODE;
    //impl GetAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}
    //impl SetAttr<SQLHWND> for SQL_ATTR_PACKET_SIZE {}

    #[identifier(104)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_TRACE;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_TRACE {}
    impl SetAttr<Trace> for SQL_ATTR_TRACE {}

    #[allow(non_camel_case_types)]
    #[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Trace {
        SQL_OPT_TRACE_OFF = 0,
        SQL_OPT_TRACE_ON = 1,
    }
    pub use Trace::SQL_OPT_TRACE_OFF as SQL_OPT_TRACE_DEFAULT;
    impl<T> AsRawParts<T, SQLINTEGER> for Trace {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(105)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_TRACEFILE;
    // TODO: Is this default really?
    //pub const SQL_OPT_TRACE_FILE_DEFAULT = "\\SQL.LOG";

    // TODO: Has to be null-terminated
    impl<T: AsCharRawSlice<SQLINTEGER>> GetAttr<T> for SQL_ATTR_TRACEFILE {}
    impl<T: AsMutCharRawSlice<SQLINTEGER>> SetAttr<T> for SQL_ATTR_TRACEFILE {}

    #[identifier(106)]
    #[derive(ConnAttribute)]
    #[allow(non_camel_case_types)]
    pub struct SQL_ATTR_TRANSLATE_LIB;

    // TODO: Has to be null-terminated
    impl<T: AsCharRawSlice<SQLINTEGER>> GetAttr<T> for SQL_ATTR_TRANSLATE_LIB {}
    impl<T: AsMutCharRawSlice<SQLINTEGER>> SetAttr<T> for SQL_ATTR_TRANSLATE_LIB {}

    // TODO: Investigate this
    //#[identifier(107)]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_TRANSLATE_OPTION;
    //impl GetAttr<SQLUINTEGER> for SQL_ATTR_TRANSLATE_OPTION {}
    //impl SetAttr<SQLUINTEGER> for SQL_ATTR_TRANSLATE_OPTION {}

    // TODO: Uncertain
    //#[identifier(108)]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_TXN_ISOLATION;

    // TODO: Can only be used with `SQLGetConnectAttr`
    #[identifier(10001)]
    #[derive(ConnAttribute)]
    pub struct SQL_ATTR_AUTO_IPD;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_AUTO_IPD {}

    #[identifier(117)]
    #[cfg(feature = "v3_8")]
    #[derive(ConnAttribute)]
    pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}
    impl SetAttr<AsyncDbcFunctionsEnable> for SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE {}

    #[cfg(feature = "v3_8")]
    #[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AsyncDbcFunctionsEnable {
        SQL_ASYNC_DBC_ENABLE_OFF = 0,
        SQL_ASYNC_DBC_ENABLE_ON = 1,
    }
    pub use AsyncDbcFunctionsEnable::SQL_ASYNC_DBC_ENABLE_OFF as SQL_ASYNC_DBC_ENABLE_DEFAULT;
    impl<T> AsRawParts<T, SQLINTEGER> for AsyncDbcFunctionsEnable {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

    #[identifier(118)]
    #[cfg(feature = "v3_8")]
    #[derive(ConnAttribute)]
    pub struct SQL_ATTR_DBC_INFO_TOKEN;
    // This is set-only attribute
    impl SetAttr<SQLPOINTER> for SQL_ATTR_DBC_INFO_TOKEN {}

    #[identifier(119)]
    #[cfg(feature = "v3_8")]
    #[derive(ConnAttribute)]
    pub struct SQL_ATTR_ASYNC_DBC_EVENT;
    // TODO: It's an Event handle. Should probably implement event handle
    impl GetAttr<SQLPOINTER> for SQL_ATTR_ASYNC_DBC_EVENT {}

    // TODO: It is not 3.5 in implementation ???
    // but it says that drivers conforming to earlier versions can support this field. HMMMMMMMMMMM
    #[identifier(1209)]
    #[cfg(feature = "v3_5")]
    #[derive(ConnAttribute)]
    pub struct SQL_ATTR_CONNECTION_DEAD;
    // Can only be used with `SQLGetConnectAttr`
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_DEAD {}

    #[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ConnectionDead {
        SQL_CD_TRUE = 1,
        SQL_CD_FALSE = 0,
    }
    impl<T> AsRawParts<T, SQLINTEGER> for ConnectionDead {
        fn as_raw_parts(&self) -> (SQLPOINTER, SQLINTEGER) {
            (*self as SQLUINTEGER as SQLPOINTER, 0)
        }
    }

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
    //#[identifier(115)]
    //#[cfg(feature = "v3_51")]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_ANSI_APP;

    //#[cfg(feature = "v3_51")]
    //pub enum AnsiApp {
    //    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
    //    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
    //}

    //#[identifier(116)]
    //#[cfg(feature = "v3_8")]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_RESET_CONNECTION;
    //impl GetAttr<SQLUINTEGER> for SQL_ATTR_RESET_CONNECTION {}
    //impl SetAttr<ResetConnection> for SQL_ATTR_RESET_CONNECTION {}

    //#[cfg(feature = "v3_8")]
    //#[derive(EqSQLUINTEGER, AnsiType, UnicodeType, Debug, PartialEq, Eq, Clone, Copy)]
    //pub enum ResetConnection {
    //    SQL_RESET_CONNECTION_YES = 1,
    //}

    //#[identifier(122)]
    //#[cfg(feature = "v4")]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_CREDENTIALS;

    //#[identifier(123)]
    //#[cfg(feature = "v4")]
    //#[derive(ConnAttribute)]
    //pub struct SQL_ATTR_REFRESH_CONNECTION;

    //#[cfg(feature = "v4")]
    //pub enum RefreshConnection {
    //    SQL_REFRESH_NOW = -1,
    //    SQL_REFRESH_AUTO = 0,
    //    SQL_REFRESH_MANUAL = 1,
    //}

    // TODO: Reexport these in conn module
    // TODO: Or derive them, but still export?
    //impl ConnAttribute for crate::stmt::SQL_ATTR_METADATA_ID {}
    //impl ConnAttribute for crate::stmt::SQL_ATTR_ASYNC_ENABLE {}
}

pub mod stmt {
    //    pub trait StmtAttrbute: Attribute<TypeIdentifier=SQLINTEGER> {}

    //    #[deprecated]
    //    enum StmtOption {
    //        SQL_QUERY_TIMEOUT = 0,
    //        SQL_MAX_ROWS = 1,
    //        SQL_NOSCAN = 2,
    //        SQL_MAX_LENGTH = 3,
    //        SQL_ASYNC_ENABLE = 4,
    //        SQL_BIND_TYPE = 5,
    //        SQL_CURSOR_TYPE = 6,
    //        SQL_CONCURRENCY = 7,
    //        SQL_KEYSET_SIZE = 8,
    //        SQL_ROWSET_SIZE = 9,
    //        SQL_SIMULATE_CURSOR = 10,
    //        SQL_RETRIEVE_DATA = 11,
    //        SQL_USE_BOOKMARKS = 12,
    //        SQL_GET_BOOKMARK = 13,
    //        SQL_ROW_NUMBER = 14,
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_QUERY_TIMEOUT;
    //    impl StmtAttribute for SQL_ATTR_QUERY_TIMEOUT {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_QUERY_TIMEOUT as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_MAX_ROWS;
    //    impl StmtAttribute for SQL_ATTR_MAX_ROWS {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_MAX_ROWS as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_NOSCAN;
    //    impl StmtAttribute for SQL_ATTR_NOSCAN {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_NOSCAN as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_MAX_LENGTH;
    //    impl StmtAttribute for SQL_ATTR_MAX_LENGTH {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_MAX_LENGTH as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_CURSOR_TYPE;
    //    impl StmtAttribute for SQL_ATTR_CURSOR_TYPE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_CURSOR_TYPE as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_CONCURRENCY;
    //    impl StmtAttribute for SQL_ATTR_CONCURRENCY {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_CONCURRENCY as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_KEYSET_SIZE;
    //    impl StmtAttribute for SQL_ATTR_KEYSET_SIZE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_KEYSET_SIZE as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_SIMULATE_CURSOR;
    //    impl StmtAttribute for SQL_ATTR_SIMULATE_CURSOR {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_SIMULATE_CURSOR as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_RETRIEVE_DATA;
    //    impl StmtAttribute for SQL_ATTR_RETRIEVE_DATA {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_RETRIEVE_DATA as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_USE_BOOKMARKS;
    //    impl StmtAttribute for SQL_ATTR_USE_BOOKMARKS {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_USE_BOOKMARKS as Self::IdentifierType
    //        }
    //    }
    //
    //    #[identifier(15)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ENABLE_AUTO_IPD;
    //
    //    #[identifier(16)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
    //
    //    // The following are Header fields--------------------------------
    //
    //    // TODO: This one could be special??
    //    // Corresponds to ARD SQL_DESC_BIND_TYPE
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_BIND_TYPE;
    //    impl StmtAttribute for SQL_ATTR_ROW_BIND_TYPE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_BIND_TYPE as Self::IdentifierType
    //        }
    //    }
    //
    //    // Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
    //    #[identifier(17)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
    //
    //    // Corresponds to APD SQL_DESC_BIND_TYPE
    //    #[identifier(18)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAM_BIND_TYPE;
    //
    //    // Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
    //    #[identifier(18)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAM_OPERATION_PTR;
    //
    //    // Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
    //    #[identifier(20)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAM_STATUS_PTR;
    //
    //    // Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
    //    #[identifier(21)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
    //
    //    // Corresponds to APD SQL_DESC_ARRAY_SIZE
    //    #[identifier(22)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_PARAMSET_SIZE;
    //
    //    // Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
    //    #[identifier(23)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
    //
    //    // Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
    //    #[identifier(24)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_OPERATION_PTR;
    //
    //    // Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
    //    #[identifier(25)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_STATUS_PTR;
    //
    //    // Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
    //    #[identifier(26)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROWS_FETCHED_PTR;
    //
    //    // Corresponds to ARD SQL_DESC_ARRAY_SIZE
    //    #[identifier(27)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_ARRAY_SIZE;
    //
    //    #[identifier(29)]
    //    #[cfg(feature = "v3_8")]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ASYNC_STMT_EVENT;
    //
    //    #[identifier(30)]
    //    #[cfg(feature = "v4")]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_SAMPLE_SIZE;
    //
    //    #[identifier(31)]
    //    #[cfg(feature = "v4")]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_DYNAMIC_COLUMNS;
    //
    //    #[identifier(32)]
    //    #[cfg(feature = "v4")]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
    //
    //    #[identifier(33)]
    //    #[cfg(feature = "v4")]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
    //
    //    #[identifier(10010)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_APP_ROW_DESC;
    //
    //    #[identifier(10010)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_APP_PARAM_DESC;
    //
    //    // TODO: Write-only - Cannot be used with SetStmtAttr
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ROW_NUMBER;
    //    impl StmtAttribute for SQL_ATTR_ROW_NUMBER {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType {
    //            StmtOption::SQL_ROW_NUMBER as Self::IdentifierType
    //        }
    //    }
    //
    //    // TODO: Write-only - Cannot be used with SetStmtAttr
    //    #[identifier(10012)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_IMP_ROW_DESC;
    //
    //    // TODO: Write-only - Cannot be used with SetStmtAttr
    //    #[identifier(10013)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_IMP_PARAM_DESC;
    //
    //    #[identifier(-1)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_CURSOR_SCROLLABLE;
    //
    //    #[identifier(-2)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_CURSOR_SENSITIVITY;
    //
    //    // TODO: Not found in implementation
    //    // #[cfg(feature = "v3_8")]
    //    // SQL_ATTR_ASYNC_STMT_PCALLBACK
    //    // #[cfg(feature = "v3_8")]
    //    // SQL_ATTR_ASYNC_STMT_PCONTEXT
    //
    //    #[identifier(10014)]
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_METADATA_ID;
    //    impl GetAttr<SQLUINTEGER> for SQL_ATTR_METADATA_ID {}
    //
    //    #[derive(StmtAttribute)]
    //    pub struct SQL_ATTR_ASYNC_ENABLE;
    //    impl StmtAttribute for SQL_ATTR_ASYNC_ENABLE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLINTEGER;
    //        fn identifier() -> Self::IdentifierType { StmtOption::SQL_ASYNC_ENABLE as Self::IdentifierType }
    //    }
    //    impl GetAttr<SQLULEN> for SQL_ATTR_ASYNC_ENABLE {}
    //}

    //pub mod col {
    //    pub trait ColAttribute: Attribute<IdentifierType=SQLUSMALLINT> {}
    //
    //    #[deprecated]
    //    enum SQLColAttributes {
    //        SQL_COLUMN_COUNT = 0,
    //        SQL_COLUMN_NAME = 1,
    //        SQL_COLUMN_TYPE = 2,
    //        SQL_COLUMN_LENGTH = 3,
    //        SQL_COLUMN_PRECISION = 4,
    //        SQL_COLUMN_SCALE = 5,
    //        SQL_COLUMN_DISPLAY_SIZE = 6,
    //        SQL_COLUMN_NULLABLE = 7,
    //        SQL_COLUMN_UNSIGNED = 8,
    //        SQL_COLUMN_MONEY = 9,
    //        SQL_COLUMN_UPDATABLE = 10,
    //        SQL_COLUMN_AUTO_INCREMENT = 11,
    //        SQL_COLUMN_CASE_SENSITIVE = 12,
    //        SQL_COLUMN_SEARCHABLE = 13,
    //        SQL_COLUMN_TYPE_NAME = 14,
    //        SQL_COLUMN_TABLE_NAME = 15,
    //        SQL_COLUMN_OWNER_NAME = 16,
    //        SQL_COLUMN_QUALIFIER_NAME = 17,
    //        SQL_COLUMN_LABEL = 18,
    //    }
    //    // TODO: These constants are not found in the documentation
    //    //use SQLColAttributes::SQL_COLUMN_COUNT as SQL_COLATT_OPT_MIN;
    //    //use SQLColAttributes::SQL_COLUMN_LABEL as SQL_COLATT_OPT_MAX;
    //
    //    // This is the only header field, others are record fields
    //    #[identifier(1001)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_COUNT;
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_CONCISE_TYPE;
    //    impl Attribute for SQL_DESC_CONCISE_TYPE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_TYPE as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_DISPLAY_SIZE;
    //    impl Attribute for SQL_DESC_DISPLAY_SIZE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_DISPLAY_SIZE as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_UNSIGNED;
    //    impl Attribute for SQL_DESC_UNSIGNED {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_UNSIGNED as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_FIXED_PREC_SCALE;
    //    impl Attribute for SQL_DESC_FIXED_PREC_SCALE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_MONEY as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_UPDATABLE;
    //    impl Attribute for SQL_DESC_UPDATABLE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_UPDATABLE as Self::IdentifierType
    //        }
    //    }
    //    impl GetAttr<SQLSMALLINT> for SQL_DESC_UPDATABLE {}
    //
    //    ///// Describes the updatability of the column in the result set, not the column in the base table.
    //    //#[repr(SQLSMALLINT)]
    //    //pub enum DescUpdatable {
    //    //    SQL_ATTR_READONLY = 0,
    //    //    SQL_ATTR_WRITE = 1,
    //    //    /// It is unclear whether a column is updatable
    //    //    SQL_ATTR_READWRITE_UNKNOWN = 2,
    //    //}
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_AUTO_UNIQUE_VALUE;
    //    impl Attribute for SQL_DESC_AUTO_UNIQUE_VALUE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_AUTO_INCREMENT as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_CASE_SENSITIVE;
    //    impl Attribute for SQL_DESC_CASE_SENSITIVE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_CASE_SENSITIVE as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_SEARCHABLE;
    //    impl Attribute for SQL_DESC_SEARCHABLE {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_SEARCHABLE as Self::IdentifierType
    //        }
    //    }
    //    // TODO:
    //    // SQLColAttributes subdefines for SQL_COLUMN_SEARCHABLE These are also used by SQLGetInfo
    //    //pub enum SQL_COLUMN_SEARCHABLE {
    //    //    SQL_UNSEARCHABLE = 0,
    //    //    SQL_LIKE_ONLY = 1,
    //    //    SQL_ALL_EXCEPT_LIKE = 2,
    //    //    SQL_SEARCHABLE = 3,
    //    //}
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_TYPE_NAME;
    //    impl Attribute for SQL_DESC_TYPE_NAME {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_TYPE_NAME as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_TABLE_NAME;
    //    impl Attribute for SQL_DESC_TABLE_NAME {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_TABLE_NAME as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_SCHEMA_NAME;
    //    impl Attribute for SQL_DESC_SCHEMA_NAME {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_OWNER_NAME as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_CATALOG_NAME;
    //    impl Attribute for SQL_DESC_CATALOG_NAME {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_QUALIFIER_NAME as Self::IdentifierType
    //        }
    //    }
    //
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_LABEL;
    //    impl Attribute for SQL_DESC_LABEL {
    //        type AttributeType = OdbcAttribute;
    //        type IdentifierType = SQLUSMALLINT;
    //        fn identifier() -> Self::IdentifierType {
    //            SQLColAttributes::SQL_COLUMN_LABEL as Self::IdentifierType
    //        }
    //    }
    //
    //    #[identifier(22)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_BASE_COLUMN_NAME;
    //
    //    #[identifier(23)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_BASE_TABLE_NAME;
    //
    //    #[identifier(27)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_LITERAL_PREFIX;
    //
    //    #[identifier(28)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_LITERAL_SUFFIX;
    //
    //    #[identifier(29)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_LOCAL_TYPE_NAME;
    //
    //    #[identifier(32)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_NUM_PREC_RADIX;
    //
    //    #[identifier(1002)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_TYPE;
    //
    //    #[identifier(1003)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_LENGTH;
    //
    //    #[identifier(1005)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_PRECISION;
    //
    //    #[identifier(1006)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_SCALE;
    //
    //    #[identifier(1008)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_NULLABLE;
    //
    //    #[identifier(1011)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_NAME;
    //
    //    #[identifier(1012)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_UNNAMED;
    //    impl GetAttr<SQLSMALLINT> for SQL_DESC_UNNAMED {}
    //
    //    //#[repr(SQLSMALLINT)]
    //    //pub enum DescUnnamed {
    //    //    /// SQL_DESC_NAME field of the IRD contains a column alias or a column name
    //    //    SQL_NAMED = 0,
    //    //    /// There is no column name or column alias
    //    //    SQL_UNNAMED = 1,
    //    //}
    //
    //    #[identifier(1013)]
    //    #[derive(ColAttribute)]
    //    pub struct SQL_DESC_OCTET_LENGTH;
    //
    //    // TODO: These are unknown, find their values
    //    // SQL_DESC_NUM_PREC_RADIX, SQL_DESC_CONCISE_TYPE, SQL_DESC_TYPE
    //}
    //
    //pub mod diag {
    //    pub use crate::{SQLSMALLINT, SQLLEN, SQLCHAR, SQLWCHAR, SQLINTEGER, SQLRETURN};
    //    pub use super::{GetAttr, AsOdbcChar};
    //
    //    pub trait DiagField: Attribute<IdentifierType=SQLSMALLINT> { }
    //
    //    // Header fields -----------------------------------------------------------------
    //    #[identifier(-1249)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_CURSOR_ROW_COUNT;
    //    impl GetAttr<SQLLEN> for SQL_DIAG_CURSOR_ROW_COUNT {}
    //
    //    #[identifier(7)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_DYNAMIC_FUNCTION;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_DYNAMIC_FUNCTION {}
    //
    //    #[identifier(12)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_DYNAMIC_FUNCTION_CODE;
    //    impl GetAttr<SQLINTEGER> for SQL_DIAG_DYNAMIC_FUNCTION_CODE {}
    //
    //    #[identifier(2)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_NUMBER;
    //    impl GetAttr<SQLINTEGER> for SQL_DIAG_NUMBER {}
    //
    //    #[identifier(1)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_RETURNCODE;
    //    impl GetAttr<SQLRETURN> for SQL_DIAG_RETURNCODE {}
    //
    //    #[identifier(3)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_ROW_COUNT;
    //    impl GetAttr<SQLLEN> for SQL_DIAG_ROW_COUNT {}
    //
    //    // Record fields ---------------------------------------------------------------
    //    #[identifier(8)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_CLASS_ORIGIN;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_CLASS_ORIGIN {}
    //
    //    #[identifier(-1247)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_COLUMN_NUMBER;
    //    impl GetAttr<SQLINTEGER> for SQL_DIAG_COLUMN_NUMBER {}
    //
    //    //#[repr(SQLINTEGER)]
    //    //pub enum ColumnNumber {
    //    //    SQL_NO_COLUMN_NUMBER = -1,
    //    //    SQL_COLUMN_NUMBER_UNKNOWN = -2,
    //    //}
    //
    //    #[identifier(10)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_CONNECTION_NAME;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_CONNECTION_NAME {}
    //
    //    #[identifier(6)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_MESSAGE_TEXT;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_MESSAGE_TEXT {}
    //
    //    #[identifier(5)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_NATIVE;
    //    impl GetAttr<SQLINTEGER> for SQL_DIAG_NATIVE {}
    //
    //    #[identifier(-1248)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_ROW_NUMBER;
    //    impl GetAttr<SQLLEN> for SQL_DIAG_ROW_NUMBER {}
    //
    //    pub enum RowNumber {
    //        SQL_NO_ROW_NUMBER = -1,
    //        SQL_ROW_NUMBER_UNKNOWN = -2,
    //    }
    //
    //    #[identifier(11)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_SERVER_NAME;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_SERVER_NAME {}
    //
    //    #[identifier(4)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_SQLSTATE;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_SQLSTATE {}
    //
    //    #[identifier(9)]
    //    #[derive(DiagField)]
    //    pub struct SQL_DIAG_SUBCLASS_ORIGIN;
    //    impl<T: AsOdbcChar> GetAttr<T> for SQL_DIAG_SUBCLASS_ORIGIN {}
    //}
    //
    //pub mod info {
    //    pub trait InfoType: Attribute<TypeIdentifier=SQLUSMALLINT> {}
    //}
    //
    //pub mod desc {
    //    pub trait DescField {
    //        fn identifier() -> SQLSMALLINT;
    //    }
    //    pub enum DescFieldIdentifier {
    //        // Header fields
    //        SQL_DESC_ALLOC_TYPE = 1099,
    //        SQL_DESC_ARRAY_SIZE = 20,
    //        SQL_DESC_ARRAY_STATUS_PTR = 21,
    //        SQL_DESC_BIND_OFFSET_PTR = 24,
    //        SQL_DESC_BIND_TYPE = 25,
    //        SQL_DESC_ROWS_PROCESSED_PTR = 34,
    //
    //        // Record fields
    //        SQL_DESC_DATA_PTR = 1010,
    //        SQL_DESC_DATETIME_INTERVAL_CODE = 1007,
    //        SQL_DESC_DATETIME_INTERVAL_PRECISION = 26,
    //        SQL_DESC_INDICATOR_PTR = 1009,
    //        SQL_DESC_OCTET_LENGTH_PTR = 1004,
    //        SQL_DESC_PARAMETER_TYPE = 33,
    //        #[cfg(feature = "v3_5")]
    //        SQL_DESC_ROWVER = 35,
    //        nQL_DESC_UNNAMED = 1012,
    //
    //        // TODO: Not mentioned anywhere in the documentation
    //        // SQL_DESC_MAXIMUM_SCALE = 30,
    //        // SQL_DESC_MINIMUM_SCALE = 31,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_CHARACTER_SET_CATALOG = 1018,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_CHARACTER_SET_SCHEMA = 1019,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_CHARACTER_SET_NAME = 1020,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_COLLATION_CATALOG = 1015,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_COLLATION_SCHEMA = 1016,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_COLLATION_NAME = 1017,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_USER_DEFINED_TYPE_CATALOG = 1026,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_USER_DEFINED_TYPE_SCHEMA = 1027,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_USER_DEFINED_TYPE_NAME = 1028,
    //        #[cfg(feature = "v4")]
    //        SQL_DESC_MIME_TYPE = 36,
    //    }
    //
    //    pub enum SQL_DESC_ALLOC_TYPE {
    //        SQL_DESC_ALLOC_AUTO = 1,
    //        SQL_DESC_ALLOC_USER = 2,
    //    }
    //
    //    pub enum SQL_DESC_ARRAY_STATUS_PTR {
    //        SQL_PARAM_SUCCESS = 0,
    //        SQL_PARAM_SUCCESS_WITH_INFO = 6,
    //        SQL_PARAM_ERROR = 5,
    //        SQL_PARAM_UNUSED = 7,
    //        SQL_PARAM_DIAG_UNAVAILABLE = 1,
    //        // TODO: What are these?
    //        //SQL_PARAM_PROCEED = 0,
    //        //SQL_PARAM_IGNORE = 1,
    //    }
}

pub mod desc {}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES
