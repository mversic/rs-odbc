use crate::c_types::DeferredBuf;
use crate::extern_api;
use crate::{sqlreturn::SQL_SUCCESS, Ident, IntoSQLPOINTER, StrLenOrInd, SQLPOINTER, SQLSMALLINT};
use std::any::type_name;
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::thread::panicking;

#[cfg(feature = "odbc_debug")]
use std::mem::{ManuallyDrop, MaybeUninit};
#[cfg(feature = "odbc_debug")]
use crate::stmt::{
    SQL_ATTR_APP_PARAM_DESC, SQL_ATTR_APP_ROW_DESC, SQL_ATTR_IMP_PARAM_DESC, SQL_ATTR_IMP_ROW_DESC,
};

pub unsafe trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}

pub trait Handle {
    type Ident: crate::Ident<Type = SQLSMALLINT>;
}

// TODO: Should be unsafe?
// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub unsafe trait Allocate<'src>: Handle + Drop {
    type SrcHandle: AsSQLHANDLE;
    fn from_raw(handle: SQLHANDLE) -> Self;
}

pub trait SQLCancelHandle: Handle {}
pub trait SQLCompleteAsyncHandle: Handle {}
pub trait SQLEndTranHandle: Handle {}

pub enum RowDesc {}
pub enum ParamDesc {}

pub trait DescType<'buf> {}

pub struct ImplDesc<T> {
    desc_type: PhantomData<T>,
}
impl<T> DescType<'_> for ImplDesc<T> {}

#[derive(Debug)]
pub struct AppDesc<'buf> {
    pub(crate) data_ptrs: PhantomData<&'buf ()>,
}
impl<'buf> DescType<'buf> for AppDesc<'buf> {}

#[derive(rs_odbc_derive::Ident)]
#[identifier(SQLSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_ENV;

#[derive(rs_odbc_derive::Ident)]
#[identifier(SQLSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DBC;

#[derive(rs_odbc_derive::Ident)]
#[identifier(SQLSMALLINT, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_STMT;

#[derive(rs_odbc_derive::Ident)]
#[identifier(SQLSMALLINT, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DESC;

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
#[repr(transparent)]
pub struct SQLHENV {
    pub(crate) handle: SQLHANDLE,
}
impl Handle for SQLHENV {
    type Ident = SQL_HANDLE_ENV;
}
unsafe impl Allocate<'_> for SQLHENV {
    type SrcHandle = SQL_NULL_HANDLE;

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHENV { handle }
    }
}
impl SQLEndTranHandle for SQLHENV {}
unsafe impl AsSQLHANDLE for SQLHENV {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHENV {
    fn drop(&mut self) {
        let sql_return =
            unsafe { extern_api::SQLFreeHandle(SQL_HANDLE_ENV::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned {:?}",
                type_name::<Self>(),
                sql_return
            )
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
#[cfg_attr(not(feature = "odbc_debug"), repr(transparent))]
pub struct SQLHDBC<'env> {
    parent: PhantomData<&'env ()>,
    pub(crate) handle: SQLHANDLE,

    #[cfg(feature = "odbc_debug")]
    connected: bool,
}
impl SQLHDBC<'_> {
    #[inline]
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn set_connected(&mut self) {
        self.connected = true;
    }

    #[inline]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn set_connected(&mut self) {}

    #[inline]
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn set_disconnected(&mut self) {
        self.connected = false;
    }

    #[inline]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn set_disconnected(&mut self) {}

    #[inline]
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn assert_connected(&self) {
        // TODO: Add a message that attribute should be set only after connection was established
        assert_eq!(true, self.connected);
    }

    #[inline]
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn assert_not_connected(&self) {
        // TODO: Add a message that attribute should be set only before connection was established
        // Also add a message that handle can only be disconnected once it's been disconnected
        assert_eq!(false, self.connected);
    }

    fn do_drop(&mut self) {
        let sql_return =
            unsafe { extern_api::SQLFreeHandle(SQL_HANDLE_DBC::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned {:?}. \
                Before being deallocated, handle must be disconnected(via SQLDisconnect).",
                type_name::<Self>(),
                sql_return
            )
        }
    }
}
impl Handle for SQLHDBC<'_> {
    type Ident = SQL_HANDLE_DBC;
}
unsafe impl<'env> Allocate<'env> for SQLHDBC<'env> {
    type SrcHandle = SQLHENV;

    #[cfg(feature = "odbc_debug")]
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDBC {
            handle,
            parent: PhantomData,
            connected: false,
        }
    }
    #[cfg(not(feature = "odbc_debug"))]
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
unsafe impl AsSQLHANDLE for SQLHDBC<'_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHDBC<'_> {
    #[cfg(feature = "odbc_debug")]
    fn drop(&mut self) {
        if !panicking() {
            self.assert_not_connected();
        }

        self.do_drop();
    }

    #[cfg(not(feature = "odbc_debug"))]
    fn drop(&mut self) {
        self.do_drop();
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
#[cfg_attr(not(feature = "odbc_debug"), repr(transparent))]
pub struct SQLHSTMT<'conn, 'stmt, 'buf> {
    parent: PhantomData<&'conn ()>,
    pub(crate) handle: SQLHANDLE,

    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_ard: Cell<Option<&'stmt SQLHDESC<'stmt, AppDesc<'buf>>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_apd: Cell<Option<&'stmt SQLHDESC<'stmt, AppDesc<'buf>>>>,

    #[cfg(feature = "odbc_debug")]
    pub(crate) ard: ManuallyDrop<SQLHDESC<'stmt, AppDesc<'buf>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) apd: ManuallyDrop<SQLHDESC<'stmt, AppDesc<'buf>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ird: ManuallyDrop<SQLHDESC<'stmt, ImplDesc<RowDesc>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ipd: ManuallyDrop<SQLHDESC<'stmt, ImplDesc<ParamDesc>>>,

    #[allow(dead_code)]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_ard: Cell<PhantomData<&'stmt SQLHDESC<'stmt, AppDesc<'buf>>>>,
    #[allow(dead_code)]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_apd: Cell<PhantomData<&'stmt SQLHDESC<'stmt, AppDesc<'buf>>>>,

    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ard: PhantomData<SQLHDESC<'stmt, AppDesc<'buf>>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) apd: PhantomData<SQLHDESC<'stmt, AppDesc<'buf>>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ird: PhantomData<SQLHDESC<'stmt, ImplDesc<RowDesc>>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ipd: PhantomData<SQLHDESC<'stmt, ImplDesc<ParamDesc>>>,
}
impl<'buf> SQLHSTMT<'_, '_, 'buf> {
    #[cfg(feature = "odbc_debug")]
    unsafe fn get_descriptor_handle<A: Ident<Type = crate::SQLINTEGER>>(
        handle: SQLHANDLE,
    ) -> SQLHANDLE {
        let mut descriptor_handle = MaybeUninit::uninit();

        let sql_return = extern_api::SQLGetStmtAttrA(
            handle,
            A::IDENTIFIER,
            descriptor_handle.as_mut_ptr() as SQLPOINTER,
            0,
            &mut 0,
        );
        if sql_return != SQL_SUCCESS {
            panic!(
                "{}: SQLGetStmtAttr returned {:?}",
                type_name::<A>(),
                sql_return
            );
        }

        descriptor_handle.assume_init()
    }

    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_col<TT: Ident, B: DeferredBuf<'buf, TT>>(&self, TargetValuePtr: Option<B>) {}
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_param<TT: Ident, B: DeferredBuf<'buf, TT>>(
        &self,
        TargetValuePtr: Option<B>,
    ) {
    }
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_strlen_or_ind(
        &self,
        StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
    ) {
    }

    #[cfg(feature = "odbc_debug")]
    pub(crate) fn bind_col<TT: Ident, B: DeferredBuf<'buf, TT>>(&self, TargetValuePtr: Option<B>) {
        if let Some(explicit_ard) = self.explicit_ard.get() {
            explicit_ard.bind_col(TargetValuePtr);
        } else {
            self.ard.bind_col(TargetValuePtr);
        }
    }
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn bind_param<TT: Ident, B: DeferredBuf<'buf, TT>>(
        &self,
        TargetValuePtr: Option<B>,
    ) {
        if let Some(explicit_apd) = self.explicit_apd.get() {
            explicit_apd.bind_param(TargetValuePtr);
        } else {
            self.apd.bind_param(TargetValuePtr);
        }
    }
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn bind_strlen_or_ind(
        &self,
        StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
    ) {
        unimplemented!();
    }
}

impl Handle for SQLHSTMT<'_, '_, '_> {
    type Ident = SQL_HANDLE_STMT;
}
unsafe impl<'conn> Allocate<'conn> for SQLHSTMT<'conn, '_, '_> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn>;

    #[cfg(feature = "odbc_debug")]
    fn from_raw(handle: SQLHANDLE) -> Self {
        unsafe {
            let ard = SQLHSTMT::get_descriptor_handle::<SQL_ATTR_APP_ROW_DESC>(handle);
            let apd = SQLHSTMT::get_descriptor_handle::<SQL_ATTR_APP_PARAM_DESC>(handle);
            let ird = SQLHSTMT::get_descriptor_handle::<SQL_ATTR_IMP_ROW_DESC>(handle);
            let ipd = SQLHSTMT::get_descriptor_handle::<SQL_ATTR_IMP_PARAM_DESC>(handle);

            SQLHSTMT {
                parent: PhantomData,
                handle,

                ard: ManuallyDrop::new(SQLHDESC::<AppDesc>::from_raw(ard)),
                apd: ManuallyDrop::new(SQLHDESC::<AppDesc>::from_raw(apd)),
                ird: ManuallyDrop::new(SQLHDESC::<ImplDesc<_>>::from_raw(ird)),
                ipd: ManuallyDrop::new(SQLHDESC::<ImplDesc<_>>::from_raw(ipd)),

                explicit_ard: Cell::new(None),
                explicit_apd: Cell::new(None),
            }
        }
    }

    #[cfg(not(feature = "odbc_debug"))]
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHSTMT {
            parent: PhantomData,
            handle,

            ard: PhantomData,
            apd: PhantomData,
            ird: PhantomData,
            ipd: PhantomData,

            explicit_ard: Cell::new(PhantomData),
            explicit_apd: Cell::new(PhantomData),
        }
    }
}
impl SQLCancelHandle for SQLHSTMT<'_, '_, '_> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_, '_, '_> {}
unsafe impl AsSQLHANDLE for SQLHSTMT<'_, '_, '_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHSTMT<'_, '_, '_> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { extern_api::SQLFreeHandle(SQL_HANDLE_STMT::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned: {:?}",
                type_name::<Self>(),
                sql_return
            )
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
#[derive(Debug)]
#[cfg_attr(not(feature = "odbc_debug"), repr(transparent))]
pub struct SQLHDESC<'conn, T> {
    parent: PhantomData<&'conn ()>,
    pub(crate) handle: SQLHANDLE,

    #[cfg(feature = "odbc_debug")]
    // TODO: Implement properly
    pub(crate) data: PhantomData<T>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) data: PhantomData<T>,
}
impl<'buf, T: DescType<'buf>> SQLHDESC<'_, T> {
    #[cfg(not(feature = "odbc_debug"))]
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC {
            handle,
            parent: PhantomData,
            data: PhantomData,
        }
    }
    #[cfg(feature = "odbc_debug")]
    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC {
            handle,
            parent: PhantomData,
            data: PhantomData,
        }
    }
}
impl<'buf, T: DescType<'buf>> Handle for SQLHDESC<'_, T> {
    type Ident = SQL_HANDLE_DESC;
}
unsafe impl<'conn, 'buf> Allocate<'conn> for SQLHDESC<'conn, AppDesc<'buf>> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn>;

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC::<AppDesc>::from_raw(handle)
    }
}
unsafe impl<T> AsSQLHANDLE for SQLHDESC<'_, T> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
unsafe impl<'buf, T: DescType<'buf>> IntoSQLPOINTER for Option<&SQLHDESC<'_, T>> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.map_or_else(std::ptr::null_mut, |handle| handle.as_SQLHANDLE().cast())
    }
}
impl<T> Drop for SQLHDESC<'_, T> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { extern_api::SQLFreeHandle(SQL_HANDLE_DESC::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned: {:?}",
                type_name::<Self>(),
                sql_return
            )
        }
    }
}
#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;
unsafe impl AsSQLHANDLE for SQL_NULL_HANDLE {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        std::ptr::null_mut()
    }
}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
pub type SQLHWND = SQLPOINTER;
