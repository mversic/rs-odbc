#![allow(non_camel_case_types)]

pub mod api;
pub mod c_types;
pub mod sql_types;
pub mod sqlreturn;

use std::cell::UnsafeCell;

pub use {api::*, c_types::*, sql_types::*, sqlreturn::*};
pub use handle::{SQLHANDLE, SQLHENV, SQLHSTMT, SQLHDBC, SQLHDESC, SQL_NULL_HANDLE}; // TODO: SQLHWND

// TODO: Think about making it newtype with private field for both
type MutSQLPOINTER = *mut std::ffi::c_void;
type ConstSQLPOINTER = *const UnsafeCell<std::ffi::c_void>;

pub trait AsMutPtr<P, LEN> {
    // TODO: Consider extracting StrLen to a separate trait
    type StrLen;
    // TODO: Could return Self::StrLen instead of LEN???
    fn as_mut_ptr(&mut self) -> (*mut Self, LEN);
}
pub trait AsPtr<P, LEN> {
    // TODO: Could return Self::StrLen instead of LEN???
    fn as_ptr(&self) -> (*const UnsafeCell<Self>, LEN);
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


// TODO: Comapare attribute types: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(i32, true)
pub enum OdbcBool {
    SQL_FALSE = 0,
    SQL_TRUE = 1,
}

pub mod handle {
    use std::mem::MaybeUninit;
    use std::marker::PhantomData;
    use std::thread::panicking;
    use super::env::{EnvState, E1};
    use super::conn::{ConnState, C1, C4};
    use super::stmt::{StmtState, S1};
    use super::desc::{DescState, D1};
    use crate::SQLSMALLINT;
    use crate::SQL_SUCCESS;
    use crate::api::FreeHandle;

    pub trait Version {}
    pub trait KnownVersion: Version {}
    pub enum V_UNDEFINED {}
    impl Version for V_UNDEFINED {}

    pub enum V3 {}
    impl Version for V3 {}
    impl KnownVersion for V3 {}

    pub enum V3_8 {}
    impl Version for V3_8 {}
    impl KnownVersion for V3_8 {}

    pub enum V4 {}
    impl Version for V4 {}
    impl KnownVersion for V4 {}

    pub trait AsSQLHANDLE {
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE;
    }

    pub trait Handle: AsSQLHANDLE {
        fn identifier() -> SQLSMALLINT;
    }

    pub trait Allocate<'env: 'conn, 'conn>: Handle {
        type SrcHandle: Handle;
    }

    pub trait SQLCancelHandle: Handle {}
    pub trait SQLCompleteAsyncHandle: Handle {}
    pub trait SQLEndTranHandle: Handle {}

    //pub struct SQL_HANDLE_ENV;
    //impl HandleIdentifier for SQL_HANDLE_ENV {
    //    fn identifier() -> SQLSMALLINT { 1 }
    //}

    //pub struct SQL_HANDLE_DBC;
    //impl HandleIdentifier for SQL_HANDLE_DBC {
    //    fn identifier() -> SQLSMALLINT { 2 }
    //}

    //pub struct SQL_HANDLE_STMT;
    //impl HandleIdentifier for SQL_HANDLE_STMT {
    //    fn identifier() -> SQLSMALLINT { 3 }
    //}

    //pub struct SQL_HANDLE_DESC;
    //impl HandleIdentifier for SQL_HANDLE_DESC {
    //    fn identifier() -> SQLSMALLINT { 4 }
    //}

    // TODO: But must it not be a void* in the end? It is void* in unixODBC
    // TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
    #[repr(C)]
    //#[cfg(feature = "RUSTC_IS_STABLE")]
    pub struct RawHandle{ _private: [u8; 0] }
    //#[cfg(feature = "RUSTC_IS_NIGHTLY")]
    //pub extern type RawHandle;

    // TODO: Think about making it newtype with private field
    // This type must not be public ever because of the issues around Drop
    pub type SQLHANDLE = *mut RawHandle;

    type HENV = SQLHANDLE;
    type HDBC = SQLHANDLE;
    type HSTMT = SQLHANDLE;
    type HDESC = SQLHANDLE;

    /// An environment is a global context which holds information such as:
    /// * The environment's state
    /// * The current environment-level diagnostics
    /// * The handles of connections currently allocated on the environment
    /// * The current settings of each environment attribute
    ///
    /// Environment handle is always used in calls to SQLDataSources and SQLDrivers and
    /// sometimes in calls to SQLAllocHandle, SQLEndTran, SQLFreeHandle, SQLGetDiagField, and
    /// SQLGetDiagRec.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/environment-handles
    #[repr(transparent)]
    pub struct SQLHENV<V: Version, E: EnvState> {
        handle: SQLHANDLE,
        version: PhantomData<V>,
        state: PhantomData<E>,
    }
    impl SQLHENV<V_UNDEFINED, E1> {
        pub fn new() -> MaybeUninit<Self> {
            MaybeUninit::uninit()
        }
    }
    impl<V: Version, E: EnvState> Handle for SQLHENV<V, E> {
        fn identifier() -> SQLSMALLINT { 1 }
    }
    impl<'env: 'conn, 'conn> Allocate<'env, 'conn> for SQLHENV<V_UNDEFINED, E1> {
        type SrcHandle = SQL_NULL_HANDLE;
    }
    impl<V: KnownVersion> SQLEndTranHandle for SQLHENV<V, E1> {}
    impl<V: Version, E: EnvState> AsSQLHANDLE for SQLHENV<V, E> {
        #[allow(non_snake_case)]
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
            self.handle
        }
    }
    impl<V: Version, E: EnvState> Drop for SQLHENV<V, E> {
        fn drop(&mut self) {
            let ret = unsafe { FreeHandle(Self::identifier(), self.as_SQLHANDLE()) };

            if ret == SQL_SUCCESS {
                unimplemented!();
            } else if !panicking() {
                unimplemented!();
            }
        }
    }

    /// Connection handle identifies a structure that contains connection information, such as the following:
    /// * The state of the connection
    /// * The current connection-level diagnostics
    /// * The handles of statements and descriptors currently allocated on the connection
    /// * The current settings of each connection attribute
    ///
    /// Connection handle is used when:
    /// * Connecting to the data source (SQLConnect, SQLDriverConnect, or SQLBrowseConnect)
    /// * Disconnecting from the data source (SQLDisconnect)
    /// * Getting information about the driver and data source (SQLGetInfo)
    /// * Retrieving diagnostics (SQLGetDiagField and SQLGetDiagRec) * Performing transactions (SQLEndTran)
    /// * Setting and getting connection attributes (SQLSetConnectAttr and SQLGetConnectAttr)
    /// * Getting the native format of an SQL statement (SQLNativeSql)
    ///
    /// Connection handles are allocated with SQLAllocHandle and freed with SQLFreeHandle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/connection-handles
    #[repr(transparent)]
    pub struct SQLHDBC<'env, V: KnownVersion, C: ConnState> {
        handle: SQLHANDLE,
        version: PhantomData<V>,
        state: PhantomData<C>,
        parent: PhantomData<&'env SQLHENV<V, E1>>
    }
    impl<'env: 'conn, 'conn, V: KnownVersion> Allocate<'env, 'conn> for SQLHDBC<'env, V, C1> {
        type SrcHandle = SQLHENV<V, E1>;
    }
    impl<V: KnownVersion, C: ConnState> Handle for SQLHDBC<'_, V, C> {
        fn identifier() -> SQLSMALLINT { 2 }
    }
    impl<V: KnownVersion> SQLHDBC<'_, V, C1> {
        pub fn new() -> MaybeUninit<Self> {
            MaybeUninit::uninit()
        }
    }
    impl<V: KnownVersion, C: ConnState> SQLCancelHandle for SQLHDBC<'_, V, C> {}
    impl<V: KnownVersion, C: ConnState> SQLCompleteAsyncHandle for SQLHDBC<'_, V, C> {}
    impl<V: KnownVersion, C: ConnState> SQLEndTranHandle for SQLHDBC<'_, V, C> {}
    impl<V: KnownVersion, C: ConnState> AsSQLHANDLE for SQLHDBC<'_, V, C> {
        #[allow(non_snake_case)]
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
            self.handle
        }
    }
    impl<V: KnownVersion, C: ConnState> Drop for SQLHDBC<'_, V, C> {
        fn drop(&mut self) {
            let ret = unsafe { FreeHandle(Self::identifier(), self.as_SQLHANDLE()) };

            if ret == SQL_SUCCESS {
                unimplemented!();
            } else if !panicking() {
                unimplemented!();
            }
        }
    }

    /// Statement handle consists of all of the information associated with a SQL statement,
    /// such as any result sets created by the statement and parameters used in the execution
    /// of the statement. A statement is associated with a single connection, and there can be
    /// multiple statements on that connection. The statement handle contains statement
    /// information, such as:
    /// * The statement's state
    /// * The current statement-level diagnostics
    /// * The addresses of the application variables bound to the statement's parameters and result set columns
    /// * The current settings of each statement attribute
    ///
    /// Statement handles are used in most ODBC functions. Notably, they are used:
    /// * to bind parameters and result set columns (SQLBindParameter and SQLBindCol)
    /// * to prepare and execute statements (SQLPrepare, SQLExecute, and SQLExecDirect)
    /// * to retrieve metadata (SQLColAttribute and SQLDescribeCol)
    /// * to fetch results (SQLFetch), and retrieve diagnostics (SQLGetDiagField and SQLGetDiagRec)
    /// * in catalog functions (SQLColumns, SQLTables, ...)
    /// * in number of other functions.
    ///
    /// Statement handles are allocated with SQLAllocHandle and freed with SQLFreeHandle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/statement-handles
    #[repr(transparent)]
    pub struct SQLHSTMT<'env: 'conn, 'conn, V: KnownVersion, S: StmtState> {
        handle: SQLHANDLE,
        version: PhantomData<V>,
        state: PhantomData<S>,
        parent: PhantomData<&'conn SQLHDBC<'env, V, C4>>
    }
    impl<V: KnownVersion> SQLHSTMT<'_, '_, V, S1> {
        pub fn new() -> MaybeUninit<Self> {
            MaybeUninit::uninit()
        }
    }
    // TODO: Why is this 'env + bound required here on Version?
    impl<'env: 'conn, 'conn, V: 'env + KnownVersion> Allocate<'env, 'conn> for SQLHSTMT<'_, '_, V, S1> {
        type SrcHandle = SQLHDBC<'env, V, C4>;
    }
    impl<V: KnownVersion, S: StmtState> Handle for SQLHSTMT<'_, '_, V, S> {
        fn identifier() -> SQLSMALLINT { 3 }
    }
    impl<V: KnownVersion, S: StmtState> SQLCancelHandle for SQLHSTMT<'_, '_, V, S> {}
    impl<V: KnownVersion, S: StmtState> SQLCompleteAsyncHandle for SQLHSTMT<'_, '_, V, S> {}
    impl<V: KnownVersion, S: StmtState> AsSQLHANDLE for SQLHSTMT<'_, '_, V, S> {
        #[allow(non_snake_case)]
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
            self.handle
        }
    }
    impl<V: KnownVersion, S: StmtState> Drop for SQLHSTMT<'_, '_, V, S> {
        fn drop(&mut self) {
            let ret = unsafe { FreeHandle(Self::identifier(), self.as_SQLHANDLE()) };

            if ret == SQL_SUCCESS {
                unimplemented!();
            } else if !panicking() {
                unimplemented!();
            }
        }
    }

    /// A descriptor is a collection of metadata that describes the parameters of an SQL
    /// statement or the columns of a result set. Thus, a descriptor can fill four roles:
    /// * (APD)Application Parameter Descriptor:
    ///     Contains information about the application buffers bound to the parameters in an
    ///     SQL statement, such as their addresses, lengths, and C data types.
    /// * (IPD)Implementation Parameter Descriptor:
    ///     Contains information about the parameters in an SQL statement, such as their SQL
    ///     data types, lengths, and nullability.
    /// * (ARD)Application Row Descriptor:
    ///     Contains information about the application buffers bound to the columns in a
    ///     result set, such as their addresses, lengths, and C data types.
    /// * (IRD)Implementation Row Descriptor:
    ///     Contains information about the columns in a result set, such as their SQL data
    ///     types, lengths, and nullability.
    ///
    /// Four descriptors are allocated automatically when a statement is allocated, but
    /// applications can also allocate descriptors with SQLAllocHandle. They are allocated on
    /// a connection and can be associated with one or more statements on that connection to
    /// fulfill the role of an APD or ARD on those statements.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/descriptor-handles
    #[repr(transparent)]
    pub struct SQLHDESC<'env: 'conn, 'conn, V: KnownVersion, D: DescState> {
        handle: SQLHANDLE,
        version: PhantomData<V>,
        state: PhantomData<D>,
        parent: PhantomData<&'conn SQLHDBC<'env, V, C4>>,
    }
    impl<V: KnownVersion> SQLHDESC<'_, '_, V, D1> {
        pub fn new() -> MaybeUninit<Self> {
            MaybeUninit::uninit()
        }
    }
    impl<'env: 'conn, 'conn, V: KnownVersion> Allocate<'env, 'conn> for SQLHDESC<'env, 'conn, V, D1> {
        type SrcHandle = SQLHDBC<'env, V, C4>;
    }
    impl<V: KnownVersion, D: DescState> Handle for SQLHDESC<'_, '_, V, D> {
        fn identifier() -> SQLSMALLINT { 4 }
    }
    impl<V: KnownVersion, D: DescState> AsSQLHANDLE for SQLHDESC<'_, '_, V, D> {
        #[allow(non_snake_case)]
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
            self.handle
        }
    }
    impl<V: KnownVersion, D: DescState> Drop for SQLHDESC<'_, '_, V, D> {
        fn drop(&mut self) {
            let ret = unsafe { FreeHandle(Self::identifier(), self.as_SQLHANDLE()) };

            if ret == SQL_SUCCESS {
                unimplemented!();
            } else if !panicking() {
                unimplemented!();
            }
        }
    }

    pub struct SQL_NULL_HANDLE;
    impl Handle for SQL_NULL_HANDLE {
        fn identifier() -> SQLSMALLINT { 0 }
    }
    impl AsSQLHANDLE for SQL_NULL_HANDLE {
        #[allow(non_snake_case)]
        fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
            std::ptr::null_mut()
        }
    }

    // TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
    // This is unixOBDC value
    //type SQLHWND = MutSQLPOINTER;
}

// TODO
//pub use SQL_COLUMN_SEARCHABLE::SQL_SEARCHABLE as SQL_PRED_SEARCHABLE;
// Special return values for SQLGetData
// SQL_NO_TOTAL = -4,

pub mod env {
    use super::AnsiType;
    use odbc_derive::EnvAttribute;
    use odbc_derive::AnsiType;

    use std::convert::TryFrom;
    use super::GetAttr;
    use super::SetAttr;
    use super::SQLINTEGER;
    use super::SQLUINTEGER;
    use super::OdbcBool;
    use super::OdbcAttribute;
    use super::Attribute;
    use super::AsMutPtr;

    // TODO: ValuePtr must be a null-terminated string for EnvAttr
    pub trait EnvAttribute: Attribute<IdentifierType=SQLINTEGER> {}

    pub trait EnvState {}
    pub struct E1;
    impl EnvState for E1 {}

    #[identifier(200)]
    #[derive(EnvAttribute)]
    pub struct SQL_ATTR_ODBC_VERSION;
    impl SetAttr<OdbcVersion> for SQL_ATTR_ODBC_VERSION {}
    impl GetAttr<SQLINTEGER> for SQL_ATTR_ODBC_VERSION {}

    #[repr(i32)]
    #[derive(AnsiType, Clone, Copy)]
    pub enum OdbcVersion {
        SQL_OV_ODBC3 = 3,
        #[cfg(feature = "v3_8")]
        SQL_OV_ODBC3_80 = 380,
        #[cfg(feature = "v4")]
        SQL_OV_ODBC4 = 400,
    }

    impl AsMutPtr<OdbcAttribute, SQLINTEGER> for OdbcVersion {
        type StrLen = ();

        fn as_mut_ptr(&mut self) -> (*mut Self, SQLINTEGER) {
            // TODO: What is len type?
            (self as *mut _, 0)
        }
    }
    impl TryFrom<SQLINTEGER> for OdbcVersion {
        type Error = SQLINTEGER;

        fn try_from(source: SQLINTEGER) -> Result<Self, Self::Error> {
            unimplemented!()
        }
    }
    impl PartialEq<SQLINTEGER> for OdbcVersion {
        fn eq(&self, other: &SQLINTEGER) -> bool { *self as SQLINTEGER == *other }
    }
    impl PartialEq<OdbcVersion> for SQLINTEGER {
        fn eq(&self, other: &OdbcVersion) -> bool { other == self }
    }

    #[identifier(201)]
    #[derive(EnvAttribute)]
    #[cfg(feature = "v3_8")]
    pub struct SQL_ATTR_CONNECTION_POOLING;
    impl SetAttr<ConnectionPooling> for SQL_ATTR_CONNECTION_POOLING {}
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_POOLING {}

    #[repr(u32)]
    #[derive(AnsiType, Clone, Copy)]
    pub enum ConnectionPooling {
        SQL_CP_OFF = 0,
        SQL_CP_ONE_PER_DRIVER = 1,
        SQL_CP_ONE_PER_HENV = 2,
        SQL_CP_DRIVER_AWARE = 3,
    }
    pub use ConnectionPooling::SQL_CP_OFF as SQL_CP_DEFAULT;

    impl AsMutPtr<OdbcAttribute, SQLINTEGER> for ConnectionPooling {
        type StrLen = ();

        fn as_mut_ptr(&mut self) -> (*mut Self, SQLINTEGER) {
            // TODO: What is len type?
            (self as *mut _, 0)
        }
    }
    impl TryFrom<SQLUINTEGER> for ConnectionPooling {
        type Error = SQLUINTEGER;

        fn try_from(source: SQLUINTEGER) -> Result<Self, Self::Error> {
            unimplemented!()
        }
    }
    impl PartialEq<SQLUINTEGER> for ConnectionPooling {
        fn eq(&self, other: &SQLUINTEGER) -> bool { *self as SQLUINTEGER == *other }
    }
    impl PartialEq<ConnectionPooling> for SQLUINTEGER {
        fn eq(&self, other: &ConnectionPooling) -> bool { other == self }
    }

    #[identifier(202)]
    #[derive(EnvAttribute)]
    pub struct SQL_ATTR_CP_MATCH;
    impl SetAttr<CpMatch> for SQL_ATTR_CP_MATCH {}
    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CP_MATCH {}

    #[repr(u32)]
    #[derive(AnsiType, Clone, Copy)]
    pub enum CpMatch {
        SQL_CP_STRICT_MATCH = 0,
        SQL_CP_RELAXED_MATCH = 1,
    }
    pub use CpMatch::SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;

    impl AsMutPtr<OdbcAttribute, SQLINTEGER> for CpMatch {
        type StrLen = ();

        fn as_mut_ptr(&mut self) -> (*mut Self, SQLINTEGER) {
            // TODO: What is len type?
            (self as *mut _, 0)
        }
    }
    impl TryFrom<SQLUINTEGER> for CpMatch {
        type Error = SQLUINTEGER;

        fn try_from(source: SQLUINTEGER) -> Result<Self, Self::Error> {
            unimplemented!()
        }
    }
    impl PartialEq<SQLUINTEGER> for CpMatch {
        fn eq(&self, other: &SQLUINTEGER) -> bool { *self as SQLUINTEGER == *other }
    }
    impl PartialEq<CpMatch> for SQLUINTEGER {
        fn eq(&self, other: &CpMatch) -> bool { other == self }
    }

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
    pub struct SQL_ATTR_OUTPUT_NTS;
    impl SetAttr<OdbcBool> for SQL_ATTR_OUTPUT_NTS {}
    impl GetAttr<SQLINTEGER> for SQL_ATTR_OUTPUT_NTS {}
}

pub mod conn {
//    pub use crate::{SQLLEN, SQLCHAR, SQLWCHAR, SQLINTEGER, SQLRETURN};
//    pub use super::{GetAttr, AsAscii, AsUnicode};
//    pub trait ConnAttribute: Attribute<IdentifierType=SQLINTEGER> {}

    pub trait ConnState {}
    pub struct C1;
    impl ConnState for C1 {}
    pub struct C4;
    impl ConnState for C4 {}

//    #[deprecated]
//    enum ConnectionAttr {
//        SQL_ACCESS_MODE = 101,
//        SQL_AUTOCOMMIT = 102,
//        SQL_LOGIN_TIMEOUT = 103,
//        SQL_OPT_TRACE = 104,
//        SQL_OPT_TRACEFILE = 105,
//        SQL_TRANSLATE_DLL = 106,
//        SQL_TRANSLATE_OPTION = 107,
//        SQL_TXN_ISOLATION = 108,
//        SQL_CURRENT_QUALIFIER = 109,
//        SQL_ODBC_CURSORS = 110,
//        SQL_QUIET_MODE = 111,
//        SQL_PACKET_SIZE = 112,
//    }
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ACCESS_MODE;
//    impl Attribute for SQL_ATTR_ACCESS_MODE {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType { ConnectionAttr::SQL_ACCESS_MODE as Self::IdentifierType }
//    }
//    impl<T: AccessMode> SetAttr<T> for SQL_ATTR_ACCESS_MODE {}
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_ACCESS_MODE {}
//
//    enum AccessMode {
//        SQL_MODE_READ_WRITE = 0,
//        SQL_MODE_READ_ONLY = 1,
//    }
//    impl AsRef<SQLUINTEGER> for AccessMode {
//        fn as_ref(&self) -> &SQLUINTEGER {
//            &(*self as SQLUINTEGER)
//        }
//    }
//    impl TryFrom<SQLUINTEGER> for AccessMode {
//        type Error = SQLUINTEGER;
//        fn try_from(source: SQLUINTEGER) -> Result<Self, Self::Error> {
//            match SQLUINTEGER {
//                x => Ok(AccessMode::SQL_MODE_READ_WRITE),
//                y => Ok(AccessMode::SQL_MODE_READ_ONLY),
//                _ => Err(Error),
//            }
//        }
//    }
//    pub use AccessMode::SQL_MODE_READ_WRITE as SQL_MODE_DEFAULT;
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_AUTOCOMMIT;
//    impl Attribute for SQL_ATTR_AUTOCOMMIT {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_AUTOCOMMIT as Self::IdentifierType
//        }
//    }
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_AUTOCOMMIT {}
//
//    enum AutoCommit {
//        SQL_AUTOCOMMIT_OFF = 0,
//        SQL_AUTOCOMMIT_ON = 1,
//    }
//    pub use AutoCommit::SQL_AUTOCOMMIT_ON as SQL_AUTOCOMMIT_DEFAULT;
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_CONNECTION_TIMEOUT;
//    impl Attribute for SQL_ATTR_CONNECTION_TIMEOUT {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            113
//        }
//    }
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_TIMEOUT {}
//    impl SetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_TIMEOUT {}
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_CURRENT_CATALOG;
//    impl Attribute for SQL_ATTR_CURRENT_CATALOG {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_CURRENT_QUALIFIER as Self::IdentifierType
//        }
//    }
//    impl<T: AsOdbcChar> GetAttr<T> for SQL_ATTR_CURRENT_CATALOG {}
//    impl<T: AsOdbcChar> SetAttr<T> for SQL_ATTR_CURRENT_CATALOG {}
//
//    #[identifier(114)]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;
//
//    pub enum DisconnectBehavior {
//        SQL_DB_RETURN_TO_POOL = 0,
//        SQL_DB_DISCONNECT = 1,
//    }
//    pub use SQL_DB_RETURN_TO_POOL as SQL_DB_DEFAULT;
//
//    #[identifier(1207)]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ENLIST_IN_DTC;
//    impl GetAttr<MutSQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}
//    impl SetAttr<ConstSQLPOINTER> for SQL_ATTR_ENLIST_IN_DTC {}
//
//    pub enum EnlistInDtc {
//        SQL_DTC_DONE = 0,
//    }
//
//    #[identifier(1208)]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ENLIST_IN_XA;
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_LOGIN_TIMEOUT;
//    impl Attribute for SQL_ATTR_LOGIN_TIMEOUT {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_LOGIN_TIMEOUT as Self::IdentifierType
//        }
//    }
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_LOGIN_TIMEOUT {}
//    impl SetAttr<SQLUINTEGER> for SQL_ATTR_LOGIN_TIMEOUT {}
//
//    // TODO: Is this or isn't it driver dependent?
//    //pub const SQL_LOGIN_TIMEOUT_DEFAULT: SQLUINTEGER> = 15;
//
//    // TODO: Consider removing this. Read the docs
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ODBC_CURSORS;
//    impl Attribute for SQL_ATTR_ODBC_CURSORS {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_ODBC_CURSORS as Self::IdentifierType
//        }
//    }
//    impl GetAttr<SQLULEN> for SQL_ATTR_ODBC_CURSORS {}
//
//    pub enum OdbcCursors {
//        SQL_CUR_USE_IF_NEEDED = 0,
//        SQL_CUR_USE_ODBC = 1,
//        SQL_CUR_USE_DRIVER = 2,
//    }
//    pub use OdbcCursors::SQL_CUR_USE_DRIVER as SQL_CUR_DEFAULT;
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_PACKET_SIZE;
//    impl Attribute for SQL_ATTR_PACKET_SIZE {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_PACKET_SIZE as Self::IdentifierType
//        }
//    }
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_PACKET_SIZE {}
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_QUIET_MODE;
//    impl Attribute for SQL_ATTR_QUIET_MODE {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_QUIET_MODE as Self::IdentifierType
//        }
//    }
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_TRACE;
//    impl Attribute for SQL_ATTR_TRACE {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_OPT_TRACE as Self::IdentifierType
//        }
//    }
//    impl GetAttr<UINTEGER> for SQL_ATTR_TRACE {}
//
//    pub enum Trace {
//        SQL_OPT_TRACE_OFF = 0,
//        SQL_OPT_TRACE_ON = 1,
//    }
//    pub use Trace::SQL_OPT_TRACE_OFF as SQL_OPT_TRACE_DEFAULT;
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_TRACEFILE;
//    impl Attribute for SQL_ATTR_TRACEFILE {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_OPT_TRACEFILE as Self::IdentifierType
//        }
//    }
//    // TODO: Has to be null-terminated
//    //impl<T: AsOdbcChar> GetAttr<T> for SQL_ATTR_TRACEFILE {}
//    //impl<T: AsOdbcChar> SetAttr<T> for SQL_ATTR_TRACEFILE {}
//    //pub const SQL_OPT_TRACE_FILE_DEFAULT = "\\SQL.LOG";
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_TRANSLATE_LIB;
//    impl Attribute for SQL_ATTR_TRANSLATE_LIB {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_TRANSLATE_DLL as Self::IdentifierType
//        }
//    }
//    // TODO: Has to be null-terminated
//    //impl<T: AsOdbcChar> GetAttr<T> for SQL_ATTR_TRANSLATE_LIB {}
//    //impl<T: AsOdbcChar> SetAttr<T> for SQL_ATTR_TRANSLATE_LIB {}
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_TRANSLATE_OPTION;
//    impl Attribute for SQL_ATTR_TRANSLATE_OPTION {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_TRANSLATE_OPTION as Self::IdentifierType
//        }
//    }
//
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_TXN_ISOLATION;
//    impl Attribute for SQL_ATTR_TXN_ISOLATION {
//        type AttributeType = OdbcAttribute;
//        type IdentifierType = SQLINTEGER;
//        fn identifier() -> Self::IdentifierType {
//            ConnectionAttr::SQL_TXN_ISOLATION as Self::IdentifierType
//        }
//    }
//
//    // TODO: Can only be used with `SQLGetConnectAttr`
//    #[identifier(10001)]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_AUTO_IPD;
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_AUTO_IPD {}
//
//    #[identifier(117)]
//    #[cfg(feature = "v3_8")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE;
//
//    #[cfg(feature = "v3_8")]
//    pub enum AsyncDbcFunctionsEnable {
//        SQL_ASYNC_DBC_ENABLE_OFF = 0,
//        SQL_ASYNC_DBC_ENABLE_ON = 1,
//    }
//    pub use AsyncDbcFunctionsEnable::SQL_ASYNC_DBC_ENABLE_OFF as SQL_ASYNC_DBC_ENABLE_DEFAULT;
//
//    #[identifier(118)]
//    #[cfg(feature = "v3_8")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_DBC_INFO_TOKEN;
//    // This is set-only attribute
//    impl SetAttr<ConstSQLPOINTER> for SQL_ATTR_DBC_INFO_TOKEN {}
//
//    #[identifier(119)]
//    #[cfg(feature = "v3_8")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//    // TODO: It's an Event handle. Should probably implement event handle
//    impl GetAttr<MutSQLPOINTER> for SQL_ATTR_ASYNC_DBC_EVENT {}
//
//    // TODO: It is not 3.5 in implementation ???
//    // but it says that drivers conforming to earlier versions can support this field. HMMMMMMMMMMM
//    #[identifier(1209)]
//    #[cfg(feature = "v3_5")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_CONNECTION_DEAD;
//    // Can only be used with `SQLGetConnectAttr`
//    impl GetAttr<SQLUINTEGER> for SQL_ATTR_CONNECTION_DEAD {}
//
//    pub enum ConnectionDead {
//        SQL_CD_TRUE = 1,
//        SQL_CD_FALSE = 0,
//    }
//
//    /*  ODBC Driver Manager sets this connection attribute to a unicode driver
//        (which supports SQLConnectW) when the application is an ANSI application
//        (which calls SQLConnect, SQLDriverConnect, or SQLBrowseConnect).
//        This is SetConnectAttr only and application does not set this attribute
//        This attribute was introduced because some unicode driver's some APIs may
//        need to behave differently on ANSI or Unicode applications. A unicode
//        driver, which  has same behavior for both ANSI or Unicode applications,
//        should return SQL_ERROR when the driver manager sets this connection
//        attribute. When a unicode driver returns SQL_SUCCESS on this attribute,
//        the driver manager treates ANSI and Unicode connections differently in
//        connection pooling.
//    */
//    // TODO: These 4 are not in Documentation??
//    #[identifier(115)]
//    #[cfg(feature = "v3_51")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_ANSI_APP;
//
//    #[cfg(feature = "v3_51")]
//    pub enum AnsiApp {
//        SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//        SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//    }
//
//    #[identifier(116)]
//    #[cfg(feature = "v3_8")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_RESET_CONNECTION;
//
//    #[cfg(feature = "v3_8")]
//    pub enum ResetConnection {
//        SQL_RESET_CONNECTION_YES = 1,
//    }
//
//    #[identifier(122)]
//    #[cfg(feature = "v4")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_CREDENTIALS;
//
//    #[identifier(123)]
//    #[cfg(feature = "v4")]
//    #[derive(ConnAttribute)]
//    pub struct SQL_ATTR_REFRESH_CONNECTION;
//
//    #[cfg(feature = "v4")]
//    pub enum RefreshConnection {
//        SQL_REFRESH_NOW = -1,
//        SQL_REFRESH_AUTO = 0,
//        SQL_REFRESH_MANUAL = 1,
//    }
//
//    // TODO: Reexport these in conn module
//    // TODO: Or derive them, but still export?
//    impl ConnAttribute for SQL_ATTR_METADATA_ID {}
//    impl ConnAttribute for SQL_ATTR_ASYNC_ENABLE {}
}

pub mod stmt {
//    pub trait StmtAttrbute: Attribute<TypeIdentifier=SQLINTEGER> {}

    pub trait StmtState {}
    pub enum S1 {}
    impl StmtState for S1 {}

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

pub mod desc {
    pub trait DescState {}
    pub enum D1 {}
    impl DescState for D1 {}
}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES
