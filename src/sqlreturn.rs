use co3::{Error, ReprC};
use rust_spec::RustSpec;

/// Each function in ODBC returns a code, known as its return code, which indicates the
/// overall success or failure of the function. Program logic is generally based on return
/// codes.
///
/// # Documentation
/// https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/return-codes-odbc
/// https://github.com/microsoft/ODBC-Specification/blob/ODBC%204.0.md
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[must_use]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum RETURN {
    /// Function failed. The application calls SQLGetDiagRec or SQLGetDiagField to
    /// retrieve additional information. The contents of any output arguments to the
    /// function are undefined.
    ERROR = -1,

    // TODO: Can this error occur through the typed public API?
    /// Function failed due to an invalid environment, connection, statement, or
    /// descriptor handle. This indicates a programming error. No additional information
    /// is available from SQLGetDiagRec or SQLGetDiagField. This code is returned only
    /// when the handle is a null pointer or is the wrong type, such as when a statement
    /// handle is passed for an argument that requires a connection handle.
    INVALID_HANDLE = -2,

    /// Function completed successfully. The application calls SQLGetDiagField to retrieve
    /// additional information from the header record.
    SUCCESS = 0,

    /// Function completed successfully, possibly with a nonfatal error (warning). The
    /// application calls SQLGetDiagRec or SQLGetDiagField to retrieve additional
    /// information.
    SUCCESS_WITH_INFO = 1,

    /// A function that was started asynchronously is still executing. The application
    /// calls SQLGetDiagRec or SQLGetDiagField to retrieve additional information, if any.
    STILL_EXECUTING = 2,

    /// More data is needed, such as when parameter data is sent at execution time or
    /// additional connection information is required. The application calls SQLGetDiagRec
    /// or SQLGetDiagField to retrieve additional information, if any.
    NEED_DATA = 99,

    /// No more data was available. The application calls SQLGetDiagRec or SQLGetDiagField
    /// to retrieve additional information. One or more driver-defined status records in
    /// class 02xxx may be returned. Note: In ODBC 2.x, this return code was named
    /// NO_DATA_FOUND.
    NO_DATA = 100,

    /// Indicates that there are streamed output parameters available for the next set of
    /// parameters to retrieve.
    PARAM_DATA_AVAILABLE = 101,

    /// Signals data-at-fetch columns are available.
    DATA_AVAILABLE = 102,

    /// The descriptor is changed by the driver when reading a column.
    METADATA_CHANGED = 103,

    /// The driver does not know how much additional data is to be written.
    MORE_DATA = 104,
}

impl Error for RETURN {
    fn trap_value() -> Self {
        RETURN::ERROR
    }

    fn unknown_tag() -> Self {
        RETURN::INVALID_HANDLE
    }

    fn soft_sync_error() -> Self {
        RETURN::ERROR
    }
}

/// Returns whether an ODBC return code represents success, with or without diagnostic information.
#[expect(non_snake_case)]
pub fn SUCCEEDED<T: Into<RETURN>>(ret: T) -> bool {
    matches!(ret.into(), RETURN::SUCCESS | RETURN::SUCCESS_WITH_INFO)
}
