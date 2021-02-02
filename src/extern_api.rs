use crate::handle::{HDBC, HDESC, HENV, HSTMT, SQLHWND};
use crate::{
    RETCODE, SQLCHAR, SQLHANDLE, SQLINTEGER, SQLLEN, SQLRETURN, SQLSETPOSIROW, SQLSMALLINT,
    SQLULEN, SQLUSMALLINT, SQLWCHAR,
};

// TODO: Replace these two types with SQLPOINTER once library is stabilized
// they are used to avoid provenance related errors during initial development
type ConstSQLPOINTER = *const std::ffi::c_void;
type MutSQLPOINTER = *mut std::ffi::c_void;

// TODO static linking is not currently supported here for windows
#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(all(not(windows), not(r#static)), link(name = "odbc"))]
#[cfg_attr(all(not(windows), r#static), link(name = "odbc", kind = "static"))]
extern "system" {
    pub(crate) fn SQLAllocHandle(
        HandleType: SQLSMALLINT,
        InputHandle: SQLHANDLE,
        OutputHandlePtr: *mut SQLHANDLE,
    ) -> SQLRETURN;

    pub(crate) fn SQLBindCol(
        StatementHandle: HSTMT,
        ColumnNumber: SQLUSMALLINT,
        TargetType: SQLSMALLINT,
        TargetValuePtr: MutSQLPOINTER,
        BufferLength: SQLLEN,
        StrLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLBindParameter(
        StatementHandle: HSTMT,
        ParameterNumber: SQLUSMALLINT,
        InputOutputType: SQLSMALLINT,
        ValueType: SQLSMALLINT,
        ParameterType: SQLSMALLINT,
        ColumnSize: SQLULEN,
        DecimalDigits: SQLSMALLINT,
        ParameterValuePtr: MutSQLPOINTER,
        BufferLength: SQLLEN,
        StrLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLBrowseConnectA(
        ConnectionHandle: HDBC,
        InConnectionString: *const SQLCHAR,
        StringLength1: SQLSMALLINT,
        OutConnectionString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLBrowseConnectW(
        ConnectionHandle: HDBC,
        InConnectionString: *const SQLWCHAR,
        StringLength1: SQLSMALLINT,
        OutConnectionString: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        StringLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLBulkOperations(StatementHandle: HSTMT, Operation: SQLUSMALLINT) -> SQLRETURN;

    pub(crate) fn SQLCancel(StatementHandle: HSTMT) -> SQLRETURN;

    #[cfg(feature = "v3_8")]
    pub(crate) fn SQLCancelHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

    pub(crate) fn SQLCloseCursor(StatementHandle: HSTMT) -> SQLRETURN;

    pub(crate) fn SQLColAttributeA(
        StatementHandle: HSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        CharacterAttributePtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
        NumericAttributePtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLColAttributeW(
        StatementHandle: HSTMT,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: SQLUSMALLINT,
        CharacterAttributePtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
        NumericAttributePtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLColumnPrivilegesA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLColumnPrivilegesW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLColumnsA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLColumnsW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    #[cfg(feature = "v3_8")]
    pub(crate) fn SQLCompleteAsync(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        AsyncRetCodePtr: *mut RETCODE,
    ) -> SQLRETURN;

    pub(crate) fn SQLConnectA(
        ConnectionHandle: HDBC,
        ServerName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        UserName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        Authentication: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLConnectW(
        ConnectionHandle: HDBC,
        ServerName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        UserName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        Authentication: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLCopyDesc(SourceDescHandle: HDESC, TargetDescHandle: HDESC) -> SQLRETURN;

    pub(crate) fn SQLDataSourcesA(
        EnvironmentHandle: HENV,
        Direction: SQLUSMALLINT,
        ServerName: *mut SQLCHAR,
        BufferLength1: SQLSMALLINT,
        NameLength1Ptr: *mut SQLSMALLINT,
        Description: *mut SQLCHAR,
        BufferLength2: SQLSMALLINT,
        NameLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDataSourcesW(
        EnvironmentHandle: HENV,
        Direction: SQLUSMALLINT,
        ServerName: *mut SQLWCHAR,
        BufferLength1: SQLSMALLINT,
        NameLength1Ptr: *mut SQLSMALLINT,
        Description: *mut SQLWCHAR,
        BufferLength2: SQLSMALLINT,
        NameLength2Ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDescribeColA(
        StatementHandle: HSTMT,
        ColumnNumber: SQLUSMALLINT,
        ColumnName: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        NameLengthPtr: *mut SQLSMALLINT,
        DataTypePtr: *mut SQLSMALLINT,
        ColumnSizePtr: *mut SQLULEN,
        DecimalDigitsPtr: *mut SQLSMALLINT,
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDescribeColW(
        StatementHandle: HSTMT,
        ColumnNumber: SQLUSMALLINT,
        ColumnName: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        NameLengthPtr: *mut SQLSMALLINT,
        DataTypePtr: *mut SQLSMALLINT,
        ColumnSizePtr: *mut SQLULEN,
        DecimalDigitsPtr: *mut SQLSMALLINT,
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDescribeParam(
        StatementHandle: HSTMT,
        ParameterNumber: SQLUSMALLINT,
        DataTypePtr: *mut SQLSMALLINT,
        ParameterSizePtr: *mut SQLULEN,
        DecimalDigitsPtr: *mut SQLSMALLINT,
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDisconnect(ConnectionHandle: HDBC) -> SQLRETURN;

    pub(crate) fn SQLDriverConnectA(
        ConnectionHandle: HDBC,
        WindowHandle: SQLHWND,
        InConnectionString: *const SQLCHAR,
        StringLength1: SQLSMALLINT,
        OutConnectionString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLength2Ptr: *mut SQLSMALLINT,
        DriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDriverConnectW(
        ConnectionHandle: HDBC,
        WindowHandle: SQLHWND,
        InConnectionString: *const SQLWCHAR,
        StringLength1: SQLSMALLINT,
        OutConnectionString: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        StringLength2Ptr: *mut SQLSMALLINT,
        DriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDriversA(
        EnvironmentHandle: HENV,
        Direction: SQLUSMALLINT,
        DriverDescription: *mut SQLCHAR,
        BufferLength1: SQLSMALLINT,
        DescriptionLengthPtr: *mut SQLSMALLINT,
        DriverAttributes: *mut SQLCHAR,
        BufferLength2: SQLSMALLINT,
        AttributesLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLDriversW(
        EnvironmentHandle: HENV,
        Direction: SQLUSMALLINT,
        DriverDescription: *mut SQLWCHAR,
        BufferLength1: SQLSMALLINT,
        DescriptionLengthPtr: *mut SQLSMALLINT,
        DriverAttributes: *mut SQLWCHAR,
        BufferLength2: SQLSMALLINT,
        AttributesLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLEndTran(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        CompletionType: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLExecDirectA(
        StatementHandle: HSTMT,
        StatementText: *const SQLCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLExecDirectW(
        StatementHandle: HSTMT,
        StatementText: *const SQLWCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLExecute(StatementHandle: HSTMT) -> SQLRETURN;

    pub(crate) fn SQLFetch(StatementHandle: HSTMT) -> SQLRETURN;

    pub(crate) fn SQLFetchScroll(
        StatementHandle: HSTMT,
        FetchOrientation: SQLSMALLINT,
        FetchOffset: SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLForeignKeysA(
        StatementHandle: HSTMT,
        PKCatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        PKSchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        PKTableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        FKCatalogName: *const SQLCHAR,
        NameLength4: SQLSMALLINT,
        FKSchemaName: *const SQLCHAR,
        NameLength5: SQLSMALLINT,
        FKTableName: *const SQLCHAR,
        NameLength6: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLForeignKeysW(
        StatementHandle: HSTMT,
        PKCatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        PKSchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        PKTableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        FKCatalogName: *const SQLWCHAR,
        NameLength4: SQLSMALLINT,
        FKSchemaName: *const SQLWCHAR,
        NameLength5: SQLSMALLINT,
        FKTableName: *const SQLWCHAR,
        NameLength6: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLFreeHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

    pub(crate) fn SQLFreeStmt(StatementHandle: HSTMT, Option: SQLUSMALLINT) -> SQLRETURN;

    pub(crate) fn SQLGetConnectAttrA(
        ConnectionHandle: HDBC,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetConnectAttrW(
        ConnectionHandle: HDBC,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetCursorNameA(
        StatementHandle: HSTMT,
        CursorName: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        NameLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetCursorNameW(
        StatementHandle: HSTMT,
        CursorName: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        NameLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetData(
        StatementHandle: HSTMT,
        Col_or_Param_Num: SQLUSMALLINT,
        TargetType: SQLSMALLINT,
        TargetValuePtr: MutSQLPOINTER,
        BufferLength: SQLLEN,
        StrLen_or_IndPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDescFieldA(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDescFieldW(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDescRecA(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        Name: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
        TypePtr: *mut SQLSMALLINT,
        SubTypePtr: *mut SQLSMALLINT,
        LengthPtr: *mut SQLLEN,
        PrecisionPtr: *mut SQLSMALLINT,
        ScalePtr: *mut SQLSMALLINT,
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDescRecW(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        Name: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
        TypePtr: *mut SQLSMALLINT,
        SubTypePtr: *mut SQLSMALLINT,
        LengthPtr: *mut SQLLEN,
        PrecisionPtr: *mut SQLSMALLINT,
        ScalePtr: *mut SQLSMALLINT,
        NullablePtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDiagFieldA(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        DiagIdentifier: SQLSMALLINT,
        DiagInfoPtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDiagFieldW(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        DiagIdentifier: SQLSMALLINT,
        DiagInfoPtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDiagRecA(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        SQLState: *mut [SQLCHAR; 6],
        NativeErrorPtr: *mut SQLINTEGER,
        MessageText: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        TextLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetDiagRecW(
        HandleType: SQLSMALLINT,
        Handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        SQLState: *mut [SQLWCHAR; 6],
        NativeErrorPtr: *mut SQLINTEGER,
        MessageText: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        TextLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetEnvAttr(
        EnvironmentHandle: HENV,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetFunctions(
        ConnectionHandle: HDBC,
        FunctionId: SQLUSMALLINT,
        SupportedPtr: *mut SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetInfoA(
        ConnectionHandle: HDBC,
        InfoType: SQLUSMALLINT,
        InfoValuePtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetInfoW(
        ConnectionHandle: HDBC,
        InfoType: SQLUSMALLINT,
        InfoValuePtr: MutSQLPOINTER,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetStmtAttrA(
        StatementHandle: HSTMT,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetStmtAttrW(
        StatementHandle: HSTMT,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLGetTypeInfoA(StatementHandle: HSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

    pub(crate) fn SQLGetTypeInfoW(StatementHandle: HSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

    pub(crate) fn SQLMoreResults(StatementHandle: HSTMT) -> SQLRETURN;

    pub(crate) fn SQLNativeSqlA(
        ConnectionHandle: HDBC,
        InStatementText: *const SQLCHAR,
        TextLength1: SQLINTEGER,
        OutStatementText: *mut SQLCHAR,
        BufferLength: SQLINTEGER,
        TextLength2Ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLNativeSqlW(
        ConnectionHandle: HDBC,
        InStatementText: *const SQLWCHAR,
        TextLength1: SQLINTEGER,
        OutStatementText: *mut SQLWCHAR,
        BufferLength: SQLINTEGER,
        TextLength2Ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLNumParams(
        StatementHandle: HSTMT,
        ParameterCountPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLNumResultCols(
        StatementHandle: HSTMT,
        ColumnCountPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLParamData(
        StatementHandle: HSTMT,
        ValuePtrPtr: *mut MutSQLPOINTER,
    ) -> SQLRETURN;

    pub(crate) fn SQLPrepareA(
        StatementHandle: HSTMT,
        StatementText: *const SQLCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLPrepareW(
        StatementHandle: HSTMT,
        StatementText: *const SQLWCHAR,
        TextLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLPrimaryKeysA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLPrimaryKeysW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLProcedureColumnsA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        ProcName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLProcedureColumnsW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        ProcName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        ColumnName: *const SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLProceduresA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        ProcName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLProceduresW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        ProcName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLPutData(
        StatementHandle: HSTMT,
        DataPtr: ConstSQLPOINTER,
        StrLen_or_Ind: SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLRowCount(StatementHandle: HSTMT, RowCountPtr: *mut SQLLEN) -> SQLRETURN;

    pub(crate) fn SQLSetConnectAttrA(
        ConnectionHandle: HDBC,
        Attribute: SQLINTEGER,
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetConnectAttrW(
        ConnectionHandle: HDBC,
        Attribute: SQLINTEGER,
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetCursorNameA(
        StatementHandle: HSTMT,
        CursorName: *const SQLCHAR,
        NameLength: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetCursorNameW(
        StatementHandle: HSTMT,
        CursorName: *const SQLWCHAR,
        NameLength: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetDescFieldA(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        ValuePtr: ConstSQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetDescFieldW(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        ValuePtr: ConstSQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetDescRec(
        DescriptorHandle: HDESC,
        RecNumber: SQLSMALLINT,
        Type: SQLSMALLINT,
        SubType: SQLSMALLINT,
        Length: SQLLEN,
        Precision: SQLSMALLINT,
        Scale: SQLSMALLINT,
        DataPtr: MutSQLPOINTER,
        StringLengthPtr: *mut SQLLEN,
        IndicatorPtr: *mut SQLLEN,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetEnvAttr(
        EnvironmentHandle: HENV,
        Attribute: SQLINTEGER,
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetPos(
        StatementHandle: HSTMT,
        RowNumber: SQLSETPOSIROW,
        Operation: SQLUSMALLINT,
        LockType: SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetStmtAttrA(
        StatementHandle: HSTMT,
        Attribute: SQLINTEGER,
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSetStmtAttrW(
        StatementHandle: HSTMT,
        Attribute: SQLINTEGER,
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    pub(crate) fn SQLSpecialColumnsA(
        StatementHandle: HSTMT,
        IdentifierType: SQLSMALLINT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        Scope: SQLSMALLINT,
        Nullable: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLSpecialColumnsW(
        StatementHandle: HSTMT,
        IdentifierType: SQLSMALLINT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        Scope: SQLSMALLINT,
        Nullable: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLStatisticsA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        Unique: SQLUSMALLINT,
        Reserved: SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLStatisticsW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        Unique: SQLUSMALLINT,
        Reserved: SQLUSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLTablePrivilegesA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLTablePrivilegesW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLTablesA(
        StatementHandle: HSTMT,
        CatalogName: *const SQLCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLCHAR,
        NameLength3: SQLSMALLINT,
        TableType: *const SQLCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;

    pub(crate) fn SQLTablesW(
        StatementHandle: HSTMT,
        CatalogName: *const SQLWCHAR,
        NameLength1: SQLSMALLINT,
        SchemaName: *const SQLWCHAR,
        NameLength2: SQLSMALLINT,
        TableName: *const SQLWCHAR,
        NameLength3: SQLSMALLINT,
        TableType: *const SQLWCHAR,
        NameLength4: SQLSMALLINT,
    ) -> SQLRETURN;
}
