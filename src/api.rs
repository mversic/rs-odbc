use crate::handle::AsMutSQLHANDLE;
use crate::{
    env::EnvAttr, AnsiType,
    handle::{Allocate, AsSQLHANDLE, HandleIdentifier, HDBC, HENV, SQLHDBC},
    AsMutPtr, AsMutRawSlice, AsRawParts, AsRawSQLCHARSlice, AsMutRawSQLCHARSlice,
    DriverCompletion, GetAttr, OdbcAttr, SetAttr, SQLCHAR, SQLHANDLE, SQLHENV, SQLINTEGER,
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
    InputHandle: &'src OH::SrcHandle,
    OutputHandlePtr: &mut MaybeUninit<OH>,
) -> SQLRETURN
where
    OH::SrcHandle: AsSQLHANDLE,
    OH: AsMutSQLHANDLE,
{
    unsafe {
        AllocHandle(
            OH::Identifier::IDENTIFIER,
            // TODO: Is this UB? Handle will be modified on the extern C side but it's passed
            // here from shared reference. It has to be shared reference to be able to allocate
            // multiple children. To resolve the issue RefCell with PhantomData<Ref<'src, T>>
            // can be used, but it's hard to implement
            //
            // Also, what about multithreaded code?
            InputHandle.as_SQLHANDLE(),
            OutputHandlePtr.as_mut_ptr().cast(),
        )
    }
}

#[inline]
pub fn SQLSetEnvAttr<A: EnvAttr, T>(
    EnvironmentHandle: &mut SQLHENV,
    Attribute: A,
    ValuePtr: &T,
) -> SQLRETURN
where
    A: SetAttr<AnsiType, T>,
    T: AsRawParts<OdbcAttr, SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_raw_parts();

    unsafe {
        SetEnvAttr(
            EnvironmentHandle.as_mut_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
        )
    }
}

pub fn SQLGetEnvAttr<A: EnvAttr, T>(
    EnvironmentHandle: &SQLHENV,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<AnsiType, T>,
    T: AsMutRawSlice<OdbcAttr, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_mut_raw_slice();

    unsafe {
        GetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::IDENTIFIER,
            ValuePtr.0,
            ValuePtr.1,
            StringLengthPtr.as_mut_ptr().cast(),
        )
    }
}

pub fn SQLDriverConnectA<
    C: AsRawSQLCHARSlice<SQLSMALLINT> + ?Sized,
    MC: AsMutRawSQLCHARSlice<SQLSMALLINT>,
>(
    ConnectionHandle: &mut SQLHDBC,
    WindowHandle: Option<SQLHANDLE>,
    InConnectionString: &C,
    OutConnectionString: &mut MC,
    StringLength2Ptr: &mut MaybeUninit<SQLSMALLINT>,
    DriverCompletion: DriverCompletion,
) -> SQLRETURN {
    let InConnectionString = InConnectionString.as_raw_SQLCHAR_slice();
    let OutConnectionString = OutConnectionString.as_mut_raw_SQLCHAR_slice();

    unsafe {
        let res = DriverConnectA(
            ConnectionHandle.as_mut_SQLHANDLE(),
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

pub fn SQLDisconnect(ConnectionHandle: &mut SQLHDBC) -> SQLRETURN {
    unsafe { Disconnect(ConnectionHandle.as_mut_SQLHANDLE()) }
}
