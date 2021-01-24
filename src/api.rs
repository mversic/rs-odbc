use crate::{
    env::EnvAttribute,
    handle::{Allocate, AsSQLHANDLE, HandleIdentifier, HDBC, HENV, SQLHDBC},
    AnsiType, AsMutPtr, AsMutRawSlice, AsMutSQLCHARRawSlice, AsRawParts, AsSQLCHARRawSlice,
    DriverCompletion, GetAttr, OdbcAttribute, SetAttr, SQLCHAR, SQLHANDLE, SQLHENV, SQLINTEGER,
    SQLPOINTER, SQLRETURN, SQLSMALLINT, SQLUSMALLINT,
};
use std::mem::MaybeUninit;

// TODO: Fix linking
// static linking is not currently supported here for windows
#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(all(not(windows), not(r#static)), link(name = "odbc"))]
#[cfg_attr(all(not(windows), r#static), link(name = "odbc", kind = "static"))]
extern "system" {
    #[link_name = "SQLAllocHandle"]
    fn AllocHandle(
        HandleType: SQLSMALLINT,
        InputHandle: SQLHANDLE,
        OutputHandlePtr: *mut SQLHANDLE,
    ) -> SQLRETURN;

    #[link_name = "SQLFreeHandle"]
    pub(crate) fn FreeHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;

    #[link_name = "SQLSetEnvAttr"]
    fn SetEnvAttr(
        EnvironmentHandle: HENV,
        Attribute: SQLINTEGER,
        ValuePtr: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    #[link_name = "SQLGetEnvAttr"]
    fn GetEnvAttr(
        EnvironmentHandle: HENV,
        Attribute: SQLINTEGER,
        ValuePtr: SQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    #[link_name = "SQLDriverConnectA"]
    fn DriverConnectA(
        ConnectionHandle: HDBC,
        WindowHandle: SQLHANDLE,
        InConnectionString: *const SQLCHAR,
        StringLength1: SQLSMALLINT,
        OutConnectionString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLength2Ptr: *mut SQLSMALLINT,
        DriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;

    #[link_name = "SQLDisconnect"]
    pub(crate) fn Disconnect(ConnectionHandle: HDBC) -> SQLRETURN;
}

#[inline]
pub fn SQLAllocHandle<'src, OH: Allocate<'src>>(
    HandleType: OH::Identifier,
    InputHandle: &'src mut OH::SrcHandle,
    OutputHandlePtr: &mut MaybeUninit<OH>,
) -> SQLRETURN
where
    OH: 'src,
{
    unsafe {
        AllocHandle(
            OH::Identifier::identifier(),
            InputHandle.as_SQLHANDLE(),
            OutputHandlePtr.as_mut_ptr().cast(),
        )
    }
}

#[inline]
pub fn SQLSetEnvAttr<A: EnvAttribute, T: AnsiType>(
    EnvironmentHandle: &mut SQLHENV,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<T>,
    T: AsRawParts<OdbcAttribute, SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_raw_parts();

    unsafe {
        SetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::identifier(),
            ValuePtr.0,
            ValuePtr.1,
        )
    }
}

pub fn SQLGetEnvAttr<A: EnvAttribute, T: AnsiType>(
    EnvironmentHandle: &mut SQLHENV,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<T>,
    T: AsMutRawSlice<OdbcAttribute, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_mut_raw_slice();

    unsafe {
        GetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::identifier(),
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.as_mut_ptr().cast(),
        )
    }
}

pub fn SQLDriverConnectA<
    C: AsSQLCHARRawSlice<SQLSMALLINT> + ?Sized,
    MC: AsMutSQLCHARRawSlice<SQLSMALLINT>,
>(
    ConnectionHandle: &mut SQLHDBC,
    WindowHandle: Option<SQLHANDLE>,
    InConnectionString: &C,
    OutConnectionString: &mut MC,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> SQLRETURN {
    let InConnectionString = InConnectionString.as_SQLCHAR_raw_slice();
    let OutConnectionString = OutConnectionString.as_mut_SQLCHAR_raw_slice();

    unsafe {
        let res = DriverConnectA(
            ConnectionHandle.as_SQLHANDLE(),
            std::ptr::null_mut(),
            InConnectionString.0,
            InConnectionString.1,
            OutConnectionString.0,
            OutConnectionString.1,
            StringLength2Ptr.as_mut_ptr(),
            DriverCompletion as SQLUSMALLINT,
        );

        res
    }
}
