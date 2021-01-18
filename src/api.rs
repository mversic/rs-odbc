use crate::{
    env::EnvAttribute,
    handle::Allocate,
    handle::{AsSQLHANDLE, Version, HENV, V3_8},
    AnsiType, AsMutPtr, AsMutSQLINTEGER, AsMutSlice, AsSlice, ConstSQLPOINTER, GetAttr,
    MutSQLPOINTER, OdbcAttribute, SetAttr, SQLHANDLE, SQLHENV, SQLINTEGER, SQLRETURN, SQLSMALLINT,
};
use std::mem::MaybeUninit;

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
        ValuePtr: ConstSQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;

    #[link_name = "SQLGetEnvAttr"]
    fn GetEnvAttr(
        EnvironmentHandle: HENV,
        Attribute: SQLINTEGER,
        ValuePtr: MutSQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLengthPtr: *mut SQLINTEGER,
    ) -> SQLRETURN;
}

#[inline]
pub fn SQLAllocHandle<'a: 'src, 'src, OH: Allocate<'a, 'src>>(
    InputHandle: &'src mut OH::SrcHandle,
    mut OutputHandlePtr: MaybeUninit<OH>,
) -> (OH, SQLRETURN) {
    unsafe {
        let ret = AllocHandle(
            OH::identifier(),
            InputHandle.as_SQLHANDLE(),
            OutputHandlePtr.as_mut_ptr().cast(),
        );
        (OutputHandlePtr.assume_init(), ret)
    }
}

#[inline]
pub fn SQLSetEnvAttr<A: EnvAttribute, T: AnsiType>(
    mut EnvironmentHandle: SQLHENV<crate::handle::V_UNDEFINED, crate::env::E1>,
    Attribute: A,
    ValuePtr: &T,
) -> (SQLHENV<V3_8, crate::env::E1>, SQLRETURN)
where
    A: SetAttr<T>,
    T: AsSlice<OdbcAttribute, SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_slice();

    let ret = unsafe {
        SetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::identifier(),
            ValuePtr.0.cast(),
            ValuePtr.1,
        )
    };

    (EnvironmentHandle.into(), ret)
}

#[inline]
pub fn SQLGetEnvAttr<V: Version, A: EnvAttribute, T: AnsiType>(
    EnvironmentHandle: &mut SQLHENV<V, crate::env::E1>,
    Attribute: A,
    ValuePtr: &mut T,
    StringLengthPtr: &mut MaybeUninit<T::StrLen>,
) -> SQLRETURN
where
    A: GetAttr<T>,
    T: AsMutSlice<OdbcAttribute, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    let ValuePtr = ValuePtr.as_mut_slice();

    unsafe {
        GetEnvAttr(
            EnvironmentHandle.as_SQLHANDLE(),
            A::identifier(),
            ValuePtr.0.cast(),
            ValuePtr.1,
            StringLengthPtr.as_mut_ptr().cast(),
        )
    }
}
