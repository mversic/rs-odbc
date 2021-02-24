use crate::extern_api::SQLFreeHandle;
use crate::{AsMutSQLPOINTER, AsSQLPOINTER, SQLPOINTER, SQLSMALLINT, SQLUSMALLINT, SQL_SUCCESS};
use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::{Rc, Weak};
use std::thread::panicking;

pub trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}

pub trait HandleIdentifier {
    const IDENTIFIER: SQLSMALLINT;
}

pub trait Handle {
    type Identifier: HandleIdentifier;
}

// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub trait Allocate<'src>: Handle + Drop {
    type SrcHandle: AsSQLHANDLE;
    fn from_raw(handle: SQLHANDLE) -> Self;
    fn uninit() -> MaybeUninit<Self>
    where
        Self: Sized,
    {
        MaybeUninit::uninit()
    }
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
pub type SQLHANDLE = *mut UnsafeCell<RawHandle>;

// TODO: Keep these?
pub type HENV = SQLHANDLE;
pub type HDBC = SQLHANDLE;
pub type HSTMT = SQLHANDLE;
pub type HDESC = SQLHANDLE;

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
pub struct SQLHENV {
    handle: SQLHANDLE,
}
impl Handle for SQLHENV {
    type Identifier = SQL_HANDLE_ENV;
}
impl<'a> Allocate<'a> for SQLHENV {
    type SrcHandle = SQL_NULL_HANDLE;
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHENV { handle }
    }
}
impl SQLEndTranHandle for SQLHENV {}
impl AsSQLHANDLE for SQLHENV {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHENV {
    fn drop(&mut self) {
        let ret = unsafe { SQLFreeHandle(SQL_HANDLE_ENV::IDENTIFIER, self.as_SQLHANDLE()) };

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
pub struct SQLHDBC<'env> {
    handle: SQLHANDLE,
    parent: PhantomData<&'env SQLHENV>,
}
impl Handle for SQLHDBC<'_> {
    type Identifier = SQL_HANDLE_DBC;
}
impl<'env> Allocate<'env> for SQLHDBC<'env> {
    type SrcHandle = SQLHENV;
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDBC {
            handle,
            parent: PhantomData,
        }
    }
}
impl SQLCancelHandle for SQLHDBC<'_> {}
impl SQLCompleteAsyncHandle for SQLHDBC<'_> {}
impl SQLEndTranHandle for SQLHDBC<'_> {}
impl AsSQLHANDLE for SQLHDBC<'_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHDBC<'_> {
    fn drop(&mut self) {
        let ret = unsafe { SQLFreeHandle(SQL_HANDLE_DBC::IDENTIFIER, self.as_SQLHANDLE()) };

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
pub struct SQLHSTMT<'env, 'conn, 'a> {
    handle: SQLHANDLE,
    parent: PhantomData<&'conn SQLHDBC<'env>>,

    // TODO: Mislim da tu cak ide i 'b
    pub(crate) explicit_ard: Weak<SQLHDESC<'conn, 'a, SQLHDBC<'env>>>,
    pub(crate) explicit_apd: Weak<SQLHDESC<'conn, 'a, SQLHDBC<'env>>>,

    // TODO: Maybe not needed
    pub(crate) bound_cols: HashMap<SQLUSMALLINT, Rc<RefCell<dyn AsMutSQLPOINTER<'a>>>>,
    pub(crate) bound_params: HashMap<SQLUSMALLINT, Rc<dyn AsSQLPOINTER>>,
}
impl Handle for SQLHSTMT<'_, '_, '_> {
    type Identifier = SQL_HANDLE_STMT;
}
impl<'env, 'conn> Allocate<'conn> for SQLHSTMT<'env, 'conn, '_> {
    type SrcHandle = SQLHDBC<'env>;

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHSTMT {
            handle,
            parent: PhantomData,

            explicit_ard: Weak::new(),
            explicit_apd: Weak::new(),

            bound_cols: HashMap::new(),
            bound_params: HashMap::new(),
        }
    }
}
impl SQLCancelHandle for SQLHSTMT<'_, '_, '_> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_, '_, '_> {}
impl AsSQLHANDLE for SQLHSTMT<'_, '_, '_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHSTMT<'_, '_, '_> {
    fn drop(&mut self) {
        let ret = unsafe { SQLFreeHandle(SQL_HANDLE_STMT::IDENTIFIER, self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
        }
        // TODO: Do I have to drop bound_cols here?
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
pub struct SQLHDESC<'conn, 'a, T> {
    handle: SQLHANDLE,
    parent: PhantomData<&'conn T>,
    // TODO: Couldn't Vec be used?
    pub(crate) data_ptrs: HashMap<SQLSMALLINT, Rc<RefCell<dyn AsMutSQLPOINTER<'a>>>>,
}
impl<T> Handle for SQLHDESC<'_, '_, T> {
    type Identifier = SQL_HANDLE_DESC;
}
impl<'env, 'conn> Allocate<'conn> for SQLHDESC<'conn, '_, SQLHDBC<'env>> {
    type SrcHandle = SQLHDBC<'env>;
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC {
            handle,
            parent: PhantomData,
            data_ptrs: HashMap::new(),
        }
    }
}
impl<T> AsSQLHANDLE for SQLHDESC<'_, '_, T> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
// TODO: use derive odbc_type somehow?
unsafe impl<T> AsSQLPOINTER for SQLHDESC<'_, '_, T> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        self.as_SQLHANDLE().cast()
    }
}
unsafe impl<'a, T> AsMutSQLPOINTER<'a> for MaybeUninit<&'a SQLHDESC<'_, '_, T>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}
unsafe impl<T, LEN: Copy> crate::Len<crate::OdbcAttr, LEN> for SQLHDESC<'_, '_, T> where LEN: From<crate::SQLSMALLINT> {
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(crate::SQL_IS_POINTER)
    }
}
unsafe impl<T, LEN: Copy> crate::Len<crate::OdbcAttr, LEN> for MaybeUninit<&SQLHDESC<'_, '_, T>> where LEN: From<crate::SQLSMALLINT> {
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(crate::SQL_IS_POINTER)
    }
}
impl<T> Drop for SQLHDESC<'_, '_, T> {
    fn drop(&mut self) {
        let ret = unsafe { SQLFreeHandle(SQL_HANDLE_DESC::IDENTIFIER, self.as_SQLHANDLE()) };

        if ret != SQL_SUCCESS && !panicking() {
            panic!("SQLFreeHandle returned: {:?}", ret)
        }
    }
}
#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;
impl AsSQLHANDLE for SQL_NULL_HANDLE {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        std::ptr::null_mut()
    }
}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
pub type SQLHWND = SQLPOINTER;
