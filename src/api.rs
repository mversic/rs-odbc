use crate::handle::*;
use crate::{
    attr::{AttrGet, AttrSet, StrLen},
    c_types::CData,
    c_types::DeferredBuf,
    col::ColAttr,
    conn::{BrowseConnect, ConnAttr, ConnState, Disconnect, C2, C3, C4},
    convert::{
        AsMutPtr, AsMutRawSlice, AsMutSQLPOINTER, AsRawSlice, AsSQLHANDLE, AsSQLPOINTER,
        IntoSQLPOINTER,
    },
    desc::{AppDesc, DescField, DescType, IPD, IRD},
    diag::{DiagField, SQLSTATE},
    env::{EnvAttr, OdbcVersion, SQL_OV_ODBC3_80, SQL_OV_ODBC4},
    handle::{RefSQLHDESC, UnsafeSQLHSTMT, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQL_HANDLE_STMT},
    info::InfoType,
    sql_types::SqlType,
    sqlreturn::{SQLRETURN, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_SUCCEEDED},
    stmt::{private::BaseStmtAttr, StmtAttr},
    str::{Ansi, OdbcStr, Unicode},
    BulkOperation, CompletionType, DatetimeIntervalCode, DriverCompletion, FreeStmtOption,
    FunctionId, IOType, Ident, IdentifierType, LockType, NullAllowed, Operation, Ref, Reserved,
    Scope, StrLenOrInd, Unique, RETCODE, SQLCHAR, SQLINTEGER, SQLLEN, SQLPOINTER, SQLSETPOSIROW,
    SQLSMALLINT, SQLULEN, SQLUSMALLINT, SQLWCHAR,
};
use core::{cell::UnsafeCell, mem::MaybeUninit, ptr};
#[cfg(test)]
use mockall::automock;

/// ODBC handle such as environment, connection, statement or descriptor.
///
/// For complete documentation, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/handles).
pub trait Handle: AsSQLHANDLE + Sized {
    type Ident: Ident<Type = SQLSMALLINT>;
}

#[allow(non_snake_case)]
pub trait Allocate<'src, SRC: AsSQLHANDLE>: Handle {
    /// Creates handle from a raw pointer
    ///
    /// # Safety
    ///
    /// The given raw pointer must point to a valid handle of the required type
    unsafe fn from_raw(output_handle: ptr::NonNull<RawHandle>) -> Self;

    /// Allocates an environment, connection, statement, or descriptor handle.
    ///
    /// For complete documentation on `SQLAllocHandle`, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlallochandle-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_INVALID_HANDLE, or SQL_ERROR.
    #[inline]
    fn SQLAllocHandle(InputHandle: &'src SRC) -> (Result<Self, ()>, SQLRETURN) {
        let mut output_handle = MaybeUninit::uninit();

        unsafe {
            let sql_return = ffi::SQLAllocHandle(
                Self::Ident::IDENTIFIER,
                InputHandle.as_SQLHANDLE(),
                output_handle.as_mut_ptr(),
            );

            if SQL_SUCCEEDED(sql_return) {
                let output_handle = ptr::NonNull::new_unchecked(output_handle.assume_init());
                (Ok(Self::from_raw(output_handle)), sql_return)
            } else {
                (Err(()), sql_return)
            }
        }
    }

    /// Frees resources associated with a specific environment, connection, statement, or descriptor handle.
    ///
    /// For complete documentation on SQLFreeHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreehandle-function).
    ///
    /// # Panics
    ///
    /// Panics if the DM returns value other than SQL_SUCCESS
    #[inline]
    fn SQLFreeHandle(self) {}
}

#[allow(non_snake_case, unused_variables)]
pub trait Diagnostics: Handle {
    /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
    ///
    /// For complete documentation on SQLGetDiagFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdiagfield-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_NO_DATA.
    #[inline]
    fn SQLGetDiagFieldA<A: Ident<Type = SQLSMALLINT>, T: DiagField<Self, A>>(
        &self,
        // TODO: Use NoneZeroI16?
        RecNumber: core::num::NonZeroI16,
        DiagIdentifier: A,
        DiagInfoPtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        let DiagInfoPtr = DiagInfoPtr.map_or((ptr::null_mut(), 0), |DiagInfoPtr| {
            if cfg!(feature = "odbc_debug") {
                DiagInfoPtr.assert_zeroed();
            }

            (DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len())
        });

        unsafe {
            ffi::SQLGetDiagFieldA(
                Self::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
                RecNumber.get(),
                A::IDENTIFIER,
                DiagInfoPtr.0,
                DiagInfoPtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    fn SQLGetDiagFieldW<A: Ident<Type = SQLSMALLINT>, T: DiagField<Self, A>>(
        &self,
        // TODO: Use NoneZeroI16?
        RecNumber: core::num::NonZeroI16,
        DiagIdentifier: A,
        DiagInfoPtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        let DiagInfoPtr = DiagInfoPtr.map_or((ptr::null_mut(), 0), |DiagInfoPtr| {
            if cfg!(feature = "odbc_debug") {
                DiagInfoPtr.assert_zeroed();
            }

            (DiagInfoPtr.as_mut_SQLPOINTER(), DiagInfoPtr.len())
        });

        unsafe {
            ffi::SQLGetDiagFieldW(
                Self::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
                RecNumber.get(),
                A::IDENTIFIER,
                DiagInfoPtr.0,
                DiagInfoPtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    fn SQLGetDiagRecA(
        &self,
        // TODO: Use NoneZeroI16?
        RecNumber: core::num::NonZeroI16,
        SQLState: &mut MaybeUninit<SQLSTATE<SQLCHAR>>,
        NativeErrorPtr: &mut impl AsMutPtr<SQLINTEGER>,
        MessageText: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        TextLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let MessageText = MessageText.as_mut_raw_slice();

        unsafe {
            ffi::SQLGetDiagRecA(
                Self::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
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
    fn SQLGetDiagRecW(
        &self,
        // TODO: Use NoneZeroI16?
        RecNumber: core::num::NonZeroI16,
        SQLState: &mut MaybeUninit<SQLSTATE<SQLWCHAR>>,
        NativeErrorPtr: &mut impl AsMutPtr<SQLINTEGER>,
        MessageText: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        TextLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let MessageText = MessageText.as_mut_raw_slice();

        unsafe {
            ffi::SQLGetDiagRecW(
                Self::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
                RecNumber.get(),
                SQLState.as_mut_ptr().cast(),
                NativeErrorPtr.as_mut_ptr(),
                MessageText.0,
                MessageText.1,
                TextLengthPtr.as_mut_ptr(),
            )
        }
    }
}

#[allow(non_snake_case)]
pub trait Statement<'desc, 'buf, V: OdbcVersion>: Handle {
    type ARD: Descriptor<'buf, AppDesc<'buf>, V>;
    type APD: Descriptor<'buf, AppDesc<'buf>, V>;
    type IRD: Descriptor<'buf, IRD, V>;
    type IPD: Descriptor<'buf, IPD, V>;

    type ExplicitARD: Descriptor<'buf, AppDesc<'buf>, V>;
    type ExplicitAPD: Descriptor<'buf, AppDesc<'buf>, V>;

    fn bind_col<TT: Ident, B: DeferredBuf<Self::ARD, TT, V>>(
        &self,
        TargetValuePtr: Option<&'buf B>,
    ) where
        B: ?Sized;
    fn bind_param<TT: Ident, B: DeferredBuf<Self::APD, TT, V>>(
        &self,
        TargetValuePtr: Option<&'buf B>,
    ) where
        B: ?Sized;
    fn bind_strlen_or_ind(&self, StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>);

    /// Binds application data buffers to columns in the result set.
    ///
    /// For complete documentation on SQLBindCol, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    fn SQLBindCol<TT: Ident<Type = SQLSMALLINT>, B: DeferredBuf<Self::ARD, TT, V>>(
        &self,
        ColumnNumber: SQLUSMALLINT,
        TargetType: TT,
        TargetValuePtr: Option<&'buf B>,
        StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
    ) -> SQLRETURN
    where
        B: ?Sized,
    {
        let sql_return = unsafe {
            let TargetValuePtr = TargetValuePtr.map_or((ptr::null_mut(), 0), |TargetValuePtr| {
                (TargetValuePtr.as_SQLPOINTER(), TargetValuePtr.len())
            });

            ffi::SQLBindCol(
                self.as_SQLHANDLE(),
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
            self.bind_col(TargetValuePtr);
            self.bind_strlen_or_ind(StrLen_or_IndPtr);
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
    #[allow(unused_variables)]
    fn SQLBindParameter<
        TT: Ident<Type = SQLSMALLINT>,
        // TODO: Check which type is used for ParameterType
        ST: SqlType<V>,
        B: DeferredBuf<Self::APD, TT, V>,
    >(
        &self,
        ParameterNumber: SQLUSMALLINT,
        InputOutputType: IOType,
        ValueType: TT,
        ParameterType: ST,
        ColumnSize: SQLULEN,
        DecimalDigits: SQLSMALLINT,
        ParameterValuePtr: Option<&'buf B>,
        StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>,
    ) -> SQLRETURN
    where
        B: ?Sized,
    {
        let sql_return = unsafe {
            let ParameterValuePtr = ParameterValuePtr
                .map_or((ptr::null_mut(), 0), |ParameterValuePtr| {
                    (ParameterValuePtr.as_SQLPOINTER(), ParameterValuePtr.len())
                });

            ffi::SQLBindParameter(
                self.as_SQLHANDLE(),
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
            self.bind_param(ParameterValuePtr);
            self.bind_strlen_or_ind(StrLen_or_IndPtr);
        }

        sql_return
    }
    /// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
    ///
    /// For complete documentation on SQLBulkOperations, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbulkoperations-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLBulkOperations(&self, Operation: BulkOperation) -> SQLRETURN {
        unsafe { ffi::SQLBulkOperations(self.as_SQLHANDLE(), Operation as SQLUSMALLINT) }
    }

    /// Closes a cursor that has been opened on a statement and discards pending results.
    ///
    /// For complete documentation on SQLCloseCursor, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlclosecursor-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLCloseCursor(&self) -> SQLRETURN {
        unsafe { ffi::SQLCloseCursor(self.as_SQLHANDLE()) }
    }

    /// Returns descriptor information for a column in a result set. Descriptor information is returned as a character string, a descriptor-dependent value, or an integer value.
    ///
    /// For complete documentation on SQLColAttributeA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolattribute-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    fn SQLColAttributeA<A: Ident<Type = SQLUSMALLINT>, T: ColAttr<A, V>>(
        &self,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: A,
        CharacterAttributePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
        NumericAttributePtr: &mut impl AsMutPtr<SQLLEN>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        // TODO: With MaybeUninit it's not possible to check that value is zeroed
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
            ffi::SQLColAttributeA(
                self.as_SQLHANDLE(),
                ColumnNumber,
                A::IDENTIFIER,
                CharacterAttributePtr.0,
                CharacterAttributePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    #[allow(unused_variables)]
    fn SQLColAttributeW<A: Ident<Type = SQLUSMALLINT>, T: ColAttr<A, V>>(
        &self,
        ColumnNumber: SQLUSMALLINT,
        FieldIdentifier: A,
        CharacterAttributePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
        NumericAttributePtr: &mut impl AsMutPtr<SQLLEN>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        // TODO: With MaybeUninit it's not possible to check that value is zeroed
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
            ffi::SQLColAttributeW(
                self.as_SQLHANDLE(),
                ColumnNumber,
                A::IDENTIFIER,
                CharacterAttributePtr.0,
                CharacterAttributePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
                NumericAttributePtr.as_mut_ptr(),
            )
        }
    }

    /// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `self`.
    ///
    /// For complete documentation on SQLColumnPrivilegesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLColumnPrivilegesA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
        ColumnName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLColumnPrivilegesA(
                self.as_SQLHANDLE(),
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

    /// Returns a list of columns and associated privileges for the specified table. The driver returns the information as a result set on the specified `self`.
    ///
    /// For complete documentation on SQLColumnPrivilegesW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumnprivileges-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLColumnPrivilegesW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
        ColumnName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLColumnPrivilegesW(
                self.as_SQLHANDLE(),
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

    /// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `self`.
    ///
    /// For complete documentation on SQLColumnsA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLColumnsA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
        ColumnName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLColumnsA(
                self.as_SQLHANDLE(),
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

    /// Returns the list of column names in specified tables. The driver returns this information as a result set on the specified `self`.
    ///
    /// For complete documentation on SQLColumnsW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcolumns-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLColumnsW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
        ColumnName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLColumnsW(
                self.as_SQLHANDLE(),
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

    /// Returns the result descriptor - column name,type, column size, decimal digits, and nullability - for one column in the result set. This information also is available in the fields of the IRD.
    ///
    /// For complete documentation on SQLDescribeColA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldescribecol-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLDescribeColA(
        &self,
        ColumnNumber: SQLUSMALLINT,
        ColumnName: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        NameLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        DataTypePtr: &mut impl AsMutPtr<SQLSMALLINT>,
        ColumnSizePtr: &mut impl AsMutPtr<SQLULEN>,
        DecimalDigitsPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        NullablePtr: &mut impl AsMutPtr<NullAllowed>,
    ) -> SQLRETURN {
        let ColumnName = ColumnName.as_mut_raw_slice();

        unsafe {
            ffi::SQLDescribeColA(
                self.as_SQLHANDLE(),
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
    fn SQLDescribeColW(
        &self,
        ColumnNumber: SQLUSMALLINT,
        ColumnName: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        NameLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        DataTypePtr: &mut impl AsMutPtr<SQLSMALLINT>,
        ColumnSizePtr: &mut impl AsMutPtr<SQLULEN>,
        DecimalDigitsPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        NullablePtr: &mut impl AsMutPtr<NullAllowed>,
    ) -> SQLRETURN {
        let ColumnName = ColumnName.as_mut_raw_slice();

        unsafe {
            ffi::SQLDescribeColW(
                self.as_SQLHANDLE(),
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
    fn SQLDescribeParam(
        &self,
        ParameterNumber: SQLUSMALLINT,
        DataTypePtr: &mut impl AsMutPtr<SQLSMALLINT>,
        ParameterSizePtr: &mut impl AsMutPtr<SQLULEN>,
        DecimalDigitsPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        NullablePtr: &mut impl AsMutPtr<NullAllowed>,
    ) -> SQLRETURN {
        unsafe {
            ffi::SQLDescribeParam(
                self.as_SQLHANDLE(),
                ParameterNumber,
                DataTypePtr.as_mut_ptr(),
                ParameterSizePtr.as_mut_ptr(),
                DecimalDigitsPtr.as_mut_ptr(),
                NullablePtr.as_mut_ptr().cast(),
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
    fn SQLForeignKeysA(
        &self,
        PKCatalogName: &OdbcStr<SQLCHAR>,
        PKSchemaName: &OdbcStr<SQLCHAR>,
        PKTableName: &OdbcStr<SQLCHAR>,
        FKCatalogName: &OdbcStr<SQLCHAR>,
        FKSchemaName: &OdbcStr<SQLCHAR>,
        FKTableName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let PKCatalogName = PKCatalogName.as_raw_slice();
        let PKSchemaName = PKSchemaName.as_raw_slice();
        let PKTableName = PKTableName.as_raw_slice();
        let FKCatalogName = FKCatalogName.as_raw_slice();
        let FKSchemaName = FKSchemaName.as_raw_slice();
        let FKTableName = FKTableName.as_raw_slice();

        unsafe {
            ffi::SQLForeignKeysA(
                self.as_SQLHANDLE(),
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
    fn SQLForeignKeysW(
        &self,
        PKCatalogName: &OdbcStr<SQLWCHAR>,
        PKSchemaName: &OdbcStr<SQLWCHAR>,
        PKTableName: &OdbcStr<SQLWCHAR>,
        FKCatalogName: &OdbcStr<SQLWCHAR>,
        FKSchemaName: &OdbcStr<SQLWCHAR>,
        FKTableName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let PKCatalogName = PKCatalogName.as_raw_slice();
        let PKSchemaName = PKSchemaName.as_raw_slice();
        let PKTableName = PKTableName.as_raw_slice();
        let FKCatalogName = FKCatalogName.as_raw_slice();
        let FKSchemaName = FKSchemaName.as_raw_slice();
        let FKTableName = FKTableName.as_raw_slice();

        unsafe {
            ffi::SQLForeignKeysW(
                self.as_SQLHANDLE(),
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

    /// Stops processing associated with a specific statement, closes any open cursors associated with the statement, discards pending results, or, optionally, frees all resources associated with the statement handle.
    ///
    /// For complete documentation on SQLFreeStmt, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfreestmt-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLFreeStmt(&self, Option: FreeStmtOption) -> SQLRETURN {
        unsafe { ffi::SQLFreeStmt(self.as_SQLHANDLE(), Option as SQLUSMALLINT) }
    }

    /// Returns the cursor name associated with a specified statement.
    ///
    /// For complete documentation on SQLGetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetcursorname-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLGetCursorNameA(
        &self,
        CursorName: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        NameLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let CursorName = CursorName.as_mut_raw_slice();

        unsafe {
            ffi::SQLGetCursorNameA(
                self.as_SQLHANDLE(),
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
    fn SQLGetCursorNameW(
        &self,
        CursorName: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        NameLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let CursorName = CursorName.as_mut_raw_slice();

        unsafe {
            ffi::SQLGetCursorNameW(
                self.as_SQLHANDLE(),
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
    #[allow(unused_variables)]
    // TODO: This function must be unsafe if SQL_ARD_TYPE and SQL_APD_TYPE are allowed to be used
    fn SQLGetData<TT: Ident<Type = SQLSMALLINT>, B: CData<TT, V>>(
        &self,
        Col_or_Param_Num: SQLUSMALLINT,
        TargetType: TT,
        TargetValuePtr: &mut B,
        StrLen_or_IndPtr: Option<&mut MaybeUninit<StrLenOrInd>>,
    ) -> SQLRETURN
    where
        B: AsMutSQLPOINTER + ?Sized,
        MaybeUninit<StrLenOrInd>: StrLen<SQLLEN>,
    {
        unsafe {
            ffi::SQLGetData(
                self.as_SQLHANDLE(),
                Col_or_Param_Num,
                TT::IDENTIFIER,
                TargetValuePtr.as_mut_SQLPOINTER(),
                TargetValuePtr.len(),
                StrLen_or_IndPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    fn SQLGetStmtAttrA<'stmt, A: Ident<Type = SQLINTEGER>, T: StmtAttr<'desc, 'buf, Self, A, V>>(
        &'stmt self,
        Attribute: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + Ref<'stmt> + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        SQLGetStmtAttrA(self, Attribute, ValuePtr, StringLengthPtr)
    }

    /// Returns the current setting of a statement attribute.
    ///
    /// For complete documentation on SQLGetStmtAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetstmtattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLGetStmtAttrW<'stmt, A: Ident<Type = SQLINTEGER>, T: StmtAttr<'desc, 'buf, Self, A, V>>(
        &'stmt self,
        Attribute: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + Ref<'stmt> + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        SQLGetStmtAttrW(self, Attribute, ValuePtr, StringLengthPtr)
    }

    /// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
    ///
    /// For complete documentation on SQLGetTypeInfoA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLGetTypeInfoA<ST: SqlType<V>>(&self, DataType: ST) -> SQLRETURN {
        unsafe { ffi::SQLGetTypeInfoA(self.as_SQLHANDLE(), DataType.identifier()) }
    }

    /// Returns information about data types supported by the data source. The driver returns the information in the form of an SQL result set. The data types are intended for use in Data Definition Language (DDL) statements.
    ///
    /// For complete documentation on SQLGetTypeInfoW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgettypeinfo-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLGetTypeInfoW<ST: SqlType<V>>(&self, DataType: ST) -> SQLRETURN {
        unsafe { ffi::SQLGetTypeInfoW(self.as_SQLHANDLE(), DataType.identifier()) }
    }

    /// Determines whether more results are available on a statement containing **SELECT**, **UPDATE**, **INSERT**, or **DELETE** statements and, if so, initializes processing for those results.
    ///
    /// For complete documentation on SQLMoreResults, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlmoreresults-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, OR SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    // TODO: Maybe this fn should be unsafe
    fn SQLMoreResults(&self) -> SQLRETURN {
        unsafe { ffi::SQLMoreResults(self.as_SQLHANDLE()) }
    }

    /// Returns the number of parameters in an SQL statement.
    ///
    /// For complete documentation on SQLNumParams, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumparams-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLNumParams(&self, ParameterCountPtr: &mut impl AsMutPtr<SQLSMALLINT>) -> SQLRETURN {
        unsafe { ffi::SQLNumParams(self.as_SQLHANDLE(), ParameterCountPtr.as_mut_ptr()) }
    }

    /// Returns the number of columns in a result set.
    ///
    /// For complete documentation on SQLNumResultCols, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnumresultcols-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLNumResultCols(&self, ColumnCountPtr: &mut impl AsMutPtr<SQLSMALLINT>) -> SQLRETURN {
        unsafe { ffi::SQLNumResultCols(self.as_SQLHANDLE(), ColumnCountPtr.as_mut_ptr()) }
    }

    /// Used together with **SQLPutData** to supply parameter data at statement execution time, and with **SQLGetData** to retrieve streamed output parameter data.
    ///
    /// For complete documentation on SQLParamData, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlparamdata-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    fn SQLParamData(&self, ValuePtrPtr: &mut MaybeUninit<SQLPOINTER>) -> SQLRETURN {
        unsafe { ffi::SQLParamData(self.as_SQLHANDLE(), ValuePtrPtr.as_mut_ptr()) }
    }

    /// Prepares an SQL string for execution.
    ///
    /// For complete documentation on SQLPrepareA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLPrepareA(&self, StatementText: &OdbcStr<SQLCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        unsafe { ffi::SQLPrepareA(self.as_SQLHANDLE(), StatementText.0, StatementText.1) }
    }

    /// Prepares an SQL string for execution.
    ///
    /// For complete documentation on SQLPrepareW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprepare-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLPrepareW(&self, StatementText: &OdbcStr<SQLWCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        unsafe { ffi::SQLPrepareW(self.as_SQLHANDLE(), StatementText.0, StatementText.1) }
    }

    /// Returns the column names that make up the primary key for a table. The driver returns the information as a result set. This function does not support returning primary keys from multiple tables in a single call.
    ///
    /// For complete documentation on SQLPrimaryKeysA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlprimarykeys-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLPrimaryKeysA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLPrimaryKeysA(
                self.as_SQLHANDLE(),
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
    fn SQLPrimaryKeysW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLPrimaryKeysW(
                self.as_SQLHANDLE(),
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
    fn SQLProcedureColumnsA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        ProcName: &OdbcStr<SQLCHAR>,
        ColumnName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let ProcName = ProcName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLProcedureColumnsA(
                self.as_SQLHANDLE(),
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
    fn SQLProcedureColumnsW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        ProcName: &OdbcStr<SQLWCHAR>,
        ColumnName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let ProcName = ProcName.as_raw_slice();
        let ColumnName = ColumnName.as_raw_slice();

        unsafe {
            ffi::SQLProcedureColumnsW(
                self.as_SQLHANDLE(),
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
    fn SQLProceduresA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        ProcName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let ProcName = ProcName.as_raw_slice();

        unsafe {
            ffi::SQLProceduresA(
                self.as_SQLHANDLE(),
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
    fn SQLProceduresW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        ProcName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let ProcName = ProcName.as_raw_slice();

        unsafe {
            ffi::SQLProceduresW(
                self.as_SQLHANDLE(),
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
    // TODO: Is it unsafe if odbc_debug is used?
    unsafe fn SQLPutData<TT: Ident, B: CData<TT, V>>(&self, DataPtr: Option<&B>) -> SQLRETURN
    where
        B: AsSQLPOINTER + ?Sized,
    {
        let DataPtr = DataPtr.map_or((ptr::null_mut(), 0), |DataPtr| {
            (DataPtr.as_SQLPOINTER(), DataPtr.len())
        });

        ffi::SQLPutData(self.as_SQLHANDLE(), DataPtr.0, DataPtr.1)
    }

    /// Returns the number of rows affected by an **UPDATE**, **INSERT**, or **DELETE** statement; an SQL_ADD, SQL_UPDATE_BY_BOOKMARK, or SQL_DELETE_BY_BOOKMARK operation in **SQLBulkOperations**; or an SQL_UPDATE or SQL_DELETE operation in **SQLSetPos**.
    ///
    /// For complete documentation on SQLRowCount, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlrowcount-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLRowCount(&self, RowCountPtr: &mut impl AsMutPtr<SQLLEN>) -> SQLRETURN {
        unsafe { ffi::SQLRowCount(self.as_SQLHANDLE(), RowCountPtr.as_mut_ptr()) }
    }

    /// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
    ///
    /// For complete documentation on SQLSetCursorNameA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLSetCursorNameA(&self, CursorName: &OdbcStr<SQLCHAR>) -> SQLRETURN {
        let CursorName = CursorName.as_raw_slice();

        unsafe { ffi::SQLSetCursorNameA(self.as_SQLHANDLE(), CursorName.0, CursorName.1) }
    }

    /// Associates a cursor name with an active statement. If an application does not call **SQLSetCursorName**, the driver generates cursor names as needed for SQL statement processing.
    ///
    /// For complete documentation on SQLSetCursorNameW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetcursorname-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLSetCursorNameW(&self, CursorName: &OdbcStr<SQLWCHAR>) -> SQLRETURN {
        let CursorName = CursorName.as_raw_slice();

        unsafe { ffi::SQLSetCursorNameW(self.as_SQLHANDLE(), CursorName.0, CursorName.1) }
    }

    /// Sets attributes related to a statement.
    ///
    /// For complete documentation on SQLSetStmtAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLSetStmtAttrA<A: Ident<Type = SQLINTEGER>, T: StmtAttr<'desc, 'buf, Self, A, V>>(
        &self,
        Attribute: A,
        ValuePtr: T,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Ansi,
    {
        SQLSetStmtAttrA(self, Attribute, ValuePtr)
    }

    /// Sets attributes related to a statement.
    ///
    /// For complete documentation on SQLSetStmtAttrW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetstmtattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLSetStmtAttrW<A: Ident<Type = SQLINTEGER>, T: StmtAttr<'desc, 'buf, Self, A, V>>(
        &self,
        Attribute: A,
        ValuePtr: T,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Unicode,
    {
        SQLSetStmtAttrW(self, Attribute, ValuePtr)
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
    fn SQLSpecialColumnsA(
        &self,
        IdentifierType: IdentifierType,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
        Scope: Scope,
        Nullable: NullAllowed,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLSpecialColumnsA(
                self.as_SQLHANDLE(),
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
    fn SQLSpecialColumnsW(
        &self,
        IdentifierType: IdentifierType,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
        Scope: Scope,
        Nullable: NullAllowed,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLSpecialColumnsW(
                self.as_SQLHANDLE(),
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
    fn SQLStatisticsA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
        Unique: Unique,
        Reserved: Reserved,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLStatisticsA(
                self.as_SQLHANDLE(),
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
    fn SQLStatisticsW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
        Unique: Unique,
        Reserved: Reserved,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLStatisticsW(
                self.as_SQLHANDLE(),
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
    fn SQLTablePrivilegesA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLTablePrivilegesA(
                self.as_SQLHANDLE(),
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
    fn SQLTablePrivilegesW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();

        unsafe {
            ffi::SQLTablePrivilegesW(
                self.as_SQLHANDLE(),
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
    fn SQLTablesA(
        &self,
        CatalogName: &OdbcStr<SQLCHAR>,
        SchemaName: &OdbcStr<SQLCHAR>,
        TableName: &OdbcStr<SQLCHAR>,
        TableType: &OdbcStr<SQLCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let TableType = TableType.as_raw_slice();

        unsafe {
            ffi::SQLTablesA(
                self.as_SQLHANDLE(),
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
    fn SQLTablesW(
        &self,
        CatalogName: &OdbcStr<SQLWCHAR>,
        SchemaName: &OdbcStr<SQLWCHAR>,
        TableName: &OdbcStr<SQLWCHAR>,
        TableType: &OdbcStr<SQLWCHAR>,
    ) -> SQLRETURN {
        let CatalogName = CatalogName.as_raw_slice();
        let SchemaName = SchemaName.as_raw_slice();
        let TableName = TableName.as_raw_slice();
        let TableType = TableType.as_raw_slice();

        unsafe {
            ffi::SQLTablesW(
                self.as_SQLHANDLE(),
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
}

#[allow(non_snake_case)]
pub trait Descriptor<'buf, DT, V: OdbcVersion>: Handle {
    /// Copies descriptor information from one descriptor handle to another.
    ///
    /// For complete documentation on SQLCopyDesc, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcopydesc-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    // TODO: Not sure if application and implementation descriptors can be interchangeably copied
    // TODO: Do they have to have the same version?
    // TODO: Is lifetime the same?
    fn SQLCopyDesc<DT2: DescType<'buf>>(&self, TargetDescHandle: &SQLHDESC<DT2, V>) -> SQLRETURN {
        unsafe { ffi::SQLCopyDesc(self.as_SQLHANDLE(), TargetDescHandle.as_SQLHANDLE()) }
    }

    /// Returns the current setting or value of a single field of a descriptor record.
    ///
    /// For complete documentation on SQLGetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetdescfield-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_NO_DATA, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    fn SQLGetDescFieldA<A: Ident<Type = SQLSMALLINT>, T: DescField<'buf, Self, DT, A, V>>(
        &self,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            if cfg!(feature = "odbc_debug") {
                ValuePtr.assert_zeroed();
            }

            (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
        });

        unsafe {
            ffi::SQLGetDescFieldA(
                self.as_SQLHANDLE(),
                RecNumber,
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    #[allow(unused_variables)]
    fn SQLGetDescFieldW<A: Ident<Type = SQLSMALLINT>, T: DescField<'buf, Self, DT, A, V>>(
        &self,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            if cfg!(feature = "odbc_debug") {
                ValuePtr.assert_zeroed();
            }

            (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
        });

        unsafe {
            ffi::SQLGetDescFieldW(
                self.as_SQLHANDLE(),
                RecNumber,
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    fn SQLGetDescRecA<ST: SqlType<V>>(
        &self,
        RecNumber: SQLSMALLINT,
        Name: Option<&mut OdbcStr<MaybeUninit<SQLCHAR>>>,
        StringLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        TypePtr: &mut impl AsMutPtr<ST>,
        SubTypePtr: &mut impl AsMutPtr<DatetimeIntervalCode>,
        LengthPtr: &mut impl AsMutPtr<SQLLEN>,
        PrecisionPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        ScalePtr: &mut impl AsMutPtr<SQLSMALLINT>,
        NullablePtr: &mut impl AsMutPtr<NullAllowed>,
    ) -> SQLRETURN {
        let Name = Name.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        unsafe {
            ffi::SQLGetDescRecA(
                self.as_SQLHANDLE(),
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
    fn SQLGetDescRecW<ST: SqlType<V>>(
        &self,
        RecNumber: SQLSMALLINT,
        Name: Option<&mut OdbcStr<MaybeUninit<SQLWCHAR>>>,
        StringLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        TypePtr: &mut impl AsMutPtr<ST>,
        SubTypePtr: &mut impl AsMutPtr<DatetimeIntervalCode>,
        LengthPtr: &mut impl AsMutPtr<SQLLEN>,
        PrecisionPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        ScalePtr: &mut impl AsMutPtr<SQLSMALLINT>,
        NullablePtr: &mut impl AsMutPtr<NullAllowed>,
    ) -> SQLRETURN {
        let Name = Name.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        unsafe {
            ffi::SQLGetDescRecW(
                self.as_SQLHANDLE(),
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

    /// Sets the value of a single field of a descriptor record.
    ///
    /// For complete documentation on SQLSetDescFieldA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetdescfield-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    fn SQLSetDescFieldA<A: Ident<Type = SQLSMALLINT>, T: DescField<'buf, Self, DT, A, V>>(
        &self,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: A,
        ValuePtr: Option<T>,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Ansi,
    {
        let sql_return = unsafe {
            let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
                (ValuePtr.into_SQLPOINTER(), ValuePtr.len())
            });

            ffi::SQLSetDescFieldA(
                self.as_SQLHANDLE(),
                RecNumber,
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            ValuePtr.map(|v| v.update_handle(self));
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
    #[allow(unused_variables)]
    fn SQLSetDescFieldW<A: Ident<Type = SQLSMALLINT>, T: DescField<'buf, Self, DT, A, V>>(
        &self,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: A,
        ValuePtr: Option<T>,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Unicode,
    {
        let sql_return = unsafe {
            let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
                (ValuePtr.into_SQLPOINTER(), ValuePtr.len())
            });

            ffi::SQLSetDescFieldW(
                self.as_SQLHANDLE(),
                RecNumber,
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            ValuePtr.map(|v| v.update_handle(self));
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
    // TODO: Must not be allowed for IRD. Handle here or with DescField
    fn SQLSetDescRec<ST: SqlType<V>, PTR>(
        &self,
        RecNumber: SQLSMALLINT,
        Type: ST,
        SubType: Option<DatetimeIntervalCode>,
        Length: SQLLEN,
        Precision: SQLSMALLINT,
        Scale: SQLSMALLINT,
        // TODO: Input or Output for both? I guess it depends on which descriptor was given
        DataPtr: Option<&'buf PTR>,
        // TODO: Shouldn't following two be UnsafeCell
        StringLengthPtr: &'buf mut impl AsMutPtr<SQLLEN>,
        IndicatorPtr: &'buf mut impl AsMutPtr<SQLLEN>,
    ) -> SQLRETURN
    where
        &'buf PTR: IntoSQLPOINTER,
    {
        unsafe {
            ffi::SQLSetDescRec(
                self.as_SQLHANDLE(),
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
}

#[allow(non_snake_case)]
pub trait Cancel<V: OdbcVersion>: Handle {
    /// Cancels the processing on a statement.
    /// To cancel processing on a connection or statement, use SQLCancelHandle Function.
    ///
    /// For complete documentation on SQLCancel, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancel-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLCancel(&self) -> SQLRETURN
    where
        Self: Handle<Ident = SQL_HANDLE_STMT>,
    {
        unsafe { ffi::SQLCancel(self.as_SQLHANDLE()) }
    }

    /// Cancels the processing on a connection or statement. The Driver Manager maps a call to **SQLCancelHandle** to a call to **SQLCancel** when `HandleType` is SQL_HANDLE_STMT.
    ///
    /// For complete documentation on SQLCancelHandle, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlcancelhandle-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    fn SQLCancelHandle(&self) -> SQLRETURN {
        unsafe { ffi::SQLCancelHandle(<Self as Handle>::Ident::IDENTIFIER, self.as_SQLHANDLE()) }
    }
}

#[allow(non_snake_case)]
pub trait Async<V: OdbcVersion>: Handle {
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
    // TODO: Should this handle be mutable or not?
    fn SQLCompleteAsync(&mut self, AsyncRetCodePtr: &mut impl AsMutPtr<RETCODE>) -> SQLRETURN {
        unsafe {
            ffi::SQLCompleteAsync(
                Self::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
                AsyncRetCodePtr.as_mut_ptr(),
            )
        }
    }
}

#[allow(non_snake_case)]
impl<V: OdbcVersion> SQLHENV<V> {
    /// Returns information about a data source. This function is implemented only by the Driver Manager.
    ///
    /// For complete documentation on SQLDataSourcesA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldatasources-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLDataSourcesA(
        &self,
        Direction: SQLUSMALLINT,
        ServerName: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        NameLength1Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
        Description: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        NameLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let ServerName = ServerName.as_mut_raw_slice();
        let Description = Description.as_mut_raw_slice();

        unsafe {
            ffi::SQLDataSourcesA(
                self.as_SQLHANDLE(),
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
    pub fn SQLDataSourcesW(
        &self,
        Direction: SQLUSMALLINT,
        ServerName: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        NameLength1Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
        Description: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        NameLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let ServerName = ServerName.as_mut_raw_slice();
        let Description = Description.as_mut_raw_slice();

        unsafe {
            ffi::SQLDataSourcesW(
                self.as_SQLHANDLE(),
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

    /// Lists driver descriptions and driver attribute keywords. This function is implemented only by the Driver Manager.
    ///
    /// For complete documentation on SQLDriversA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldrivers-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLDriversA(
        &self,
        Direction: SQLUSMALLINT,
        DriverDescription: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        DescriptionLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        DriverAttributes: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        AttributesLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let DriverDescription = DriverDescription.as_mut_raw_slice();
        let DriverAttributes = DriverAttributes.as_mut_raw_slice();

        unsafe {
            ffi::SQLDriversA(
                self.as_SQLHANDLE(),
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
    pub fn SQLDriversW(
        &self,
        Direction: SQLUSMALLINT,
        DriverDescription: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        DescriptionLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
        DriverAttributes: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        AttributesLengthPtr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> SQLRETURN {
        let DriverDescription = DriverDescription.as_mut_raw_slice();
        let DriverAttributes = DriverAttributes.as_mut_raw_slice();

        unsafe {
            ffi::SQLDriversW(
                self.as_SQLHANDLE(),
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

    /// Returns the current setting of an environment attribute.
    ///
    /// For complete documentation on SQLGetEnvAttr, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetenvattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    pub fn SQLGetEnvAttr<A: Ident<Type = SQLINTEGER>, T: EnvAttr<A, V>>(
        &self,
        Attribute: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
        });

        unsafe {
            ffi::SQLGetEnvAttr(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    #[allow(unused_variables)]
    pub fn SQLSetEnvAttr<A: Ident<Type = SQLINTEGER>, T: EnvAttr<A, V>>(
        // Reference to SQLHENV is mutable to make it impossible to have a connection
        // handle allocated on the environment handle when calling this function
        &mut self,
        Attribute: A,
        ValuePtr: T,
    ) -> SQLRETURN
    where
        T: AttrSet<A>,
    {
        unsafe {
            ffi::SQLSetEnvAttr(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                ValuePtr.into_SQLPOINTER(),
                ValuePtr.len(),
            )
        }
    }
}

#[allow(non_snake_case)]
impl<'env, C: ConnState, V: OdbcVersion> SQLHDBC<'env, C, V> {
    /// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
    ///
    /// For complete documentation on SQLBrowseConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    #[allow(unused_variables)]
    pub fn SQLBrowseConnectA(
        self,
        InConnectionString: &OdbcStr<SQLCHAR>,
        OutConnectionString: Option<&mut OdbcStr<MaybeUninit<SQLCHAR>>>,
        StringLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, Result<SQLHDBC<'env, C3, V>, SQLHDBC<'env, C2, V>>>,
        SQLRETURN,
    )
    where
        Self: BrowseConnect,
    {
        let InConnectionString = InConnectionString.as_raw_slice();
        let OutConnectionString =
            OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        let sql_return = unsafe {
            ffi::SQLBrowseConnectA(
                self.as_SQLHANDLE(),
                InConnectionString.0,
                InConnectionString.1,
                OutConnectionString.0,
                OutConnectionString.1,
                StringLength2Ptr.as_mut_ptr(),
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            (Ok(self.connect()), sql_return)
        } else if sql_return == SQL_NEED_DATA {
            (Err(Ok(self.need_data())), sql_return)
        } else if sql_return == SQL_STILL_EXECUTING {
            unimplemented!("Asynchronous execution not supported")
        } else {
            (Err(Err(self.disconnect())), sql_return)
        }
    }

    /// Supports an iterative method of discovering and enumerating the attributes and attribute values required to connect to a data source. Each call to **SQLBrowseConnect** returns successive levels of attributes and attribute values. When all levels have been enumerated, a connection to the data source is completed and a complete connection string is returned by **SQLBrowseConnect**. A return code of SQL_SUCCESS or SQL_SUCCESS_WITH_INFO indicates that all connection information has been specified and the application is now connected to the data source.
    ///
    /// For complete documentation on SQLBrowseConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbrowseconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLBrowseConnectW(
        self,
        InConnectionString: &OdbcStr<SQLWCHAR>,
        OutConnectionString: Option<&mut OdbcStr<MaybeUninit<SQLWCHAR>>>,
        StringLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, Result<SQLHDBC<'env, C3, V>, SQLHDBC<'env, C2, V>>>,
        SQLRETURN,
    )
    where
        Self: BrowseConnect,
    {
        let InConnectionString = InConnectionString.as_raw_slice();
        let OutConnectionString =
            OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        let sql_return = unsafe {
            ffi::SQLBrowseConnectW(
                self.as_SQLHANDLE(),
                InConnectionString.0,
                InConnectionString.1,
                OutConnectionString.0,
                OutConnectionString.1,
                StringLength2Ptr.as_mut_ptr(),
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            (Ok(self.connect()), sql_return)
        } else if sql_return == SQL_NEED_DATA {
            (Err(Ok(self.need_data())), sql_return)
        } else if sql_return == SQL_STILL_EXECUTING {
            unimplemented!("Asynchronous execution not supported")
        } else {
            (Err(Err(self.disconnect())), sql_return)
        }
    }

    /// Closes the connection associated with a specific connection handle.
    ///
    /// For complete documentation on SQLDisconnect, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldisconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLDisconnect(self) -> (Result<SQLHDBC<'env, C2, V>, Self>, SQLRETURN)
    where
        Self: Disconnect,
    {
        let sql_return = unsafe { ffi::SQLDisconnect(self.as_SQLHANDLE()) };

        if SQL_SUCCEEDED(sql_return) {
            (Ok(self.disconnect()), sql_return)
        } else {
            (Err(self), sql_return)
        }
    }

    /// Returns the current setting of a connection attribute.
    ///
    /// For complete documentation on SQLGetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetconnectattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    #[allow(unused_variables)]
    pub fn SQLGetConnectAttrA<A: Ident<Type = SQLINTEGER>, T: ConnAttr<C, A, V>>(
        // TODO: Not sure whether attributes should be checked when getting them with SQLGetConnectAttr
        &self,
        Attribute: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            if cfg!(feature = "odbc_debug") {
                ValuePtr.assert_zeroed();
            }

            (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
        });

        unsafe {
            ffi::SQLGetConnectAttrA(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    #[allow(unused_variables)]
    pub fn SQLGetConnectAttrW<A: Ident<Type = SQLINTEGER>, T: ConnAttr<C, A, V>>(
        // TODO: Not really sure whether attributes should be checked when getting them with SQLGetConnectAttr
        &self,
        Attribute: A,
        ValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
    {
        let ValuePtr = ValuePtr.map_or((ptr::null_mut(), 0), |ValuePtr| {
            if cfg!(feature = "odbc_debug") {
                ValuePtr.assert_zeroed();
            }

            (ValuePtr.as_mut_SQLPOINTER(), ValuePtr.len())
        });

        unsafe {
            ffi::SQLGetConnectAttrW(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                ValuePtr.0,
                ValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
            )
        }
    }

    /// Sets attributes that govern aspects of connections.
    ///
    /// For complete documentation on SQLSetConnectAttrA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    #[allow(unused_variables)]
    pub fn SQLSetConnectAttrA<A: Ident<Type = SQLINTEGER>, T: ConnAttr<C, A, V>>(
        &self,
        Attribute: A,
        ValuePtr: T,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Ansi,
    {
        unsafe {
            ffi::SQLSetConnectAttrA(
                self.as_SQLHANDLE(),
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
    #[allow(unused_variables)]
    pub fn SQLSetConnectAttrW<A: Ident<Type = SQLINTEGER>, T: ConnAttr<C, A, V>>(
        &self,
        Attribute: A,
        ValuePtr: T,
    ) -> SQLRETURN
    where
        T: AttrSet<A> + Unicode,
    {
        unsafe {
            ffi::SQLSetConnectAttrW(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                ValuePtr.into_SQLPOINTER(),
                ValuePtr.len(),
            )
        }
    }
}

#[allow(non_snake_case)]
impl<'env, V: OdbcVersion> SQLHDBC<'env, C2, V> {
    /// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
    ///
    /// For complete documentation on SQLConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLConnectA(
        self,
        ServerName: &OdbcStr<SQLCHAR>,
        UserName: &OdbcStr<SQLCHAR>,
        Authentication: &OdbcStr<SQLCHAR>,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
        SQLRETURN,
    ) {
        let ServerName = ServerName.as_raw_slice();
        let UserName = UserName.as_raw_slice();
        let Authentication = Authentication.as_raw_slice();

        let sql_return = unsafe {
            ffi::SQLConnectA(
                self.as_SQLHANDLE(),
                ServerName.0,
                ServerName.1,
                UserName.0,
                UserName.1,
                Authentication.0,
                Authentication.1,
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            (Ok(self.connect()), sql_return)
        } else {
            (Err(self), sql_return)
        }
    }

    /// Establishes connections to a driver and a data source. The connection handle references storage of all information about the connection to the data source, including status, transaction state, and error information.
    ///
    /// For complete documentation on SQLConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLConnectW(
        self,
        ServerName: &OdbcStr<SQLWCHAR>,
        UserName: &OdbcStr<SQLWCHAR>,
        Authentication: &OdbcStr<SQLWCHAR>,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
        SQLRETURN,
    ) {
        let ServerName = ServerName.as_raw_slice();
        let UserName = UserName.as_raw_slice();
        let Authentication = Authentication.as_raw_slice();

        let sql_return = unsafe {
            ffi::SQLConnectW(
                self.as_SQLHANDLE(),
                ServerName.0,
                ServerName.1,
                UserName.0,
                UserName.1,
                Authentication.0,
                Authentication.1,
            )
        };

        if SQL_SUCCEEDED(sql_return) {
            (Ok(self.connect()), sql_return)
        } else {
            (Err(self), sql_return)
        }
    }

    /// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
    ///
    /// For complete documentation on SQLDriverConnectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLDriverConnectA(
        self,
        _WindowHandle: Option<SQLHWND>,
        InConnectionString: &OdbcStr<SQLCHAR>,
        OutConnectionString: Option<&mut OdbcStr<MaybeUninit<SQLCHAR>>>,
        StringLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
        DriverCompletion: DriverCompletion,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
        SQLRETURN,
    ) {
        let InConnectionString = InConnectionString.as_raw_slice();
        let OutConnectionString =
            OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        let sql_return = unsafe {
            ffi::SQLDriverConnectA(
                self.as_SQLHANDLE(),
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
            (Ok(self.connect()), sql_return)
        } else {
            (Err(self), sql_return)
        }
    }

    /// An alternative to **SQLConnect**. It supports data sources that require more connection information than the three arguments in **SQLConnect**, dialog boxes to prompt the user for all connection information, and data sources that are not defined in the system information. For more information, see Connecting with SQLDriverConnect.
    ///
    /// For complete documentation on SQLDriverConnectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqldriverconnect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_ERROR, SQL_INVALID_HANDLE, or SQL_STILL_EXECUTING.
    #[inline]
    pub fn SQLDriverConnectW(
        self,
        _WindowHandle: Option<SQLHWND>,
        InConnectionString: &OdbcStr<SQLWCHAR>,
        OutConnectionString: Option<&mut OdbcStr<MaybeUninit<SQLWCHAR>>>,
        StringLength2Ptr: &mut impl AsMutPtr<SQLSMALLINT>,
        DriverCompletion: DriverCompletion,
    ) -> (
        Result<SQLHDBC<'env, C4, V>, SQLHDBC<'env, C2, V>>,
        SQLRETURN,
    ) {
        let InConnectionString = InConnectionString.as_raw_slice();
        let OutConnectionString =
            OutConnectionString.map_or((ptr::null_mut(), 0), AsMutRawSlice::as_mut_raw_slice);

        let sql_return = unsafe {
            ffi::SQLDriverConnectW(
                self.as_SQLHANDLE(),
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
            (Ok(self.connect()), sql_return)
        } else {
            (Err(self), sql_return)
        }
    }
}

#[allow(non_snake_case)]
impl<'env, V: OdbcVersion> SQLHDBC<'env, C4, V> {
    /// Returns information about whether a driver supports a specific ODBC function. This function is implemented in the Driver Manager; it can also be implemented in drivers. If a driver implements **SQLGetFunctions**, the Driver Manager calls the function in the driver. Otherwise, it executes the function itself.
    ///
    /// For complete documentation on SQLGetFunctions, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetfunctions-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLGetFunctions(
        &self,
        FunctionId: FunctionId,
        SupportedPtr: &mut impl AsMutPtr<SQLUSMALLINT>,
    ) -> SQLRETURN {
        unsafe {
            ffi::SQLGetFunctions(
                self.as_SQLHANDLE(),
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
    #[allow(unused_variables)]
    pub fn SQLGetInfoA<A: Ident<Type = SQLUSMALLINT>, T: InfoType<A, V>>(
        // TODO: SQL_ODBC_VER can be called on connection that is not open
        &self,
        InfoType: A,
        InfoValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Ansi + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        let InfoValuePtr = InfoValuePtr.map_or((ptr::null_mut(), 0), |InfoValuePtr| {
            (InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len())
        });

        unsafe {
            ffi::SQLGetInfoA(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                InfoValuePtr.0,
                InfoValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
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
    #[allow(unused_variables)]
    pub fn SQLGetInfoW<A: Ident<Type = SQLUSMALLINT>, T: InfoType<A, V>>(
        // TODO: SQL_ODBC_VER can be called on connection that is not open
        &self,
        InfoType: A,
        InfoValuePtr: Option<&mut T>,
        StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
    ) -> SQLRETURN
    where
        T: AttrGet<A> + Unicode + ?Sized,
        MaybeUninit<T::StrLen>: StrLen<SQLSMALLINT>,
    {
        let InfoValuePtr = InfoValuePtr.map_or((ptr::null_mut(), 0), |InfoValuePtr| {
            (InfoValuePtr.as_mut_SQLPOINTER(), InfoValuePtr.len())
        });

        unsafe {
            ffi::SQLGetInfoW(
                self.as_SQLHANDLE(),
                A::IDENTIFIER,
                InfoValuePtr.0,
                InfoValuePtr.1,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
            )
        }
    }

    /// Returns the SQL string as modified by the driver. **SQLNativeSql** does not execute the SQL statement.
    ///
    /// For complete documentation on SQLNativeSqlA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlnativesql-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLNativeSqlA(
        &self,
        InStatementText: &OdbcStr<SQLCHAR>,
        OutStatementText: &mut OdbcStr<MaybeUninit<SQLCHAR>>,
        TextLength2Ptr: &mut impl AsMutPtr<SQLINTEGER>,
    ) -> SQLRETURN {
        let InStatementText = InStatementText.as_raw_slice();
        let OutStatementText = OutStatementText.as_mut_raw_slice();

        unsafe {
            ffi::SQLNativeSqlA(
                self.as_SQLHANDLE(),
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
    pub fn SQLNativeSqlW(
        &self,
        InStatementText: &OdbcStr<SQLWCHAR>,
        OutStatementText: &mut OdbcStr<MaybeUninit<SQLWCHAR>>,
        TextLength2Ptr: &mut impl AsMutPtr<SQLINTEGER>,
    ) -> SQLRETURN {
        let InStatementText = InStatementText.as_raw_slice();
        let OutStatementText = OutStatementText.as_mut_raw_slice();

        unsafe {
            ffi::SQLNativeSqlW(
                self.as_SQLHANDLE(),
                InStatementText.0,
                InStatementText.1,
                OutStatementText.0,
                OutStatementText.1,
                TextLength2Ptr.as_mut_ptr(),
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
    pub fn SQLEndTran(&self, CompletionType: CompletionType) -> SQLRETURN {
        // Not implemented on SQLHENV so as to to avoid confusion, considering that the same
        // functionality can be achieved by calling SQLEndTran repeatedly on SQLHDBC handle

        unsafe {
            ffi::SQLEndTran(
                <Self as Handle>::Ident::IDENTIFIER,
                self.as_SQLHANDLE(),
                CompletionType as SQLSMALLINT,
            )
        }
    }
}

#[allow(non_snake_case)]
impl<'desc, 'buf, V: OdbcVersion> SQLHSTMT<'_, 'desc, 'buf, V> {
    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// For complete documentation on SQLExecDirectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub fn SQLExecDirectA(&self, StatementText: &OdbcStr<SQLCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        unsafe { ffi::SQLExecDirectA(self.as_SQLHANDLE(), StatementText.0, StatementText.1) }
    }

    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// For complete documentation on SQLExecDirectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub fn SQLExecDirectW(&self, StatementText: &OdbcStr<SQLWCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        unsafe { ffi::SQLExecDirectW(self.as_SQLHANDLE(), StatementText.0, StatementText.1) }
    }

    /// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
    ///
    /// For complete documentation on SQLExecute, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub fn SQLExecute(&self) -> SQLRETURN {
        unsafe { ffi::SQLExecute(self.as_SQLHANDLE()) }
    }

    /// Fetches the next rowset of data from the result set and returns data for all bound columns.
    ///
    /// For complete documentation on SQLFetch, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLFetch(&self) -> SQLRETURN {
        unsafe { ffi::SQLFetch(self.as_SQLHANDLE()) }
    }

    /// Fetches the specified rowset of data from the result set and returns data for all bound columns. Rowsets can be specified at an absolute or relative position or by bookmark.
    ///
    /// For complete documentation on SQLFetchScroll, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetchscroll-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLFetchScroll(&self, FetchOrientation: SQLSMALLINT, FetchOffset: SQLLEN) -> SQLRETURN {
        unsafe { ffi::SQLFetchScroll(self.as_SQLHANDLE(), FetchOrientation, FetchOffset) }
    }

    /// Sets the cursor position in a rowset and allows an application to refresh data in the rowset or to update or delete data in the result set.
    ///
    /// For complete documentation on SQLSetPos, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetpos-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub fn SQLSetPos(
        &self,
        RowNumber: SQLSETPOSIROW,
        Operation: Operation,
        LockType: LockType,
    ) -> SQLRETURN {
        unsafe {
            ffi::SQLSetPos(
                self.as_SQLHANDLE(),
                RowNumber,
                Operation as SQLUSMALLINT,
                LockType as SQLUSMALLINT,
            )
        }
    }
}

#[allow(non_snake_case)]
impl<'desc, 'buf, V: OdbcVersion> UnsafeSQLHSTMT<'_, 'desc, 'buf, V> {
    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// For complete documentation on SQLExecDirectA, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub unsafe fn SQLExecDirectA(&self, StatementText: &OdbcStr<SQLCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        ffi::SQLExecDirectA(self.as_SQLHANDLE(), StatementText.0, StatementText.1)
    }

    /// Executes a preparable statement, using the current values of the parameter marker variables if any parameters exist in the statement. **SQLExecDirect** is the fastest way to submit an SQL statement for one-time execution.
    ///
    /// For complete documentation on SQLExecDirectW, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecdirect-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub unsafe fn SQLExecDirectW(&self, StatementText: &OdbcStr<SQLWCHAR>) -> SQLRETURN {
        let StatementText = StatementText.as_raw_slice();

        ffi::SQLExecDirectW(self.as_SQLHANDLE(), StatementText.0, StatementText.1)
    }

    /// Executes a prepared statement, using the current values of the parameter marker variables if any parameter markers exist in the statement.
    ///
    /// For complete documentation on SQLExecute, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlexecute-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, SQL_NO_DATA, SQL_INVALID_HANDLE, or SQL_PARAM_DATA_AVAILABLE.
    #[inline]
    pub unsafe fn SQLExecute(&self) -> SQLRETURN {
        ffi::SQLExecute(self.as_SQLHANDLE())
    }

    /// Fetches the next rowset of data from the result set and returns data for all bound columns.
    ///
    /// For complete documentation on SQLFetch, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetch-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub unsafe fn SQLFetch(&self) -> SQLRETURN {
        ffi::SQLFetch(self.as_SQLHANDLE())
    }

    /// Fetches the specified rowset of data from the result set and returns data for all bound columns. Rowsets can be specified at an absolute or relative position or by bookmark.
    ///
    /// For complete documentation on SQLFetchScroll, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlfetchscroll-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NO_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub unsafe fn SQLFetchScroll(
        &self,
        FetchOrientation: SQLSMALLINT,
        FetchOffset: SQLLEN,
    ) -> SQLRETURN {
        ffi::SQLFetchScroll(self.as_SQLHANDLE(), FetchOrientation, FetchOffset)
    }

    /// Sets the cursor position in a rowset and allows an application to refresh data in the rowset or to update or delete data in the result set.
    ///
    /// For complete documentation on SQLSetPos, see [API reference](https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetpos-function).
    ///
    /// # Returns
    /// SQL_SUCCESS, SQL_SUCCESS_WITH_INFO, SQL_NEED_DATA, SQL_STILL_EXECUTING, SQL_ERROR, or SQL_INVALID_HANDLE.
    #[inline]
    pub unsafe fn SQLSetPos(
        &self,
        RowNumber: SQLSETPOSIROW,
        Operation: Operation,
        LockType: LockType,
    ) -> SQLRETURN {
        ffi::SQLSetPos(
            self.as_SQLHANDLE(),
            RowNumber,
            Operation as SQLUSMALLINT,
            LockType as SQLUSMALLINT,
        )
    }
}

#[allow(non_snake_case)]
impl<'conn, 'desc, 'buf, V: OdbcVersion> Statement<'desc, 'buf, V>
    for SQLHSTMT<'conn, 'desc, 'buf, V>
{
    // TODO: When GATs are implemented use 'stmt instead of 'conn
    // because implicit descriptors are managed by the DM
    type ARD = RefSQLHDESC<'conn, AppDesc<'buf>, V>;
    type APD = RefSQLHDESC<'conn, AppDesc<'buf>, V>;
    type IRD = RefSQLHDESC<'conn, IRD, V>;
    type IPD = RefSQLHDESC<'conn, IPD, V>;

    type ExplicitARD = SQLHDESC<'conn, AppDesc<'buf>, V>;
    type ExplicitAPD = SQLHDESC<'conn, AppDesc<'buf>, V>;

    fn bind_col<TT: Ident, B: DeferredBuf<Self::ARD, TT, V>>(&self, _: Option<&'buf B>)
    where
        B: ?Sized,
    {
        //TODO:
        //self.0.bind_col(TargetValuePtr)
    }

    fn bind_param<TT: Ident, B: DeferredBuf<Self::APD, TT, V>>(&self, _: Option<&'buf B>)
    where
        B: ?Sized,
    {
        // TODO:
        //self.0.bind_param(TargetValuePtr)
    }

    fn bind_strlen_or_ind(&self, StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>) {
        self.0.bind_strlen_or_ind(StrLen_or_IndPtr)
    }
}

#[allow(non_snake_case)]
impl<'conn, 'desc, 'buf, V: OdbcVersion> Statement<'desc, 'buf, V>
    for UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>
{
    // TODO: When GATs are implemented use 'stmt instead of 'conn
    // because implicit descriptors are managed by the DM
    type ARD = RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>;
    type APD = RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>;
    type IRD = RefUnsafeSQLHDESC<'conn, IRD, V>;
    type IPD = RefUnsafeSQLHDESC<'conn, IPD, V>;

    type ExplicitARD = UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>;
    type ExplicitAPD = UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>;

    // TODO: Don't bind (SQLPOINTER, SQLLEN) fat pointer when using raw_api
    #[cfg(not(feature = "odbc_debug"))]
    fn bind_col<TT: Ident, B: DeferredBuf<Self::ARD, TT, V>>(&self, _: Option<&'buf B>)
    where
        B: ?Sized,
    {
    }
    #[cfg(not(feature = "odbc_debug"))]
    fn bind_param<TT: Ident, B: DeferredBuf<Self::APD, TT, V>>(&self, _: Option<&'buf B>)
    where
        B: ?Sized,
    {
    }
    #[cfg(not(feature = "odbc_debug"))]
    fn bind_strlen_or_ind(&self, _: Option<&'buf UnsafeCell<StrLenOrInd>>) {}

    #[cfg(feature = "odbc_debug")]
    fn bind_col<TT: Ident, B: DeferredBuf<Self::ARD, TT, V>>(&self, _: Option<&'buf B>)
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
    fn bind_param<TT: Ident, B: DeferredBuf<Self::APD, TT, V>>(
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
    fn bind_strlen_or_ind(&self, StrLen_or_IndPtr: Option<&'buf UnsafeCell<StrLenOrInd>>) {
        unimplemented!();
    }
}

impl<'conn, 'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V>
    for SQLHDESC<'conn, DT, V>
{
}
impl<'conn, 'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V>
    for RefSQLHDESC<'conn, DT, V>
{
}
impl<'conn, 'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V>
    for UnsafeSQLHDESC<'conn, DT, V>
{
}
impl<'conn, 'buf, DT: DescType<'buf>, V: OdbcVersion> Descriptor<'buf, DT, V>
    for RefUnsafeSQLHDESC<'conn, DT, V>
{
}

// TODO: If Connection trait is introduced implement for all connections
impl Cancel<SQL_OV_ODBC3_80> for SQLHDBC<'_, C4, SQL_OV_ODBC3_80> {}
impl Cancel<SQL_OV_ODBC4> for SQLHDBC<'_, C4, SQL_OV_ODBC4> {}

impl Async<SQL_OV_ODBC3_80> for SQLHDBC<'_, C4, SQL_OV_ODBC3_80> {}
impl Async<SQL_OV_ODBC4> for SQLHDBC<'_, C4, SQL_OV_ODBC4> {}

impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3_80>> Cancel<SQL_OV_ODBC3_80> for S {}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC4>> Cancel<SQL_OV_ODBC4> for S {}

impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3_80>> Async<SQL_OV_ODBC3_80> for S {}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC4>> Async<SQL_OV_ODBC4> for S {}

#[allow(non_snake_case, unused_variables)]
fn SQLGetStmtAttrA<
    'stmt,
    'desc,
    'buf,
    S: Statement<'desc, 'buf, V>,
    A: Ident<Type = SQLINTEGER>,
    T: BaseStmtAttr<'desc, 'buf, S, A, V>,
    V: OdbcVersion,
>(
    Handle: &'stmt S,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + Ansi + Ref<'stmt> + ?Sized,
    MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
{
    if let Some(ValuePtr) = ValuePtr {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        ValuePtr.readA(Handle, StringLengthPtr)
    } else {
        unsafe {
            ffi::SQLGetStmtAttrA(
                Handle.as_SQLHANDLE(),
                A::IDENTIFIER,
                ptr::null_mut(),
                0,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
            )
        }
    }
}

#[allow(non_snake_case, unused_variables)]
fn SQLGetStmtAttrW<
    'stmt,
    'desc,
    'buf,
    S: Statement<'desc, 'buf, V>,
    A: Ident<Type = SQLINTEGER>,
    T: BaseStmtAttr<'desc, 'buf, S, A, V>,
    V: OdbcVersion,
>(
    Handle: &'stmt S,
    Attribute: A,
    ValuePtr: Option<&mut T>,
    StringLengthPtr: Option<&mut MaybeUninit<T::StrLen>>,
) -> SQLRETURN
where
    T: AttrGet<A> + Unicode + Ref<'stmt> + ?Sized,
    MaybeUninit<T::StrLen>: StrLen<SQLINTEGER>,
{
    if let Some(ValuePtr) = ValuePtr {
        if cfg!(feature = "odbc_debug") {
            ValuePtr.assert_zeroed();
        }

        ValuePtr.readW(Handle, StringLengthPtr)
    } else {
        unsafe {
            ffi::SQLGetStmtAttrW(
                Handle.as_SQLHANDLE(),
                A::IDENTIFIER,
                ptr::null_mut(),
                0,
                StringLengthPtr.map_or_else(ptr::null_mut, StrLen::as_mut_ptr),
            )
        }
    }
}

#[allow(non_snake_case, unused_variables)]
fn SQLSetStmtAttrA<
    'desc,
    'buf,
    S: Statement<'desc, 'buf, V>,
    A: Ident<Type = SQLINTEGER>,
    T: BaseStmtAttr<'desc, 'buf, S, A, V>,
    V: OdbcVersion,
>(
    Handle: &S,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + Ansi,
{
    let sql_return = unsafe {
        ffi::SQLSetStmtAttrA(
            Handle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.update_handle(Handle);
    }

    sql_return
}

#[allow(non_snake_case, unused_variables)]
fn SQLSetStmtAttrW<
    'desc,
    'buf,
    S: Statement<'desc, 'buf, V>,
    A: Ident<Type = SQLINTEGER>,
    T: BaseStmtAttr<'desc, 'buf, S, A, V>,
    V: OdbcVersion,
>(
    Handle: &S,
    Attribute: A,
    ValuePtr: T,
) -> SQLRETURN
where
    T: AttrSet<A> + Unicode,
{
    let sql_return = unsafe {
        ffi::SQLSetStmtAttrW(
            Handle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.into_SQLPOINTER(),
            ValuePtr.len(),
        )
    };

    if SQL_SUCCEEDED(sql_return) {
        ValuePtr.update_handle(Handle);
    }

    sql_return
}

#[cfg_attr(test, automock)]
pub(crate) mod ffi {
    use crate::handle::SQLHWND;
    use crate::{
        diag::SQLSTATE_SIZE, handle::SQLHANDLE, sqlreturn::SQLRETURN, RETCODE, SQLCHAR, SQLINTEGER,
        SQLLEN, SQLPOINTER, SQLSETPOSIROW, SQLSMALLINT, SQLULEN, SQLUSMALLINT, SQLWCHAR,
    };

    type HENV = SQLHANDLE;
    type HDBC = SQLHANDLE;
    type HSTMT = SQLHANDLE;
    type HDESC = SQLHANDLE;

    type ConstSQLPOINTER = *const core::ffi::c_void;
    type MutSQLPOINTER = *mut core::ffi::c_void;

    // TODO: static linking is not supported for windows
    #[cfg_attr(windows, link(name = "odbc32", kind = "dylib"))]
    #[cfg_attr(
        all(not(windows), feature = "static"),
        link(name = "odbc", kind = "static")
    )]
    #[cfg_attr(
        all(not(windows), not(feature = "static")),
        link(name = "odbc", kind = "dylib")
    )]
    extern "system" {
        #[allow(non_snake_case)]
        pub fn SQLAllocHandle(
            HandleType: SQLSMALLINT,
            InputHandle: SQLHANDLE,
            OutputHandlePtr: *mut SQLHANDLE,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLBindCol(
            StatementHandle: HSTMT,
            ColumnNumber: SQLUSMALLINT,
            TargetType: SQLSMALLINT,
            TargetValuePtr: MutSQLPOINTER,
            BufferLength: SQLLEN,
            StrLen_or_IndPtr: *mut SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLBindParameter(
            StatementHandle: HSTMT,
            ParameterNumber: SQLUSMALLINT,
            InputOutputType: SQLSMALLINT,
            ValueType: SQLSMALLINT,
            ParameterType: SQLSMALLINT,
            ColumnSize: SQLULEN,
            DecimalDigits: SQLSMALLINT,
            ParameterValuePtr: SQLPOINTER,
            BufferLength: SQLLEN,
            StrLen_or_IndPtr: *const SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLBrowseConnectA(
            ConnectionHandle: HDBC,
            InConnectionString: *const SQLCHAR,
            StringLength1: SQLSMALLINT,
            OutConnectionString: *mut SQLCHAR,
            BufferLength: SQLSMALLINT,
            StringLength2Ptr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLBrowseConnectW(
            ConnectionHandle: HDBC,
            InConnectionString: *const SQLWCHAR,
            StringLength1: SQLSMALLINT,
            OutConnectionString: *mut SQLWCHAR,
            BufferLength: SQLSMALLINT,
            StringLength2Ptr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLBulkOperations(StatementHandle: HSTMT, Operation: SQLUSMALLINT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLCancel(StatementHandle: HSTMT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLCancelHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLCloseCursor(StatementHandle: HSTMT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLColAttributeA(
            StatementHandle: HSTMT,
            ColumnNumber: SQLUSMALLINT,
            FieldIdentifier: SQLUSMALLINT,
            CharacterAttributePtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
            NumericAttributePtr: *mut SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLColAttributeW(
            StatementHandle: HSTMT,
            ColumnNumber: SQLUSMALLINT,
            FieldIdentifier: SQLUSMALLINT,
            CharacterAttributePtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
            NumericAttributePtr: *mut SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLColumnPrivilegesA(
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

        #[allow(non_snake_case)]
        pub fn SQLColumnPrivilegesW(
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

        #[allow(non_snake_case)]
        pub fn SQLColumnsA(
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

        #[allow(non_snake_case)]
        pub fn SQLColumnsW(
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

        #[allow(non_snake_case)]
        pub fn SQLCompleteAsync(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            AsyncRetCodePtr: *mut RETCODE,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLConnectA(
            ConnectionHandle: HDBC,
            ServerName: *const SQLCHAR,
            NameLength1: SQLSMALLINT,
            UserName: *const SQLCHAR,
            NameLength2: SQLSMALLINT,
            Authentication: *const SQLCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLConnectW(
            ConnectionHandle: HDBC,
            ServerName: *const SQLWCHAR,
            NameLength1: SQLSMALLINT,
            UserName: *const SQLWCHAR,
            NameLength2: SQLSMALLINT,
            Authentication: *const SQLWCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLCopyDesc(SourceDescHandle: HDESC, TargetDescHandle: HDESC) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDataSourcesA(
            EnvironmentHandle: HENV,
            Direction: SQLUSMALLINT,
            ServerName: *mut SQLCHAR,
            BufferLength1: SQLSMALLINT,
            NameLength1Ptr: *mut SQLSMALLINT,
            Description: *mut SQLCHAR,
            BufferLength2: SQLSMALLINT,
            NameLength2Ptr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDataSourcesW(
            EnvironmentHandle: HENV,
            Direction: SQLUSMALLINT,
            ServerName: *mut SQLWCHAR,
            BufferLength1: SQLSMALLINT,
            NameLength1Ptr: *mut SQLSMALLINT,
            Description: *mut SQLWCHAR,
            BufferLength2: SQLSMALLINT,
            NameLength2Ptr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDescribeColA(
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

        #[allow(non_snake_case)]
        pub fn SQLDescribeColW(
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

        #[allow(non_snake_case)]
        pub fn SQLDescribeParam(
            StatementHandle: HSTMT,
            ParameterNumber: SQLUSMALLINT,
            DataTypePtr: *mut SQLSMALLINT,
            ParameterSizePtr: *mut SQLULEN,
            DecimalDigitsPtr: *mut SQLSMALLINT,
            NullablePtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDisconnect(ConnectionHandle: HDBC) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDriverConnectA(
            ConnectionHandle: HDBC,
            WindowHandle: SQLHWND,
            InConnectionString: *const SQLCHAR,
            StringLength1: SQLSMALLINT,
            OutConnectionString: *mut SQLCHAR,
            BufferLength: SQLSMALLINT,
            StringLength2Ptr: *mut SQLSMALLINT,
            DriverCompletion: SQLUSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDriverConnectW(
            ConnectionHandle: HDBC,
            WindowHandle: SQLHWND,
            InConnectionString: *const SQLWCHAR,
            StringLength1: SQLSMALLINT,
            OutConnectionString: *mut SQLWCHAR,
            BufferLength: SQLSMALLINT,
            StringLength2Ptr: *mut SQLSMALLINT,
            DriverCompletion: SQLUSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDriversA(
            EnvironmentHandle: HENV,
            Direction: SQLUSMALLINT,
            DriverDescription: *mut SQLCHAR,
            BufferLength1: SQLSMALLINT,
            DescriptionLengthPtr: *mut SQLSMALLINT,
            DriverAttributes: *mut SQLCHAR,
            BufferLength2: SQLSMALLINT,
            AttributesLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLDriversW(
            EnvironmentHandle: HENV,
            Direction: SQLUSMALLINT,
            DriverDescription: *mut SQLWCHAR,
            BufferLength1: SQLSMALLINT,
            DescriptionLengthPtr: *mut SQLSMALLINT,
            DriverAttributes: *mut SQLWCHAR,
            BufferLength2: SQLSMALLINT,
            AttributesLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLEndTran(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            CompletionType: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLExecDirectA(
            StatementHandle: HSTMT,
            StatementText: *const SQLCHAR,
            TextLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLExecDirectW(
            StatementHandle: HSTMT,
            StatementText: *const SQLWCHAR,
            TextLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLExecute(StatementHandle: HSTMT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLFetch(StatementHandle: HSTMT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLFetchScroll(
            StatementHandle: HSTMT,
            FetchOrientation: SQLSMALLINT,
            FetchOffset: SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLForeignKeysA(
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

        #[allow(non_snake_case)]
        pub fn SQLForeignKeysW(
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

        #[allow(non_snake_case)]
        pub fn SQLFreeHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLFreeStmt(StatementHandle: HSTMT, Option: SQLUSMALLINT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetConnectAttrA(
            ConnectionHandle: HDBC,
            Attribute: SQLINTEGER,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetConnectAttrW(
            ConnectionHandle: HDBC,
            Attribute: SQLINTEGER,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetCursorNameA(
            StatementHandle: HSTMT,
            CursorName: *mut SQLCHAR,
            BufferLength: SQLSMALLINT,
            NameLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetCursorNameW(
            StatementHandle: HSTMT,
            CursorName: *mut SQLWCHAR,
            BufferLength: SQLSMALLINT,
            NameLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetData(
            StatementHandle: HSTMT,
            Col_or_Param_Num: SQLUSMALLINT,
            TargetType: SQLSMALLINT,
            TargetValuePtr: MutSQLPOINTER,
            BufferLength: SQLLEN,
            StrLen_or_IndPtr: *mut SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDescFieldA(
            DescriptorHandle: HDESC,
            RecNumber: SQLSMALLINT,
            FieldIdentifier: SQLSMALLINT,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDescFieldW(
            DescriptorHandle: HDESC,
            RecNumber: SQLSMALLINT,
            FieldIdentifier: SQLSMALLINT,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDescRecA(
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

        #[allow(non_snake_case)]
        pub fn SQLGetDescRecW(
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

        #[allow(non_snake_case)]
        pub fn SQLGetDiagFieldA(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            RecNumber: SQLSMALLINT,
            DiagIdentifier: SQLSMALLINT,
            DiagInfoPtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDiagFieldW(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            RecNumber: SQLSMALLINT,
            DiagIdentifier: SQLSMALLINT,
            DiagInfoPtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDiagRecA(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            RecNumber: SQLSMALLINT,
            SQLState: *mut [SQLCHAR; SQLSTATE_SIZE + 1],
            NativeErrorPtr: *mut SQLINTEGER,
            MessageText: *mut SQLCHAR,
            BufferLength: SQLSMALLINT,
            TextLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetDiagRecW(
            HandleType: SQLSMALLINT,
            Handle: SQLHANDLE,
            RecNumber: SQLSMALLINT,
            SQLState: *mut [SQLWCHAR; SQLSTATE_SIZE + 1],
            NativeErrorPtr: *mut SQLINTEGER,
            MessageText: *mut SQLWCHAR,
            BufferLength: SQLSMALLINT,
            TextLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetEnvAttr(
            EnvironmentHandle: HENV,
            Attribute: SQLINTEGER,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetFunctions(
            ConnectionHandle: HDBC,
            FunctionId: SQLUSMALLINT,
            SupportedPtr: *mut SQLUSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetInfoA(
            ConnectionHandle: HDBC,
            InfoType: SQLUSMALLINT,
            InfoValuePtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetInfoW(
            ConnectionHandle: HDBC,
            InfoType: SQLUSMALLINT,
            InfoValuePtr: MutSQLPOINTER,
            BufferLength: SQLSMALLINT,
            StringLengthPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetStmtAttrA(
            StatementHandle: HSTMT,
            Attribute: SQLINTEGER,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetStmtAttrW(
            StatementHandle: HSTMT,
            Attribute: SQLINTEGER,
            ValuePtr: MutSQLPOINTER,
            BufferLength: SQLINTEGER,
            StringLengthPtr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetTypeInfoA(StatementHandle: HSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLGetTypeInfoW(StatementHandle: HSTMT, DataType: SQLSMALLINT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLMoreResults(StatementHandle: HSTMT) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLNativeSqlA(
            ConnectionHandle: HDBC,
            InStatementText: *const SQLCHAR,
            TextLength1: SQLINTEGER,
            OutStatementText: *mut SQLCHAR,
            BufferLength: SQLINTEGER,
            TextLength2Ptr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLNativeSqlW(
            ConnectionHandle: HDBC,
            InStatementText: *const SQLWCHAR,
            TextLength1: SQLINTEGER,
            OutStatementText: *mut SQLWCHAR,
            BufferLength: SQLINTEGER,
            TextLength2Ptr: *mut SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLNumParams(
            StatementHandle: HSTMT,
            ParameterCountPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLNumResultCols(
            StatementHandle: HSTMT,
            ColumnCountPtr: *mut SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLParamData(StatementHandle: HSTMT, ValuePtrPtr: *mut MutSQLPOINTER) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLPrepareA(
            StatementHandle: HSTMT,
            StatementText: *const SQLCHAR,
            TextLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLPrepareW(
            StatementHandle: HSTMT,
            StatementText: *const SQLWCHAR,
            TextLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLPrimaryKeysA(
            StatementHandle: HSTMT,
            CatalogName: *const SQLCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLCHAR,
            NameLength2: SQLSMALLINT,
            TableName: *const SQLCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLPrimaryKeysW(
            StatementHandle: HSTMT,
            CatalogName: *const SQLWCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLWCHAR,
            NameLength2: SQLSMALLINT,
            TableName: *const SQLWCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLProcedureColumnsA(
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

        #[allow(non_snake_case)]
        pub fn SQLProcedureColumnsW(
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

        #[allow(non_snake_case)]
        pub fn SQLProceduresA(
            StatementHandle: HSTMT,
            CatalogName: *const SQLCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLCHAR,
            NameLength2: SQLSMALLINT,
            ProcName: *const SQLCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLProceduresW(
            StatementHandle: HSTMT,
            CatalogName: *const SQLWCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLWCHAR,
            NameLength2: SQLSMALLINT,
            ProcName: *const SQLWCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLPutData(
            StatementHandle: HSTMT,
            DataPtr: ConstSQLPOINTER,
            StrLen_or_Ind: SQLLEN,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLRowCount(StatementHandle: HSTMT, RowCountPtr: *mut SQLLEN) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetConnectAttrA(
            ConnectionHandle: HDBC,
            Attribute: SQLINTEGER,
            ValuePtr: ConstSQLPOINTER,
            StringLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetConnectAttrW(
            ConnectionHandle: HDBC,
            Attribute: SQLINTEGER,
            ValuePtr: ConstSQLPOINTER,
            StringLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetCursorNameA(
            StatementHandle: HSTMT,
            CursorName: *const SQLCHAR,
            NameLength: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetCursorNameW(
            StatementHandle: HSTMT,
            CursorName: *const SQLWCHAR,
            NameLength: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetDescFieldA(
            DescriptorHandle: HDESC,
            RecNumber: SQLSMALLINT,
            FieldIdentifier: SQLSMALLINT,
            ValuePtr: ConstSQLPOINTER,
            BufferLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetDescFieldW(
            DescriptorHandle: HDESC,
            RecNumber: SQLSMALLINT,
            FieldIdentifier: SQLSMALLINT,
            ValuePtr: ConstSQLPOINTER,
            BufferLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetDescRec(
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

        #[allow(non_snake_case)]
        pub fn SQLSetEnvAttr(
            EnvironmentHandle: HENV,
            Attribute: SQLINTEGER,
            ValuePtr: ConstSQLPOINTER,
            StringLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetPos(
            StatementHandle: HSTMT,
            RowNumber: SQLSETPOSIROW,
            Operation: SQLUSMALLINT,
            LockType: SQLUSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetStmtAttrA(
            StatementHandle: HSTMT,
            Attribute: SQLINTEGER,
            ValuePtr: ConstSQLPOINTER,
            StringLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSetStmtAttrW(
            StatementHandle: HSTMT,
            Attribute: SQLINTEGER,
            ValuePtr: ConstSQLPOINTER,
            StringLength: SQLINTEGER,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLSpecialColumnsA(
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

        #[allow(non_snake_case)]
        pub fn SQLSpecialColumnsW(
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

        #[allow(non_snake_case)]
        pub fn SQLStatisticsA(
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

        #[allow(non_snake_case)]
        pub fn SQLStatisticsW(
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

        #[allow(non_snake_case)]
        pub fn SQLTablePrivilegesA(
            StatementHandle: HSTMT,
            CatalogName: *const SQLCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLCHAR,
            NameLength2: SQLSMALLINT,
            TableName: *const SQLCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLTablePrivilegesW(
            StatementHandle: HSTMT,
            CatalogName: *const SQLWCHAR,
            NameLength1: SQLSMALLINT,
            SchemaName: *const SQLWCHAR,
            NameLength2: SQLSMALLINT,
            TableName: *const SQLWCHAR,
            NameLength3: SQLSMALLINT,
        ) -> SQLRETURN;

        #[allow(non_snake_case)]
        pub fn SQLTablesA(
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

        #[allow(non_snake_case)]
        pub fn SQLTablesW(
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
}
