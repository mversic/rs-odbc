use crate::extern_api;
use crate::handle::*;
use std::mem::MaybeUninit;

use crate::{
    col::ColAttr, conn::ConnAttr, env::EnvAttr, stmt::StmtAttr, AnsiType, AsMutPtr, AsMutRawSlice,
    AsMutSQLPOINTER, AsRawSlice, AsSQLPOINTER, CType, DriverCompletion, GetAttr, Len, Nullable,
    OdbcAttr, SetAttr, UnicodeType, RETCODE, SQLCHAR, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT,
    SQLINTEGER, SQLLEN, SQLRETURN, SQLSETPOSIROW, SQLSMALLINT, SQLULEN, SQLUSMALLINT, SQLWCHAR,
    SQL_SUCCESS,
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
    HandleType: OH::Identifier,
    InputHandle: &'src OH::SrcHandle,
    OutputHandlePtr: &mut MaybeUninit<OH>,
) -> SQLRETURN
where
    OH::SrcHandle: AsSQLHANDLE,
    OH: AsMutSQLHANDLE,
{
    unsafe {
        extern_api::SQLAllocHandle(
            OH::Identifier::IDENTIFIER,
            InputHandle.as_SQLHANDLE(),
            OutputHandlePtr.as_mut_ptr().cast(),
        )
    }
}

// TODO:
///// Binds application data buffers to columns in the result set.
/////
///// For complete documentation on SQLBindCol, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLBindCol<TT: TargetType, T: AsMutSQLPOINTER>(StatementHandle: &mut SQLHSTMT, ColumnNumber: SQLUSMALLINT, TargetType: TT, TargetValuePtr: &mut T, StrLen_or_IndPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where TT: GetAttr<AnsiType, T>, T: Len<OdbcAttr, SQLLEN>, MaybeUninit<T::StrLen>: AsMutPtr<SQLLEN> {
//    unsafe{ extern_api::SQLBindCol(StatementHandle.as_mut_SQLHANDLE(), ColumnNumber, TargetType, TargetValuePtr.as_mut_SQLPOINTER(), TargetValuePtr.len(), StrLen_or_IndPtr.as_mut_ptr()) }
//}

///// Binds a buffer to a parameter marker in an SQL statement. **SQLBindParameter** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
/////
///// For complete documentation on SQLBindParameter, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindparameter-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLBindParameter<PTR: AsMutRawSlice<SQLLEN>>(StatementHandle: &mut SQLHSTMT, ParameterNumber: SQLUSMALLINT, InputOutputType: SQLSMALLINT, ValueType: SQLSMALLINT, ParameterType: SQLSMALLINT, ColumnSize: SQLULEN, DecimalDigits: SQLSMALLINT, ParameterValuePtr: PTR, StrLen_or_IndPtr: &mut MaybeUninit<SQLLEN>) -> SQLRETURN {
//    unsafe{ extern_api::SQLBindParameter(StatementHandle.as_mut_SQLHANDLE(), ParameterNumber, InputOutputType, ValueType, ParameterType, ColumnSize, DecimalDigits, ParameterValuePtr.as_mut_SQLPOINTER(), ParameterValuePtr.len(), StrLen_or_IndPtr.as_mut_ptr()) }
//}

///// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
/////
///// For complete documentation on SQLBrowseConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLBrowseConnectA<
//    C: AsRawSlice<SQLCHAR, SQLSMALLINT>,
//    MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>,
//>(
//    ConnectionHandle: &mut SQLHDBC,
//    InConnectionString: &C,
//    OutConnectionString: &mut MC,
//    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
//) -> SQLRETURN where C: ?Sized {
//    let InConnectionString = InConnectionString.as_raw_slice();
//    let OutConnectionString = OutConnectionString.as_mut_raw_slice();
//
//    unsafe {
//        extern_api::SQLBrowseConnectA(
//            ConnectionHandle.as_mut_SQLHANDLE(),
//            InConnectionString.0,
//            InConnectionString.1,
//            OutConnectionString.0,
//            OutConnectionString.1,
//            StringLength2Ptr.as_mut_ptr(),
//        )
//    }
//}
//
///// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
/////
///// For complete documentation on SQLBrowseConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLBrowseConnectW<
//    C: AsRawSlice<SQLWCHAR, SQLSMALLINT>,
//    MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>,
//>(
//    ConnectionHandle: &mut SQLHDBC,
//    InConnectionString: &C,
//    OutConnectionString: &mut MC,
//    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
//) -> SQLRETURN
//where
//    C: ?Sized,
//{
//    let InConnectionString = InConnectionString.as_raw_slice();
//    let OutConnectionString = OutConnectionString.as_mut_raw_slice();
//
//    unsafe {
//        extern_api::SQLBrowseConnectW(
//            ConnectionHandle.as_mut_SQLHANDLE(),
//            InConnectionString.0,
//            InConnectionString.1,
//            OutConnectionString.0,
//            OutConnectionString.1,
//            StringLength2Ptr.as_mut_ptr(),
//        )
//    }
//}

/// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
///
/// For complete documentation on SQLBulkOperations, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbulkoperations-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLBulkOperations(StatementHandle: &mut SQLHSTMT, Operation: SQLUSMALLINT) -> SQLRETURN {
    unsafe { extern_api::SQLBulkOperations(StatementHandle.as_mut_SQLHANDLE(), Operation) }
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
pub fn SQLCancel(StatementHandle: &mut SQLHSTMT) -> SQLRETURN {
    unsafe { extern_api::SQLCancel(StatementHandle.as_mut_SQLHANDLE()) }
}

/// Cancels the processing on a connection or statement. The Driver Manager maps a call to **SQLCancelHandle** to a call to **SQLCancel** when `HandleType` is SQL_HANDLE_STMT.
///
/// For complete documentation on SQLCancelHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancelhandle-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[cfg(feature = "v3_8")]
#[allow(non_snake_case, unused_variables)]
pub fn SQLCancelHandle<H: Handle>(HandleType: H::Identifier, Handle: &mut H) -> SQLRETURN
where
    H: AsMutSQLHANDLE + SQLCancelHandle,
{
    unsafe { extern_api::SQLCancelHandle(H::Identifier::IDENTIFIER, Handle.as_mut_SQLHANDLE()) }
}

/// Closes a cursor that has been opened on a statement and discards pending results.
///
/// For complete documentation on SQLCloseCursor, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlclosecursor-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLCloseCursor(StatementHandle: &mut SQLHSTMT) -> SQLRETURN {
    unsafe { extern_api::SQLCloseCursor(StatementHandle.as_mut_SQLHANDLE()) }
}

/// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
///
/// For complete documentation on SQLColAttributeA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLColAttributeA<A: ColAttr, T: AsMutSQLPOINTER>(
    StatementHandle: &mut SQLHSTMT,
    ColumnNumber: SQLUSMALLINT,
    FieldIdentifier: A,
    CharacterAttributePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
    NumericAttributePtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN
where
    A: GetAttr<AnsiType, T>,
    T: Len<A::AttrType, SQLSMALLINT>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    unsafe {
        extern_api::SQLColAttributeA(
            StatementHandle.as_mut_SQLHANDLE(),
            ColumnNumber,
            A::IDENTIFIER,
            CharacterAttributePtr.as_mut_SQLPOINTER(),
            CharacterAttributePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
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
pub fn SQLColAttributeW<A: ColAttr, T: AsMutSQLPOINTER>(
    StatementHandle: &mut SQLHSTMT,
    ColumnNumber: SQLUSMALLINT,
    FieldIdentifier: A,
    CharacterAttributePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
    NumericAttributePtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN
where
    A: GetAttr<UnicodeType, T>,
    T: Len<A::AttrType, SQLSMALLINT>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT>,
{
    unsafe {
        extern_api::SQLColAttributeW(
            StatementHandle.as_mut_SQLHANDLE(),
            ColumnNumber,
            A::IDENTIFIER,
            CharacterAttributePtr.as_mut_SQLPOINTER(),
            CharacterAttributePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
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
pub fn SQLColumnPrivilegesA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnPrivilegesA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLColumnPrivilegesW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnPrivilegesW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLColumnsA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnsA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLColumnsW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLColumnsW(
            StatementHandle.as_mut_SQLHANDLE(),
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
#[cfg(feature = "v3_8")]
#[allow(non_snake_case, unused_variables)]
pub fn SQLCompleteAsync<H: Handle>(
    HandleType: H::Identifier,
    Handle: &mut H,
    AsyncRetCodePtr: &mut MaybeUninit<RETCODE>,
) -> SQLRETURN
where
    H: AsMutSQLHANDLE + SQLCompleteAsyncHandle,
{
    unsafe {
        extern_api::SQLCompleteAsync(
            H::Identifier::IDENTIFIER,
            Handle.as_mut_SQLHANDLE(),
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
pub fn SQLConnectA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    ConnectionHandle: &mut SQLHDBC,
    ServerName: &C,
    UserName: &C,
    Authentication: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let ServerName = ServerName.as_raw_slice();
    let UserName = UserName.as_raw_slice();
    let Authentication = Authentication.as_raw_slice();

    unsafe {
        extern_api::SQLConnectA(
            ConnectionHandle.as_mut_SQLHANDLE(),
            ServerName.0,
            ServerName.1,
            UserName.0,
            UserName.1,
            Authentication.0,
            Authentication.1,
        )
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
pub fn SQLConnectW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    ConnectionHandle: &mut SQLHDBC,
    ServerName: &C,
    UserName: &C,
    Authentication: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let ServerName = ServerName.as_raw_slice();
    let UserName = UserName.as_raw_slice();
    let Authentication = Authentication.as_raw_slice();

    unsafe {
        extern_api::SQLConnectW(
            ConnectionHandle.as_mut_SQLHANDLE(),
            ServerName.0,
            ServerName.1,
            UserName.0,
            UserName.1,
            Authentication.0,
            Authentication.1,
        )
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
pub fn SQLCopyDesc(SourceDescHandle: &mut SQLHDESC, TargetDescHandle: &mut SQLHDESC) -> SQLRETURN {
    unsafe {
        extern_api::SQLCopyDesc(
            SourceDescHandle.as_mut_SQLHANDLE(),
            TargetDescHandle.as_mut_SQLHANDLE(),
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
pub fn SQLDataSourcesA<MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(
    EnvironmentHandle: &mut SQLHENV,
    Direction: SQLUSMALLINT,
    ServerName: &mut MC,
    NameLength1Ptr: &mut MaybeUninit<SQLSMALLINT>,
    Description: &mut MC,
    NameLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ServerName = ServerName.as_mut_raw_slice();
    let Description = Description.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDataSourcesA(
            EnvironmentHandle.as_mut_SQLHANDLE(),
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
pub fn SQLDataSourcesW<MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(
    EnvironmentHandle: &mut SQLHENV,
    Direction: SQLUSMALLINT,
    ServerName: &mut MC,
    NameLength1Ptr: &mut MaybeUninit<SQLSMALLINT>,
    Description: &mut MC,
    NameLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ServerName = ServerName.as_mut_raw_slice();
    let Description = Description.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDataSourcesW(
            EnvironmentHandle.as_mut_SQLHANDLE(),
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
pub fn SQLDescribeColA<MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    ColumnNumber: SQLUSMALLINT,
    ColumnName: &mut MC,
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ColumnSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ColumnName = ColumnName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDescribeColA(
            StatementHandle.as_mut_SQLHANDLE(),
            ColumnNumber,
            ColumnName.0,
            ColumnName.1,
            NameLengthPtr.as_mut_ptr(),
            DataTypePtr.as_mut_ptr(),
            ColumnSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr(),
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
pub fn SQLDescribeColW<MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    ColumnNumber: SQLUSMALLINT,
    ColumnName: &mut MC,
    NameLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ColumnSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let ColumnName = ColumnName.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDescribeColW(
            StatementHandle.as_mut_SQLHANDLE(),
            ColumnNumber,
            ColumnName.0,
            ColumnName.1,
            NameLengthPtr.as_mut_ptr(),
            DataTypePtr.as_mut_ptr(),
            ColumnSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr(),
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
pub fn SQLDescribeParam(
    StatementHandle: &mut SQLHSTMT,
    ParameterNumber: SQLUSMALLINT,
    DataTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    ParameterSizePtr: &mut MaybeUninit<SQLULEN>,
    DecimalDigitsPtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLDescribeParam(
            StatementHandle.as_mut_SQLHANDLE(),
            ParameterNumber,
            DataTypePtr.as_mut_ptr(),
            ParameterSizePtr.as_mut_ptr(),
            DecimalDigitsPtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr(),
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
pub fn SQLDisconnect(ConnectionHandle: &mut SQLHDBC) -> SQLRETURN {
    unsafe { extern_api::SQLDisconnect(ConnectionHandle.as_mut_SQLHANDLE()) }
}

/// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
///
/// For complete documentation on SQLDriverConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLDriverConnectA<
    C: AsRawSlice<SQLCHAR, SQLSMALLINT>,
    MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>,
>(
    ConnectionHandle: &mut SQLHDBC,
    WindowHandle: Option<SQLHWND>,
    InConnectionString: &C,
    OutConnectionString: &mut MC,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> SQLRETURN
where
    C: ?Sized,
{
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString = OutConnectionString.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriverConnectA(
            ConnectionHandle.as_mut_SQLHANDLE(),
            // TODO: Fix this
            std::ptr::null_mut(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
            DriverCompletion as SQLUSMALLINT,
        )
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
pub fn SQLDriverConnectW<
    C: AsRawSlice<SQLWCHAR, SQLSMALLINT>,
    MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>,
>(
    ConnectionHandle: &mut SQLHDBC,
    WindowHandle: Option<SQLHWND>,
    InConnectionString: &C,
    OutConnectionString: &mut MC,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> SQLRETURN
where
    C: ?Sized,
{
    let InConnectionString = InConnectionString.as_raw_slice();
    let OutConnectionString = OutConnectionString.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriverConnectW(
            ConnectionHandle.as_mut_SQLHANDLE(),
            // TODO: Fix this
            std::ptr::null_mut(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
            DriverCompletion as SQLUSMALLINT,
        )
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
pub fn SQLDriversA<MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(
    EnvironmentHandle: &mut SQLHENV,
    Direction: SQLUSMALLINT,
    DriverDescription: &mut MC,
    DescriptionLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DriverAttributes: &mut MC,
    AttributesLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let DriverDescription = DriverDescription.as_mut_raw_slice();
    let DriverAttributes = DriverAttributes.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriversA(
            EnvironmentHandle.as_mut_SQLHANDLE(),
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
pub fn SQLDriversW<MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(
    EnvironmentHandle: &mut SQLHENV,
    Direction: SQLUSMALLINT,
    DriverDescription: &mut MC,
    DescriptionLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    DriverAttributes: &mut MC,
    AttributesLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let DriverDescription = DriverDescription.as_mut_raw_slice();
    let DriverAttributes = DriverAttributes.as_mut_raw_slice();

    unsafe {
        extern_api::SQLDriversW(
            EnvironmentHandle.as_mut_SQLHANDLE(),
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
    HandleType: H::Identifier,
    Handle: &mut H,
    CompletionType: SQLSMALLINT,
) -> SQLRETURN
where
    H: AsMutSQLHANDLE + SQLEndTranHandle,
{
    unsafe {
        extern_api::SQLEndTran(
            H::Identifier::IDENTIFIER,
            Handle.as_mut_SQLHANDLE(),
            CompletionType,
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
pub fn SQLExecDirectA<C: AsRawSlice<SQLCHAR, SQLINTEGER>>(
    StatementHandle: &mut SQLHSTMT,
    StatementText: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLExecDirectA(
            StatementHandle.as_mut_SQLHANDLE(),
            StatementText.0,
            StatementText.1,
        )
    }
}

/// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
///
/// For complete documentation on SQLExecDirectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLExecDirectW<C: AsRawSlice<SQLWCHAR, SQLINTEGER>>(
    StatementHandle: &mut SQLHSTMT,
    StatementText: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLExecDirectW(
            StatementHandle.as_mut_SQLHANDLE(),
            StatementText.0,
            StatementText.1,
        )
    }
}

/// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
///
/// For complete documentation on SQLExecute, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLExecute(StatementHandle: &mut SQLHSTMT) -> SQLRETURN {
    unsafe { extern_api::SQLExecute(StatementHandle.as_mut_SQLHANDLE()) }
}

/// Fetches the next rowset of data from the result set and returns data for all bound columns.
///
/// For complete documentation on SQLFetch, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLFetch(StatementHandle: &mut SQLHSTMT) -> SQLRETURN {
    unsafe { extern_api::SQLFetch(StatementHandle.as_mut_SQLHANDLE()) }
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
pub fn SQLFetchScroll(
    StatementHandle: &mut SQLHSTMT,
    FetchOrientation: SQLSMALLINT,
    FetchOffset: SQLLEN,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLFetchScroll(
            StatementHandle.as_mut_SQLHANDLE(),
            FetchOrientation,
            FetchOffset,
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
/// For complete documentation on SQLForeignKeysA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlforeignkeys-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLForeignKeysA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    PKCatalogName: &C,
    PKSchemaName: &C,
    PKTableName: &C,
    FKCatalogName: &C,
    FKSchemaName: &C,
    FKTableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let PKCatalogName = PKCatalogName.as_raw_slice();
    let PKSchemaName = PKSchemaName.as_raw_slice();
    let PKTableName = PKTableName.as_raw_slice();
    let FKCatalogName = FKCatalogName.as_raw_slice();
    let FKSchemaName = FKSchemaName.as_raw_slice();
    let FKTableName = FKTableName.as_raw_slice();

    unsafe {
        extern_api::SQLForeignKeysA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLForeignKeysW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    PKCatalogName: &C,
    PKSchemaName: &C,
    PKTableName: &C,
    FKCatalogName: &C,
    FKSchemaName: &C,
    FKTableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let PKCatalogName = PKCatalogName.as_raw_slice();
    let PKSchemaName = PKSchemaName.as_raw_slice();
    let PKTableName = PKTableName.as_raw_slice();
    let FKCatalogName = FKCatalogName.as_raw_slice();
    let FKSchemaName = FKSchemaName.as_raw_slice();
    let FKTableName = FKTableName.as_raw_slice();

    unsafe {
        extern_api::SQLForeignKeysW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLFreeHandle<H: Handle>(HandleType: H::Identifier, Handle: H) -> SQLRETURN {
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
pub fn SQLFreeStmt(StatementHandle: &mut SQLHSTMT, Option: SQLUSMALLINT) -> SQLRETURN {
    unsafe { extern_api::SQLFreeStmt(StatementHandle.as_mut_SQLHANDLE(), Option) }
}

/// Returns the current setting of a connection attribute.
///
/// For complete documentation on SQLGetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetConnectAttrA<A: ConnAttr, T: AsMutSQLPOINTER>(
    ConnectionHandle: &SQLHDBC,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<AnsiType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    unsafe {
        extern_api::SQLGetConnectAttrA(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
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
pub fn SQLGetConnectAttrW<A: ConnAttr, T: AsMutSQLPOINTER>(
    ConnectionHandle: &SQLHDBC,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<UnicodeType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    unsafe {
        extern_api::SQLGetConnectAttrW(
            ConnectionHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
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
pub fn SQLGetCursorNameA<MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &SQLHSTMT,
    CursorName: &mut MC,
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
pub fn SQLGetCursorNameW<MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &SQLHSTMT,
    CursorName: &mut MC,
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
#[allow(non_snake_case)]
pub fn SQLGetData<TT: CType, T: AsMutSQLPOINTER>(
    StatementHandle: &mut SQLHSTMT,
    Col_or_Param_Num: SQLUSMALLINT,
    TargetType: TT,
    TargetValuePtr: &mut T,
    StrLen_or_IndPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    TT: GetAttr<AnsiType, T>,
    T: Len<OdbcAttr, SQLLEN>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLLEN>,
{
    unsafe {
        extern_api::SQLGetData(
            StatementHandle.as_SQLHANDLE(),
            Col_or_Param_Num,
            TT::IDENTIFIER,
            TargetValuePtr.as_mut_SQLPOINTER(),
            TargetValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StrLen_or_IndPtr),
        )
    }
}

///// Returns the current setting or value of a single field of a descriptor record.
/////
///// For complete documentation on SQLGetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLGetDescFieldA<A: DescField, T: AsMutSQLPOINTER>(DescriptorHandle: &SQLHDESC, RecNumber: SQLSMALLINT, FieldIdentifier: A, ValuePtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where A: GetAttr<AnsiType, T>, T: Len<A::AttrType, SQLINTEGER>, MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER> {
//    unsafe{ extern_api::SQLGetDescFieldA(DescriptorHandle.as_SQLHANDLE(), RecNumber, A::IDENTIFIER, ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len(), StringLengthPtr.as_mut_ptr()) }
//}

///// Returns the current setting or value of a single field of a descriptor record.
/////
///// For complete documentation on SQLGetDescFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLGetDescFieldW<A: DescField, T: AsMutSQLPOINTER>(DescriptorHandle: &SQLHDESC, RecNumber: SQLSMALLINT, FieldIdentifier: A, ValuePtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where A: GetAttr<UnicodeType, T>, T: Len<A::AttrType, SQLINTEGER>, MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER> {
//    unsafe{ extern_api::SQLGetDescFieldW(DescriptorHandle.as_SQLHANDLE(), RecNumber, A::IDENTIFIER, ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len(), StringLengthPtr.as_mut_ptr()) }
//}

/// Returns the current settings or values of multiple fields of a descriptor record. The fields returned describe the name, data type, and storage of column or parameter data.
///
/// For complete documentation on SQLGetDescRecA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescrec-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetDescRecA<MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(
    DescriptorHandle: &SQLHDESC,
    RecNumber: SQLSMALLINT,
    Name: &mut MC,
    StringLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    TypePtr: &mut MaybeUninit<SQLSMALLINT>,
    SubTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    LengthPtr: &mut MaybeUninit<SQLLEN>,
    PrecisionPtr: &mut MaybeUninit<SQLSMALLINT>,
    ScalePtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let Name = Name.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetDescRecA(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            Name.0,
            Name.1,
            StringLengthPtr.as_mut_ptr(),
            TypePtr.as_mut_ptr(),
            SubTypePtr.as_mut_ptr(),
            LengthPtr.as_mut_ptr(),
            PrecisionPtr.as_mut_ptr(),
            ScalePtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr(),
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
pub fn SQLGetDescRecW<MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(
    DescriptorHandle: &SQLHDESC,
    RecNumber: SQLSMALLINT,
    Name: &mut MC,
    StringLengthPtr: &mut MaybeUninit<SQLSMALLINT>,
    TypePtr: &mut MaybeUninit<SQLSMALLINT>,
    SubTypePtr: &mut MaybeUninit<SQLSMALLINT>,
    LengthPtr: &mut MaybeUninit<SQLLEN>,
    PrecisionPtr: &mut MaybeUninit<SQLSMALLINT>,
    ScalePtr: &mut MaybeUninit<SQLSMALLINT>,
    NullablePtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    let Name = Name.as_mut_raw_slice();

    unsafe {
        extern_api::SQLGetDescRecW(
            DescriptorHandle.as_SQLHANDLE(),
            RecNumber,
            Name.0,
            Name.1,
            StringLengthPtr.as_mut_ptr(),
            TypePtr.as_mut_ptr(),
            SubTypePtr.as_mut_ptr(),
            LengthPtr.as_mut_ptr(),
            PrecisionPtr.as_mut_ptr(),
            ScalePtr.as_mut_ptr(),
            NullablePtr.as_mut_ptr(),
        )
    }
}

///// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
/////
///// For complete documentation on SQLGetDiagFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
//#[inline]
//#[allow(non_snake_case, unused_variables)]
//pub fn SQLGetDiagFieldA<H: Handle, D: DiagField, T: AsMutSQLPOINTER>(HandleType: H::Identifier, Handle: &H, RecNumber: SQLSMALLINT, DiagIdentifier: D, DiagInfoPtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where H: AsSQLHANDLE, D: GetAttr<AnsiType, T>, T: Len<D::AttrType, SQLSMALLINT>, MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT> {
//    unsafe{ extern_api::SQLGetDiagFieldA(H::Identifier::IDENTIFIER, Handle.as_SQLHANDLE(), RecNumber, D::IDENTIFIER, DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len(), StringLengthPtr.as_mut_ptr()) }
//}
//
///// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
/////
///// For complete documentation on SQLGetDiagFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
//#[inline]
//#[allow(non_snake_case, unused_variables)]
//pub fn SQLGetDiagFieldW<H: Handle, D: DiagField, T: AsMutSQLPOINTER>(HandleType: H::Identifier, Handle: &H, RecNumber: SQLSMALLINT, DiagIdentifier: D, DiagInfoPtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where H: AsSQLHANDLE, D: GetAttr<UnicodeType, T>, T: Len<D::AttrType, SQLSMALLINT>, MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT> {
//    unsafe{ extern_api::SQLGetDiagFieldW(H::Identifier::IDENTIFIER, Handle.as_SQLHANDLE(), RecNumber, D::IDENTIFIER, DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len(), StringLengthPtr.as_mut_ptr()) }
//}

///// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
/////
///// For complete documentation on SQLGetDiagRecA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case, unused_variables)]
//pub fn SQLGetDiagRecA<H: Handle, MC: AsMutRawSlice<SQLCHAR, SQLSMALLINT>>(HandleType: H::Identifier, Handle: &H, SQLState: SqlStateA, NativeErrorPtr: &mut MaybeUninit<SQLINTEGER>, MessageText: &mut MC, TextLengthPtr: &mut MaybeUninit<SQLSMALLINT>) -> SQLRETURN
//where H: AsSQLHANDLE {
//    let MessageText = MessageText.as_mut_raw_slice();
//
//    unsafe{ extern_api::SQLGetDiagRecA(H::Identifier::IDENTIFIER, Handle.as_SQLHANDLE(), SqlState.0.as_mut_ptr(), NativeErrorPtr.as_mut_ptr(), MessageText.0, MessageText.1, TextLengthPtr.as_mut_ptr()) }
//}
//
///// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
/////
///// For complete documentation on SQLGetDiagRecW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case, unused_variables)]
//pub fn SQLGetDiagRecW<H: Handle, MC: AsMutRawSlice<SQLWCHAR, SQLSMALLINT>>(HandleType: H::Identifier, Handle: &H, SQLState: SqlStateW, NativeErrorPtr: &mut MaybeUninit<SQLINTEGER>, MessageText: &mut MC, TextLengthPtr: &mut MaybeUninit<SQLSMALLINT>) -> SQLRETURN
//where H: AsSQLHANDLE {
//    let MessageText = MessageText.as_mut_raw_slice();
//
//    unsafe{ extern_api::SQLGetDiagRecW(H::Identifier::IDENTIFIER, Handle.as_SQLHANDLE(), SqlState.0.as_mut_ptr(), NativeErrorPtr.as_mut_ptr(), MessageText.0, MessageText.1, TextLengthPtr.as_mut_ptr()) }
//}

/// Returns the current setting of an environment attribute.
///
/// For complete documentation on SQLGetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetenvattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetEnvAttr<A: EnvAttr, T: AsMutSQLPOINTER>(
    EnvironmentHandle: &SQLHENV,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<AnsiType, T>,
    T: Len<OdbcAttr, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    unsafe {
        extern_api::SQLGetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
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
pub fn SQLGetFunctions(
    ConnectionHandle: &SQLHDBC,
    FunctionId: SQLUSMALLINT,
    SupportedPtr: &mut MaybeUninit<SQLUSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLGetFunctions(
            ConnectionHandle.as_SQLHANDLE(),
            FunctionId,
            SupportedPtr.as_mut_ptr(),
        )
    }
}

///// Returns general information about the driver and data source associated with a connection.
/////
///// For complete documentation on SQLGetInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLGetInfoA<I: InfoType, T: AsMutSQLPOINTER>(ConnectionHandle: &SQLHDBC, InfoType: I, InfoValuePtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where I: GetAttr<AnsiType, T>, T: Len<I::AttrType, SQLSMALLINT>, MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT> {
//    unsafe{ extern_api::SQLGetInfoA(ConnectionHandle.as_SQLHANDLE(), I::IDENTIFIER, InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len(), StringLengthPtr.as_mut_ptr()) }
//}
//
///// Returns general information about the driver and data source associated with a connection.
/////
///// For complete documentation on SQLGetInfoW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLGetInfoW<I: InfoType, T: AsMutSQLPOINTER>(ConnectionHandle: &SQLHDBC, InfoType: I, InfoValuePtr: &mut T, StringLengthPtr: &mut MaybeUninit<T::StrLen>) -> SQLRETURN
//where I: GetAttr<UnicodeType, T>, T: Len<I::AttrType, SQLSMALLINT>, MaybeUninit<T::StrLen>: AsMutPtr<SQLSMALLINT> {
//    unsafe{ extern_api::SQLGetInfoW(ConnectionHandle.as_SQLHANDLE(), I::IDENTIFIER, InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len(), StringLengthPtr.as_mut_ptr()) }
//}

/// Returns the current setting of a statement attribute.
///
/// For complete documentation on SQLGetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLGetStmtAttrA<A: StmtAttr, T: AsMutSQLPOINTER>(
    StatementHandle: &SQLHSTMT,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<AnsiType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    unsafe {
        extern_api::SQLGetStmtAttrA(
            StatementHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
        )
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
pub fn SQLGetStmtAttrW<A: StmtAttr, T: AsMutSQLPOINTER>(
    StatementHandle: &SQLHSTMT,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<UnicodeType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    unsafe {
        extern_api::SQLGetStmtAttrW(
            StatementHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtr.len(),
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
        )
    }
}

/// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
///
/// For complete documentation on SQLGetTypeInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetTypeInfoA(StatementHandle: &SQLHSTMT, DataType: SQLSMALLINT) -> SQLRETURN {
    unsafe { extern_api::SQLGetTypeInfoA(StatementHandle.as_SQLHANDLE(), DataType) }
}

/// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
///
/// For complete documentation on SQLGetTypeInfoW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLGetTypeInfoW(StatementHandle: &SQLHSTMT, DataType: SQLSMALLINT) -> SQLRETURN {
    unsafe { extern_api::SQLGetTypeInfoW(StatementHandle.as_SQLHANDLE(), DataType) }
}

/// Determines whether more results are available on a statement containing **SELECT**, **UPDATE**, **INSERT**, or **DELETE** statements and, if so, initializes processing for those results.
///
/// For complete documentation on SQLMoreResults, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlmoreresults-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, OR SQL_PARAM_DATA_AVAILABLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLMoreResults(StatementHandle: &mut SQLHSTMT) -> SQLRETURN {
    unsafe { extern_api::SQLMoreResults(StatementHandle.as_mut_SQLHANDLE()) }
}

/// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
///
/// For complete documentation on SQLNativeSqlA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLNativeSqlA<C: AsRawSlice<SQLCHAR, SQLINTEGER>, MC: AsMutRawSlice<SQLCHAR, SQLINTEGER>>(
    ConnectionHandle: &mut SQLHDBC,
    InStatementText: &C,
    OutStatementText: &mut MC,
    TextLength2Ptr: &mut MaybeUninit<SQLINTEGER>,
) -> SQLRETURN
where
    C: ?Sized,
{
    let InStatementText = InStatementText.as_raw_slice();
    let OutStatementText = OutStatementText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLNativeSqlA(
            ConnectionHandle.as_mut_SQLHANDLE(),
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
pub fn SQLNativeSqlW<C: AsRawSlice<SQLWCHAR, SQLINTEGER>, MC: AsMutRawSlice<SQLWCHAR, SQLINTEGER>>(
    ConnectionHandle: &mut SQLHDBC,
    InStatementText: &C,
    OutStatementText: &mut MC,
    TextLength2Ptr: &mut MaybeUninit<SQLINTEGER>,
) -> SQLRETURN
where
    C: ?Sized,
{
    let InStatementText = InStatementText.as_raw_slice();
    let OutStatementText = OutStatementText.as_mut_raw_slice();

    unsafe {
        extern_api::SQLNativeSqlW(
            ConnectionHandle.as_mut_SQLHANDLE(),
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
pub fn SQLNumParams(
    StatementHandle: &mut SQLHSTMT,
    ParameterCountPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLNumParams(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLNumResultCols(
    StatementHandle: &mut SQLHSTMT,
    ColumnCountPtr: &mut MaybeUninit<SQLSMALLINT>,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLNumResultCols(StatementHandle.as_SQLHANDLE(), ColumnCountPtr.as_mut_ptr())
    }
}

///// Used together with **SQLPutData** to supply parameter data at statement execution time, and with **SQLGetData** to retrieve streamed output parameter data.
/////
///// For complete documentation on SQLParamData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlparamdata-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLParamData<PTR: AsMutRawSlice<()>>(StatementHandle: &mut SQLHSTMT, ValuePtrPtr: &mut PTR) -> SQLRETURN {
//    unsafe{ extern_api::SQLParamData(StatementHandle.as_mut_SQLHANDLE(), ValuePtrPtr.as_mut_ptr()) }
//}

/// Prepares an SQL string for execution.
///
/// For complete documentation on SQLPrepareA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLPrepareA<C: AsRawSlice<SQLCHAR, SQLINTEGER>>(
    StatementHandle: &mut SQLHSTMT,
    StatementText: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLPrepareA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLPrepareW<C: AsRawSlice<SQLWCHAR, SQLINTEGER>>(
    StatementHandle: &mut SQLHSTMT,
    StatementText: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let StatementText = StatementText.as_raw_slice();

    unsafe {
        extern_api::SQLPrepareW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLPrimaryKeysA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLPrimaryKeysA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLPrimaryKeysW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLPrimaryKeysW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLProcedureColumnsA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    ProcName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLProcedureColumnsA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLProcedureColumnsW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    ProcName: &C,
    ColumnName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();
    let ColumnName = ColumnName.as_raw_slice();

    unsafe {
        extern_api::SQLProcedureColumnsW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLProceduresA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    ProcName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();

    unsafe {
        extern_api::SQLProceduresA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLProceduresW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    ProcName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let ProcName = ProcName.as_raw_slice();

    unsafe {
        extern_api::SQLProceduresW(
            StatementHandle.as_mut_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            ProcName.0,
            ProcName.1,
        )
    }
}

///// Allows an application to send data for a parameter or column to the driver at statement execution time. This function can be used to send character or binary data values in parts to a column with a character, binary, or data source-specific data type (for example, parameters of the SQL_LONGVARBINARY or SQL_LONGVARCHAR types). **SQLPutData** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
/////
///// For complete documentation on SQLPutData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlputdata-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLPutData<PTR: AsRawSlice<SQLLEN>>(StatementHandle: &mut SQLHSTMT, DataPtr: PTR) -> SQLRETURN where C: ?Sized {
//    unsafe{ extern_api::SQLPutData(StatementHandle.as_mut_SQLHANDLE(), DataPtr.as_SQLPOINTER(), DataPtr.len()) }
//}

/// Returns the number of rows affected by an **UPDATE**, **INSERT**, or **DELETE** statement; an SQL_ADD, SQL_UPDATE_BY_BOOKMARK, or SQL_DELETE_BY_BOOKMARK operation in **SQLBulkOperations**; or an SQL_UPDATE or SQL_DELETE operation in **SQLSetPos**.
///
/// For complete documentation on SQLRowCount, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlrowcount-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case)]
pub fn SQLRowCount(
    StatementHandle: &mut SQLHSTMT,
    RowCountPtr: &mut MaybeUninit<SQLLEN>,
) -> SQLRETURN {
    unsafe { extern_api::SQLRowCount(StatementHandle.as_mut_SQLHANDLE(), RowCountPtr.as_mut_ptr()) }
}

/// Sets attributes that govern aspects of connections.
///
/// For complete documentation on SQLSetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetConnectAttrA<A: ConnAttr, T: AsSQLPOINTER>(
    ConnectionHandle: &mut SQLHDBC,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<AnsiType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
{
    unsafe {
        extern_api::SQLSetConnectAttrA(
            ConnectionHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_SQLPOINTER(),
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
pub fn SQLSetConnectAttrW<A: ConnAttr, T: AsSQLPOINTER>(
    ConnectionHandle: &mut SQLHDBC,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<UnicodeType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
{
    unsafe {
        extern_api::SQLSetConnectAttrW(
            ConnectionHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_SQLPOINTER(),
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
pub fn SQLSetCursorNameA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CursorName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CursorName = CursorName.as_raw_slice();

    unsafe {
        extern_api::SQLSetCursorNameA(
            StatementHandle.as_mut_SQLHANDLE(),
            CursorName.0,
            CursorName.1,
        )
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
pub fn SQLSetCursorNameW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CursorName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CursorName = CursorName.as_raw_slice();

    unsafe {
        extern_api::SQLSetCursorNameW(
            StatementHandle.as_mut_SQLHANDLE(),
            CursorName.0,
            CursorName.1,
        )
    }
}

///// Sets the value of a single field of a descriptor record.
/////
///// For complete documentation on SQLSetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLSetDescFieldA<A: DescField, T: AsSQLPOINTER>(DescriptorHandle: &mut SQLHDESC, RecNumber: SQLSMALLINT, FieldIdentifier: A, ValuePtr: &T) -> SQLRETURN
//where A: SetAttr<AnsiType, T>, T: Len<A::AttrType, SQLINTEGER> {
//    unsafe{ extern_api::SQLSetDescFieldA(DescriptorHandle.as_mut_SQLHANDLE(), RecNumber, A::IDENTIFIER, ValuePtr.as_SQLPOINTER(), ValuePtr.len()) }
//}
//
///// Sets the value of a single field of a descriptor record.
/////
///// For complete documentation on SQLSetDescFieldW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLSetDescFieldW<A: DescField, T: AsSQLPOINTER>(DescriptorHandle: &mut SQLHDESC, RecNumber: SQLSMALLINT, FieldIdentifier: A, ValuePtr: &T) -> SQLRETURN
//where A: SetAttr<UnicodeType, T>, T: Len<A::AttrType, SQLINTEGER> {
//    unsafe{ extern_api::SQLSetDescFieldW(DescriptorHandle.as_mut_SQLHANDLE(), RecNumber, A::IDENTIFIER, ValuePtr.as_SQLPOINTER(), ValuePtr.len()) }
//}

///// Sets multiple descriptor fields that affect the data type and buffer bound to a column or parameter data.
/////
///// For complete documentation on SQLSetDescRec, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescrec-function).
/////
///// # Returns
///// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
//#[inline]
//#[allow(non_snake_case)]
//pub fn SQLSetDescRec<PTR: AsMutRawSlice<SQLLEN>>(DescriptorHandle: &mut SQLHDESC, RecNumber: SQLSMALLINT, Type: SQLSMALLINT, SubType: SQLSMALLINT, Length: SQLLEN, Precision: SQLSMALLINT, Scale: SQLSMALLINT, DataPtr: PTR, IndicatorPtr: &mut MaybeUninit<SQLLEN>) -> SQLRETURN {
//    unsafe{ extern_api::SQLSetDescRec(DescriptorHandle.as_mut_SQLHANDLE(), RecNumber, Type, SubType, Length, Precision, Scale, DataPtr.as_mut_SQLPOINTER(), DataPtr.len(), IndicatorPtr.as_mut_ptr()) }
//}

/// Sets attributes that govern aspects of environments.
///
/// For complete documentation on SQLSetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetenvattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetEnvAttr<A: EnvAttr, T: AsSQLPOINTER>(
    EnvironmentHandle: &mut SQLHENV,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<AnsiType, T>,
    T: Len<OdbcAttr, SQLINTEGER>,
{
    unsafe {
        extern_api::SQLSetEnvAttr(
            EnvironmentHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_SQLPOINTER(),
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
pub fn SQLSetPos(
    StatementHandle: &mut SQLHSTMT,
    RowNumber: SQLSETPOSIROW,
    Operation: SQLUSMALLINT,
    LockType: SQLUSMALLINT,
) -> SQLRETURN {
    unsafe {
        extern_api::SQLSetPos(
            StatementHandle.as_mut_SQLHANDLE(),
            RowNumber,
            Operation,
            LockType,
        )
    }
}

/// Sets attributes related to a statement.
///
/// For complete documentation on SQLSetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetStmtAttrA<A: StmtAttr, T: AsSQLPOINTER>(
    StatementHandle: &mut SQLHSTMT,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<AnsiType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
{
    unsafe {
        extern_api::SQLSetStmtAttrA(
            StatementHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_SQLPOINTER(),
            ValuePtr.len(),
        )
    }
}

/// Sets attributes related to a statement.
///
/// For complete documentation on SQLSetStmtAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
///
/// # Returns
/// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
#[inline]
#[allow(non_snake_case, unused_variables)]
pub fn SQLSetStmtAttrW<A: StmtAttr, T: AsSQLPOINTER>(
    StatementHandle: &mut SQLHSTMT,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<UnicodeType, T>,
    T: Len<A::AttrType, SQLINTEGER>,
{
    unsafe {
        extern_api::SQLSetStmtAttrW(
            StatementHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.as_SQLPOINTER(),
            ValuePtr.len(),
        )
    }
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
pub fn SQLSpecialColumnsA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    IdentifierType: SQLSMALLINT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    Scope: SQLSMALLINT,
    Nullable: Nullable,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLSpecialColumnsA(
            StatementHandle.as_mut_SQLHANDLE(),
            IdentifierType,
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Scope,
            Nullable as SQLSMALLINT,
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
pub fn SQLSpecialColumnsW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    IdentifierType: SQLSMALLINT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    Scope: SQLSMALLINT,
    Nullable: Nullable,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLSpecialColumnsW(
            StatementHandle.as_mut_SQLHANDLE(),
            IdentifierType,
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Scope,
            Nullable as SQLSMALLINT,
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
pub fn SQLStatisticsA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    Unique: SQLUSMALLINT,
    Reserved: SQLUSMALLINT,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLStatisticsA(
            StatementHandle.as_mut_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Unique,
            Reserved,
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
pub fn SQLStatisticsW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
    Unique: SQLUSMALLINT,
    Reserved: SQLUSMALLINT,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLStatisticsW(
            StatementHandle.as_mut_SQLHANDLE(),
            CatalogName.0,
            CatalogName.1,
            SchemaName.0,
            SchemaName.1,
            TableName.0,
            TableName.1,
            Unique,
            Reserved,
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
pub fn SQLTablePrivilegesA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLTablePrivilegesA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLTablePrivilegesW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: &C,
    SchemaName: &C,
    TableName: &C,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.as_raw_slice();
    let SchemaName = SchemaName.as_raw_slice();
    let TableName = TableName.as_raw_slice();

    unsafe {
        extern_api::SQLTablePrivilegesW(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLTablesA<C: AsRawSlice<SQLCHAR, SQLSMALLINT>>(
    // TODO: Which can be NULL?
    StatementHandle: &mut SQLHSTMT,
    CatalogName: Option<&C>,
    SchemaName: Option<&C>,
    TableName: Option<&C>,
    TableType: Option<&C>,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let SchemaName = SchemaName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let TableName = TableName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let TableType = TableType.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());

    unsafe {
        extern_api::SQLTablesA(
            StatementHandle.as_mut_SQLHANDLE(),
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
pub fn SQLTablesW<C: AsRawSlice<SQLWCHAR, SQLSMALLINT>>(
    StatementHandle: &mut SQLHSTMT,
    CatalogName: Option<&C>,
    SchemaName: Option<&C>,
    TableName: Option<&C>,
    TableType: Option<&C>,
) -> SQLRETURN
where
    C: ?Sized,
{
    let CatalogName = CatalogName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let SchemaName = SchemaName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let TableName = TableName.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());
    let TableType = TableType.map_or((std::ptr::null(), 0), |x| x.as_raw_slice());

    unsafe {
        extern_api::SQLTablesW(
            StatementHandle.as_mut_SQLHANDLE(),
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
