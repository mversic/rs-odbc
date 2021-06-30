use crate::extern_api;
use crate::handle::*;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ptr;

use crate::{
    c_types::CData,
    c_types::DeferredBuf,
    col::ColAttr,
    conn::ConnAttr,
    desc::{DescField, WriteDescField},
    diag::{DiagField, SQLSTATE},
    env::{EnvAttr, OdbcVersion},
    handle::{ConnState, C2, C4, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT},
    info::InfoType,
    sql_types::SqlType,
    sqlreturn::{SQLRETURN, SQL_SUCCEEDED, SQL_SUCCESS},
    stmt::StmtAttr,
    AnsiType, AsMutPtr, AsMutRawSlice, AsMutSQLPOINTER, AsRawSlice, AsSQLPOINTER, AttrGet, AttrSet,
    BulkOperation, CompletionType, DatetimeIntervalCode, DriverCompletion, FreeStmtOption,
    FunctionId, IOType, Ident, IdentifierType, IntoSQLPOINTER, LockType, NullAllowed, Operation,
    Reserved, Scope, StrLenOrInd, UnicodeType, Unique, RETCODE, SQLCHAR, SQLINTEGER, SQLLEN,
    SQLPOINTER, SQLSETPOSIROW, SQLSMALLINT, SQLULEN, SQLUSMALLINT, SQLWCHAR,
};

/// Allocates an environment, connection, statement, or descriptor handle.
///
/// For complete documentation on SQLAllocHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlallochandle-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_INVALID_HANDLE, or SQL_ERROR.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLAllocHandle<'src, OH: Allocate<'src>>(
    HandleType: OH::Ident,
    InputHandle: &'src OH::SrcHandle,
    OutputHandlePtr: &mut MaybeUninit<OH>,
) -> SQLRETURN
where
    OH::SrcHandle: AsSQLHANDLE,
{
    let mut output_handle: SQLHANDLE = ptr::null_mut();

    unsafe {
        let sql_return = extern_api::SQLAllocHandle(
            OH::Ident::IDENTIFIER,
            InputHandle.as_SQLHANDLE(),
            &mut output_handle,
        );

        if SQL_SUCCEEDED(sql_return) {
            OutputHandlePtr
                .as_mut_ptr()
                .write(OH::from_raw(output_handle));
        }

        sql_return
    }
}

/// Binds application data buffers to columns in the result set.
///
/// For complete documentation on SQLBindCol, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLBindCol<
    'buf,
    TT: Ident<Type = SQLSMALLINT>,
    B: DeferredBuf<'buf, TT, V>,
    V: OdbcVersion,
>(
    StatementHandle: &SQLHSTMT<'_, '_, 'buf, V>,
    ColumnNumber: SQLUSMALLINT,
    TargetType: TT,
    TargetValuePtr: Option<B>,
    StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
) -> SQLRETURN {
    let sql_return = unsafe {
        let TargetValuePtr = TargetValuePtr.map_or((ptr::null_mut(), 0), |TargetValuePtr| {
            (TargetValuePtr.into_SQLPOINTER(), TargetValuePtr.len())
        });

        extern_api::SQLBindCol(
            StatementHandle.as_SQLHANDLE(),
            ColumnNumber,
            TT::IDENTIFIER,
            TargetValuePtr.0,
            TargetValuePtr.1,
            StrLen_or_IndPtr.map_or_else(ptr::null_mut, |StrLen_or_IndPtr| {
                StrLen_or_IndPtr.get().cast()
            }),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        StatementHandle.bind_col(TargetValuePtr);
        StatementHandle.bind_strlen_or_ind(StrLen_or_IndPtr);
    }

    sql_return
}

/// Binds a buffer to a parameter marker in an SQL statement. **SQLBindParameter** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
///
/// For complete documentation on SQLBindParameter, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindparameter-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLBindParameter<
    'buf,
    TT: Ident<Type = SQLSMALLINT>,
    // TODO: Check which type is used for ParameterType
    ST: SqlType<V>,
    B: DeferredBuf<'buf, TT, V>,
    V: OdbcVersion,
>(
    StatementHandle: &SQLHSTMT<'_, '_, 'buf, V>,
    ParameterNumber: SQLUSMALLINT,
    InputOutputType: IOType,
    ValueType: TT,
    ParameterType: ST,
    ColumnSize: SQLULEN,
    DecimalDigits: SQLSMALLINT,
    ParameterValuePtr: Option<B>,
    StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
) -> SQLRETURN {
    let sql_return = unsafe {
        let ParameterValuePtr = ParameterValuePtr
            .map_or((ptr::null_mut(), 0), |ParameterValuePtr| {
                (ParameterValuePtr.into_SQLPOINTER(), ParameterValuePtr.len())
            });

        extern_api::SQLBindParameter(
            StatementHandle.as_SQLHANDLE(),
            ParameterNumber,
            InputOutputType.identifier(),
            TT::IDENTIFIER,
            ParameterType.identifier(),
            ColumnSize,
            DecimalDigits,
            ParameterValuePtr.0,
            ParameterValuePtr.1,
            StrLen_or_IndPtr.map_or_else(ptr::null_mut, |StrLen_or_IndPtr| {
                StrLen_or_IndPtr.get().cast()
            }),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        StatementHandle.bind_param(ParameterValuePtr);
        StatementHandle.bind_strlen_or_ind(StrLen_or_IndPtr);
    }

    sql_return
}
/// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
///
/// For complete documentation on SQLBrowseConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLBrowseConnectA<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    InConnectionString: &[SQLCHAR],
    OutConnectionString: Option<&mut [MaybeUninit<SQLCHAR>]>,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString =
        OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    let sql_return = unsafe {
        extern_api::SQLBrowseConnectA(
            ConnectionHandle.as_SQLHANDLE(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
///
/// For complete documentation on SQLBrowseConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLBrowseConnectW<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    InConnectionString: &[SQLWCHAR],
    OutConnectionString: Option<&mut [MaybeUninit<SQLWCHAR>]>,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString =
        OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    let sql_return = unsafe {
        extern_api::SQLBrowseConnectW(
            ConnectionHandle.as_SQLHANDLE(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
///
/// For complete documentation on SQLBulkOperations, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbulkoperations-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLBulkOperations<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    Operation: BulkOperation,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLBulkOperations(StatementHandle.as_SQLHANDLE(), Operation as SQLUSMALLINT)
    }
}

/// Cancels the processing on a statement.
/// To cancel processing on a connection or statement, use SQLCancelHandle Function.
///
/// For complete documentation on SQLCancel, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancel-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLCancel<V: OdbcVersion>(StatementHandle: &SQLHSTMT<V>) -> SQLRETURN {
    unsafe { extern_api::SQLCancel(StatementHandle.as_SQLHANDLE()) }
}

/// Cancels the processing on a connection or statement. The Driver Manager maps a call to **SQLCancelHandle** to a call to **SQLCancel** when `HandleType` is SQL_HANDLE_STMT.
///
/// For complete documentation on SQLCancelHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancelhandle-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLCancelHandle<H: Handle>(HandleType: H::Ident, Handle: &H) -> SQLRETURN
where
    H: AsSQLHANDLE + SQLCancelHandle,
{
    unsafe { extern_api::SQLCancelHandle(H::Ident::IDENTIFIER, Handle.as_SQLHANDLE()) }
}

/// Closes a cursor that has been opened on a statement and discards pending results.
///
/// For complete documentation on SQLCloseCursor, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlclosecursor-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLCloseCursor<V: OdbcVersion>(StatementHandle: &SQLHSTMT<V>) -> SQLRETURN {
    unsafe { extern_api::SQLCloseCursor(StatementHandle.as_SQLHANDLE()) }
}

/// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
///
/// For complete documentation on SQLColAttributeA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLColAttributeA<A: Ident<Type = SQLUSMALLINT>, T: ColAttr<A, V>, V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ColumnNumber: SQLUSMALLINT,
    FieldIdentifier: A,
    CharacterAttributePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    NumericAttributePtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN
where
    T: AttrGet<A> + AnsiType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    // TODO: With MaybeUnint it's not possible to check that value is zeroed
    //if cfg!(feature = "odbc_debug") {
    //    NumericAttributePtr.assert_zeroed();
    //}

    let CharacterAttributePtr =
        CharacterAttributePtr.map_or((ptr::null_mut(), 0), |CharacterAttributePtr| {
            (
                CharacterAttributePtr.as_mut_SQLPOINTER(),
                CharacterAttributePtr.len(),
            )
        });

    unsafe {
        extern_api::SQLColAttributeA(
            StatementHandle.as_SQLHANDLE(),
            ColumnNumber,
            A::IDENTIFIER,
            CharacterAttributePtr.0,
            CharacterAttributePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
            NumericAttributePtr.as_mut_ptr(),
        )
    }
}

/// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
///
/// For complete documentation on SQLColAttributeW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLColAttributeW<A: Ident<Type = SQLUSMALLINT>, T: ColAttr<A, V>, V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ColumnNumber: SQLUSMALLINT,
    FieldIdentifier: A,
    CharacterAttributePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    NumericAttributePtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN
where
    T: AttrGet<A> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    // TODO: With MaybeUnint it's not possible to check that value is zeroed
    //if cfg!(feature = "odbc_debug") {
    //    NumericAttributePtr.assert_zeroed();
    //}

    let CharacterAttributePtr =
        CharacterAttributePtr.map_or((ptr::null_mut(), 0), |CharacterAttributePtr| {
            (
                CharacterAttributePtr.as_mut_SQLPOINTER(),
                CharacterAttributePtr.len(),
            )
        });

    unsafe {
        extern_api::SQLColAttributeW(
            StatementHandle.as_SQLHANDLE(),
            ColumnNumber,
            A::IDENTIFIER,
            CharacterAttributePtr.0,
            CharacterAttributePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
            NumericAttributePtr.as_mut_ptr(),
        )
    }
}

/// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `StatementHandle`.
///
/// For complete documentation on SQLColumnPrivilegesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLColumnPrivilegesA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
    ColumnName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnPrivilegesA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `StatementHandle`.
///
/// For complete documentation on SQLColumnPrivilegesW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLColumnPrivilegesW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
    ColumnName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnPrivilegesW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `StatementHandle`.
///
/// For complete documentation on SQLColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLColumnsA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
    ColumnName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnsA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `StatementHandle`.
///
/// For complete documentation on SQLColumnsW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLColumnsW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
    ColumnName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnsW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Can be used to determine when an asynchronous function is complete using either notification- or polling-based processing. For more information about asynchronous operations, see Asynchronous Execution.
/// **SQLCompleteAsync** is only implemented in the ODBC Driver Manager.
/// In notification based asynchronous processing mode, **SQLCompleteAsync** must be called after the Driver Manager raises the event object used for notification. **SQLCompleteAsync** completes the asynchronous processing and the asynchronous function will generate a return code.
/// In polling based asynchronous processing mode, **SQLCompleteAsync** is an alternative to calling the original asynchronous function, without needing to specify the arguments in the original asynchronous function call. **SQLCompleteAsync** can be used regardless whether the ODBC Cursor Library is enabled.
///
/// For complete documentation on SQLCompleteAsync, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcompleteasync-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLCompleteAsync<H: Handle>(
    HandleType: H::Ident,
    // TODO: Should this handle be mutable or not?
    Handle: &mut H,
    AsyncRetCodePtr: &mut MaybeUninit<RETCODE>,
) -> SQLRETURN
where
    H: AsSQLHANDLE + SQLCompleteAsyncHandle,
{
    unsafe {
        extern_api::SQLCompleteAsync(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            AsyncRetCodePtr.as_mut_ptr(),
        )
    }
}

/// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
///
/// For complete documentation on SQLConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLConnectA<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    ServerName: &[SQLCHAR],
    UserName: &[SQLCHAR],
    Authentication: &[SQLCHAR],
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let ServerName = ServerName.as_raw_slice();
    let UserName = UserName.as_raw_slice();
    let Authentication = Authentication.as_raw_slice();

    let sql_return = unsafe {
        extern_api::SQLConnectA(
            ConnectionHandle.as_SQLHANDLE(),
            ServerName.0,
            ServerName.1,
            UserName.0,
            UserName.1,
            Authentication.0,
            Authentication.1,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
///
/// For complete documentation on SQLConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLConnectW<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    ServerName: &[SQLWCHAR],
    UserName: &[SQLWCHAR],
    Authentication: &[SQLWCHAR],
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let ServerName = ServerName.as_raw_slice();
    let UserName = UserName.as_raw_slice();
    let Authentication = Authentication.as_raw_slice();

    let sql_return = unsafe {
        extern_api::SQLConnectW(
            ConnectionHandle.as_SQLHANDLE(),
            ServerName.0,
            ServerName.1,
            UserName.0,
            UserName.1,
            Authentication.0,
            Authentication.1,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// Copies descriptor information from one descriptor handle to another.
///
/// For complete documentation on SQLCopyDesc, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcopydesc-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
// TODO: Not sure if application and implementation descriptors can be interchangeably copied
// TODO: Do they have to have the same version?
pub fn SQLCopyDesc<DT1, DT2, V: OdbcVersion>(
    SourceDescHandle: &SQLHDESC<DT1, V>,
    TargetDescHandle: &SQLHDESC<DT2, V>,
) -> SQLRETURN
where
    DT1: for<'buf> DescType<'buf>,
    DT2: for<'buf> DescType<'buf>,
{
    unsafe {
        extern_api::SQLCopyDesc(
            SourceDescHandle.as_SQLHANDLE(),
            TargetDescHandle.as_SQLHANDLE(),
        )
    }
}

/// Returns information about a data source. This function is implemented only by the Driver Manager.
///
/// For complete documentation on SQLDataSourcesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDataSourcesA<V: OdbcVersion>(
    EnvironmentHandle: &SQLHENV<V>,
    Direction: SQLUSMALLINT,
    ServerName: &mut [MaybeUninit<SQLCHAR>],
    NameLength1Ptr: &mut MaybeUninit<SQLSMALLINT>,
    Description: &mut [MaybeUninit<SQLCHAR>],
    NameLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ServerName = ServerName.as_mut_raw_slice();
    let Description = Description.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDataSourcesA(
            EnvironmentHandle.as_SQLHANDLE(),
            Direction,
            ServerName.0,
            ServerName.1,
            NameLength1Ptr.as_mut_ptr(),
            Description.0,
            Description.1,
            NameLength2Ptr.as_mut_ptr(),
        )
    }
}

/// Returns information about a data source. This function is implemented only by the Driver Manager.
///
/// For complete documentation on SQLDataSourcesW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDataSourcesW<V: OdbcVersion>(
    EnvironmentHandle: &SQLHENV<V>,
    Direction: SQLUSMALLINT,
    ServerName: &mut [MaybeUninit<SQLWCHAR>],
    NameLength1Ptr: &mut MaybeUninit<SQLSMALLINT>,
    Description: &mut [MaybeUninit<SQLWCHAR>],
    NameLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ServerName = ServerName.as_mut_raw_slice();
    let Description = Description.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDataSourcesW(
            EnvironmentHandle.as_SQLHANDLE(),
            Direction,
            ServerName.0,
            ServerName.1,
            NameLength1Ptr.as_mut_ptr(),
            Description.0,
            Description.1,
            NameLength2Ptr.as_mut_ptr(),
        )
    }
}

/// Returns the result descriptor - column name,type, column size, decimal digits, and nullability - for one column in the result set. This information also is available in the fields of the IRD.
///
/// For complete documentation on SQLDescribeColA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribecol-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDescribeColA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ColumnNumber: SQLUSMALLINT,
    ColumnName: &mut [MaybeUninit<SQLCHAR>],
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ColumnSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<NullAllowed>,
) -> SQLRETURN {
    let ColumnName = ColumnName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDescribeColA(
            StatementHandle.as_SQLHANDLE(),
            ColumnNumber,
            ColumnName.0,
            ColumnName.1,
            NameLengthPtr.as_mut_ptr(),
            DataTypePtr.as_mut_ptr(),
            ColumnSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr().cast(),
        )
    }
}

/// Returns the result descriptor - column name,type, column size, decimal digits, and nullability - for one column in the result set. This information also is available in the fields of the IRD.
///
/// For complete documentation on SQLDescribeColW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribecol-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDescribeColW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ColumnNumber: SQLUSMALLINT,
    ColumnName: &mut [MaybeUninit<SQLWCHAR>],
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ColumnSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<NullAllowed>,
) -> SQLRETURN {
    let ColumnName = ColumnName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDescribeColW(
            StatementHandle.as_SQLHANDLE(),
            ColumnNumber,
            ColumnName.0,
            ColumnName.1,
            NameLengthPtr.as_mut_ptr(),
            DataTypePtr.as_mut_ptr(),
            ColumnSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr().cast(),
        )
    }
}

/// Returns the description of a parameter marker associated with a prepared SQL statement. This information is also available in the fields of the IPD.
///
/// For complete documentation on SQLDescribeParam, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribeparam-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDescribeParam<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ParameterNumber: SQLUSMALLINT,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ParameterSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<NullAllowed>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLDescribeParam(
            StatementHandle.as_SQLHANDLE(),
            ParameterNumber,
            DataTypePtr.as_mut_ptr(),
            ParameterSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr().cast(),
        )
    }
}

/// Closes the connection associated with a specific connection handle.
///
/// For complete documentation on SQLDisconnect, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldisconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDisconnect<V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<C4, V>,
) -> (Result<SQLHDBC<C2, V>, SQLHDBC<C4, V>>, SQLRETURN) {
    let sql_return = unsafe { extern_api::SQLDisconnect(ConnectionHandle.as_SQLHANDLE()) };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.disconnect()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
///
/// For complete documentation on SQLDriverConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDriverConnectA<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    WindowHandle: Option<SQLHWND>,
    InConnectionString: &[SQLCHAR],
    OutConnectionString: Option<&mut [MaybeUninit<SQLCHAR>]>,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString =
        OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    let sql_return = unsafe {
        extern_api::SQLDriverConnectA(
            ConnectionHandle.as_SQLHANDLE(),
            // TODO: Fix this
            ptr::null_mut(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
            DriverCompletion as SQLUSMALLINT,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
///
/// For complete documentation on SQLDriverConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDriverConnectW<'env, V: OdbcVersion>(
    ConnectionHandle: SQLHDBC<'env, C2, V>,
    WindowHandle: Option<SQLHWND>,
    InConnectionString: &[SQLWCHAR],
    OutConnectionString: Option<&mut [MaybeUninit<SQLWCHAR>]>,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> (
    Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
    SQLRETURN,
) {
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString =
        OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    let sql_return = unsafe {
        extern_api::SQLDriverConnectW(
            ConnectionHandle.as_SQLHANDLE(),
            // TODO: Fix this
            ptr::null_mut(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
            DriverCompletion as SQLUSMALLINT,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        (Ok(ConnectionHandle.connected()), sql_return)
    } else {
        (Err(ConnectionHandle), sql_return)
    }
}

/// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
///
/// For complete documentation on SQLDriversA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDriversA<V: OdbcVersion>(
    EnvironmentHandle: &SQLHENV<V>,
    Direction: SQLUSMALLINT,
    DriverDescription: &mut [MaybeUninit<SQLCHAR>],
    DescriptionLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DriverAttributes: &mut [MaybeUninit<SQLCHAR>],
    AttributesLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let DriverDescription = DriverDescription.as_mut_raw_slice();
    let DriverAttributes = DriverAttributes.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriversA(
            EnvironmentHandle.as_SQLHANDLE(),
            Direction,
            DriverDescription.0,
            DriverDescription.1,
            DescriptionLengthPtr.as_mut_ptr(),
            DriverAttributes.0,
            DriverAttributes.1,
            AttributesLengthPtr.as_mut_ptr(),
        )
    }
}

/// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
///
/// For complete documentation on SQLDriversW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLDriversW<V: OdbcVersion>(
    EnvironmentHandle: &SQLHENV<V>,
    Direction: SQLUSMALLINT,
    DriverDescription: &mut [MaybeUninit<SQLWCHAR>],
    DescriptionLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DriverAttributes: &mut [MaybeUninit<SQLWCHAR>],
    AttributesLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let DriverDescription = DriverDescription.as_mut_raw_slice();
    let DriverAttributes = DriverAttributes.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriversW(
            EnvironmentHandle.as_SQLHANDLE(),
            Direction,
            DriverDescription.0,
            DriverDescription.1,
            DescriptionLengthPtr.as_mut_ptr(),
            DriverAttributes.0,
            DriverAttributes.1,
            AttributesLengthPtr.as_mut_ptr(),
        )
    }
}

/// Requests a commit or rollback operation for all active operations on all statements associated with a connection. **SQLEndTran** can also request that a commit or rollback operation be performed for all connections associated with an environment.
///
/// For complete documentation on SQLEndTran, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlendtran-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLEndTran<H: Handle>(
    HandleType: H::Ident,
    Handle: &H,
    CompletionType: CompletionType,
) -> SQLRETURN
where
    H: AsSQLHANDLE + SQLEndTranHandle,
{
    unsafe {
        extern_api::SQLEndTran(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            CompletionType as SQLSMALLINT,
        )
    }
}

/// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
///
/// For complete documentation on SQLExecDirectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLExecDirectA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    StatementText: &[SQLCHAR],
) -> SQLRETURN {
    let StatementText = StatementText.as_raw_slice();

    extern_api::SQLExecDirectA(
        StatementHandle.as_SQLHANDLE(),
        StatementText.0,
        StatementText.1,
    )
}

/// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
///
/// For complete documentation on SQLExecDirectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLExecDirectW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    StatementText: &[SQLWCHAR],
) -> SQLRETURN {
    let StatementText = StatementText.as_raw_slice();

    extern_api::SQLExecDirectW(
        StatementHandle.as_SQLHANDLE(),
        StatementText.0,
        StatementText.1,
    )
}

/// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
///
/// For complete documentation on SQLExecute, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLExecute<V: OdbcVersion>(StatementHandle: &SQLHSTMT<V>) -> SQLRETURN {
    extern_api::SQLExecute(StatementHandle.as_SQLHANDLE())
}

/// Fetches the next rowset of data from the result set and returns data for all bound columns.
///
/// For complete documentation on SQLFetch, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLFetch<V: OdbcVersion>(StatementHandle: &SQLHSTMT<V>) -> SQLRETURN {
    extern_api::SQLFetch(StatementHandle.as_SQLHANDLE())
}

/// Fetches the specified rowset of data from the result set and returns data for all bound columns. Rowsets can be specified at an absolute or relative position or by bookmark.
/// When working with an ODBC 2.x driver, the Driver Manager maps this function to **SQLExtendedFetch**. For more information, see Mapping Replacement Functions for Backward Compatibility of Applications.
///
/// For complete documentation on SQLFetchScroll, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetchscroll-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLFetchScroll<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    FetchOrientation: SQLSMALLINT,
    FetchOffset: SQLLEN,
) -> SQLRETURN {
    extern_api::SQLFetchScroll(
        StatementHandle.as_SQLHANDLE(),
        FetchOrientation,
        FetchOffset,
    )
}

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
#[inline]
#[allow(non_snake_case)]
pub fn SQLForeignKeysA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    PKCatalogName: &[SQLCHAR],
    PKSchemaName: &[SQLCHAR],
    PKTableName: &[SQLCHAR],
    FKCatalogName: &[SQLCHAR],
    FKSchemaName: &[SQLCHAR],
    FKTableName: &[SQLCHAR],
) -> SQLRETURN {
    let PKCatalogName = PKCatalogName.as_raw_slice();
    let PKSchemaName = PKSchemaName.as_raw_slice();
    let PKTableName = PKTableName.as_raw_slice();
    let FKCatalogName = FKCatalogName.as_raw_slice();
    let FKSchemaName = FKSchemaName.as_raw_slice();
    let FKTableName = FKTableName.as_raw_slice();

    unsafe {
        extern_api::SQLForeignKeysA(
            StatementHandle.as_SQLHANDLE(),
            PKCatalogName.0,
            PKCatalogName.1,
            PKSchemaName.0,
            PKSchemaName.1,
            PKTableName.0,
            PKTableName.1,
            FKCatalogName.0,
            FKCatalogName.1,
            FKSchemaName.0,
            FKSchemaName.1,
            FKTableName.0,
            FKTableName.1,
        )
    }
}

/// Can return:
///
/// * A list of foreign keys in the specified table (columns in the specified table that refer to primary keys in other tables).
/// * A list of foreign keys in other tables that refer to the primary key in the specified table.
///
/// The driver returns each list as a result set on the specified statement.
///
/// For complete documentation on SQLForeignKeysW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlforeignkeys-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLForeignKeysW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    PKCatalogName: &[SQLWCHAR],
    PKSchemaName: &[SQLWCHAR],
    PKTableName: &[SQLWCHAR],
    FKCatalogName: &[SQLWCHAR],
    FKSchemaName: &[SQLWCHAR],
    FKTableName: &[SQLWCHAR],
) -> SQLRETURN {
    let PKCatalogName = PKCatalogName.as_raw_slice();
    let PKSchemaName = PKSchemaName.as_raw_slice();
    let PKTableName = PKTableName.as_raw_slice();
    let FKCatalogName = FKCatalogName.as_raw_slice();
    let FKSchemaName = FKSchemaName.as_raw_slice();
    let FKTableName = FKTableName.as_raw_slice();

    unsafe {
        extern_api::SQLForeignKeysW(
            StatementHandle.as_SQLHANDLE(),
            PKCatalogName.0,
            PKCatalogName.1,
            PKSchemaName.0,
            PKSchemaName.1,
            PKTableName.0,
            PKTableName.1,
            FKCatalogName.0,
            FKCatalogName.1,
            FKSchemaName.0,
            FKSchemaName.1,
            FKTableName.0,
            FKTableName.1,
        )
    }
}

/// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
///
/// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLFreeHandle<H: Handle>(HandleType: H::Ident, Handle: H) -> SQLRETURN {
    SQL_SUCCESS
}

/// Stops processing associated with a specific statement, closes any open cursors associated with the statement, discards pending results, or, optionally, frees all resources associated with the statement handle.
///
/// For complete documentation on SQLFreeStmt, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreestmt-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLFreeStmt<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    Option: FreeStmtOption,
) -> SQLRETURN {
    unsafe { extern_api::SQLFreeStmt(StatementHandle.as_SQLHANDLE(), Option as SQLUSMALLINT) }
}

/// Returns the current setting of a connection attribute.
///
/// For complete documentation on SQLGetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetConnectAttrA<
    A: Ident<Type = SQLINTEGER>,
    T: ConnAttr<A, C, V>,
    C: ConnState,
    V: OdbcVersion,
>(
    // TODO: Not sure whether attributes should be checked when getting them with SQLGetConnectAttr
    ConnectionHandle: &SQLHDBC<C, V>,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + AnsiType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetConnectAttrA(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current setting of a connection attribute.
///
/// For complete documentation on SQLGetConnectAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetConnectAttrW<
    A: Ident<Type = SQLINTEGER>,
    T: ConnAttr<A, C, V>,
    C: ConnState,
    V: OdbcVersion,
>(
    // TODO: Not really sure whether attributes should be checked when getting them with SQLGetConnectAttr
    ConnectionHandle: &SQLHDBC<C, V>,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetConnectAttrW(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the cursor name associated with a specified statement.
///
/// For complete documentation on SQLGetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetCursorNameA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CursorName: &mut [MaybeUninit<SQLCHAR>],
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let CursorName = CursorName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetCursorNameA(
            StatementHandle.as_SQLHANDLE(),
            CursorName.0,
            CursorName.1,
            NameLengthPtr.as_mut_ptr(),
        )
    }
}

/// Returns the cursor name associated with a specified statement.
///
/// For complete documentation on SQLGetCursorNameW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetCursorNameW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CursorName: &mut [MaybeUninit<SQLWCHAR>],
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let CursorName = CursorName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetCursorNameW(
            StatementHandle.as_SQLHANDLE(),
            CursorName.0,
            CursorName.1,
            NameLengthPtr.as_mut_ptr(),
        )
    }
}

/// Retrieves data for a single column in the result set or for a single parameter after **SQLParamData** returns SQL_PARAM_DATA_AVAILABLE. It can be called multiple times to retrieve variable-length data in parts.
///
/// For complete documentation on SQLGetData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdata-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
// TODO: This function must be unsafe if SQL_ARD_TYPE and SQL_APD_TYPE are allowed to be used
pub fn SQLGetData<TT: Ident<Type = SQLSMALLINT>, B: CData<TT, V>, V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    Col_or_Param_Num: SQLUSMALLINT,
    TargetType: TT,
    TargetValuePtr: &mut B,
    StrLen_or_IndPtr: Option<&mut MaybeUninit<StrLenOrInd>>,
) -> SQLRETURN
where
    B: AsMutSQLPOINTER + ?Sized,
    MaybeUninit<StrLenOrInd>: AsMutPtr<SQLLEN>,
{
    unsafe {
        extern_api::SQLGetData(
            StatementHandle.as_SQLHANDLE(),
            Col_or_Param_Num,
            TT::IDENTIFIER,
            TargetValuePtr.as_mut_SQLPOINTER(),
            TargetValuePtr.len(),
            StrLen_or_IndPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current setting or value of a single field of a descriptor record.
///
/// For complete documentation on SQLGetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDescFieldA<DT, A: Ident<Type = SQLSMALLINT>, T: DescField<DT, A>, V: OdbcVersion>(
    DescriptorHandle: &mut SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    FieldIdentifier: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + AnsiType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetDescFieldA(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current setting or value of a single field of a descriptor record.
///
/// For complete documentation on SQLGetDescFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDescFieldW<DT, A: Ident<Type = SQLSMALLINT>, T: DescField<DT, A>, V: OdbcVersion>(
    DescriptorHandle: &mut SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    FieldIdentifier: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetDescFieldW(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current settings or values of multiple fields of a descriptor record. The fields returned describe the name, data type, and storage of column or parameter data.
///
/// For complete documentation on SQLGetDescRecA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetDescRecA<DT, ST: SqlType<V>, V: OdbcVersion>(
    DescriptorHandle: &SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    Name: Option<&mut [MaybeUninit<SQLCHAR>]>,
    StringLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    TypePtr: &mut MaybeUninit<ST>,
    SubTypePtr: &mut MaybeUninit<DatetimeIntervalCode>,
    LengthPtr: &mut MaybeUninit<SQLLEN>,
    PrecisionPtr: &mut MaybeUninit<SQLSMALLINT>,
    ScalePtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<NullAllowed>,
) -> SQLRETURN
where
    DT: for<'buf> DescType<'buf>,
{
    let Name = Name.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    unsafe {
        extern_api::SQLGetDescRecA(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            Name.0,
            Name.1,
            StringLengthPtr.as_mut_ptr(),
            TypePtr.as_mut_ptr().cast(),
            SubTypePtr.as_mut_ptr().cast(),
            LengthPtr.as_mut_ptr(),
            PrecisionPtr.as_mut_ptr(),
            ScalePtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr().cast(),
        )
    }
}

/// Returns the current settings or values of multiple fields of a descriptor record. The fields returned describe the name, data type, and storage of column or parameter data.
///
/// For complete documentation on SQLGetDescRecW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetDescRecW<DT, ST: SqlType<V>, V: OdbcVersion>(
    DescriptorHandle: &SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    Name: Option<&mut [MaybeUninit<SQLWCHAR>]>,
    StringLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    TypePtr: &mut MaybeUninit<ST>,
    SubTypePtr: &mut MaybeUninit<DatetimeIntervalCode>,
    LengthPtr: &mut MaybeUninit<SQLLEN>,
    PrecisionPtr: &mut MaybeUninit<SQLSMALLINT>,
    ScalePtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<NullAllowed>,
) -> SQLRETURN
where
    DT: for<'buf> DescType<'buf>,
{
    let Name = Name.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

    unsafe {
        extern_api::SQLGetDescRecW(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            Name.0,
            Name.1,
            StringLengthPtr.as_mut_ptr(),
            TypePtr.as_mut_ptr().cast(),
            SubTypePtr.as_mut_ptr().cast(),
            LengthPtr.as_mut_ptr(),
            PrecisionPtr.as_mut_ptr(),
            ScalePtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr().cast(),
        )
    }
}

/// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
///
/// For complete documentation on SQLGetDiagFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDiagFieldA<H: Handle, D: Ident<Type = SQLSMALLINT>, T: DiagField<H, D>>(
    HandleType: H::Ident,
    Handle: &H,
    // TODO: Use NoneZeroI16?
    RecNumber: std::num::NonZeroI16,
    DiagIdentifier: D,
    DiagInfoPtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    H: AsSQLHANDLE,
    T: AttrGet<D> + AnsiType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    let DiagInfoPtr = DiagInfoPtr.map_or((ptr::null_mut(), 0), |DiagInfoPtr| {
        if cfg!(feature = "odbc_debug") {
            DiagInfoPtr.assert_zeroed();
        }

        (DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len())
    });

    unsafe {
        extern_api::SQLGetDiagFieldA(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            RecNumber.get(),
            D::IDENTIFIER,
            DiagInfoPtr.0,
            DiagInfoPtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
///
/// For complete documentation on SQLGetDiagFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDiagFieldW<H: Handle, D: Ident<Type = SQLSMALLINT>, T: DiagField<H, D>>(
    HandleType: H::Ident,
    Handle: &H,
    // TODO: Use NoneZeroI16?
    RecNumber: std::num::NonZeroI16,
    DiagIdentifier: D,
    DiagInfoPtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    H: AsSQLHANDLE,
    T: AttrGet<D> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    let DiagInfoPtr = DiagInfoPtr.map_or((ptr::null_mut(), 0), |DiagInfoPtr| {
        if cfg!(feature = "odbc_debug") {
            DiagInfoPtr.assert_zeroed();
        }

        (DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len())
    });

    unsafe {
        extern_api::SQLGetDiagFieldW(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            RecNumber.get(),
            D::IDENTIFIER,
            DiagInfoPtr.0,
            DiagInfoPtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
///
/// For complete documentation on SQLGetDiagRecA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDiagRecA<H: Handle>(
    HandleType: H::Ident,
    Handle: &H,
    // TODO: Use NoneZeroI16?
    RecNumber: std::num::NonZeroI16,
    SQLState: &mut MaybeUninit<SQLSTATE<SQLCHAR>>,
    NativeErrorPtr: &mut MaybeUninit<SQLINTEGER>,
    MessageText: &mut [MaybeUninit<SQLCHAR>],
    TextLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN
where
    H: AsSQLHANDLE,
{
    let MessageText = MessageText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetDiagRecA(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            RecNumber.get(),
            SQLState.as_mut_ptr().cast(),
            NativeErrorPtr.as_mut_ptr(),
            MessageText.0,
            MessageText.1,
            TextLengthPtr.as_mut_ptr(),
        )
    }
}

/// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
///
/// For complete documentation on SQLGetDiagRecW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetDiagRecW<H: Handle>(
    HandleType: H::Ident,
    Handle: &H,
    // TODO: Use NoneZeroI16?
    RecNumber: std::num::NonZeroI16,
    SQLState: &mut MaybeUninit<SQLSTATE<SQLWCHAR>>,
    NativeErrorPtr: &mut MaybeUninit<SQLINTEGER>,
    MessageText: &mut [MaybeUninit<SQLWCHAR>],
    TextLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN
where
    H: AsSQLHANDLE,
{
    let MessageText = MessageText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetDiagRecW(
            H::Ident::IDENTIFIER,
            Handle.as_SQLHANDLE(),
            RecNumber.get(),
            SQLState.as_mut_ptr().cast(),
            NativeErrorPtr.as_mut_ptr(),
            MessageText.0,
            MessageText.1,
            TextLengthPtr.as_mut_ptr(),
        )
    }
}

/// Returns the current setting of an environment attribute.
///
/// For complete documentation on SQLGetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetenvattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetEnvAttr<A: Ident<Type = SQLINTEGER>, T: EnvAttr<A, V>, V: OdbcVersion>(
    EnvironmentHandle: &SQLHENV<V>,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
        (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns information about whether a driver supports a specific ODBC function. This function is implemented in the Driver Manager; it can also be implemented in drivers. If a driver implements **SQLGetFunctions**, the Driver Manager calls the function in the driver. Otherwise, it executes the function itself.
///
/// For complete documentation on SQLGetFunctions, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetfunctions-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetFunctions<V: OdbcVersion>(
    ConnectionHandle: &SQLHDBC<C4, V>,
    FunctionId: FunctionId,
    SupportedPtr: &mut MaybeUninit<SQLUSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLGetFunctions(
            ConnectionHandle.as_SQLHANDLE(),
            FunctionId as SQLUSMALLINT,
            SupportedPtr.as_mut_ptr(),
        )
    }
}

/// Returns general information about the driver and data source associated with a connection.
///
/// For complete documentation on SQLGetInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetInfoA<I: Ident<Type = SQLUSMALLINT>, T: InfoType<I, V>, V: OdbcVersion>(
    // TODO: SQL_ODBC_VER can be called on connection that is not open
    ConnectionHandle: &SQLHDBC<C4, V>,
    InfoType: I,
    InfoValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<I> + AnsiType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    let InfoValuePtr = InfoValuePtr.map_or((ptr::null_mut(), 0), |InfoValuePtr| {
        (InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetInfoA(
            ConnectionHandle.as_SQLHANDLE(),
            I::IDENTIFIER,
            InfoValuePtr.0,
            InfoValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns general information about the driver and data source associated with a connection.
///
/// For complete documentation on SQLGetInfoW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetInfoW<I: Ident<Type = SQLUSMALLINT>, T: InfoType<I, V>, V: OdbcVersion>(
    // TODO: SQL_ODBC_VER can be called on connection that is not open
    ConnectionHandle: &SQLHDBC<C4, V>,
    InfoType: I,
    InfoValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<I> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    let InfoValuePtr = InfoValuePtr.map_or((ptr::null_mut(), 0), |InfoValuePtr| {
        (InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len())
    });

    unsafe {
        extern_api::SQLGetInfoW(
            ConnectionHandle.as_SQLHANDLE(),
            I::IDENTIFIER,
            InfoValuePtr.0,
            InfoValuePtr.1,
            StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
        )
    }
}

/// Returns the current setting of a statement attribute.
///
/// For complete documentation on SQLGetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetStmtAttrA<
    'stmt,
    'buf,
    A: Ident<Type = SQLINTEGER>,
    T: StmtAttr<'stmt, 'buf, A, V>,
    V: OdbcVersion,
>(
    StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf, V>,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + AnsiType,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    if let Some(ValuePtr) = ValuePtr {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        ValuePtr.readA(StatementHandle, StringLengthPtr)
    } else {
        unsafe {
            extern_api::SQLGetStmtAttrA(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                ptr::null_mut(),
                0,
                StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
            )
        }
    }
}

/// Returns the current setting of a statement attribute.
///
/// For complete documentation on SQLGetStmtAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetStmtAttrW<
    'stmt,
    'buf,
    A: Ident<Type = SQLINTEGER>,
    T: StmtAttr<'stmt, 'buf, A, V>,
    V: OdbcVersion,
>(
    StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf, V>,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + UnicodeType + ?Sized,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    if let Some(ValuePtr) = ValuePtr {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        ValuePtr.readW(StatementHandle, StringLengthPtr)
    } else {
        unsafe {
            extern_api::SQLGetStmtAttrW(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                ptr::null_mut(),
                0,
                StringLengthPtr.map_or_else(ptr::null_mut, AsMutPtr::as_mut_ptr),
            )
        }
    }
}

/// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
///
/// For complete documentation on SQLGetTypeInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetTypeInfoA<V: OdbcVersion, ST: SqlType<V>>(
    StatementHandle: &SQLHSTMT<V>,
    DataType: ST,
) -> SQLRETURN {
    unsafe { extern_api::SQLGetTypeInfoA(StatementHandle.as_SQLHANDLE(), DataType.identifier()) }
}

/// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
///
/// For complete documentation on SQLGetTypeInfoW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetTypeInfoW<V: OdbcVersion, ST: SqlType<V>>(
    StatementHandle: &SQLHSTMT<V>,
    DataType: ST,
) -> SQLRETURN {
    unsafe { extern_api::SQLGetTypeInfoW(StatementHandle.as_SQLHANDLE(), DataType.identifier()) }
}

/// Determines whether more results are available on a statement containing **SELECT**, **UPDATE**, **INSERT**, or **DELETE** statements and, if so, initializes processing for those results.
///
/// For complete documentation on SQLMoreResults, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlmoreresults-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, OR SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLMoreResults<V: OdbcVersion>(StatementHandle: &SQLHSTMT<V>) -> SQLRETURN {
    unsafe { extern_api::SQLMoreResults(StatementHandle.as_SQLHANDLE()) }
}

/// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
///
/// For complete documentation on SQLNativeSqlA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLNativeSqlA<V: OdbcVersion>(
    ConnectionHandle: &SQLHDBC<C4, V>,
    InStatementText: &[SQLCHAR],
    OutStatementText: &mut [MaybeUninit<SQLCHAR>],
    TextLength2Ptr: &mut MaybeUninit<SQLINTEGER>,
) -> SQLRETURN {
    let InStatementText = InStatementText.as_raw_slice();
    let OutStatementText = OutStatementText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLNativeSqlA(
            ConnectionHandle.as_SQLHANDLE(),
            InStatementText.0,
            InStatementText.1,
            OutStatementText.0,
            OutStatementText.1,
            TextLength2Ptr.as_mut_ptr(),
        )
    }
}

/// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
///
/// For complete documentation on SQLNativeSqlW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLNativeSqlW<V: OdbcVersion>(
    ConnectionHandle: &SQLHDBC<C4, V>,
    InStatementText: &[SQLWCHAR],
    OutStatementText: &mut [MaybeUninit<SQLWCHAR>],
    TextLength2Ptr: &mut MaybeUninit<SQLINTEGER>,
) -> SQLRETURN {
    let InStatementText = InStatementText.as_raw_slice();
    let OutStatementText = OutStatementText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLNativeSqlW(
            ConnectionHandle.as_SQLHANDLE(),
            InStatementText.0,
            InStatementText.1,
            OutStatementText.0,
            OutStatementText.1,
            TextLength2Ptr.as_mut_ptr(),
        )
    }
}

/// Returns the number of parameters in an SQL statement.
///
/// For complete documentation on SQLNumParams, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumparams-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLNumParams<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ParameterCountPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLNumParams(
            StatementHandle.as_SQLHANDLE(),
            ParameterCountPtr.as_mut_ptr(),
        )
    }
}

/// Returns the number of columns in a result set.
///
/// For complete documentation on SQLNumResultCols, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumresultcols-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLNumResultCols<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ColumnCountPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLNumResultCols(StatementHandle.as_SQLHANDLE(), ColumnCountPtr.as_mut_ptr())
    }
}

/// Used together with **SQLPutData** to supply parameter data at statement execution time, and with **SQLGetData** to retrieve streamed output parameter data.
///
/// For complete documentation on SQLParamData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlparamdata-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLParamData<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    ValuePtrPtr: &mut MaybeUninit<SQLPOINTER>,
) -> SQLRETURN {
    unsafe { extern_api::SQLParamData(StatementHandle.as_SQLHANDLE(), ValuePtrPtr.as_mut_ptr()) }
}

/// Prepares an SQL string for execution.
///
/// For complete documentation on SQLPrepareA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLPrepareA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    StatementText: &[SQLCHAR],
) -> SQLRETURN {
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLPrepareA(
            StatementHandle.as_SQLHANDLE(),
            StatementText.0,
            StatementText.1,
        )
    }
}

/// Prepares an SQL string for execution.
///
/// For complete documentation on SQLPrepareW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLPrepareW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    StatementText: &[SQLWCHAR],
) -> SQLRETURN {
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLPrepareW(
            StatementHandle.as_SQLHANDLE(),
            StatementText.0,
            StatementText.1,
        )
    }
}

/// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
///
/// For complete documentation on SQLPrimaryKeysA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLPrimaryKeysA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLPrimaryKeysA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
        )
    }
}

/// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
///
/// For complete documentation on SQLPrimaryKeysW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLPrimaryKeysW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLPrimaryKeysW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
        )
    }
}

/// Returns the list of input and output parameters, as well as the columns that make up the result set for the specified procedures. The driver returns the information as a result set on the specified statement.
///
/// For complete documentation on SQLProcedureColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedurecolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLProcedureColumnsA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    ProcName: &[SQLCHAR],
    ColumnName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLProcedureColumnsA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            ProcName.0,
            ProcName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Returns the list of input and output parameters, as well as the columns that make up the result set for the specified procedures. The driver returns the information as a result set on the specified statement.
///
/// For complete documentation on SQLProcedureColumnsW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedurecolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLProcedureColumnsW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    ProcName: &[SQLWCHAR],
    ColumnName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLProcedureColumnsW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            ProcName.0,
            ProcName.1,
            ColumnName.0,
            ColumnName.1,
        )
    }
}

/// Returns the list of procedure names stored in a specific data source. `Procedure` is a generic term used to describe an `executable object`, or a named entity that can be invoked using input and output parameters. For more information on procedures, see the Procedures.
///
/// For complete documentation on SQLProceduresA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedures-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLProceduresA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    ProcName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();

    unsafe {
        extern_api::SQLProceduresA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            ProcName.0,
            ProcName.1,
        )
    }
}

/// Returns the list of procedure names stored in a specific data source. `Procedure` is a generic term used to describe an `executable object`, or a named entity that can be invoked using input and output parameters. For more information on procedures, see the Procedures.
///
/// For complete documentation on SQLProceduresW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedures-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLProceduresW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    ProcName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();

    unsafe {
        extern_api::SQLProceduresW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            ProcName.0,
            ProcName.1,
        )
    }
}

/// Allows an application to send data for a parameter or column to the driver at statement execution time. This function can be used to send character or binary data values in parts to a column with a character, binary, or data source-specific data type (for example, parameters of the SQL_LONGVARBINARY or SQL_LONGVARCHAR types). **SQLPutData** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
///
/// For complete documentation on SQLPutData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlputdata-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLPutData<TT: Ident, B: CData<TT, V>, V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    DataPtr: Option<&B>,
) -> SQLRETURN
where
    B: AsSQLPOINTER + ?Sized,
{
    let DataPtr = DataPtr.map_or((ptr::null_mut(), 0), |DataPtr| {
        (DataPtr.as_SQLPOINTER(), DataPtr.len())
    });

    // TODO: This function is incredibly unsafe. How to know which type to provide and panic at runtime if incorrect?
    extern_api::SQLPutData(StatementHandle.as_SQLHANDLE(), DataPtr.0, DataPtr.1)
}

/// Returns the number of rows affected by an **UPDATE**, **INSERT**, or **DELETE** statement; an SQL_ADD, SQL_UPDATE_BY_BOOKMARK, or SQL_DELETE_BY_BOOKMARK operation in **SQLBulkOperations**; or an SQL_UPDATE or SQL_DELETE operation in **SQLSetPos**.
///
/// For complete documentation on SQLRowCount, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlrowcount-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLRowCount<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    RowCountPtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN {
    unsafe { extern_api::SQLRowCount(StatementHandle.as_SQLHANDLE(), RowCountPtr.as_mut_ptr()) }
}

/// Sets attributes that govern aspects of connections.
///
/// For complete documentation on SQLSetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetConnectAttrA<
    A: Ident<Type = SQLINTEGER>,
    T: ConnAttr<A, C, V>,
    C: ConnState,
    V: OdbcVersion,
>(
    ConnectionHandle: &SQLHDBC<C, V>,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + AnsiType,
{
    unsafe {
        extern_api::SQLSetConnectAttrA(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    }
}

/// Sets attributes that govern aspects of connections.
///
/// For complete documentation on SQLSetConnectAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetConnectAttrW<
    A: Ident<Type = SQLINTEGER>,
    T: ConnAttr<A, C, V>,
    C: ConnState,
    V: OdbcVersion,
>(
    ConnectionHandle: &SQLHDBC<C, V>,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + UnicodeType,
{
    unsafe {
        extern_api::SQLSetConnectAttrW(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    }
}

/// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
///
/// For complete documentation on SQLSetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLSetCursorNameA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CursorName: &[SQLCHAR],
) -> SQLRETURN {
    let CursorName = CursorName.as_raw_slice();

    unsafe {
        extern_api::SQLSetCursorNameA(StatementHandle.as_SQLHANDLE(), CursorName.0, CursorName.1)
    }
}

/// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
///
/// For complete documentation on SQLSetCursorNameW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLSetCursorNameW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CursorName: &[SQLWCHAR],
) -> SQLRETURN {
    let CursorName = CursorName.as_raw_slice();

    unsafe {
        extern_api::SQLSetCursorNameW(StatementHandle.as_SQLHANDLE(), CursorName.0, CursorName.1)
    }
}

/// Sets the value of a single field of a descriptor record.
///
/// For complete documentation on SQLSetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetDescFieldA<
    DT,
    A: Ident<Type = SQLSMALLINT>,
    T: WriteDescField<DT, A>,
    V: OdbcVersion,
>(
    DescriptorHandle: &SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    FieldIdentifier: A,
    ValuePtr: Option<T>,
) -> SQLRETURN
where
    T: AnsiType,
{
    let sql_return = unsafe {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            (ValuePtr.into_SQLPOINTER(), ValuePtr.len())
        });

        extern_api::SQLSetDescFieldA(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.map(|v| v.update_handle(DescriptorHandle));
    }

    sql_return
}

/// Sets the value of a single field of a descriptor record.
///
/// For complete documentation on SQLSetDescFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetDescFieldW<
    DT,
    A: Ident<Type = SQLSMALLINT>,
    T: WriteDescField<DT, A>,
    V: OdbcVersion,
>(
    DescriptorHandle: &SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    FieldIdentifier: A,
    ValuePtr: Option<T>,
) -> SQLRETURN
where
    T: UnicodeType,
{
    let sql_return = unsafe {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            (ValuePtr.into_SQLPOINTER(), ValuePtr.len())
        });

        extern_api::SQLSetDescFieldW(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.map(|v| v.update_handle(DescriptorHandle));
    }

    sql_return
}

/// Sets multiple descriptor fields that affect the data type and buffer bound to a column or parameter data.
///
/// For complete documentation on SQLSetDescRec, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
// TODO: Must not accept IRD
pub fn SQLSetDescRec<'buf, DT, ST: SqlType<V>, PTR, V: OdbcVersion>(
    DescriptorHandle: &SQLHDESC<DT, V>,
    RecNumber: SQLSMALLINT,
    Type: ST,
    SubType: Option<DatetimeIntervalCode>,
    Length: SQLLEN,
    Precision: SQLSMALLINT,
    Scale: SQLSMALLINT,
    // TODO: Input or Output for both? I guess it depends on which descriptor was given
    DataPtr: Option<&'buf PTR>,
    // TODO: Shouldn't following two be UnsafeCell
    StringLengthPtr: &'buf mut MaybeUninit<SQLLEN>,
    IndicatorPtr: &'buf mut MaybeUninit<SQLLEN>,
) -> SQLRETURN
where
    DT: DescType<'buf>,
    &'buf PTR: IntoSQLPOINTER,
{
    unsafe {
        extern_api::SQLSetDescRec(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            Type.identifier(),
            SubType.map_or(0, |v| v.identifier()),
            Length,
            Precision,
            Scale,
            DataPtr.map_or_else(ptr::null_mut, IntoSQLPOINTER::into_SQLPOINTER),
            StringLengthPtr.as_mut_ptr(),
            IndicatorPtr.as_mut_ptr(),
        )
    }
}

/// Sets attributes that govern aspects of environments.
///
/// For complete documentation on SQLSetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetenvattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetEnvAttr<A: Ident<Type = SQLINTEGER>, T: EnvAttr<A, V>, V: OdbcVersion>(
    // Reference to SQLHENV is mutable to make it impossible to have a connection
    // handle allocated on the environment handle when calling this function
    EnvironmentHandle: &mut SQLHENV<V>,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A>,
{
    unsafe {
        extern_api::SQLSetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    }
}

/// Sets the cursor position in a rowset and allows an application to refresh data in the rowset or to update or delete data in the result set.
///
/// For complete documentation on SQLSetPos, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetpos-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub unsafe fn SQLSetPos<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    RowNumber: SQLSETPOSIROW,
    Operation: Operation,
    LockType: LockType,
) -> SQLRETURN {
    extern_api::SQLSetPos(
        StatementHandle.as_SQLHANDLE(),
        RowNumber,
        Operation as SQLUSMALLINT,
        LockType as SQLUSMALLINT,
    )
}

/// Sets attributes related to a statement.
///
/// For complete documentation on SQLSetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetStmtAttrA<
    'stmt,
    'buf,
    A: Ident<Type = SQLINTEGER>,
    T: StmtAttr<'stmt, 'buf, A, V>,
    V: OdbcVersion,
>(
    StatementHandle: &SQLHSTMT<'_, 'stmt, 'buf, V>,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + AnsiType,
{
    let sql_return = unsafe {
        extern_api::SQLSetStmtAttrA(
            StatementHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.update_handle(&StatementHandle);
    }

    sql_return
}

/// Sets attributes related to a statement.
///
/// For complete documentation on SQLSetStmtAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetStmtAttrW<
    'stmt,
    'buf,
    A: Ident<Type = SQLINTEGER>,
    T: StmtAttr<'stmt, 'buf, A, V>,
    V: OdbcVersion,
>(
    StatementHandle: &SQLHSTMT<'_, 'stmt, 'buf, V>,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + UnicodeType,
{
    let sql_return = unsafe {
        extern_api::SQLSetStmtAttrW(
            StatementHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.update_handle(&StatementHandle);
    }

    sql_return
}

/// Retrieves the following information about columns within a specified table:
///
/// * The optimal set of columns that uniquely identifies a row in the table.
/// * Columns that are automatically updated when any value in the row is updated by a transaction.
///
/// For complete documentation on SQLSpecialColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlspecialcolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLSpecialColumnsA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    IdentifierType: IdentifierType,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
    Scope: Scope,
    Nullable: NullAllowed,
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLSpecialColumnsA(
            StatementHandle.as_SQLHANDLE(),
            IdentifierType as SQLSMALLINT,
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Scope as SQLSMALLINT,
            Nullable.identifier(),
        )
    }
}

/// Retrieves the following information about columns within a specified table:
///
/// * The optimal set of columns that uniquely identifies a row in the table.
/// * Columns that are automatically updated when any value in the row is updated by a transaction.
///
/// For complete documentation on SQLSpecialColumnsW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlspecialcolumns-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLSpecialColumnsW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    IdentifierType: IdentifierType,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
    Scope: Scope,
    Nullable: NullAllowed,
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLSpecialColumnsW(
            StatementHandle.as_SQLHANDLE(),
            IdentifierType as SQLSMALLINT,
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Scope as SQLSMALLINT,
            Nullable.identifier(),
        )
    }
}

/// Retrieves a list of statistics about a single table and the indexes associated with the table. The driver returns the information as a result set.
///
/// For complete documentation on SQLStatisticsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlstatistics-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLStatisticsA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
    Unique: Unique,
    Reserved: Reserved,
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLStatisticsA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Unique as SQLUSMALLINT,
            Reserved as SQLUSMALLINT,
        )
    }
}

/// Retrieves a list of statistics about a single table and the indexes associated with the table. The driver returns the information as a result set.
///
/// For complete documentation on SQLStatisticsW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlstatistics-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLStatisticsW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
    Unique: Unique,
    Reserved: Reserved,
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLStatisticsW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Unique as SQLUSMALLINT,
            Reserved as SQLUSMALLINT,
        )
    }
}

/// Returns a list of tables and the privileges associated with each table. The driver returns the information as a result set on the specified statement.
///
/// For complete documentation on SQLTablePrivilegesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltableprivileges-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLTablePrivilegesA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLTablePrivilegesA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
        )
    }
}

/// Returns a list of tables and the privileges associated with each table. The driver returns the information as a result set on the specified statement.
///
/// For complete documentation on SQLTablePrivilegesW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltableprivileges-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLTablePrivilegesW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLTablePrivilegesW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
        )
    }
}

/// Returns the list of table, catalog, or schema names, and table types, stored in a specific data source. The driver returns the information as a result set.
///
/// For complete documentation on SQLTablesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltables-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLTablesA<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLCHAR],
    SchemaName: &[SQLCHAR],
    TableName: &[SQLCHAR],
    TableType: &[SQLCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let TableType = TableType.as_raw_slice();

    unsafe {
        extern_api::SQLTablesA(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            TableType.0,
            TableType.1,
        )
    }
}

/// Returns the list of table, catalog, or schema names, and table types, stored in a specific data source. The driver returns the information as a result set.
///
/// For complete documentation on SQLTablesW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltables-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLTablesW<V: OdbcVersion>(
    StatementHandle: &SQLHSTMT<V>,
    CatalogName: &[SQLWCHAR],
    SchemaName: &[SQLWCHAR],
    TableName: &[SQLWCHAR],
    TableType: &[SQLWCHAR],
) -> SQLRETURN {
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let TableType = TableType.as_raw_slice();

    unsafe {
        extern_api::SQLTablesW(
            StatementHandle.as_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            TableType.0,
            TableType.1,
        )
    }
}
