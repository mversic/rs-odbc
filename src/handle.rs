use super::conn::{ConnState, C2, C4};
use super::desc::{DescState, D1};
use super::stmt::{StmtState, S1};
use crate::api::{Disconnect, FreeHandle};
use crate::{SQLSMALLINT, SQL_SUCCESS};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::thread::panicking;

pub trait Version {}
pub trait KnownVersion: Version {}

#[allow(non_camel_case_types)]
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
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&mut self) -> SQLHANDLE;
}

pub trait HandleIdentifier {
    fn identifier() -> SQLSMALLINT;
}

pub trait Handle: AsSQLHANDLE {
    type Identifier: HandleIdentifier;
}

// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub trait Allocate<'a, 'b>: Handle + Drop {
    type SrcHandle: AsSQLHANDLE;
}

pub trait SQLCancelHandle: Handle {}
pub trait SQLCompleteAsyncHandle: Handle {}
pub trait SQLEndTranHandle: Handle {}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_ENV;
impl HandleIdentifier for SQL_HANDLE_ENV {
    fn identifier() -> SQLSMALLINT {
        1
    }
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DBC;
impl HandleIdentifier for SQL_HANDLE_DBC {
    fn identifier() -> SQLSMALLINT {
        2
    }
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_STMT;
impl HandleIdentifier for SQL_HANDLE_STMT {
    fn identifier() -> SQLSMALLINT {
        3
    }
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DESC;
impl HandleIdentifier for SQL_HANDLE_DESC {
    fn identifier() -> SQLSMALLINT {
        4
    }
}

// TODO: But must it not be a void* in the end? It is void* in unixODBC
// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
#[repr(C)]
//#[cfg(feature = "RUSTC_IS_STABLE")]
pub struct RawHandle {
    _private: [u8; 0],
}
//#[cfg(feature = "RUSTC_IS_NIGHTLY")]
//pub extern type RawHandle;

// TODO: Think about making it newtype with private field
// This type must not be public ever because of the issues around Drop
#[allow(non_camel_case_types)]
pub type SQLHANDLE = *mut RawHandle;

pub type HENV = SQLHANDLE;
pub type HDBC = SQLHANDLE;
//type HSTMT = SQLHANDLE;
//type HDESC = SQLHANDLE;

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
pub struct SQLHENV<V: Version> {
    pub handle: SQLHANDLE,
    version: PhantomData<V>,
}
impl SQLHENV<V_UNDEFINED> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    // TODO: Consider using transmute
    pub fn assume_version<V: KnownVersion>(self) -> SQLHENV<V> {
        let source = std::mem::ManuallyDrop::new(self);

        SQLHENV {
            handle: source.handle,
            version: PhantomData,
        }
    }
}
impl<V: Version> Handle for SQLHENV<V> {
    type Identifier = SQL_HANDLE_ENV;
}
impl<'a, 'b> Allocate<'a, 'b> for SQLHENV<V_UNDEFINED> {
    type SrcHandle = SQL_NULL_HANDLE;
}
impl<V: KnownVersion> SQLEndTranHandle for SQLHENV<V> {}
impl<V: Version> AsSQLHANDLE for SQLHENV<V> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl<V: Version> Drop for SQLHENV<V> {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_ENV::identifier(), self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
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
    parent: PhantomData<&'env SQLHENV<V>>,
}
impl<'a, 'env, V: KnownVersion> Allocate<'a, 'env> for SQLHDBC<'env, V, C2> {
    type SrcHandle = SQLHENV<V>;
}
impl<V: KnownVersion, C: ConnState> Handle for SQLHDBC<'_, V, C> {
    type Identifier = SQL_HANDLE_DBC;
}
impl<'env, V: KnownVersion> SQLHDBC<'env, V, C2> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    // TODO: Consider using transmute
    pub fn assume_connected(self) -> SQLHDBC<'env, V, C4> {
        let source = std::mem::ManuallyDrop::new(self);

        SQLHDBC {
            handle: source.handle,
            version: PhantomData,
            state: PhantomData,
            parent: PhantomData,
        }
    }
}
impl<V: KnownVersion> SQLCancelHandle for SQLHDBC<'_, V, C4> {}
impl<V: KnownVersion> SQLCompleteAsyncHandle for SQLHDBC<'_, V, C4> {}
impl<V: KnownVersion> SQLEndTranHandle for SQLHDBC<'_, V, C4> {}
impl<V: KnownVersion, C: ConnState> AsSQLHANDLE for SQLHDBC<'_, V, C> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl<V: KnownVersion, C: ConnState> Drop for SQLHDBC<'_, V, C> {
    fn drop(&mut self) {
        if C::connected() {
            let ret = unsafe { Disconnect(self.as_SQLHANDLE()) };
            if ret != SQL_SUCCESS && !panicking() {
                panic!("SQLDisconnect -> {:?}", ret)
            }
        }

        let ret = unsafe { FreeHandle(SQL_HANDLE_DBC::identifier(), self.as_SQLHANDLE()) };
        if ret != SQL_SUCCESS && !panicking() {
            // TODO: Improve this scenario by checking the reason for failure
            let ret = unsafe { Disconnect(self.as_SQLHANDLE()) };
            if ret != SQL_SUCCESS && !panicking() {
                panic!("SQLDisconnect -> {:?}", ret)
            }

            panic!("SQLFreeHandle -> {:?}", ret)
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
pub struct SQLHSTMT<'env, 'conn, V: KnownVersion, S: StmtState> {
    handle: SQLHANDLE,
    version: PhantomData<V>,
    state: PhantomData<S>,
    parent: PhantomData<&'conn SQLHDBC<'env, V, C4>>,
}
impl<V: KnownVersion> SQLHSTMT<'_, '_, V, S1> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}
impl<'env, 'conn, V: KnownVersion> Allocate<'env, 'conn> for SQLHSTMT<'env, 'conn, V, S1> {
    type SrcHandle = SQLHDBC<'env, V, C4>;
}
impl<V: KnownVersion, S: StmtState> Handle for SQLHSTMT<'_, '_, V, S> {
    type Identifier = SQL_HANDLE_STMT;
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
        let ret = unsafe { FreeHandle(SQL_HANDLE_STMT::identifier(), self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
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
pub struct SQLHDESC<'env, 'conn, V: KnownVersion, D: DescState> {
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
impl<'env, 'conn, V: KnownVersion> Allocate<'env, 'conn> for SQLHDESC<'env, 'conn, V, D1> {
    type SrcHandle = SQLHDBC<'env, V, C4>;
}
impl<V: KnownVersion, D: DescState> Handle for SQLHDESC<'_, '_, V, D> {
    type Identifier = SQL_HANDLE_DESC;
}
impl<V: KnownVersion, D: DescState> AsSQLHANDLE for SQLHDESC<'_, '_, V, D> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl<V: KnownVersion, D: DescState> Drop for SQLHDESC<'_, '_, V, D> {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_DESC::identifier(), self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
        }
    }
}

#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;
impl AsSQLHANDLE for SQL_NULL_HANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&mut self) -> SQLHANDLE {
        std::ptr::null_mut()
    }
}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
//type SQLHWND = SQLPOINTER;
