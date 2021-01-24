use crate::api::{Disconnect, FreeHandle};
use crate::{SQLSMALLINT, SQL_SUCCESS};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::thread::panicking;

pub trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}
pub trait AsMutSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_mut_SQLHANDLE(&mut self) -> SQLHANDLE;
}

pub trait HandleIdentifier {
    const IDENTIFIER: SQLSMALLINT;
}

pub trait Handle: AsSQLHANDLE {
    type Identifier: HandleIdentifier;
}

// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub trait Allocate<'src>: Handle + Drop {
    type SrcHandle: AsSQLHANDLE;
}

pub trait SQLCancelHandle: Handle {}
pub trait SQLCompleteAsyncHandle: Handle {}
pub trait SQLEndTranHandle: Handle {}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_ENV;
impl HandleIdentifier for SQL_HANDLE_ENV {
    const IDENTIFIER: SQLSMALLINT = 1;
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DBC;
impl HandleIdentifier for SQL_HANDLE_DBC {
    const IDENTIFIER: SQLSMALLINT = 2;
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_STMT;
impl HandleIdentifier for SQL_HANDLE_STMT {
    const IDENTIFIER: SQLSMALLINT = 3;
}

#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DESC;
impl HandleIdentifier for SQL_HANDLE_DESC {
    const IDENTIFIER: SQLSMALLINT = 4;
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
pub struct SQLHENV {
    handle: SQLHANDLE,
}
impl SQLHENV {
    // TODO: Consider removing this function
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}
impl Handle for SQLHENV {
    type Identifier = SQL_HANDLE_ENV;
}
impl<'a> Allocate<'a> for SQLHENV {
    type SrcHandle = SQL_NULL_HANDLE;
}
impl SQLEndTranHandle for SQLHENV {}
impl AsSQLHANDLE for SQLHENV {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl AsMutSQLHANDLE for SQLHENV {
    #[allow(non_snake_case)]
    fn as_mut_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHENV {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_ENV::IDENTIFIER, self.as_SQLHANDLE()) };

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
pub struct SQLHDBC<'env> {
    handle: SQLHANDLE,
    parent: PhantomData<&'env SQLHENV>,
}
impl<'env> Allocate<'env> for SQLHDBC<'env> {
    type SrcHandle = SQLHENV;
}
impl Handle for SQLHDBC<'_> {
    type Identifier = SQL_HANDLE_DBC;
}
impl<'env> SQLHDBC<'env> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}
impl SQLCancelHandle for SQLHDBC<'_> {}
impl SQLCompleteAsyncHandle for SQLHDBC<'_> {}
impl SQLEndTranHandle for SQLHDBC<'_> {}
impl AsSQLHANDLE for SQLHDBC<'_> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl AsMutSQLHANDLE for SQLHDBC<'_> {
    #[allow(non_snake_case)]
    fn as_mut_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHDBC<'_> {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_DBC::IDENTIFIER, self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
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
pub struct SQLHSTMT<'env, 'conn> {
    handle: SQLHANDLE,
    parent: PhantomData<&'conn SQLHDBC<'env>>,
}
impl SQLHSTMT<'_, '_> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}
impl<'env, 'conn> Allocate<'conn> for SQLHSTMT<'env, 'conn> {
    type SrcHandle = SQLHDBC<'env>;
}
impl Handle for SQLHSTMT<'_, '_> {
    type Identifier = SQL_HANDLE_STMT;
}
impl SQLCancelHandle for SQLHSTMT<'_, '_> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_, '_> {}
impl AsSQLHANDLE for SQLHSTMT<'_, '_> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl AsMutSQLHANDLE for SQLHSTMT<'_, '_> {
    #[allow(non_snake_case)]
    fn as_mut_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHSTMT<'_, '_> {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_STMT::IDENTIFIER, self.as_SQLHANDLE()) };

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
pub struct SQLHDESC<'env, 'conn> {
    handle: SQLHANDLE,
    parent: PhantomData<&'conn SQLHDBC<'env>>,
}
impl SQLHDESC<'_, '_> {
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }
}
impl<'env, 'conn> Allocate<'conn> for SQLHDESC<'env, 'conn> {
    type SrcHandle = SQLHDBC<'env>;
}
impl Handle for SQLHDESC<'_, '_> {
    type Identifier = SQL_HANDLE_DESC;
}
impl AsSQLHANDLE for SQLHDESC<'_, '_> {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl AsMutSQLHANDLE for SQLHDESC<'_, '_> {
    #[allow(non_snake_case)]
    fn as_mut_SQLHANDLE(&mut self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHDESC<'_, '_> {
    fn drop(&mut self) {
        let ret = unsafe { FreeHandle(SQL_HANDLE_DESC::IDENTIFIER, self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
        }
    }
}

#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;
impl AsSQLHANDLE for SQL_NULL_HANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        std::ptr::null_mut()
    }
}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
//type SQLHWND = SQLPOINTER;
