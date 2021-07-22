#[double]
use crate::api::ffi;
use crate::c_types::DeferredBuf;
use crate::conn::{ConnState, C2, C3, C4};
use crate::convert::IntoSQLPOINTER;
use crate::env::{OdbcVersion, SQL_ATTR_ODBC_VERSION, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::{sqlreturn::SQL_SUCCESS, Ident, StrLenOrInd, SQLPOINTER, SQLSMALLINT};
use mockall_double::double;
use std::any::type_name;
use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::thread::panicking;

#[cfg(feature = "odbc_debug")]
use crate::stmt::{
    SQL_ATTR_APP_PARAM_DESC, SQL_ATTR_APP_ROW_DESC, SQL_ATTR_IMP_PARAM_DESC, SQL_ATTR_IMP_ROW_DESC,
};
#[cfg(feature = "odbc_debug")]
use crate::SQLINTEGER;
use std::mem::ManuallyDrop;
#[cfg(feature = "odbc_debug")]
use std::mem::MaybeUninit;

pub trait Handle {
    type Ident: Ident<Type = SQLSMALLINT>;
}

pub unsafe trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}

// TODO: Should be unsafe?
// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub unsafe trait Allocate<'src>: Handle + Drop {
    type SrcHandle: AsSQLHANDLE;
    fn from_raw(handle: SQLHANDLE) -> Self;
}

pub trait SQLCancelHandle {}
pub trait SQLCompleteAsyncHandle {}
pub trait SQLEndTranHandle {}

#[derive(Debug)]
pub enum RowDesc {}
#[derive(Debug)]
pub enum ParamDesc {}

pub trait DescType<'buf> {}

#[derive(Debug)]
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

#[allow(non_camel_case_types)]
pub struct SQL_NULL_HANDLE;
unsafe impl AsSQLHANDLE for SQL_NULL_HANDLE {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        std::ptr::null_mut()
    }
}

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
impl<V: OdbcVersion> Handle for SQLHENV<V> {
    type Ident = SQL_HANDLE_ENV;
}
unsafe impl<V: OdbcVersion> Allocate<'_> for SQLHENV<V> {
    type SrcHandle = SQL_NULL_HANDLE;

    fn from_raw(handle: SQLHANDLE) -> Self {
        let val = Self {
            handle,
            version: PhantomData,
        };

        let sql_return = unsafe {
            ffi::SQLSetEnvAttr(
                val.as_SQLHANDLE(),
                SQL_ATTR_ODBC_VERSION::IDENTIFIER,
                V::IDENTIFIER.into_SQLPOINTER(),
                0, // TODO: Use AttrLen::len()
            )
        };

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
unsafe impl<V: OdbcVersion> AsSQLHANDLE for SQLHENV<V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl<V: OdbcVersion> Drop for SQLHENV<V> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { ffi::SQLFreeHandle(SQL_HANDLE_ENV::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned {:?}",
                type_name::<Self>(),
                sql_return
            )
        }
    }
}
impl<V: OdbcVersion> SQLEndTranHandle for SQLHENV<V> {}

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
impl<C: ConnState, V: OdbcVersion> Handle for SQLHDBC<'_, C, V> {
    type Ident = SQL_HANDLE_DBC;
}
unsafe impl<'env, V: OdbcVersion> Allocate<'env> for SQLHDBC<'env, C2, V> {
    type SrcHandle = SQLHENV<V>;

    fn from_raw(handle: SQLHANDLE) -> Self {
        Self {
            handle,

            parent: PhantomData,
            connected: PhantomData,
            version: PhantomData,
        }
    }
}
impl SQLCancelHandle for SQLHDBC<'_, C4, SQL_OV_ODBC3_80> {}
impl SQLCancelHandle for SQLHDBC<'_, C4, SQL_OV_ODBC4> {}
impl SQLCompleteAsyncHandle for SQLHDBC<'_, C4, SQL_OV_ODBC3_80> {}
impl SQLCompleteAsyncHandle for SQLHDBC<'_, C4, SQL_OV_ODBC4> {}
impl<V: OdbcVersion> SQLEndTranHandle for SQLHDBC<'_, C4, V> {}
unsafe impl<C: ConnState, V: OdbcVersion> AsSQLHANDLE for SQLHDBC<'_, C, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl<C: ConnState, V: OdbcVersion> Drop for SQLHDBC<'_, C, V> {
    fn drop(&mut self) {
        C::disconnect(self);

        let sql_return =
            unsafe { ffi::SQLFreeHandle(SQL_HANDLE_DBC::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned {:?}",
                type_name::<Self>(),
                sql_return
            )
        }
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
pub trait Disconnect<'env, V: OdbcVersion> {}
pub trait BrowseConnect<'env, V: OdbcVersion> {}
impl<'env, V: OdbcVersion> BrowseConnect<'env, V> for SQLHDBC<'env, C2, V> {}
impl<'env, V: OdbcVersion> BrowseConnect<'env, V> for SQLHDBC<'env, C3, V> {}
impl<'env, V: OdbcVersion> Disconnect<'env, V> for SQLHDBC<'env, C3, V> {}
impl<'env, V: OdbcVersion> Disconnect<'env, V> for SQLHDBC<'env, C4, V> {}

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
pub struct SQLHSTMT<'conn, 'stmt, 'buf, V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,

    parent: PhantomData<&'conn ()>,
    version: PhantomData<V>,

    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_ard: Cell<Option<&'stmt SQLHDESC<'stmt, AppDesc<'buf>, V>>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) explicit_apd: Cell<Option<&'stmt SQLHDESC<'stmt, AppDesc<'buf>, V>>>,

    #[cfg(feature = "odbc_debug")]
    pub(crate) ard: ManuallyDrop<SQLHDESC<'stmt, AppDesc<'buf>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) apd: ManuallyDrop<SQLHDESC<'stmt, AppDesc<'buf>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ird: ManuallyDrop<SQLHDESC<'stmt, ImplDesc<RowDesc>, V>>,
    #[cfg(feature = "odbc_debug")]
    pub(crate) ipd: ManuallyDrop<SQLHDESC<'stmt, ImplDesc<ParamDesc>, V>>,

    #[allow(dead_code)]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_ard: Cell<PhantomData<&'stmt SQLHDESC<'stmt, AppDesc<'buf>, V>>>,
    #[allow(dead_code)]
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) explicit_apd: Cell<PhantomData<&'stmt SQLHDESC<'stmt, AppDesc<'buf>, V>>>,

    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ard: PhantomData<SQLHDESC<'stmt, AppDesc<'buf>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) apd: PhantomData<SQLHDESC<'stmt, AppDesc<'buf>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ird: PhantomData<SQLHDESC<'stmt, ImplDesc<RowDesc>, V>>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) ipd: PhantomData<SQLHDESC<'stmt, ImplDesc<ParamDesc>, V>>,
}
impl<'buf, V: OdbcVersion> SQLHSTMT<'_, '_, 'buf, V> {
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

    // TODO: Don't bind (SQLPOINTER, SQLLEN) fat pointer when using raw_api
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_col<TT: Ident, B: DeferredBuf<TT, V>>(&self, TargetValuePtr: Option<&'buf B>)
    where
        B: ?Sized,
    {
    }
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_param<TT: Ident, B: DeferredBuf<TT, V>>(
        &self,
        TargetValuePtr: Option<&'buf B>,
    ) where
        B: ?Sized,
    {
    }
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) fn bind_strlen_or_ind(
        &self,
        StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
    ) {
    }

    #[cfg(feature = "odbc_debug")]
    pub(crate) fn bind_col<TT: Ident, B: DeferredBuf<TT, V>>(&self, TargetValuePtr: Option<&'buf B>)
    where
        B: ?Sized,
    {
        if let Some(explicit_ard) = self.explicit_ard.get() {
            // TODO:
            //explicit_ard.bind_col(TargetValuePtr);
        } else {
            // TODO:
            //self.ard.bind_col(TargetValuePtr);
        }
    }
    #[cfg(feature = "odbc_debug")]
    pub(crate) fn bind_param<TT: Ident, B: DeferredBuf<TT, V>>(
        &self,
        TargetValuePtr: Option<&'buf B>,
    ) where
        B: ?Sized,
    {
        if let Some(explicit_apd) = self.explicit_apd.get() {
            // TODO:
            //explicit_apd.bind_param(TargetValuePtr);
        } else {
            // TODO:
            //self.apd.bind_param(TargetValuePtr);
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

impl<V: OdbcVersion> Handle for SQLHSTMT<'_, '_, '_, V> {
    type Ident = SQL_HANDLE_STMT;
}
unsafe impl<'conn, V: OdbcVersion> Allocate<'conn> for SQLHSTMT<'conn, '_, '_, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn, C4, V>;

    #[cfg(feature = "odbc_debug")]
    fn from_raw(handle: SQLHANDLE) -> Self {
        unsafe {
            let ard = SQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_APP_ROW_DESC>(handle);
            let apd = SQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_APP_PARAM_DESC>(handle);
            let ird = SQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_IMP_ROW_DESC>(handle);
            let ipd = SQLHSTMT::<V>::get_descriptor_handle::<SQL_ATTR_IMP_PARAM_DESC>(handle);

            Self {
                parent: PhantomData,
                version: PhantomData,

                handle,

                ard: ManuallyDrop::new(SQLHDESC::from_raw(ard)),
                apd: ManuallyDrop::new(SQLHDESC::from_raw(apd)),
                ird: ManuallyDrop::new(SQLHDESC::from_raw(ird)),
                ipd: ManuallyDrop::new(SQLHDESC::from_raw(ipd)),

                explicit_ard: Cell::new(None),
                explicit_apd: Cell::new(None),
            }
        }
    }

    #[cfg(not(feature = "odbc_debug"))]
    fn from_raw(handle: SQLHANDLE) -> Self {
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
impl SQLCancelHandle for SQLHSTMT<'_, '_, '_, SQL_OV_ODBC3_80> {}
impl SQLCancelHandle for SQLHSTMT<'_, '_, '_, SQL_OV_ODBC4> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_, '_, '_, SQL_OV_ODBC3_80> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_, '_, '_, SQL_OV_ODBC4> {}
unsafe impl<V: OdbcVersion> AsSQLHANDLE for SQLHSTMT<'_, '_, '_, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl<V: OdbcVersion> Drop for SQLHSTMT<'_, '_, '_, V> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { ffi::SQLFreeHandle(SQL_HANDLE_STMT::IDENTIFIER, self.as_SQLHANDLE()) };

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
pub struct SQLHDESC<'conn, T, V: OdbcVersion> {
    pub(crate) handle: SQLHANDLE,

    parent: PhantomData<&'conn ()>,
    version: PhantomData<V>,

    #[cfg(feature = "odbc_debug")]
    // TODO: Implement properly
    pub(crate) data: PhantomData<T>,
    #[cfg(not(feature = "odbc_debug"))]
    pub(crate) data: PhantomData<T>,
}
impl<'buf, V: OdbcVersion, T: DescType<'buf>> SQLHDESC<'_, T, V> {
    fn from_raw(handle: SQLHANDLE) -> Self {
        Self {
            handle,

            parent: PhantomData,
            version: PhantomData,

            data: PhantomData,
        }
    }
}
impl<'buf, V: OdbcVersion, T: DescType<'buf>> Handle for SQLHDESC<'_, T, V> {
    type Ident = SQL_HANDLE_DESC;
}
unsafe impl<'conn, 'buf, V: OdbcVersion> Allocate<'conn> for SQLHDESC<'conn, AppDesc<'buf>, V> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn, C4, V>;

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC::from_raw(handle)
    }
}
unsafe impl<V: OdbcVersion, T> AsSQLHANDLE for SQLHDESC<'_, T, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
unsafe impl<'buf, V: OdbcVersion, T: DescType<'buf>> IntoSQLPOINTER
    for Option<&SQLHDESC<'_, T, V>>
{
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.map_or_else(std::ptr::null_mut, |handle| handle.as_SQLHANDLE().cast())
    }
}
impl<V: OdbcVersion, T> Drop for SQLHDESC<'_, T, V> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { ffi::SQLFreeHandle(SQL_HANDLE_DESC::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned: {:?}",
                type_name::<Self>(),
                sql_return
            )
        }
    }
}

// TODO: Check https://github.com/microsoft/ODBC-Specification/blob/b7ef71fba508ed010cd979428efae3091b732d75/Windows/inc/sqltypes.h
// This is unixOBDC value
pub type SQLHWND = SQLPOINTER;

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]
    use super::*;

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

        SQLHDBC::<C2, SQL_OV_ODBC3_80>::from_raw(raw_handle);
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

        SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle).need_data();
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

        SQLHDBC::<_, SQL_OV_ODBC3_80>::from_raw(raw_handle).connect();
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
