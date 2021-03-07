use crate::extern_api;
use crate::stmt::{
    StmtAttr, SQL_ATTR_APP_PARAM_DESC, SQL_ATTR_APP_ROW_DESC, SQL_ATTR_IMP_PARAM_DESC,
    SQL_ATTR_IMP_ROW_DESC,
};
use crate::{
    AsMutSQLPOINTER, AsSQLPOINTER, Identifier, Len, OdbcAttr, SQLPOINTER, SQLSMALLINT,
    SQLUSMALLINT, SQL_SUCCESS,
};
use std::any::type_name;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;
use std::thread::panicking;

pub unsafe trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}

pub trait Handle {
    type Identifier: crate::Identifier<IdentType = SQLSMALLINT>;
}

// TODO: Should be unsafe?
// TODO: Where to require Drop? I could make a generic Drop implementation, hmmmm
pub unsafe trait Allocate<'src>: Handle + Drop {
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

pub trait DescType {
    fn new() -> Self;
    fn unbind_record(&self, RecNumber: SQLSMALLINT);
    fn bind_record(&self, RecNumber: SQLSMALLINT, DataPtr: Rc<RefCell<dyn AsMutSQLPOINTER>>) {}
}
pub struct ImplDesc {}
pub struct AppDesc {
    // TODO: Couldn't Vec be used? Try using vec
    // TODO: Check which type to use as a key?
    // TODO: Use UnsafeCell instead?
    pub(crate) SQL_DESC_DATA_PTR: Cell<HashMap<SQLSMALLINT, Rc<RefCell<dyn AsMutSQLPOINTER>>>>,
}
impl AppDesc {
    fn unbind_records(&self) {
        self.SQL_DESC_DATA_PTR.set(HashMap::new())
    }
}
impl DescType for ImplDesc {
    fn new() -> Self {
        Self {}
    }
    fn unbind_record(&self, RecNumber: SQLSMALLINT) {}
    fn bind_record(&self, RecNumber: SQLSMALLINT, DataPtr: Rc<RefCell<dyn AsMutSQLPOINTER>>) {}
}
impl DescType for AppDesc {
    fn new() -> Self {
        Self {
            SQL_DESC_DATA_PTR: Cell::new(HashMap::new()),
        }
    }
    fn unbind_record(&self, RecNumber: SQLSMALLINT) {
        let mut ptrs = self.SQL_DESC_DATA_PTR.take();
        ptrs.remove(&RecNumber);
        self.SQL_DESC_DATA_PTR.set(ptrs);
    }
    fn bind_record(&self, RecNumber: SQLSMALLINT, DataPtr: Rc<RefCell<dyn AsMutSQLPOINTER>>) {
        let mut ptrs = self.SQL_DESC_DATA_PTR.take();
        ptrs.insert(RecNumber, Rc::clone(&DataPtr));
        self.SQL_DESC_DATA_PTR.set(ptrs);
    }
}

#[derive(rs_odbc_derive::Identifier)]
#[identifier(SQLSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_ENV;

#[derive(rs_odbc_derive::Identifier)]
#[identifier(SQLSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_DBC;

#[derive(rs_odbc_derive::Identifier)]
#[identifier(SQLSMALLINT, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_HANDLE_STMT;

#[derive(rs_odbc_derive::Identifier)]
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
pub struct SQLHENV {
    pub(crate) handle: SQLHANDLE,
}
impl Handle for SQLHENV {
    type Identifier = SQL_HANDLE_ENV;
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
pub struct SQLHDBC<'env> {
    parent: PhantomData<&'env ()>,
    pub(crate) handle: SQLHANDLE,
}
impl Handle for SQLHDBC<'_> {
    type Identifier = SQL_HANDLE_DBC;
}
unsafe impl<'env> Allocate<'env> for SQLHDBC<'env> {
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
unsafe impl AsSQLHANDLE for SQLHDBC<'_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHDBC<'_> {
    fn drop(&mut self) {
        let sql_return =
            unsafe { extern_api::SQLFreeHandle(SQL_HANDLE_DBC::IDENTIFIER, self.as_SQLHANDLE()) };

        if sql_return != SQL_SUCCESS && !panicking() {
            panic!(
                "{}: SQLFreeHandle returned {:?}",
                type_name::<Self>(),
                sql_return
            )
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
pub struct SQLHSTMT<'conn> {
    parent: PhantomData<&'conn ()>,
    pub(crate) handle: SQLHANDLE,

    pub(crate) ard: ManuallyDrop<SQLHDESC<'conn, AppDesc>>,
    pub(crate) apd: ManuallyDrop<SQLHDESC<'conn, AppDesc>>,
    pub(crate) ird: ManuallyDrop<SQLHDESC<'conn, ImplDesc>>,
    pub(crate) ipd: ManuallyDrop<SQLHDESC<'conn, ImplDesc>>,

    // TODO: If compliance with ODBC standard is desired, use Weak<_> instead of Option<Rc<_>>
    // Current implementation is expected to be less surprising to a naive user of the library
    pub(crate) explicit_ard: Cell<Option<Rc<SQLHDESC<'conn, AppDesc>>>>,
    pub(crate) explicit_apd: Cell<Option<Rc<SQLHDESC<'conn, AppDesc>>>>,
}
impl SQLHSTMT<'_> {
    unsafe fn get_descriptor_handle<T: StmtAttr>(handle: SQLHANDLE) -> SQLHANDLE {
        let mut descriptor_handle = MaybeUninit::uninit();

        let sql_return = extern_api::SQLGetStmtAttrA(
            handle,
            T::IDENTIFIER,
            descriptor_handle.as_mut_ptr() as SQLPOINTER,
            0,
            &mut 0,
        );
        if sql_return != SQL_SUCCESS {
            panic!(
                "{}: SQLGetStmtAttr returned {:?}",
                type_name::<T>(),
                sql_return
            );
        }

        descriptor_handle.assume_init()
    }

    #[allow(non_snake_case)]
    pub(crate) fn bind_param(
        &self,
        ColumnNumber: SQLSMALLINT,
        TargetValuePtr: Rc<RefCell<dyn AsMutSQLPOINTER>>,
    ) {
        let explicit_apd = self.explicit_apd.take();
        if let Some(explicit_apd) = &explicit_apd {
            explicit_apd.data.bind_record(ColumnNumber, TargetValuePtr);
        } else {
            self.apd.data.bind_record(ColumnNumber, TargetValuePtr);
        }

        self.explicit_apd.set(explicit_apd);
    }
    #[allow(non_snake_case)]
    pub(crate) fn bind_col(
        &self,
        ColumnNumber: SQLSMALLINT,
        ParameterValuePtr: Rc<RefCell<dyn AsMutSQLPOINTER>>,
    ) {
        let explicit_ard = self.explicit_ard.take();
        if let Some(explicit_ard) = &explicit_ard {
            explicit_ard
                .data
                .bind_record(ColumnNumber, ParameterValuePtr);
        } else {
            self.ard.data.bind_record(ColumnNumber, ParameterValuePtr);
        }

        self.explicit_ard.set(explicit_ard);
    }
    #[allow(non_snake_case)]
    pub(crate) fn unbind_param(&self, ColumnNumber: SQLSMALLINT) {
        let explicit_apd = self.explicit_apd.take();
        if let Some(explicit_apd) = &explicit_apd {
            explicit_apd.data.unbind_record(ColumnNumber);
        } else {
            self.apd.data.unbind_record(ColumnNumber);
        }

        self.explicit_apd.set(explicit_apd);
    }
    #[allow(non_snake_case)]
    pub(crate) fn unbind_col(&self, ColumnNumber: SQLSMALLINT) {
        let explicit_ard = self.explicit_ard.take();
        if let Some(explicit_ard) = &explicit_ard {
            explicit_ard.data.unbind_record(ColumnNumber);
        } else {
            self.ard.data.unbind_record(ColumnNumber);
        }

        self.explicit_ard.set(explicit_ard);
    }
    pub(crate) fn unbind_cols(&self) {
        let explicit_ard = self.explicit_ard.take();
        if let Some(explicit_ard) = &explicit_ard {
            explicit_ard.data.unbind_records();
        } else {
            self.ard.data.unbind_records();
        }

        self.explicit_ard.set(explicit_ard);
    }
    pub(crate) fn unbind_params(&self) {
        let explicit_apd = self.explicit_apd.take();
        if let Some(explicit_apd) = &explicit_apd {
            explicit_apd.data.unbind_records();
        } else {
            self.apd.data.unbind_records();
        }

        self.explicit_apd.set(explicit_apd);
    }
}
impl Handle for SQLHSTMT<'_> {
    type Identifier = SQL_HANDLE_STMT;
}
unsafe impl<'conn> Allocate<'conn> for SQLHSTMT<'conn> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn>;

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
                ird: ManuallyDrop::new(SQLHDESC::<ImplDesc>::from_raw(ird)),
                ipd: ManuallyDrop::new(SQLHDESC::<ImplDesc>::from_raw(ipd)),

                explicit_ard: Cell::new(None),
                explicit_apd: Cell::new(None),
            }
        }
    }
}
impl SQLCancelHandle for SQLHSTMT<'_> {}
impl SQLCompleteAsyncHandle for SQLHSTMT<'_> {}
unsafe impl AsSQLHANDLE for SQLHSTMT<'_> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
impl Drop for SQLHSTMT<'_> {
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
pub struct SQLHDESC<'conn, T: DescType> {
    parent: PhantomData<&'conn ()>,
    pub(crate) handle: SQLHANDLE,
    // TODO
    //pub(crate) bookmark: Rc<RefCell<dyn AsMutSQLPOINTER>>,
    pub(crate) data: T,
}
impl<T: DescType> SQLHDESC<'_, T> {
    pub fn unbind_record(&self, RecNumber: SQLSMALLINT) {
        self.data.unbind_record(RecNumber);
    }
    pub fn bind_record(&self, RecNumber: SQLSMALLINT, DataPtr: Rc<RefCell<dyn AsMutSQLPOINTER>>) {
        self.data.bind_record(RecNumber, DataPtr);
    }

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC {
            handle,
            parent: PhantomData,
            data: T::new(),
        }
    }
}
impl<T: DescType> Handle for SQLHDESC<'_, T> {
    type Identifier = SQL_HANDLE_DESC;
}
unsafe impl<'conn> Allocate<'conn> for SQLHDESC<'conn, AppDesc> {
    // Valid because SQLHDBC is covariant
    type SrcHandle = SQLHDBC<'conn>;

    fn from_raw(handle: SQLHANDLE) -> Self {
        SQLHDESC::<AppDesc>::from_raw(handle)
    }
}
unsafe impl<T: DescType> AsSQLHANDLE for SQLHDESC<'_, T> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
// TODO: use derive odbc_type somehow?
unsafe impl<T: DescType> AsSQLPOINTER for Rc<SQLHDESC<'_, T>> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        self.as_SQLHANDLE().cast()
    }
}
// TODO: This can be removed
unsafe impl<LEN: Copy, T: DescType> Len<OdbcAttr, LEN> for Rc<SQLHDESC<'_, T>>
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(crate::SQL_IS_POINTER)
    }
}
// TODO: This can be removed
unsafe impl AsMutSQLPOINTER for MaybeUninit<&SQLHDESC<'_, ImplDesc>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        unimplemented!()
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for MaybeUninit<&SQLHDESC<'_, ImplDesc>>
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();

    fn len(&self) -> LEN {
        LEN::from(crate::SQL_IS_POINTER)
    }
}
impl<T: DescType> Drop for SQLHDESC<'_, T> {
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
