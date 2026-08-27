use core::{
    cell::UnsafeCell,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::{
    Defined,
    attr::{AttrLen, CONNECTION_POOLING, CP_MATCH, ODBC_VERSION},
    c_type::CType,
    col::{ColAttrGet, Unavailable},
    conn::{
        self, BrowseConnect, C2, C3, C4, CompletionType, ConnAttrGet, ConnAttrSet, ConnState,
        Disconnect, DriverCompletion,
    },
    data::{CHAR, INTEGER, LEN, POINTER, SETPOSIROW, SMALLINT, WCHAR},
    desc::{AppDesc, DescFieldGet, DescFieldSet, DescType},
    diag::{DiagField, STATE_SIZE},
    env::{DataSourceDirection, EnvAttrGet, EnvAttrSet, OdbcVersion, VersionValue},
    info::InfoType,
    sql_type::{SqlType, SqlTypeCode},
    sqlreturn::RETURN,
    stmt::{
        BulkOperation, FreeStmtOption, IOType, LockType, NullAllowed, Operation, StmtAttrGet,
        StmtAttrSet,
    },
    str::{OdbcChar, OdbcStr},
};

/// Marker for identifiers accepted by environment attribute functions.
trait ToOwnedHandle {
    type Owned;
}

trait FfiAllocate: ToOwnedHandle {
    type Source: ?Sized;
}

trait BrowseConnectTransition: BrowseConnect {
    fn transition<'env, V: OdbcVersion>(
        handle: OwnedHDBC<'env, V, Self>,
        result: RETURN,
    ) -> BrowseConnectResult<'env, V, Self>
    where
        Self: Sized;
}

#[derive(Debug)]
#[must_use]
pub enum AllocHandleResult<H> {
    /// Allocation completed with diagnostic information.
    SuccessWithInfo(H),
    /// Allocation completed successfully.
    Success(H),
    /// Allocation failed.
    Error,
}

/// Result of `SQLConnect` together with the connection state implied by the ODBC return code.
#[must_use]
pub enum ConnectResult<'env, V: OdbcVersion> {
    /// The connection completed with diagnostic information and advanced to C4.
    SuccessWithInfo(OwnedHDBC<'env, V, C4>),
    /// The connection completed successfully and advanced to C4.
    Success(OwnedHDBC<'env, V, C4>),
    /// The connection failed and remains in C2.
    Error(OwnedHDBC<'env, V, C2>),
    /// An asynchronous connection operation is still executing.
    StillExecuting(OwnedHDBC<'env, V, C2>),
}

impl<'env, V: OdbcVersion> ConnectResult<'env, V> {
    /// Returns the connected C4 handle for either successful ODBC return code.
    ///
    /// # Panics
    ///
    /// Panics if the connection was not established.
    pub fn unwrap(self) -> OwnedHDBC<'env, V, C4> {
        match self {
            Self::SuccessWithInfo(handle) | Self::Success(handle) => handle,
            Self::Error(_) => panic!("called `ConnectResult::unwrap()` on an `Error` value"),
            Self::StillExecuting(_) => {
                panic!("called `ConnectResult::unwrap()` on a `StillExecuting` value")
            }
        }
    }

    /// Returns the C2 handle when the connection operation returned an error.
    ///
    /// # Panics
    ///
    /// Panics if the operation succeeded, is still executing, or was cancelled.
    pub fn unwrap_err(self) -> OwnedHDBC<'env, V, C2> {
        match self {
            Self::Error(handle) => handle,
            Self::SuccessWithInfo(_) | Self::Success(_) => {
                panic!("called `ConnectResult::unwrap_err()` on a success value")
            }
            Self::StillExecuting(_) => {
                panic!("called `ConnectResult::unwrap_err()` on a `StillExecuting` value")
            }
        }
    }
}

/// Result of `SQLDriverConnect` together with the connection state implied by
/// the ODBC return code.
#[must_use]
pub enum DriverConnectResult<'env, V: OdbcVersion> {
    /// The connection completed with diagnostic information and advanced to C4.
    SuccessWithInfo(OwnedHDBC<'env, V, C4>),
    /// The connection completed successfully and advanced to C4.
    Success(OwnedHDBC<'env, V, C4>),
    /// The connection failed and remains in C2.
    Error(OwnedHDBC<'env, V, C2>),
    /// An asynchronous connection operation is still executing.
    StillExecuting(OwnedHDBC<'env, V, C2>),
    /// The operation was cancelled before establishing a connection.
    NoData(OwnedHDBC<'env, V, C2>),
}

impl<'env, V: OdbcVersion> DriverConnectResult<'env, V> {
    /// Returns the connected C4 handle for either successful ODBC return code.
    ///
    /// # Panics
    ///
    /// Panics if the connection was not established.
    pub fn unwrap(self) -> OwnedHDBC<'env, V, C4> {
        match self {
            Self::SuccessWithInfo(handle) | Self::Success(handle) => handle,
            Self::Error(_) => panic!("called `DriverConnectResult::unwrap()` on an `Error` value"),
            Self::StillExecuting(_) => {
                panic!("called `DriverConnectResult::unwrap()` on a `StillExecuting` value")
            }
            Self::NoData(_) => {
                panic!("called `DriverConnectResult::unwrap()` on a `NoData` value")
            }
        }
    }

    /// Returns the C2 handle when the connection operation returned an error.
    ///
    /// # Panics
    ///
    /// Panics if the operation succeeded, is still executing, or was cancelled.
    pub fn unwrap_err(self) -> OwnedHDBC<'env, V, C2> {
        match self {
            Self::Error(handle) => handle,
            Self::SuccessWithInfo(_) | Self::Success(_) => {
                panic!("called `DriverConnectResult::unwrap_err()` on a success value")
            }
            Self::StillExecuting(_) => {
                panic!("called `DriverConnectResult::unwrap_err()` on a `StillExecuting` value")
            }
            Self::NoData(_) => {
                panic!("called `DriverConnectResult::unwrap_err()` on a `NoData` value")
            }
        }
    }
}

/// Result of `SQLDisconnect` together with the connection state implied by the
/// ODBC return code.
#[must_use]
pub enum DisconnectResult<'env, V: OdbcVersion, S: Disconnect> {
    /// Disconnection completed with diagnostic information and returned the
    /// connection to C2.
    SuccessWithInfo(OwnedHDBC<'env, V, C2>),
    /// Disconnection completed successfully and returned the connection to C2.
    Success(OwnedHDBC<'env, V, C2>),
    /// Disconnection failed and left the connection in its original state.
    Error(OwnedHDBC<'env, V, S>),
    /// An asynchronous disconnection is still executing.
    StillExecuting(OwnedHDBC<'env, V, S>),
}

impl<'env, V: OdbcVersion, S: Disconnect> DisconnectResult<'env, V, S> {
    /// Returns the disconnected C2 handle for either successful ODBC return code.
    ///
    /// # Panics
    ///
    /// Panics if disconnection did not complete successfully.
    pub fn unwrap(self) -> OwnedHDBC<'env, V, C2> {
        match self {
            Self::SuccessWithInfo(handle) | Self::Success(handle) => handle,
            Self::Error(_) => panic!("called `DisconnectResult::unwrap()` on an `Error` value"),
            Self::StillExecuting(_) => {
                panic!("called `DisconnectResult::unwrap()` on a `StillExecuting` value")
            }
        }
    }

    /// Returns the original handle when disconnection returned an error.
    ///
    /// # Panics
    ///
    /// Panics if disconnection succeeded or is still executing.
    pub fn unwrap_err(self) -> OwnedHDBC<'env, V, S> {
        match self {
            Self::Error(handle) => handle,
            Self::SuccessWithInfo(_) | Self::Success(_) => {
                panic!("called `DisconnectResult::unwrap_err()` on a success value")
            }
            Self::StillExecuting(_) => {
                panic!("called `DisconnectResult::unwrap_err()` on a `StillExecuting` value")
            }
        }
    }
}

/// Result of `SQLBrowseConnect` together with the connection state implied by
/// the ODBC return code.
#[must_use]
pub enum BrowseConnectResult<'env, V: OdbcVersion, S: BrowseConnect> {
    /// Browsing completed with diagnostic information and advanced to C4.
    SuccessWithInfo(OwnedHDBC<'env, V, C4>),
    /// Browsing completed successfully and advanced to C4.
    Success(OwnedHDBC<'env, V, C4>),
    /// More connection information is required and the connection is in C3.
    NeedData(OwnedHDBC<'env, V, C3>),
    /// Browsing did not establish a connection, leaving or returning it to C2.
    Error(OwnedHDBC<'env, V, C2>),
    /// An asynchronous browse operation is still executing. The connection
    /// remains in the state in which the operation was started.
    StillExecuting(OwnedHDBC<'env, V, S>),
}

impl<'env, V: OdbcVersion, S: BrowseConnect> BrowseConnectResult<'env, V, S> {
    /// Returns the connected C4 handle.
    ///
    /// # Panics
    ///
    /// Panics if browsing did not complete successfully.
    pub fn unwrap(self) -> OwnedHDBC<'env, V, C4> {
        match self {
            Self::SuccessWithInfo(handle) | Self::Success(handle) => handle,
            Self::NeedData(_) => {
                panic!("called `BrowseConnectResult::unwrap()` on a `NeedData` value")
            }
            Self::Error(_) => {
                panic!("called `BrowseConnectResult::unwrap()` on an `Error` value")
            }
            Self::StillExecuting(_) => {
                panic!("called `BrowseConnectResult::unwrap()` on a `StillExecuting` value")
            }
        }
    }

    /// Returns the unconnected C2 handle produced by an ODBC error.
    ///
    /// # Panics
    ///
    /// Panics if browsing connected, needs more data, or is still executing.
    pub fn unwrap_err(self) -> OwnedHDBC<'env, V, C2> {
        match self {
            Self::Error(handle) => handle,
            Self::SuccessWithInfo(_) | Self::Success(_) => {
                panic!("called `BrowseConnectResult::unwrap_err()` on a success value")
            }
            Self::NeedData(_) => {
                panic!("called `BrowseConnectResult::unwrap_err()` on a `NeedData` value")
            }
            Self::StillExecuting(_) => {
                panic!("called `BrowseConnectResult::unwrap_err()` on a `StillExecuting` value")
            }
        }
    }
}

impl<H> AllocHandleResult<H> {
    /// Returns the allocated handle for either successful ODBC return code.
    ///
    /// # Panics
    ///
    /// Panics if handle allocation returned an error.
    pub fn unwrap(self) -> H {
        match self {
            Self::SuccessWithInfo(handle) | Self::Success(handle) => handle,
            Self::Error => panic!("called `AllocHandleResult::unwrap()` on an `Error` value"),
        }
    }

    /// Confirms that handle allocation returned an error.
    ///
    /// # Panics
    ///
    /// Panics if handle allocation succeeded.
    pub fn unwrap_err(self) {
        match self {
            Self::Error => {}
            Self::SuccessWithInfo(_) | Self::Success(_) => {
                panic!("called `AllocHandleResult::unwrap_err()` on a success value")
            }
        }
    }
}

impl<V: OdbcVersion> FfiAllocate for HENV<V> {
    // FIXME: It would have been better to use c_void but it's not allowed by co3
    // if not c_void then what other opaque type can be used here? However, it
    // would have been more typesafe for us to have used a direct type like
    // Option<*const c_void> and likewise for others because atm we're allowing
    // all of FfiAllocate::Source types to be None
    type Source = Unavailable<u32>;
}

impl<'env, V: OdbcVersion> FfiAllocate for HDBC<'env, V, C2> {
    type Source = HENV<V>;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion> FfiAllocate for HSTMT<'conn, 'desc, 'buf, V> {
    type Source = HDBC<'conn, V, C4>;
}

impl<'conn, 'buf, V: OdbcVersion> FfiAllocate for HDESC<'conn, V, AppDesc<'buf>> {
    type Source = HDBC<'conn, V, C4>;
}

impl<V: OdbcVersion> ToOwnedHandle for HENV<V> {
    type Owned = OwnedHENV<V>;
}

impl<'env, V: OdbcVersion, C: ConnState> ToOwnedHandle for HDBC<'env, V, C> {
    type Owned = OwnedHDBC<'env, V, C>;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion> ToOwnedHandle for HSTMT<'conn, 'desc, 'buf, V> {
    type Owned = OwnedHSTMT<'conn, 'desc, 'buf, V>;
}

impl<'conn, V: OdbcVersion, DT: DescType> ToOwnedHandle for HDESC<'conn, V, DT> {
    type Owned = OwnedHDESC<'conn, V, DT>;
}

#[cfg_attr(windows, link(name = "odbc32", kind = "dylib"))]
#[cfg_attr(
    all(not(windows), feature = "static"),
    link(name = "odbc", kind = "static")
)]
#[cfg_attr(
    all(not(windows), feature = "static"),
    link(name = "ltdl", kind = "static")
)]
#[cfg_attr(
    all(not(windows), not(feature = "static")),
    link(name = "odbc", kind = "dylib")
)]
#[expect(clippy::duplicated_attributes)]
unsafe extern "system" {}

co3::ffi! {
    #![unsafe(extern("system"))]

    #![symbol_fragments {
        CHAR = "A",
        WCHAR = "W"
    }]

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
    #[tag(i16, unsafe(1))]
    pub type HENV<V: OdbcVersion = OV_ODBC3_80>;

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
    #[tag(i16, unsafe(2))]
    #[unsafe(covariant('env))]
    pub type HDBC<'env, V: OdbcVersion = OV_ODBC3_80, C: conn::ConnState>;

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
    #[tag(i16, unsafe(3))]
    #[unsafe(covariant('conn, 'desc, 'buf))]
    pub type HSTMT<'conn, 'desc, 'buf, V: OdbcVersion = OV_ODBC3_80>;

    /// A descriptor is a collection of metadata that describes the parameters of an SQL
    /// statement or the columns of a result set. Thus, a descriptor can fill four roles:
    /// * (APD)Application Parameter Descriptor:
    ///   Contains information about the application buffers bound to the parameters in an
    ///   SQL statement, such as their addresses, lengths, and C data types.
    /// * (IPD)Implementation Parameter Descriptor:
    ///   Contains information about the parameters in an SQL statement, such as their SQL
    ///   data types, lengths, and nullability.
    /// * (ARD)Application Row Descriptor:
    ///   Contains information about the application buffers bound to the columns in a
    ///   result set, such as their addresses, lengths, and C data types.
    /// * (IRD)Implementation Row Descriptor:
    ///   Contains information about the columns in a result set, such as their SQL data
    ///   types, lengths, and nullability.
    ///
    /// Four descriptors are allocated automatically when a statement is allocated, but
    /// applications can also allocate descriptors with SQLAllocHandle. They are allocated on
    /// a connection and can be associated with one or more statements on that connection to
    /// fulfill the role of an APD or ARD on those statements.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/descriptor-handles
    #[tag(i16, unsafe(4))]
    #[unsafe(covariant('conn))]
    pub type HDESC<'conn, V: OdbcVersion = OV_ODBC3_80, DT: DescType>;

    impl<V: OdbcVersion> Drop for dyn HENV<V> {
        /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
        ///
        /// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
        ///
        /// # Panics
        ///
        /// Panics if the DM returns value other than SQL_SUCCESS
        #[symbol_name = "SQLFreeHandle"]
        fn drop(&mut self) -> RETURN;
    }

    impl<V: OdbcVersion, C: conn::ConnState> Drop for dyn HDBC<'_, V, C> {
        /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
        ///
        /// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
        ///
        /// # Panics
        ///
        /// Panics if the DM returns value other than SQL_SUCCESS
        #[symbol_name = "SQLFreeHandle"]
        fn drop(&mut self) -> RETURN;
    }

    impl<V: OdbcVersion> Drop for dyn HSTMT<'_, '_, '_, V> {
        /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
        ///
        /// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
        ///
        /// # Panics
        ///
        /// Panics if the DM returns value other than SQL_SUCCESS
        #[symbol_name = "SQLFreeHandle"]
        fn drop(&mut self) -> RETURN;
    }

    impl<V: OdbcVersion, DT: DescType> Drop for dyn HDESC<'_, V, DT> {
        /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
        ///
        /// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
        ///
        /// # Panics
        ///
        /// Panics if the DM returns value other than SQL_SUCCESS
        #[symbol_name = "SQLFreeHandle"]
        fn drop(&mut self) -> RETURN;
    }

    /// Allocates an environment, connection, statement, or descriptor handle.
    ///
    /// For complete documentation on `SQLAllocHandle`, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlallochandle-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_INVALID_HANDLE, or SQL_ERROR.
    #[symbol_name = "SQLAllocHandle"]
    fn alloc_handle<'src, dyn(i16) H: FfiAllocate, V: OdbcVersion>(
        input_handle: Option<&'src H::Source>,
        output_handle: &mut MaybeUninit<H::Owned>,
    ) -> RETURN
    where
        use<H> @ (
            <HENV<V>> |
            <HDBC<'_, V, C2>> |
            <HSTMT<'_, '_, '_, V>> |
            <HDESC<'_, V, AppDesc<'_>>>
        );

    /// Sets an environment attribute on `SQL_NULL_HANDLE`.
    //#[symbol_name = "SQLSetEnvAttr"]
    //pub fn set_attr<'a, dyn(i32) A: EnvAttrSet<OV_ODBC3_80>>(
    //    attribute: <dyn A>::TAG,
    //    #[unpack(POINTER, INTEGER)]
    //    move value: <A as EnvAttrSet<OV_ODBC3_80>>::Value<'a, CHAR>,
    //) -> RETURN
    //where
    //    use<A> @ <CONNECTION_POOLING>;

    impl<V: OdbcVersion> HENV<V> {
        /// Sets attributes that govern aspects of environments.
        ///
        /// For complete documentation on SQLSetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetenvattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLSetEnvAttr"]
        pub fn set_attr<'a, dyn(i32) A: EnvAttrSet<V>>(
            &mut self,
            attribute: <dyn A>::TAG,
            #[unpack(POINTER, INTEGER)]
            move value: <A as EnvAttrSet<V>>::Value<'a, CHAR>,
        ) -> RETURN
        where
            use<A> @ (<ODBC_VERSION> | <CP_MATCH> | <CONNECTION_POOLING>);

        /// Returns the current setting of an environment attribute.
        ///
        /// For complete documentation on SQLGetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetenvattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetEnvAttr"]
        pub fn get_attr<dyn(i32) A: EnvAttrGet<V>>(
            &self,
            attribute: <dyn A>::TAG,
            // TODO: Would be good to support unpack3?
            #[unpack(POINTER, INTEGER)]
            value: Option<&mut <A as EnvAttrGet<V>>::Buffer<CHAR>>,
            string_length: Option<&mut MaybeUninit<i32>>,
        ) -> RETURN
        where
            use<A> @ (<CP_MATCH> | <CONNECTION_POOLING>);

        /// Returns information about a data source. This function is implemented only by the Driver Manager.
        ///
        /// For complete documentation on SQLDataSourcesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLDataSources{C}"]
        pub fn data_sources<C>(
            &self,
            move direction: DataSourceDirection,
            #[unpack(_, SMALLINT)]
            server_name: Option<&mut OdbcStr<MaybeUninit<C>>>,
            name_length1: Option<&mut MaybeUninit<i16>>,
            #[unpack(_, SMALLINT)]
            description: Option<&mut OdbcStr<MaybeUninit<C>>>,
            name_length2: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
        ///
        /// For complete documentation on SQLDriversA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLDrivers{C}"]
        pub fn drivers<C>(
            &self,
            direction: u16,
            #[unpack(_, SMALLINT)]
            deriver_description: Option<&mut OdbcStr<MaybeUninit<C>>>,
            description_length: Option<&mut MaybeUninit<i16>>,
            #[unpack(_, SMALLINT)]
            driver_attributes: Option<&mut OdbcStr<MaybeUninit<C>>>,
            attributes_length: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);
    }

    impl<V: OdbcVersion, S: conn::ConnState> HDBC<'_, V, S> {
        /// Sets attributes that govern aspects of connections.
        ///
        /// For complete documentation on SQLSetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLSetConnectAttr{C}"]
        pub fn set_attr<'a, dyn(i32) A: ConnAttrSet<V, S>, C: OdbcChar>(
            &mut self,
            attribute: <dyn A>::TAG,
            #[unpack(POINTER, AttrLen<<A as Defined>::By, i32> => INTEGER)]
            move value: <A as ConnAttrSet<V, S>>::Value<'a, C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the current setting of a connection attribute.
        ///
        /// For complete documentation on SQLGetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetConnectAttr{C}"]
        pub fn get_attr<dyn(i32) A: ConnAttrGet<V, S>, C: OdbcChar>(
            &self,
            attribute: <dyn A>::TAG,
            #[unpack(POINTER, AttrLen<<A as Defined>::By, i32> => INTEGER)]
            value: Option<&mut <A as ConnAttrGet<V, S>>::Buffer<C>>,
            string_length: Option<&mut MaybeUninit<i32>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        //#[symbol_name = "SQLGetFunctions"]
        //pub fn get_functions<dyn(u16) F: Function>(
        //    &self,
        //    function_id: <dyn F>::TAG,
        //    TODO: I think it has to erased to a common representation
        //    supported: &mut <F as Function>::Supported,
        //) -> RETURN;

        /// Returns general information about the driver and data source associated with a connection.
        ///
        /// For complete documentation on SQLGetInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetInfo{C}"]
        pub fn get_info<dyn(u16) I: InfoType<V>, C: OdbcChar>(
            &self,
            info_type: <dyn I>::TAG,
            #[unpack(POINTER, SMALLINT)]
            info_value: Option<&mut <I as InfoType<V>>::Buffer<C>>,
            string_length: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
        ///
        /// For complete documentation on SQLNativeSqlA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLNativeSql{C}"]
        pub fn native_sql<C>(
            &self,
            #[unpack(_, INTEGER)]
            in_statement_text: &OdbcStr<C>,
            #[unpack(_, INTEGER)]
            out_statement_text: Option<&mut OdbcStr<MaybeUninit<C>>>,
            text_length2_ptr: Option<&mut MaybeUninit<i32>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);
    }

    impl<V: OdbcVersion, S: BrowseConnect> HDBC<'_, V, S> {
        /// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
        ///
        /// For complete documentation on SQLBrowseConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLBrowseConnect{C}"]
        fn browse_connect<C>(
            &mut self,
            #[unpack(_, SMALLINT)] in_connection_string: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] out_connection_string: Option<&mut OdbcStr<MaybeUninit<C>>>,
            string_length2: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);
    }

    impl<V: OdbcVersion> HDBC<'_, V, C2> {
        /// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
        ///
        /// For complete documentation on SQLConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLConnect{C}"]
        fn connect<C>(
            &mut self,
            #[unpack(_, SMALLINT)] server_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] user_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] authentication: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
        ///
        /// For complete documentation on SQLDriverConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLDriverConnect{C}"]
        fn driver_connect<C>(
            &mut self,
            // TODO: This is a raw pointer, yet it's nullable
            move window_handle: POINTER,
            #[unpack(_, SMALLINT)] in_connection_string: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] out_connection_string: Option<&mut OdbcStr<MaybeUninit<C>>>,
            string_length2: Option<&mut MaybeUninit<i16>>,
            move driver_completion: DriverCompletion,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);
    }

    impl<V: OdbcVersion, S: Disconnect> HDBC<'_, V, S> {
        /// Closes the connection associated with a specific connection handle.
        ///
        /// For complete documentation on SQLDisconnect, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldisconnect-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLDisconnect"]
        fn disconnect(&mut self) -> RETURN;
    }

    impl<V: OdbcVersion> HSTMT<'_, '_, '_, V> {
        /// Sets attributes related to a statement.
        ///
        /// For complete documentation on SQLSetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLSetStmtAttr{C}"]
        pub fn set_attr<'a, dyn(i32) A: StmtAttrSet<V>, C: OdbcChar>(
            &mut self,
            attribute: <dyn A>::TAG,
            #[unpack(POINTER, AttrLen<<A as Defined>::By, i32> => INTEGER)]
            move value: <A as StmtAttrSet<V>>::Value<'a, C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the current setting of a statement attribute.
        ///
        /// For complete documentation on SQLGetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetStmtAttr{C}"]
        pub fn get_attr<dyn(i32) A: StmtAttrGet<V>, C: OdbcChar>(
            &self,
            attribute: <dyn A>::TAG,
            #[unpack(POINTER, AttrLen<<A as Defined>::By, i32> => INTEGER)]
            value: Option<&mut <A as StmtAttrGet<V>>::Buffer<C>>,
            string_length: Option<&mut MaybeUninit<i32>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
        ///
        /// For complete documentation on SQLExecute, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
        #[symbol_name = "SQLExecute"]
        pub fn execute(&mut self) -> RETURN;

        /// Fetches the next rowset of data from the result set and returns data for all bound columns.
        ///
        /// For complete documentation on SQLFetch, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLFetch"]
        pub fn fetch(&self) -> RETURN;

        /// Cancels the processing on a statement.
        /// To cancel processing on a connection or statement, use SQLCancelHandle Function.
        ///
        /// For complete documentation on SQLCancel, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancel-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLCancel"]
        pub fn cancel(&self) -> RETURN;

        /// Closes a cursor that has been opened on a statement and discards pending results.
        ///
        /// For complete documentation on SQLCloseCursor, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlclosecursor-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLCloseCursor"]
        pub fn close_cursor(&mut self) -> RETURN;

        /// Advances to the next result produced by the statement, if any.
        // TODO: Maybe this function should be unsafe.
        #[symbol_name = "SQLMoreResults"]
        pub fn more_results(&self) -> RETURN;

        /// Returns the number of parameters in an SQL statement.
        ///
        /// For complete documentation on SQLNumParams, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumparams-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLNumParams"]
        pub fn num_params(&self, parameter_count: &mut MaybeUninit<i16>) -> RETURN;

        /// Returns the number of columns in a result set.
        ///
        /// For complete documentation on SQLNumResultCols, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumresultcols-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLNumResultCols"]
        pub fn num_result_cols(&self, column_count: &mut MaybeUninit<i16>) -> RETURN;

        /// Prepares an SQL string for execution.
        ///
        /// For complete documentation on SQLPrepareA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLPrepare{C}"]
        pub fn prepare<C>(
            &mut self,
            #[unpack(_, INTEGER)] statement_text: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
        ///
        /// For complete documentation on SQLExecDirectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
        #[symbol_name = "SQLExecDirect{C}"]
        pub fn exec_direct<C>(
            &mut self,
            #[unpack(_, INTEGER)] statement_text: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Can return:
        ///
        /// * A list of foreign keys in the specified table (columns in the specified table that refer to primary keys in other tables).
        /// * A list of foreign keys in other tables that refer to the primary key in the specified table.
        ///
        /// The driver returns each list as a result set on the specified statement.
        ///
        /// For complete documentation on SQLForeignKeysA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlforeignkeys-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLForeignKeys{C}"]
        pub fn foreign_keys<C>(
            &self,
            #[unpack(_, SMALLINT)] pk_catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] pk_schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] pk_table_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] fk_catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] fk_schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] fk_table_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
        ///
        /// For complete documentation on SQLPrimaryKeysA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLPrimaryKeys{C}"]
        pub fn primary_keys<C>(
            &self,
            #[unpack(_, SMALLINT)] pk_catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] pk_schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] pk_table_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the cursor name associated with a specified statement.
        ///
        /// For complete documentation on SQLGetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetCursorName{C}"]
        pub fn get_cursor_name<C>(
            &self,
            #[unpack(_, SMALLINT)]
            cursor_name: &mut OdbcStr<MaybeUninit<C>>,
            name_length: &mut MaybeUninit<i16>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        //#[symbol_name = "SQLParamData"]
        //pub fn param_data(&mut self, value: &mut MutSQLPOINTER) -> RETURN;

    //    #[symbol_name = "SQLPutData"]
    //    pub fn put_data(
    //        &mut self,
    //        data_ptr: POINTER,
    //        str_len_or_ind: isize,
    //    ) -> RETURN;

    //    #[symbol_name = "SQLFetchScroll"]
    //    pub fn fetch_scroll<dyn(i16) FO: FetchOrientation>(
    //        &self,
    //        fetch_orientation: FO,
    //        fetch_offset: <FO as FetchOrientation>::Offset,
    //    ) -> RETURN;

        /// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
        ///
        /// For complete documentation on SQLBulkOperations, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbulkoperations-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLBulkOperations"]
        pub fn bulk_operations(&mut self, move operation: BulkOperation) -> RETURN;

        /// Stops processing associated with a specific statement, closes any open cursors associated with the statement, discards pending results, or, optionally, frees all resources associated with the statement handle.
        ///
        /// For complete documentation on SQLFreeStmt, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreestmt-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLFreeStmt"]
        pub fn free_stmt(&mut self, move option: FreeStmtOption) -> RETURN;

        //#[symbol_name = "SQLGetTypeInfo{C}"]
        //pub fn get_type_info<C>(&self, data_type: DataType) -> RETURN
        //where
        //    use<C> @ (<CHAR> | <WCHAR>);

        #[symbol_name = "SQLColAttribute{C}"]
        pub fn col_attribute<dyn(u16) A: ColAttrGet<V>, C: OdbcChar>(
            &self,
            column_number: u16,
            field_identifier: <dyn A>::TAG,
            #[unpack(POINTER, AttrLen<<A as Defined>::By, i16> => SMALLINT)]
            character_attribute: Option<&mut <A as ColAttrGet<V>>::CharacterBuffer<C>>,
            string_length: Option<&mut MaybeUninit<i16>>,
            // TODO:This should best and most correct be
            //#[unpack(Option<&mut MaybeUninit<LEN>>)]
            #[unpack(*mut MaybeUninit<LEN>)]
            numeric_attribute: Option<&mut <A as ColAttrGet<V>>::NumericBuffer>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `self`.
        ///
        /// For complete documentation on SQLColumnPrivilegesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLColumnPrivileges{C}"]
        pub fn column_privileges<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] column_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `self`.
        ///
        /// For complete documentation on SQLColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLColumns{C}"]
        pub fn columns<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] column_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

    //    #[symbol_name = "SQLDescribeCol{C}"]
    //    pub fn describe_col<C>(
    //        &self,
    //        column_number: u16,
    //        #[unpack(_, SMALLINT)] column_name: &OdbcStr<C>,
    //        name_length_ptr: Option<&mut MaybeUninit<i16>>,
    //        data_type_ptr: *mut i16,
    //        column_size_ptr: *mut usize,
    //        decimal_digits_ptr: *mut i16,
    //        nullable_ptr: *mut i16,
    //    ) -> RETURN
    //    where
    //        use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the description of a parameter marker associated with a prepared SQL statement. This information is also available in the fields of the IPD.
        ///
        /// For complete documentation on SQLDescribeParam, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribeparam-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLDescribeParam"]
        pub fn describe_param(
            &self,
            parameter_number: u16,
            data_type: &mut MaybeUninit<SqlTypeCode>,
            parameter_size: &mut MaybeUninit<usize>,
            decimal_digits: &mut MaybeUninit<i16>,
            nullable: &mut MaybeUninit<NullAllowed>,
        ) -> RETURN;

        /// Returns the list of input and output parameters, as well as the columns that make up the result set for the specified procedures. The driver returns the information as a result set on the specified statement.
        ///
        /// For complete documentation on SQLProcedureColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedurecolumns-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLProcedureColumns{C}"]
        pub fn procedure_columns<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] proc_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] column_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the list of procedure names stored in a specific data source. `Procedure` is a generic term used to describe an `executable object`, or a named entity that can be invoked using input and output parameters. For more information on procedures, see the Procedures.
        ///
        /// For complete documentation on SQLProceduresA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedures-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLProcedures{C}"]
        pub fn procedures<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] proc_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the number of rows affected by an **UPDATE**, **INSERT**, or **DELETE** statement; an SQL_ADD, SQL_UPDATE_BY_BOOKMARK, or SQL_DELETE_BY_BOOKMARK operation in **SQLBulkOperations**; or an SQL_UPDATE or SQL_DELETE operation in **SQLSetPos**.
        ///
        /// For complete documentation on SQLRowCount, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlrowcount-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLRowCount"]
        pub fn row_count(&self, row_count_ptr: &mut MaybeUninit<isize>) -> RETURN;

        /// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
        ///
        /// For complete documentation on SQLSetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLSetCursorName{C}"]
        pub fn set_cursor_name<C>(
            &mut self,
            #[unpack(_, SMALLINT)] cursor_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Sets the cursor position in a rowset and allows an application to refresh data in the rowset or to update or delete data in the result set.
        ///
        /// For complete documentation on SQLSetPos, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetpos-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLSetPos"]
        pub fn set_pos(
            &mut self,
            row_number: SETPOSIROW,
            move operation: Operation,
            move lock_type: LockType,
        ) -> RETURN;

        /// Returns a list of tables and the privileges associated with each table. The driver returns the information as a result set on the specified statement.
        ///
        /// For complete documentation on SQLTablePrivilegesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltableprivileges-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLTablePrivileges{C}"]
        pub fn table_privileges<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the list of table, catalog, or schema names, and table types, stored in a specific data source. The driver returns the information as a result set.
        ///
        /// For complete documentation on SQLTablesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltables-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLTables{C}"]
        pub fn tables<C>(
            &self,
            #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
            #[unpack(_, SMALLINT)] column_name: &OdbcStr<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Retrieves data for one result-set column or streamed output parameter.
        // TODO: This function must be unsafe if SQL_ARD_TYPE and SQL_APD_TYPE are
        // allowed as target types.
        #[symbol_name = "SQLGetData"]
        pub fn get_data<dyn(i16) TT: CType<V>>(
            &self,
            col_or_param_num: u16,
            target_type: <dyn TT>::TAG,
            #[unpack(POINTER, LEN)]
            target_value: &mut <TT as CType<V>>::Value,
            str_len_or_ind: Option<&mut MaybeUninit<isize>>,
        ) -> RETURN;
    }

    //    #[symbol_name = "SQLSpecialColumns{C}"]
    //    pub fn special_columns<C>(
    //        &self,
    //        identifier_type: i16,
    //        #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
    //        #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
    //        #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
    //        scope: i16,
    //        nullable: i16,
    //    ) -> RETURN
    //    where
    //        use<C> @ (<CHAR> | <WCHAR>);

    //    #[symbol_name = "SQLStatistics{C}"]
    //    pub fn statistics<C>(
    //        &self,
    //        #[unpack(_, SMALLINT)] catalog_name: &OdbcStr<C>,
    //        #[unpack(_, SMALLINT)] schema_name: &OdbcStr<C>,
    //        #[unpack(_, SMALLINT)] table_name: &OdbcStr<C>,
    //        unique: u16,
    //        reserved: u16,
    //    ) -> RETURN
    //    where
    //        use<C> @ (<CHAR> | <WCHAR>);

    impl<V: OdbcVersion, DT: DescType> HDESC<'_, V, DT> {
        #[symbol_name = "SQLSetDescField{C}"]
        pub fn set_desc_field<dyn(i16) F: DescFieldSet<V, DT>, C: OdbcChar>(
            &mut self,
            rec_number: i16,
            field_identifier: <dyn F>::TAG,
            #[unpack(POINTER, AttrLen<<F as Defined>::By, i32> => INTEGER)]
            move value: <F as DescFieldSet<V, DT>>::Value<C>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        #[symbol_name = "SQLGetDescField{C}"]
        pub fn get_desc_field<dyn(i16) F: DescFieldGet<V, DT>, C: OdbcChar>(
            &self,
            rec_number: i16,
            field_identifier: <dyn F>::TAG,
            #[unpack(POINTER, AttrLen<<F as Defined>::By, i32> => INTEGER)]
            value: Option<&mut <F as DescFieldGet<V, DT>>::Buffer<C>>,
            string_length: Option<&mut MaybeUninit<i32>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

    //    #[symbol_name = "SQLGetDescRec{C}"]
    //    pub fn get_desc_rec<C>(
    //        &self,
    //        rec_number: i16,
    //        #[unpack(_, SMALLINT)] name: &OdbcStr<C>,
    //        string_length_ptr: Option<&mut MaybeUninit<i16>>,
    //        type_ptr: Option<&mut MaybeUninit<i16>>,
    //        sub_type_ptr: Option<&mut MaybeUninit<i16>>,
    //        length_ptr: Option<&mut MaybeUninit<usize>>,
    //        precision_ptr: Option<&mut MaybeUninit<i16>>,
    //        scale_ptr: Option<&mut MaybeUninit<i16>>,
    //        nullable_ptr: Option<&mut MaybeUninit<i16>>,
    //    ) -> RETURN
    //    where
    //        use<C> @ (<CHAR> | <WCHAR>);

    //    #[symbol_name = "SQLSetDescRec"]
    //    pub fn set_desc_rec(
    //        &mut self,
    //        rec_number: i16,
    //        data_type: i16,
    //        sub_type: i16,
    //        length: isize,
    //        precision: i16,
    //        scale: i16,
    //        data_ptr: POINTER,
    //        string_length_ptr: *mut isize,
    //        indicator_ptr: *mut isize,
    //    ) -> RETURN;

    //    #[symbol_name = "SQLCopyDesc"]
    //    pub fn copy_desc(&self, target_desc_handle: &mut HDESC<'_, V, DT>) -> RETURN;
    }

    impl<V: OdbcVersion, dyn(i16) H> H
    where
        use<H> @ (<HDBC<'_, V, C4>> | <HSTMT<'_, '_, '_, V>>)
    {
        /// Cancels the processing on a connection or statement. The Driver Manager maps a call to **SQLCancelHandle** to a call to **SQLCancel** when `HandleType` is SQL_HANDLE_STMT.
        ///
        /// For complete documentation on SQLCancelHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancelhandle-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLCancelHandle"]
        pub fn cancel_handle(&self) -> RETURN;

    //    #[symbol_name = "SQLCompleteAsync"]
    //    pub fn complete_async(&mut self, async_return_code: &mut MaybeUninit<i16>) -> RETURN;
    }

    impl<V: OdbcVersion, dyn(i16) H> H
    where
        use<H> @ (<HENV<V>> | <HDBC<'_, V, C4>>)
    {
        /// Requests a commit or rollback operation for all active operations on all statements associated with a connection. **SQLEndTran** can also request that a commit or rollback operation be performed for all connections associated with an environment.
        ///
        /// For complete documentation on SQLEndTran, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlendtran-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
        #[symbol_name = "SQLEndTran"]
        pub fn end_tran(&mut self, move completion_type: CompletionType) -> RETURN;
    }

    impl<'buf, V: OdbcVersion> HSTMT<'_, '_, 'buf, V> {
        /// Binds a buffer to a parameter marker in an SQL statement. **SQLBindParameter** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
        ///
        /// For complete documentation on SQLBindParameter, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindparameter-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        // TODO: Is this unsafe when `odbc_debug` is enabled?
        #[symbol_name = "SQLBindParameter"]
        pub fn bind_parameter<dyn(i16) TT: CType<V>, dyn(i16) PT: SqlType<V>>(
            &mut self,
            parameter_number: u16,
            move input_output_type: IOType,
            value_type: <dyn TT>::TAG,
            parameter_type: <dyn PT>::TAG,
            column_size: usize,
            decimal_digits: i16,
            #[unpack(POINTER, LEN)]
            parameter_value: Option<&'buf UnsafeCell<<TT as CType<V>>::Value>>,
            str_len_or_ind: Option<&'buf UnsafeCell<MaybeUninit<isize>>>,
        ) -> RETURN;

        /// Binds application data buffers to columns in the result set.
        ///
        /// For complete documentation on SQLBindCol, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLBindCol"]
        pub fn bind_col<dyn(i16) TT: CType<V>>(
            &mut self,
            column_number: u16,
            target_type: <dyn TT>::TAG,
            #[unpack(POINTER, LEN)]
            target_value: Option<&'buf UnsafeCell<<TT as CType<V>>::Value>>,
            str_len_or_ind: Option<&'buf UnsafeCell<MaybeUninit<isize>>>,
        ) -> RETURN;
    }

    impl<V: OdbcVersion, dyn(i16) H> H
    where
        use<H> @ (
            <HENV<V>>
            // FIXME:
            //<HDBC<'_, V, C2>> |
            //<HDBC<'_, V, C3>> |
            //<HDBC<'_, V, C4>> |
            //<HSTMT<'_, '_, '_, V>> |
            //<HDESC<'_, V, AppDesc<'_>>> |
            //<HDESC<'_, V, crate::desc::IRD>> |
            //<HDESC<'_, V, crate::desc::IPD>>
        )
    {
        /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
        ///
        /// For complete documentation on SQLGetDiagFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
        #[symbol_name = "SQLGetDiagField{C}"]
        pub fn get_diag_field<dyn(i16) DF: DiagField<H>, C: OdbcChar>(
            &self,
            rec_number: i16,
            diag_identifier: <dyn DF>::TAG,
            #[unpack(POINTER, SMALLINT)]
            diag_info: Option<&mut <DF as DiagField<H>>::Value<C>>,
            string_length: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);

        /// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
        ///
        /// For complete documentation on SQLGetDiagRecA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function).
        ///
        /// # Returns
        /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
        #[symbol_name = "SQLGetDiagRec{C}"]
        pub fn get_diag_rec<C: OdbcChar>(
            &self,
            rec_number: i16,
            state: &mut [C; STATE_SIZE + 1],
            native_error: &mut MaybeUninit<i32>,
            #[unpack(_, SMALLINT)]
            message_text: Option<&mut OdbcStr<MaybeUninit<C>>>,
            text_length: Option<&mut MaybeUninit<i16>>,
        ) -> RETURN
        where
            use<C> @ (<CHAR> | <WCHAR>);
    }
}

impl<V: OdbcVersion> HENV<V> {
    /// Allocates an environment handle and configures its ODBC version.
    #[expect(private_bounds)]
    pub fn alloc_handle() -> AllocHandleResult<OwnedHENV<V>>
    where
        for<'a> ODBC_VERSION: EnvAttrSet<V, Value<'a, CHAR> = VersionValue>,
    {
        let mut output_handle = MaybeUninit::uninit();
        let alloc_result = alloc_handle::<HENV<V>, V>(None, &mut output_handle);

        let alloc_with_info = match alloc_result {
            RETURN::SUCCESS => false,
            RETURN::SUCCESS_WITH_INFO => true,
            RETURN::ERROR => return AllocHandleResult::Error,
            result => panic!("SQLAllocHandle returned invalid return code: {result:?}"),
        };

        let mut output_handle = unsafe { output_handle.assume_init() };
        let version_result = output_handle.set_attr::<ODBC_VERSION>(VersionValue(V::ID));

        match version_result {
            RETURN::SUCCESS if alloc_with_info => AllocHandleResult::SuccessWithInfo(output_handle),
            RETURN::SUCCESS => AllocHandleResult::Success(output_handle),
            RETURN::SUCCESS_WITH_INFO => AllocHandleResult::SuccessWithInfo(output_handle),
            RETURN::ERROR => {
                drop(output_handle);
                AllocHandleResult::Error
            }
            RETURN::INVALID_HANDLE => panic! {
                "SQLSetEnvAttr rejected a newly allocated environment"
            },
            result => panic!("SQLSetEnvAttr returned invalid return code: {result:?}"),
        }
    }
}

impl<'env, V: OdbcVersion> HDBC<'env, V, C2> {
    /// Allocates a connection handle on `env`.
    pub fn alloc_handle(env: &'env HENV<V>) -> AllocHandleResult<OwnedHDBC<'env, V, C2>> {
        let mut output_handle = MaybeUninit::uninit();
        let result = alloc_handle::<HDBC<'env, V, C2>, V>(Some(env), &mut output_handle);

        alloc_handle_result(result, output_handle)
    }
}

impl<'conn, 'desc, 'buf, V: OdbcVersion> HSTMT<'conn, 'desc, 'buf, V> {
    /// Allocates a statement handle on `conn`.
    pub fn alloc_handle<'env: 'conn>(
        conn: &'conn HDBC<'env, V, C4>,
    ) -> AllocHandleResult<OwnedHSTMT<'conn, 'desc, 'buf, V>> {
        let mut output_handle = MaybeUninit::uninit();
        let result =
            alloc_handle::<HSTMT<'conn, 'desc, 'buf, V>, V>(Some(conn), &mut output_handle);

        alloc_handle_result(result, output_handle)
    }
}

impl<'conn, 'buf, V: OdbcVersion> HDESC<'conn, V, AppDesc<'buf>> {
    /// Allocates an application descriptor handle on `conn`.
    pub fn alloc_handle<'env: 'conn>(
        conn: &'conn HDBC<'env, V, C4>,
    ) -> AllocHandleResult<OwnedHDESC<'conn, V, AppDesc<'buf>>> {
        let mut output_handle = MaybeUninit::uninit();
        let result =
            alloc_handle::<HDESC<'conn, V, AppDesc<'buf>>, V>(Some(conn), &mut output_handle);

        alloc_handle_result(result, output_handle)
    }
}

fn alloc_handle_result<H>(result: RETURN, output_handle: MaybeUninit<H>) -> AllocHandleResult<H> {
    match result {
        RETURN::SUCCESS => AllocHandleResult::Success(unsafe { output_handle.assume_init() }),
        RETURN::SUCCESS_WITH_INFO => {
            AllocHandleResult::SuccessWithInfo(unsafe { output_handle.assume_init() })
        }
        RETURN::ERROR => AllocHandleResult::Error,
        RETURN::INVALID_HANDLE => panic!("SQLAllocHandle rejected a typed source handle"),
        result => panic!("SQLAllocHandle returned invalid return code: {result:?}"),
    }
}

unsafe fn change_owned_type<T, U>(handle: T) -> U {
    assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<U>());
    let handle = ManuallyDrop::new(handle);
    unsafe { core::mem::transmute_copy(&handle) }
}

unsafe impl<V: OdbcVersion> Send for OwnedHENV<V> {}
unsafe impl<V: OdbcVersion> Sync for OwnedHENV<V> {}

unsafe impl<V: OdbcVersion, C: ConnState> Send for OwnedHDBC<'_, V, C> {}

impl<'env, V: OdbcVersion, OC: ConnState> OwnedHDBC<'env, V, OC> {
    pub(crate) fn into_c2(self) -> OwnedHDBC<'env, V, C2> {
        unsafe { change_owned_type(self) }
    }
    pub(crate) fn need_data(self) -> OwnedHDBC<'env, V, C3> {
        unsafe { change_owned_type(self) }
    }
    pub(crate) fn into_c4(self) -> OwnedHDBC<'env, V, C4> {
        unsafe { change_owned_type(self) }
    }
}

impl<'env, V: OdbcVersion> OwnedHDBC<'env, V, C2> {
    fn connect_outcome(self, result: RETURN) -> ConnectResult<'env, V> {
        match result {
            RETURN::SUCCESS_WITH_INFO => ConnectResult::SuccessWithInfo(self.into_c4()),
            RETURN::SUCCESS => ConnectResult::Success(self.into_c4()),
            RETURN::ERROR => ConnectResult::Error(self),
            RETURN::STILL_EXECUTING => ConnectResult::StillExecuting(self),
            RETURN::INVALID_HANDLE => panic!("SQLConnect rejected a typed handle"),
            result => panic!("SQLConnect returned unexpected status {result:?}"),
        }
    }

    fn driver_connect_outcome(self, result: RETURN) -> DriverConnectResult<'env, V> {
        match result {
            RETURN::SUCCESS_WITH_INFO => DriverConnectResult::SuccessWithInfo(self.into_c4()),
            RETURN::SUCCESS => DriverConnectResult::Success(self.into_c4()),
            RETURN::ERROR => DriverConnectResult::Error(self),
            RETURN::STILL_EXECUTING => DriverConnectResult::StillExecuting(self),
            RETURN::NO_DATA => DriverConnectResult::NoData(self),
            RETURN::INVALID_HANDLE => panic!("SQLDriverConnect rejected a typed handle"),
            result => panic!("SQLDriverConnect returned unexpected status {result:?}"),
        }
    }

    /// Establishes a connection to a driver and data source, advancing this
    /// connection from C2 to C4 on success.
    ///
    /// The handle retains connection status, transaction state, and diagnostic
    /// information. See the ODBC [`SQLConnect` API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
    ///
    /// The returned [`ConnectResult`] represents `SQL_SUCCESS`,
    /// `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_STILL_EXECUTING` while
    /// preserving the handle in the state implied by that return code.
    #[expect(private_bounds)]
    pub fn connect<'server, 'user, 'auth, C>(
        mut self,
        server_name: &'server OdbcStr<C>,
        user_name: &'user OdbcStr<C>,
        authentication: &'auth OdbcStr<C>,
    ) -> DriverConnectResult<'env, V>
    where
        for<'handle> (): hdbc_connect::DispatchSet<'handle, 'server, 'user, 'auth, V, C>,
    {
        let result = HDBC::connect(&mut self, server_name, user_name, authentication);
        self.driver_connect_outcome(result)
    }

    /// Connects using a connection string, advancing this connection from C2 to
    /// C4 on success.
    ///
    /// Unlike [`Self::connect`], this supports data sources requiring additional
    /// connection information, optional prompting through a window handle, and
    /// data sources not defined in system information. See the ODBC
    /// [`SQLDriverConnect` API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
    ///
    /// The returned [`DriverConnectResult`] represents `SQL_SUCCESS`,
    /// `SQL_SUCCESS_WITH_INFO`, `SQL_NO_DATA`, `SQL_ERROR`, or
    /// `SQL_STILL_EXECUTING` while preserving the typed connection state.
    #[expect(private_bounds)]
    pub fn driver_connect<'input, 'output, 'length, C>(
        mut self,
        window_handle: Option<POINTER>,
        in_connection_string: &'input OdbcStr<C>,
        out_connection_string: Option<&'output mut OdbcStr<MaybeUninit<C>>>,
        string_length2: Option<&'length mut MaybeUninit<i16>>,
        driver_completion: DriverCompletion,
    ) -> ConnectResult<'env, V>
    where
        for<'handle> (): hdbc_driver_connect::DispatchSet<'handle, 'input, 'output, 'length, V, C>,
    {
        let result = HDBC::driver_connect(
            &mut self,
            window_handle.unwrap_or(POINTER::NULL),
            in_connection_string,
            out_connection_string,
            string_length2,
            driver_completion,
        );
        self.connect_outcome(result)
    }
}

impl BrowseConnectTransition for C2 {
    fn transition<'env, V: OdbcVersion>(
        handle: OwnedHDBC<'env, V, Self>,
        result: RETURN,
    ) -> BrowseConnectResult<'env, V, Self> {
        match result {
            RETURN::SUCCESS_WITH_INFO => BrowseConnectResult::SuccessWithInfo(handle.into_c4()),
            RETURN::SUCCESS => BrowseConnectResult::Success(handle.into_c4()),
            RETURN::NEED_DATA => BrowseConnectResult::NeedData(handle.need_data()),
            RETURN::ERROR => BrowseConnectResult::Error(handle),
            RETURN::STILL_EXECUTING => BrowseConnectResult::StillExecuting(handle),
            RETURN::INVALID_HANDLE => panic!("SQLBrowseConnect rejected a typed handle"),
            result => panic!("SQLBrowseConnect returned unexpected status {result:?}"),
        }
    }
}

impl BrowseConnectTransition for C3 {
    fn transition<'env, V: OdbcVersion>(
        handle: OwnedHDBC<'env, V, Self>,
        result: RETURN,
    ) -> BrowseConnectResult<'env, V, Self> {
        match result {
            RETURN::SUCCESS_WITH_INFO => BrowseConnectResult::SuccessWithInfo(handle.into_c4()),
            RETURN::SUCCESS => BrowseConnectResult::Success(handle.into_c4()),
            RETURN::NEED_DATA => BrowseConnectResult::NeedData(handle),
            RETURN::ERROR => BrowseConnectResult::Error(handle.into_c2()),
            RETURN::STILL_EXECUTING => BrowseConnectResult::StillExecuting(handle),
            RETURN::INVALID_HANDLE => panic!("SQLBrowseConnect rejected a typed handle"),
            result => panic!("SQLBrowseConnect returned unexpected status {result:?}"),
        }
    }
}

#[expect(private_bounds)]
impl<'env, V: OdbcVersion, S: BrowseConnectTransition> OwnedHDBC<'env, V, S> {
    /// Iteratively discovers the attributes required to connect to a data source.
    ///
    /// Success completes the connection, while `SQL_NEED_DATA` returns a C3
    /// handle for the next iteration. See the ODBC
    /// [`SQLBrowseConnect` API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
    ///
    /// The returned [`BrowseConnectResult`] represents `SQL_SUCCESS`,
    /// `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_ERROR`, or
    /// `SQL_STILL_EXECUTING` while preserving the typed connection state.
    pub fn browse_connect<'input, 'output, 'length, C>(
        mut self,
        in_connection_string: &'input OdbcStr<C>,
        out_connection_string: Option<&'output mut OdbcStr<MaybeUninit<C>>>,
        string_length2: Option<&'length mut MaybeUninit<i16>>,
    ) -> BrowseConnectResult<'env, V, S>
    where
        for<'handle> ():
            hdbc_browse_connect::DispatchSet<'handle, 'input, 'output, 'length, V, S, C>,
    {
        let result = HDBC::browse_connect(
            &mut self,
            in_connection_string,
            out_connection_string,
            string_length2,
        );
        S::transition(self, result)
    }
}

impl<'env, V: OdbcVersion, S: Disconnect> OwnedHDBC<'env, V, S> {
    /// Closes the connection associated with this handle.
    ///
    /// See the ODBC [`SQLDisconnect` API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldisconnect-function).
    ///
    /// The returned [`DisconnectResult`] represents `SQL_SUCCESS`,
    /// `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_STILL_EXECUTING` while
    /// preserving the handle in the state implied by that return code.
    pub fn disconnect(mut self) -> DisconnectResult<'env, V, S> {
        let result = HDBC::disconnect(&mut self);
        match result {
            RETURN::SUCCESS_WITH_INFO => DisconnectResult::SuccessWithInfo(self.into_c2()),
            RETURN::SUCCESS => DisconnectResult::Success(self.into_c2()),
            RETURN::ERROR => DisconnectResult::Error(self),
            RETURN::STILL_EXECUTING => DisconnectResult::StillExecuting(self),
            RETURN::INVALID_HANDLE => panic!("SQLDisconnect rejected a typed handle"),
            result => panic!("SQLDisconnect returned unexpected status {result:?}"),
        }
    }
}
