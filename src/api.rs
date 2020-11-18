use std::ptr::NonNull;
use crate::{SQLRETURN, SQLSMALLINT, SQLUSMALLINT, SQLINTEGER, SQLPOINTER, SQLLEN, SQLULEN, SQLCHAR, SQLWCHAR, RETCODE, SQLSETPOSIROW, SQLHSTMT, SQLHANDLE, SQLHDBC, SQLHENV, SQLHDESC, SQLHWND};

extern "system" {

    /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
    pub fn SQLGetDiagFieldA(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        DiagIdentifier: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        DiagInfoPtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
    pub fn SQLGetDiagFieldW(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        DiagIdentifier: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        DiagInfoPtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Allocates an environment, connection, statement, or descriptor handle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlallochandle-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_INVALID_HANDLE, or SQL_ERROR.
    pub fn SQLAllocHandle(
        HandleType: SQLSMALLINT,
        InputHandle: SQLHANDLE,
        // _Out_
        OutputHandlePtr: NonNull<SQLHANDLE>,
    ) -> SQLRETURN;

    /// Binds application data buffers to columns in the result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLBindCol(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        TargetType: SQLSMALLINT,
        // _Inout_updates_opt_(_Inexpressible_(BufferLength))
        TargetValuePtr: SQLPOINTER,
        BufferLength: SQLLEN,
        // _Inout_opt_
        StrLen_or_Ind: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Binds a buffer to a parameter marker in an SQL statement. **SQLBindParameter** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindparameter-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLBindParameter(
        StatementHandle: SQLHSTMT,
        ParameterNumber: SQLUSMALLINT,
        InputOutputType: SQLSMALLINT,
        ValueType: SQLSMALLINT,
        ParameterType: SQLSMALLINT,
        ColumnSize: SQLULEN,
        DecimalDigits: SQLSMALLINT,
        ParameterValuePtr: SQLPOINTER,
        BufferLength: SQLLEN,
        StrLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLBrowseConnectA(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(StringLength1)
	// TODO: Unknown
        InConnectionString: NonNull<SQLCHAR>,
        StringLength1: SQLSMALLINT,
        // _Out_writes_opt_(BufferLength)
        OutConnectionString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLBrowseConnectW(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(StringLength1)
	// TODO: Unknown
        InConnectionString: NonNull<SQLWCHAR>,
        StringLength1: SQLSMALLINT,
        // _Out_writes_opt_(BufferLength)
        OutConnectionString: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbulkoperations-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLBulkOperations(StatementHandle: SQLHSTMT, Operation: SQLUSMALLINT) -> SQLRETURN;

    /// Cancels the processing on a statement.
    /// To cancel processing on a connection or statement, use SQLCancelHandle Function.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancel-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLCancel(StatementHandle: SQLHSTMT) -> SQLRETURN;

    /// Cancels the processing on a connection or statement. The Driver Manager maps a call to **SQLCancelHandle** to a call to **SQLCancel** when `HandleType` is SQL_HANDLE_STMT.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancelhandle-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[cfg(feature = "odbc_version_3_8")]
    pub fn SQLCancelHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

    /// Closes a cursor that has been opened on a statement and discards pending results.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlclosecursor-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLCloseCursor(StatementHandle: SQLHSTMT) -> SQLRETURN;

    /// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[cfg(target_pointer_width = "32")]
    pub fn SQLColAttributeA(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        CharacterAttributePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NumericAttributePtr: *mut SQLPOINTER,
    ) -> SQLRETURN;

    /// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[cfg(target_pointer_width = "64")]
    pub fn SQLColAttributeA(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        CharacterAttributePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NumericAttributePtr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[cfg(target_pointer_width = "32")]
    pub fn SQLColAttributeW(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        CharacterAttributePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NumericAttributePtr: *mut SQLPOINTER,
    ) -> SQLRETURN;

    /// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[cfg(target_pointer_width = "64")]
    pub fn SQLColAttributeW(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        CharacterAttributePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NumericAttributePtr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `StatementHandle`.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLColumnPrivilegesA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `StatementHandle`.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLColumnPrivilegesW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `StatementHandle`.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLColumnsA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `StatementHandle`.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLColumnsW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Can be used to determine when an asynchronous function is complete using either notification- or polling-based processing. For more information about asynchronous operations, see Asynchronous Execution.
    /// **SQLCompleteAsync** is only implemented in the ODBC Driver Manager.
    /// In notification based asynchronous processing mode, **SQLCompleteAsync** must be called after the Driver Manager raises the event object used for notification. **SQLCompleteAsync** completes the asynchronous processing and the asynchronous function will generate a return code.
    /// In polling based asynchronous processing mode, **SQLCompleteAsync** is an alternative to calling the original asynchronous function, without needing to specify the arguments in the original asynchronous function call. **SQLCompleteAsync** can be used regardless whether the ODBC Cursor Library is enabled.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcompleteasync-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    #[cfg(feature = "odbc_version_3_8")]
    pub fn SQLCompleteAsync(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        // _Out_
        AsyncRetCodePtr: NonNull<RETCODE>,
    ) -> SQLRETURN;

    /// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLConnectA(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(NameLength1)
        ServerName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_(NameLength2)
        UserName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_(NameLength3)
        Authentication: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLConnectW(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(NameLength1)
        ServerName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_(NameLength2)
        UserName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_(NameLength3)
        Authentication: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Copies descriptor information from one descriptor handle to another.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcopydesc-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLCopyDesc(SourceDescHandle: SQLHDESC, TargetDescHandle: SQLHDESC) -> SQLRETURN;

    /// Returns information about a data source. This function is implemented only by the Driver Manager.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDataSourcesA(
        EnvironmentHandle: SQLHENV,
        Direction: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength1)
        ServerName: *mut SQLCHAR,
        BufferLength1: SQLSMALLINT,
        // _Out_opt_
        NameLength1Ptr: *mut SQLSMALLINT,
        // _Out_writes_opt_(BufferLength2)
        Description: *mut SQLCHAR,
        BufferLength2: SQLSMALLINT,
        // _Out_opt_
        NameLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns information about a data source. This function is implemented only by the Driver Manager.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDataSourcesW(
        EnvironmentHandle: SQLHENV,
        Direction: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength1)
        ServerName: *mut SQLWCHAR,
        BufferLength1: SQLSMALLINT,
        // _Out_opt_
        NameLength1Ptr: *mut SQLSMALLINT,
        // _Out_writes_opt_(BufferLength2)
        Description: *mut SQLWCHAR,
        BufferLength2: SQLSMALLINT,
        // _Out_opt_
        NameLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the result descriptor - column name,type, column size, decimal digits, and nullability - for one column in the result set. This information also is available in the fields of the IRD.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribecol-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDescribeColA(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength)
        ColumnName: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        NameLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        DataTypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        ColumnSizePtr: *mut SQLULEN,
        // _Out_opt_
        DecimalDigitsPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the result descriptor - column name,type, column size, decimal digits, and nullability - for one column in the result set. This information also is available in the fields of the IRD.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribecol-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDescribeColW(
        StatementHandle: SQLHSTMT,
        ColumnNumber: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength)
        ColumnName: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        NameLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        DataTypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        ColumnSizePtr: *mut SQLULEN,
        // _Out_opt_
        DecimalDigitsPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the description of a parameter marker associated with a prepared SQL statement. This information is also available in the fields of the IPD.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribeparam-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDescribeParam(
        StatementHandle: SQLHSTMT,
        ParameterNumber: SQLUSMALLINT,
        // _Out_opt_
        DataTypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        ParameterSizePtr: *mut SQLULEN,
        // _Out_opt_
        DecimalDigitsPtr: *mut SQLSMALLINT,
        // _Out_opt_
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Closes the connection associated with a specific connection handle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldisconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLDisconnect(ConnectionHandle: SQLHDBC) -> SQLRETURN;

    /// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLDriverConnectA(
        ConnectionHandle: SQLHDBC,
        WindowHandle: SQLHWND,
        // _In_reads_(StringLength1)
	// TODO: Unknown
        InConnectionString: NonNull<SQLCHAR>,
        StringLength1: SQLSMALLINT,
        // _Out_writes_opt_(BufferLength)
        OutConnectionString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLength2Ptr: *mut SQLSMALLINT,
        DriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLDriverConnectW(
        ConnectionHandle: SQLHDBC,
        WindowHandle: SQLHWND,
        // _In_reads_(StringLength1)
	// TODO: Unknown
        InConnectionString: NonNull<SQLWCHAR>,
        StringLength1: SQLSMALLINT,
        // _Out_writes_opt_(BufferLength)
        OutConnectionString: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLength2Ptr: *mut SQLSMALLINT,
        DriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDriversA(
        EnvironmentHandle: SQLHENV,
        Direction: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength1)
        DriverDescription: *mut SQLCHAR,
        BufferLength1: SQLSMALLINT,
        // _Out_opt_
        DescriptionLengthPtr: *mut SQLSMALLINT,
        // _Out_writes_opt_(BufferLength2)
        DriverAttributes: *mut SQLCHAR,
        BufferLength2: SQLSMALLINT,
        // _Out_opt_
        AttributesLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLDriversW(
        EnvironmentHandle: SQLHENV,
        Direction: SQLUSMALLINT,
        // _Out_writes_opt_(BufferLength1)
        DriverDescription: *mut SQLWCHAR,
        BufferLength1: SQLSMALLINT,
        // _Out_opt_
        DescriptionLengthPtr: *mut SQLSMALLINT,
        // _Out_writes_opt_(BufferLength2)
        DriverAttributes: *mut SQLWCHAR,
        BufferLength2: SQLSMALLINT,
        // _Out_opt_
        AttributesLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Requests a commit or rollback operation for all active operations on all statements associated with a connection. **SQLEndTran** can also request that a commit or rollback operation be performed for all connections associated with an environment.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlendtran-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLEndTran(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        CompletionType: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    pub fn SQLExecDirectA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(TextLength)
        StatementText: *mut SQLCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    pub fn SQLExecDirectW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(TextLength)
        StatementText: *mut SQLWCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    pub fn SQLExecute(StatementHandle: SQLHSTMT) -> SQLRETURN;

    /// Fetches the next rowset of data from the result set and returns data for all bound columns.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLFetch(StatementHandle: SQLHSTMT) -> SQLRETURN;

    /// Fetches the specified rowset of data from the result set and returns data for all bound columns. Rowsets can be specified at an absolute or relative position or by bookmark.
    /// When working with an ODBC 2.x driver, the Driver Manager maps this function to **SQLExtendedFetch**. For more information, see Mapping Replacement Functions for Backward Compatibility of Applications.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetchscroll-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLFetchScroll(
        StatementHandle: SQLHSTMT,
        FetchOrientation: SQLSMALLINT,
        FetchOffset: SQLLEN,
    ) -> SQLRETURN;

    /// Can return:
    ///
    /// * A list of foreign keys in the specified table (columns in the specified table that refer to primary keys in other tables).
    /// * A list of foreign keys in other tables that refer to the primary key in the specified table.
    ///
    /// The driver returns each list as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlforeignkeys-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLForeignKeysA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        PKCatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        PKSchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        PKTableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        FKCatalogName: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
        // _In_reads_opt_(NameLength5)
        FKSchemaName: *mut SQLCHAR,
        NameLength5: SQLSMALLINT,
        // _In_reads_opt_(NameLength6)
        FKTableName: *mut SQLCHAR,
        NameLength6: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Can return:
    ///
    /// * A list of foreign keys in the specified table (columns in the specified table that refer to primary keys in other tables).
    /// * A list of foreign keys in other tables that refer to the primary key in the specified table.
    ///
    /// The driver returns each list as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlforeignkeys-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLForeignKeysW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        PKCatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        PKSchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        PKTableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        FKCatalogName: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
        // _In_reads_opt_(NameLength5)
        FKSchemaName: *mut SQLWCHAR,
        NameLength5: SQLSMALLINT,
        // _In_reads_opt_(NameLength6)
        FKTableName: *mut SQLWCHAR,
        NameLength6: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLFreeHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

    /// Stops processing associated with a specific statement, closes any open cursors associated with the statement, discards pending results, or, optionally, frees all resources associated with the statement handle.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreestmt-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLFreeStmt(StatementHandle: SQLHSTMT, Option: SQLUSMALLINT) -> SQLRETURN;

    /// Returns the current setting of a connection attribute.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetConnectAttrA(
        ConnectionHandle: SQLHDBC,
        Attribute: SQLINTEGER,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the current setting of a connection attribute.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetConnectAttrW(
        ConnectionHandle: SQLHDBC,
        Attribute: SQLINTEGER,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the cursor name associated with a specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetCursorNameA(
        StatementHandle: SQLHSTMT,
        // _Out_writes_opt_(BufferLength)
        CursorName: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        NameLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the cursor name associated with a specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetCursorNameW(
        StatementHandle: SQLHSTMT,
        // _Out_writes_opt_(BufferLength)
        CursorName: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        NameLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Retrieves data for a single column in the result set or for a single parameter after **SQLParamData** returns SQL_PARAM_DATA_AVAILABLE. It can be called multiple times to retrieve variable-length data in parts.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdata-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetData(
        StatementHandle: SQLHSTMT,
        Col_or_Param_Num: SQLUSMALLINT,
        TargetType: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        TargetValuePtr: SQLPOINTER,
        BufferLength: SQLLEN,
        // _Out_opt_
        StrLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Returns the current setting or value of a single field of a descriptor record.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    pub fn SQLGetDescFieldA(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the current setting or value of a single field of a descriptor record.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    pub fn SQLGetDescFieldW(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the current settings or values of multiple fields of a descriptor record. The fields returned describe the name, data type, and storage of column or parameter data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescrec-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    pub fn SQLGetDescRecA(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        Name: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        TypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        SubTypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        LengthPtr: *mut SQLLEN,
        // _Out_opt_
        PrecisionPtr: *mut SQLSMALLINT,
        // _Out_opt_
        ScalePtr: *mut SQLSMALLINT,
        // _Out_opt_
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current settings or values of multiple fields of a descriptor record. The fields returned describe the name, data type, and storage of column or parameter data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescrec-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    pub fn SQLGetDescRecW(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        Name: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
        // _Out_opt_
        TypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        SubTypePtr: *mut SQLSMALLINT,
        // _Out_opt_
        LengthPtr: *mut SQLLEN,
        // _Out_opt_
        PrecisionPtr: *mut SQLSMALLINT,
        // _Out_opt_
        ScalePtr: *mut SQLSMALLINT,
        // _Out_opt_
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
    pub fn SQLGetDiagField(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        DiagIdentifier: SQLSMALLINT,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        DiagInfoPtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetDiagRecA(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        // _Out_writes_opt_(6)
        SQLState: *mut SQLCHAR,
        NativeErrorPtr: *mut SQLINTEGER,
        // _Out_writes_opt_(BufferLength)
        MessageText: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        TextLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current values of multiple fields of a diagnostic record that contains error, warning, and status information. Unlike **SQLGetDiagField**, which returns one diagnostic field per call, **SQLGetDiagRec** returns several commonly used fields of a diagnostic record, including the SQLSTATE, the native error code, and the diagnostic message text.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagrec-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetDiagRecW(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        // _Out_writes_opt_(6)
        SQLState: *mut SQLWCHAR,
        NativeErrorPtr: *mut SQLINTEGER,
        // _Out_writes_opt_(BufferLength)
        MessageText: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        TextLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current setting of an environment attribute.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetenvattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetEnvAttr(
        EnvironmentHandle: SQLHENV,
        Attribute: SQLINTEGER,
        // _Out_writes_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns information about whether a driver supports a specific ODBC function. This function is implemented in the Driver Manager; it can also be implemented in drivers. If a driver implements **SQLGetFunctions**, the Driver Manager calls the function in the driver. Otherwise, it executes the function itself.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetfunctions-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetFunctions(
        ConnectionHandle: SQLHDBC,
        FunctionId: SQLUSMALLINT,
        // _Out_writes_opt_(_Inexpressible_("Buffer length pfExists points to depends on fFunction value."))
        SupportedPtr: *mut SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Returns general information about the driver and data source associated with a connection.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    // _Success_(return == SQL_SUCCESS)
    pub fn SQLGetInfoA(
        ConnectionHandle: SQLHDBC,
        InfoType: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        InfoValuePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns general information about the driver and data source associated with a connection.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetinfo-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    // _Success_(return == SQL_SUCCESS)
    pub fn SQLGetInfoW(
        ConnectionHandle: SQLHDBC,
        InfoType: SQLUSMALLINT,
        // _Out_writes_bytes_opt_(BufferLength)
        InfoValuePtr: SQLPOINTER,
        BufferLength: SQLSMALLINT,
        // _Out_opt_
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current setting of a statement attribute.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetStmtAttrA(
        StatementHandle: SQLHSTMT,
        Attribute: SQLINTEGER,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the current setting of a statement attribute.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetStmtAttrW(
        StatementHandle: SQLHSTMT,
        Attribute: SQLINTEGER,
        // _Out_writes_opt_(_Inexpressible_(BufferLength))
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        // _Out_opt_
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetTypeInfoA(StatementHandle: SQLHSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

    /// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLGetTypeInfoW(StatementHandle: SQLHSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

    /// Determines whether more results are available on a statement containing **SELECT**, **UPDATE**, **INSERT**, or **DELETE** statements and, if so, initializes processing for those results.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlmoreresults-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, OR SQL_PARAM_DATA_AVAILABLE.
    pub fn SQLMoreResults(StatementHandle: SQLHSTMT) -> SQLRETURN;

    /// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLNativeSqlA(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(TextLength1)
        InStatementText: NonNull<SQLCHAR>,
        TextLength1: SQLINTEGER,
        // _Out_writes_opt_(BufferLength)
        OutStatementText: *mut SQLCHAR,
        BufferLength: SQLINTEGER,
        TextLength2Ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLNativeSqlW(
        ConnectionHandle: SQLHDBC,
        // _In_reads_(TextLength1)
        InStatementText: NonNull<SQLWCHAR>,
        TextLength1: SQLINTEGER,
        // _Out_writes_opt_(BufferLength)
        OutStatementText: *mut SQLWCHAR,
        BufferLength: SQLINTEGER,
        TextLength2Ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the number of parameters in an SQL statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumparams-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLNumParams(
        StatementHandle: SQLHSTMT,
        // _Out_opt_
        ParameterCountPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the number of columns in a result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumresultcols-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLNumResultCols(
        StatementHandle: SQLHSTMT,
        // _Out_
	// TODO: Unknown
        ColumnCountPtr: NonNull<SQLSMALLINT>,
    ) -> SQLRETURN;

    /// Used together with **SQLPutData** to supply parameter data at statement execution time, and with **SQLGetData** to retrieve streamed output parameter data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlparamdata-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    pub fn SQLParamData(
        StatementHandle: SQLHSTMT,
        // _Out_
	// TODO: Unknown
        ValuePtrPtr: NonNull<SQLPOINTER>,
    ) -> SQLRETURN;

    /// Prepares an SQL string for execution.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLPrepareA(
        StatementHandle: SQLHSTMT,
        // _In_reads_(TextLength)
        StatementText: NonNull<SQLCHAR>,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Prepares an SQL string for execution.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLPrepareW(
        StatementHandle: SQLHSTMT,
        // _In_reads_(TextLength)
        StatementText: NonNull<SQLWCHAR>,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLPrimaryKeysA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLPrimaryKeysW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of input and output parameters, as well as the columns that make up the result set for the specified procedures. The driver returns the information as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedurecolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLProcedureColumnsA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        ProcName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of input and output parameters, as well as the columns that make up the result set for the specified procedures. The driver returns the information as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedurecolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLProcedureColumnsW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        ProcName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of procedure names stored in a specific data source. `Procedure` is a generic term used to describe an `executable object`, or a named entity that can be invoked using input and output parameters. For more information on procedures, see the Procedures.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedures-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLProceduresA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        ProcName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of procedure names stored in a specific data source. `Procedure` is a generic term used to describe an `executable object`, or a named entity that can be invoked using input and output parameters. For more information on procedures, see the Procedures.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprocedures-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLProceduresW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        ProcName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Allows an application to send data for a parameter or column to the driver at statement execution time. This function can be used to send character or binary data values in parts to a column with a character, binary, or data source-specific data type (for example, parameters of the SQL_LONGVARBINARY or SQL_LONGVARCHAR types). **SQLPutData** supports binding to a Unicode C data type, even if the underlying driver does not support Unicode data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlputdata-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLPutData(
        StatementHandle: SQLHSTMT,
        // _In_reads_(_Inexpressible_(StrLen_or_Ind))
        DataPtr: SQLPOINTER,
        StrLen_or_Ind: SQLLEN,
    ) -> SQLRETURN;

    /// Returns the number of rows affected by an **UPDATE**, **INSERT**, or **DELETE** statement; an SQL_ADD, SQL_UPDATE_BY_BOOKMARK, or SQL_DELETE_BY_BOOKMARK operation in **SQLBulkOperations**; or an SQL_UPDATE or SQL_DELETE operation in **SQLSetPos**.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlrowcount-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLRowCount(
        // _In_
	// TODO
        StatementHandle: SQLHSTMT,
        // _Out_
	// TODO: Unknown
        RowCountPtr: NonNull<SQLLEN>,
    ) -> SQLRETURN;

    /// Sets attributes that govern aspects of connections.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLSetConnectAttrA(
        ConnectionHandle: SQLHDBC,
        Attribute: SQLINTEGER,
        // _In_reads_bytes_opt_(StringLength)
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets attributes that govern aspects of connections.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    pub fn SQLSetConnectAttrW(
        ConnectionHandle: SQLHDBC,
        Attribute: SQLINTEGER,
        // _In_reads_bytes_opt_(StringLength)
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetCursorNameA(
        StatementHandle: SQLHSTMT,
        // _In_reads_(NameLength)
        CursorName: NonNull<SQLCHAR>,
        NameLength: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetCursorNameW(
        StatementHandle: SQLHSTMT,
        // _In_reads_(NameLength)
        CursorName: NonNull<SQLWCHAR>,
        NameLength: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Sets the value of a single field of a descriptor record.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetDescFieldA(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        // _In_reads_(_Inexpressible_(BufferLength))
	// TODO: Unknown
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets the value of a single field of a descriptor record.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetDescFieldW(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        // _In_reads_(_Inexpressible_(BufferLength))
	// TODO: Unknown
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets multiple descriptor fields that affect the data type and buffer bound to a column or parameter data.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescrec-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetDescRec(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        Type: SQLSMALLINT,
        SubType: SQLSMALLINT,
        Length: SQLLEN,
        Precision: SQLSMALLINT,
        Scale: SQLSMALLINT,
        // _Inout_updates_bytes_opt_(Length)
        DataPtr: SQLPOINTER,
        // _Inout_opt_
        StringLengthPtr: *mut SQLLEN,
        // _Inout_opt_
        IndicatorPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Sets attributes that govern aspects of environments.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetenvattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetEnvAttr(
        EnvironmentHandle: SQLHENV,
        Attribute: SQLINTEGER,
        // _In_reads_bytes_opt_(StringLength)
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets the cursor position in a rowset and allows an application to refresh data in the rowset or to update or delete data in the result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetpos-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetPos(
        StatementHandle: SQLHSTMT,
        RowNumber: SQLSETPOSIROW,
        Operation: SQLUSMALLINT,
        LockType: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Sets attributes related to a statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetStmtAttrA(
        StatementHandle: SQLHSTMT,
        Attribute: SQLINTEGER,
        // _In_reads_(_Inexpressible_(StringLength))
	// TODO: Unknown
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets attributes related to a statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSetStmtAttrW(
        StatementHandle: SQLHSTMT,
        Attribute: SQLINTEGER,
        // _In_reads_(_Inexpressible_(StringLength))
	// TODO: Unknown
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    /// Retrieves the following information about columns within a specified table:
    ///
    /// * The optimal set of columns that uniquely identifies a row in the table.
    /// * Columns that are automatically updated when any value in the row is updated by a transaction.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlspecialcolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSpecialColumnsA(
        StatementHandle: SQLHSTMT,
        IdentifierType: SQLSMALLINT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        Scope: SQLSMALLINT,
        Nullable: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Retrieves the following information about columns within a specified table:
    ///
    /// * The optimal set of columns that uniquely identifies a row in the table.
    /// * Columns that are automatically updated when any value in the row is updated by a transaction.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlspecialcolumns-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLSpecialColumnsW(
        StatementHandle: SQLHSTMT,
        IdentifierType: SQLSMALLINT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        Scope: SQLSMALLINT,
        Nullable: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Retrieves a list of statistics about a single table and the indexes associated with the table. The driver returns the information as a result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlstatistics-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLStatisticsA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        Unique: SQLUSMALLINT,
        Reserved: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Retrieves a list of statistics about a single table and the indexes associated with the table. The driver returns the information as a result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlstatistics-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLStatisticsW(
        StatementHandle: SQLHSTMT,
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        Unique: SQLUSMALLINT,
        Reserved: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Returns a list of tables and the privileges associated with each table. The driver returns the information as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltableprivileges-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLTablePrivilegesA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns a list of tables and the privileges associated with each table. The driver returns the information as a result set on the specified statement.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltableprivileges-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLTablePrivilegesW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of table, catalog, or schema names, and table types, stored in a specific data source. The driver returns the information as a result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltables-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLTablesA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        TableName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        TableType: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of table, catalog, or schema names, and table types, stored in a specific data source. The driver returns the information as a result set.
    ///
    /// # Documentation
    /// https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqltables-function
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    pub fn SQLTablesW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TableName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        TableType: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Retrieves the column of data currently available to be read.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLNextColumn(
        StatementHandle: SQLHSTMT,
        // _Out_
        Col_or_Param_Num: SQLUSMALLINT,
    ) -> SQLRETURN;

    /// Is called in to retrieve a handle for reading or writing structured or collection-valued columns and parameters.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLGetNestedHandle(
        ParentStatementHandle: SQLHSTMT,
        Col_or_Param_Num: SQLUSMALLINT,
        // _Out_
        OutputChildStatementHandle: NonNull<SQLHSTMT>,
    ) -> SQLRETURN;

    /// Enumerates named structural types.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLStructuredTypesA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TypeName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Enumerates named structural types.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLStructuredTypesW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TypeName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Describes the columns of a named structural type.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLStructuredTypeColumnsA(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TypeName: *mut SQLCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Describes the columns of a named structural type.
    ///
    /// # Documentation
    /// https://github.com/microsoft/ODBC-Specification/blob/master/ODBC%204.0.md
    ///
    /// # Returns
    ///
    #[cfg(feature = "odbc_version_4")]
    pub fn SQLStructuredTypeColumnsW(
        StatementHandle: SQLHSTMT,
        // _In_reads_opt_(NameLength1)
        CatalogName: *mut SQLWCHAR,
        NameLength1: SQLSMALLINT,
        // _In_reads_opt_(NameLength2)
        SchemaName: *mut SQLWCHAR,
        NameLength2: SQLSMALLINT,
        // _In_reads_opt_(NameLength3)
        TypeName: *mut SQLWCHAR,
        NameLength3: SQLSMALLINT,
        // _In_reads_opt_(NameLength4)
        ColumnName: *mut SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;
}
