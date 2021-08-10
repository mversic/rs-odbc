#[double]
use crate::api::ffi;
use crate::api::{Allocate, Handle, Descriptor};
use crate::c_types::DeferredBuf;
use crate::conn::{ConnState, C2, C3, C4};
use crate::convert::{AsSQLHANDLE, IntoSQLPOINTER};
use crate::desc::{AppDesc, DescType, ImplDesc, IPD, IRD};
use crate::env::{OdbcVersion, SQL_ATTR_ODBC_VERSION};
#[cfg(feature = "odbc_debug")]
use crate::stmt::{
    SQL_ATTR_APP_PARAM_DESC, SQL_ATTR_APP_ROW_DESC, SQL_ATTR_IMP_PARAM_DESC, SQL_ATTR_IMP_ROW_DESC,
};
use crate::{sqlreturn::SQL_SUCCESS, Ident, StrLenOrInd, SQLPOINTER};
use mockall_double::double;
use std::any::type_name;
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::thread::panicking;

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

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// Try placing it into src/api/ffi.rs?
// This is unixOBDC value
pub type SQLHWND = SQLPOINTER;

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

#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;

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
#[derive(Debug)]
#[repr(transparent)]
pub struct SQLHENV<V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,
    version: PhantomData<V>,
}

unsafe impl<V: OdbcVersion> Send for SQLHENV<V> {}
unsafe impl<V: OdbcVersion> Sync for SQLHENV<V> {}

impl<V: OdbcVersion> Handle for SQLHENV<V> {
    type Ident = SQL_HANDLE_ENV;
}

impl<V: OdbcVersion> Allocate<'_> for SQLHENV<V> {
    type SrcHandle = SQL_NULL_HANDLE;

    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        let val = Self {
            handle,
            version: PhantomData,
        };

        let sql_return = ffi::SQLSetEnvAttr(
            val.as_SQLHANDLE(),
            SQL_ATTR_ODBC_VERSION::IDENTIFIER,
            V::IDENTIFIER.into_SQLPOINTER(),
            0, // TODO: Use AttrLen::len()
        );

        if sql_return != SQL_SUCCESS {
            panic!(
                "SQL_ATTR_ODBC_VERSION({}): SQLSetEnvAttr returned {:?}",
                type_name::<V>(),
                sql_return
            )
        }

        val
    }
}

impl<V: OdbcVersion> Drop for SQLHENV<V> {
    fn drop(&mut self) {
        drop_handle(self);
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
#[derive(Debug)]
#[repr(transparent)]
pub struct SQLHDBC<'env, C: ConnState, V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,

    parent: PhantomData<&'env ()>,
    connected: PhantomData<C>,
    version: PhantomData<V>,
}

unsafe impl<C: ConnState, V: OdbcVersion> Send for SQLHDBC<'_, C, V> {}
// TODO: Reference: http://www.firstsql.com/ithread5.htm
//  Connection Options (set with SQLSetConnectOption) should be set before sharing begins and should not be changed.
//  Connection-level Statement Options (set with SQLSetConnectOption) should be set before sharing begins and should not be changed.
//  Transactions, there are several choices:
//      autocommit, each statement is implicitly committed,
//      connection-wide, a single transaction during the entire connection,
//      otherwise, commits and rollbacks must be synchronized between threads.
// unsafe impl<V: OdbcVersion> Sync for SQLHDBC<'_, C4, V> {}

impl<C: ConnState, V: OdbcVersion> Handle for SQLHDBC<'_, C, V> {
    type Ident = SQL_HANDLE_DBC;
}

impl<'env, V: OdbcVersion> Allocate<'env> for SQLHDBC<'env, C2, V> {
    type SrcHandle = SQLHENV<V>;

    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        Self {
            handle,

            parent: PhantomData,
            connected: PhantomData,
            version: PhantomData,
        }
    }
}

impl<C: ConnState, V: OdbcVersion> Drop for SQLHDBC<'_, C, V> {
    fn drop(&mut self) {
        C::disconnect(self);
        drop_handle(self);
    }
}

impl<'env, OC: ConnState, V: OdbcVersion> SQLHDBC<'env, OC, V> {
    pub(crate) fn disconnect(self) -> SQLHDBC<'env, C2, V> {
        let handle = ManuallyDrop::new(self);

        SQLHDBC {
            handle: handle.handle,
            parent: handle.parent,
            connected: PhantomData,
            version: PhantomData,
        }
    }
    pub(crate) fn need_data(self) -> SQLHDBC<'env, C3, V> {
        let handle = ManuallyDrop::new(self);

        SQLHDBC {
            handle: handle.handle,
            parent: handle.parent,
            connected: PhantomData,
            version: PhantomData,
        }
    }
    pub(crate) fn connect(self) -> SQLHDBC<'env, C4, V> {
        let handle = ManuallyDrop::new(self);

        SQLHDBC {
            handle: handle.handle,
            parent: handle.parent,
            connected: PhantomData,
            version: PhantomData,
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
#[derive(Debug)]
#[repr(transparent)]
pub struct SQLHSTMT<'conn, 'desc, 'buf, V: OdbcVersion>(
    pub(crate) UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
);

unsafe impl<V: OdbcVersion> Send for SQLHSTMT<'_, '_, '_, V> {}

impl<'conn, 'desc, 'buf, V: OdbcVersion> Handle for SQLHSTMT<'conn, 'desc, 'buf, V> {
    type Ident = <UnsafeSQLHSTMT<'conn, 'desc, 'buf, V> as Handle>::Ident;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion> Allocate<'conn> for SQLHSTMT<'conn, 'desc, 'buf, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = <UnsafeSQLHSTMT<'conn, 'desc, 'buf, V> as Allocate<'conn>>::SrcHandle;

    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        Self(UnsafeSQLHSTMT::from_raw(handle))
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
#[derive(Debug)]
#[cfg_attr(not(feature = "odbc_debug"), repr(transparent))]
pub struct UnsafeSQLHSTMT<'conn, 'desc, 'buf, V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,

    parent: PhantomData<&'conn ()>,
    version: PhantomData<V>,

    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_ard: Cell<Option<&'desc UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_apd: Cell<Option<&'desc UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>>,

    #[cfg(feature = "odbc_debug")]
    pub(crate) ard: ManuallyDrop<UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) apd: ManuallyDrop<UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ird: ManuallyDrop<UnsafeSQLHDESC<'desc, ImplDesc<IRD>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ipd: ManuallyDrop<UnsafeSQLHDESC<'desc, ImplDesc<IPD>, V>>,

    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_ard: Cell<PhantomData<&'desc UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_apd: Cell<PhantomData<&'desc UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>>,

    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ard: PhantomData<UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) apd: PhantomData<UnsafeSQLHDESC<'desc, AppDesc<'buf>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ird: PhantomData<UnsafeSQLHDESC<'desc, ImplDesc<IRD>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ipd: PhantomData<UnsafeSQLHDESC<'desc, ImplDesc<IPD>, V>>,
}

unsafe impl<V: OdbcVersion> Send for UnsafeSQLHSTMT<'_, '_, '_, V> {}

impl<'buf, V: OdbcVersion> UnsafeSQLHSTMT<'_, '_, 'buf, V> {
    #[cfg(feature = "odbc_debug")]
    unsafe fn get_descriptor_handle<A: Ident<Type = SQLINTEGER>>(handle: SQLHANDLE) -> SQLHANDLE {
        let mut descriptor_handle = MaybeUninit::uninit();

        let sql_return = ffi::SQLGetStmtAttrA(
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
}

impl<V: OdbcVersion> Handle for UnsafeSQLHSTMT<'_, '_, '_, V> {
    type Ident = SQL_HANDLE_STMT;
}

impl<'conn, V: OdbcVersion> Allocate<'conn> for UnsafeSQLHSTMT<'conn, '_, '_, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn, C4, V>;

    #[cfg(feature = "odbc_debug")]
    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        unsafe {
            let ard = UnsafeSQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_APP_ROW_DESC>(handle);
            let apd = UnsafeSQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_APP_PARAM_DESC>(handle);
            let ird = UnsafeSQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_IMP_ROW_DESC>(handle);
            let ipd = UnsafeSQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_IMP_PARAM_DESC>(handle);

            Self {
                parent: PhantomData,
                version: PhantomData,

                handle,

                ard: ManuallyDrop::new(UnsafeSQLHDESC::from_raw(ard)),
                apd: ManuallyDrop::new(UnsafeSQLHDESC::from_raw(apd)),
                ird: ManuallyDrop::new(UnsafeSQLHDESC::from_raw(ird)),
                ipd: ManuallyDrop::new(UnsafeSQLHDESC::from_raw(ipd)),

                explicit_ard: Cell::new(None),
                explicit_apd: Cell::new(None),
            }
        }
    }

    #[cfg(not(feature = "odbc_debug"))]
    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        Self {
            handle,

            parent: PhantomData,
            version: PhantomData,

            ard: PhantomData,
            apd: PhantomData,
            ird: PhantomData,
            ipd: PhantomData,

            explicit_ard: Cell::new(PhantomData),
            explicit_apd: Cell::new(PhantomData),
        }
    }
}

impl<V: OdbcVersion> Drop for UnsafeSQLHSTMT<'_, '_, '_, V> {
    fn drop(&mut self) {
        drop_handle(self);
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
#[repr(transparent)]
pub struct SQLHDESC<'conn, DT, V: OdbcVersion>(pub(crate) UnsafeSQLHDESC<'conn, DT, V>);

unsafe impl<DT, V: OdbcVersion> Send for SQLHDESC<'_, DT, V> {}

impl<'buf, V: OdbcVersion, T: DescType<'buf>> Handle for SQLHDESC<'_, T, V> {
    type Ident = SQL_HANDLE_DESC;
}

impl<'conn, 'buf, V: OdbcVersion> Allocate<'conn> for SQLHDESC<'conn, AppDesc<'buf>, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = <UnsafeSQLHDESC<'conn, AppDesc<'buf>, V> as Allocate<'conn>>::SrcHandle;

    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        Self(UnsafeSQLHDESC::from_raw(handle))
    }
}

impl<'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V> for SQLHDESC<'_, DT, V> {}

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
pub struct UnsafeSQLHDESC<'conn, T, V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,

    parent: PhantomData<&'conn ()>,
    version: PhantomData<V>,

    #[cfg(feature = "odbc_debug")]
    // TODO: Implement properly
    pub(crate) inner: PhantomData<T>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) inner: PhantomData<T>,
}

unsafe impl<DT, V: OdbcVersion> Send for UnsafeSQLHDESC<'_, DT, V> {}

impl<V: OdbcVersion, T> Handle for UnsafeSQLHDESC<'_, T, V> {
    type Ident = SQL_HANDLE_DESC;
}

impl<'conn, 'buf, V: OdbcVersion> Allocate<'conn> for UnsafeSQLHDESC<'conn, AppDesc<'buf>, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn, C4, V>;

    unsafe fn from_raw(handle: SQLHANDLE) -> Self {
        Self {
            handle,

            parent: PhantomData,
            version: PhantomData,

            inner: PhantomData,
        }
    }
}

impl<'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V> for UnsafeSQLHDESC<'_, DT, V> {}

impl<V: OdbcVersion, T> Drop for UnsafeSQLHDESC<'_, T, V> {
    fn drop(&mut self) {
        drop_handle(self);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RefUnsafeSQLHDESC<'conn, DT, V: OdbcVersion>(ManuallyDrop<UnsafeSQLHDESC<'conn, DT, V>>);
unsafe impl<V: OdbcVersion, T> AsSQLHANDLE for RefUnsafeSQLHDESC<'_, T, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.0.handle
    }
}
impl<V: OdbcVersion, T> Handle for RefUnsafeSQLHDESC<'_, T, V> {
    type Ident = SQL_HANDLE_DESC;
}
impl<'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V> for RefUnsafeSQLHDESC<'_, DT, V> {}

#[derive(Debug)]
#[repr(transparent)]
pub struct RefSQLHDESC<'conn, DT, V: OdbcVersion>(RefUnsafeSQLHDESC<'conn, DT, V>);
unsafe impl<V: OdbcVersion, T> AsSQLHANDLE for RefSQLHDESC<'_, T, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.0.as_SQLHANDLE()
    }
}
impl<V: OdbcVersion, T> Handle for RefSQLHDESC<'_, T, V> {
    type Ident = SQL_HANDLE_DESC;
}
impl<'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V> for RefSQLHDESC<'_, DT, V> {}

fn drop_handle<H: Handle>(handle: &mut H) {
    let sql_return = unsafe { ffi::SQLFreeHandle(H::Ident::IDENTIFIER, handle.as_SQLHANDLE()) };

    if sql_return != SQL_SUCCESS && !panicking() {
        panic!(
            "{}: SQLFreeHandle returned: {:?}",
            type_name::<H>(),
            sql_return
        )
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::*;
    use crate::env::SQL_OV_ODBC3_80;

    #[test]
    fn disconnect_C2() {
        let raw_handle = 13 as SQLHANDLE;

        let SQLDisconnect_ctx = ffi::SQLDisconnect_context();
        let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();

        SQLDisconnect_ctx.expect().never();
        SQLFreeHandle_ctx
            .expect()
            .once()
            .withf_st(move |x, y| *x == SQL_HANDLE_DBC::IDENTIFIER && *y == raw_handle)
            .return_const(SQL_SUCCESS);

        unsafe { SQLHDBC::<C2, SQL_OV_ODBC3_80>::from_raw(raw_handle) };
    }

    #[test]
    fn disconnect_C3() {
        let raw_handle = 13 as SQLHANDLE;

        let SQLDisconnect_ctx = ffi::SQLDisconnect_context();
        let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();

        SQLDisconnect_ctx
            .expect()
            .once()
            .withf_st(move |x| *x == raw_handle)
            .return_const(SQL_SUCCESS);
        SQLFreeHandle_ctx
            .expect()
            .once()
            .withf_st(move |x, y| *x == SQL_HANDLE_DBC::IDENTIFIER && *y == raw_handle)
            .return_const(SQL_SUCCESS);

        unsafe { SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle) }.need_data();
    }

    #[test]
    fn disconnect_C4() {
        let raw_handle = 13 as SQLHANDLE;

        let SQLDisconnect_ctx = ffi::SQLDisconnect_context();
        let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();

        SQLDisconnect_ctx
            .expect()
            .once()
            .withf_st(move |x| *x == raw_handle)
            .return_const(SQL_SUCCESS);
        SQLFreeHandle_ctx
            .expect()
            .once()
            .withf_st(move |x, y| *x == SQL_HANDLE_DBC::IDENTIFIER && *y == raw_handle)
            .return_const(SQL_SUCCESS);

        unsafe { SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle) }.connect();
    }

    // TODO: Mockall is buggy and these tests fail more often
    //#[test]
    //#[should_panic]
    //fn disconnect_C3_panic() {
    //    let raw_handle = 13 as SQLHANDLE;

    //    let SQLDisconnect_ctx = ffi::SQLDisconnect_context();
    //    let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();

    //    SQLDisconnect_ctx
    //        .expect()
    //        .once()
    //        .withf_st(move |x| *x == raw_handle)
    //        .return_const(SQL_ERROR);
    //    SQLFreeHandle_ctx
    //        .expect()
    //        .once()
    //        .withf_st(move |x, y| *x == SQL_HANDLE_DBC::IDENTIFIER && *y == raw_handle)
    //        .return_const(SQL_SUCCESS);

    //    SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle).need_data();
    //}

    //#[test]
    //#[should_panic]
    //fn disconnect_C4_panic() {
    //    let raw_handle = 13 as SQLHANDLE;

    //    let SQLDisconnect_ctx = ffi::SQLDisconnect_context();
    //    let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();

    //    SQLDisconnect_ctx
    //        .expect()
    //        .once()
    //        .withf_st(move |x| *x == raw_handle)
    //        .return_const(SQL_ERROR);
    //    SQLFreeHandle_ctx
    //        .expect()
    //        .once()
    //        .withf_st(move |x, y| *x == SQL_HANDLE_DBC::IDENTIFIER && *y == raw_handle)
    //        .return_const(SQL_SUCCESS);

    //    SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle).connect();
    //}
}
