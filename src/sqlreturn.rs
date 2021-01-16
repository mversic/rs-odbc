use crate::SQLSMALLINT;

/// Each function in ODBC returns a code, known as its return code, which indicates the
/// overall success or failure of the function. Program logic is generally based on return
/// codes.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/return-codes-odbc
/// https://github.com/microsoft/ODBC-Specification/blob/ODBC%204.0.md
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct SQLRETURN(SQLSMALLINT);

impl SQLRETURN {
    pub const fn raw_value(&self) -> SQLSMALLINT {
        self.0
    }
}

/// Function completed successfully. The application calls SQLGetDiagField to retrieve
/// additional information from the header record.
pub const SQL_SUCCESS: SQLRETURN = SQLRETURN(0);

/// Function completed successfully, possibly with a nonfatal error (warning). The
/// application calls SQLGetDiagRec or SQLGetDiagField to retrieve additional
/// information.
pub const SQL_SUCCESS_WITH_INFO: SQLRETURN = SQLRETURN(1);

/// Function failed. The application calls SQLGetDiagRec or SQLGetDiagField to
/// retrieve additional information. The contents of any output arguments to the
/// function are undefined.
pub const SQL_ERROR: SQLRETURN = SQLRETURN(-1);

/// Function failed due to an invalid environment, connection, statement, or
/// descriptor handle. This indicates a programming error. No additional information
/// is available from SQLGetDiagRec or SQLGetDiagField. This code is returned only
/// when the handle is a null pointer or is the wrong type, such as when a statement
/// handle is passed for an argument that requires a connection handle.
pub const SQL_INVALID_HANDLE: SQLRETURN = SQLRETURN(-2);

/// No more data was available. The application calls SQLGetDiagRec or SQLGetDiagField
/// to retrieve additional information. One or more driver-defined status records in
/// class 02xxx may be returned. Note:  In ODBC 2.x, this return code was named
/// SQL_NO_DATA_FOUND.
pub const SQL_NO_DATA: SQLRETURN = SQLRETURN(100);

/// More data is needed, such as when parameter data is sent at execution time or
/// additional connection information is required. The application calls SQLGetDiagRec
/// or SQLGetDiagField to retrieve additional information, if any.
pub const SQL_NEED_DATA: SQLRETURN = SQLRETURN(99);

/// A function that was started asynchronously is still executing. The application
/// calls SQLGetDiagRec or SQLGetDiagField to retrieve additional information, if any.
pub const SQL_STILL_EXECUTING: SQLRETURN = SQLRETURN(2);

/// Indicates that there are streamed output parameters available for the next set of
/// parameters to retrieve.
#[cfg(feature = "v3_8")]
pub const SQL_PARAM_DATA_AVAILABLE: SQLRETURN = SQLRETURN(101);

/// Signals data-at-fetch columns are available.
#[cfg(feature = "v4")]
pub const SQL_DATA_AVAILABLE: SQLRETURN = SQLRETURN(102);

/// The descriptor is changed by the driver when reading a column.
#[cfg(feature = "v4")]
pub const SQL_METADATA_CHANGED: SQLRETURN = SQLRETURN(103);

/// The driver does not know how much additional data is to be written.
#[cfg(feature = "v4")]
pub const SQL_MORE_DATA: SQLRETURN = SQLRETURN(104);
