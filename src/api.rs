use crate::{SQLSMALLINT, SQLHANDLE, SQLRETURN, handle::Allocate, handle::AsSQLHANDLE};
use std::mem::MaybeUninit;

extern "system" {
    #[link_name = "SQLAllocHandle"]
    fn AllocHandle(HandleType: SQLSMALLINT, InputHandle: SQLHANDLE, OutputHandlePtr: *mut SQLHANDLE) -> SQLRETURN;

    #[link_name = "SQLFreeHandle"]
    pub(crate) fn FreeHandle(HandleType: SQLSMALLINT, Handle: SQLHANDLE) -> SQLRETURN;
}

#[inline]
pub fn SQLAllocHandle<'a: 'src, 'src, OH: Allocate<'a, 'src>>(InputHandle: &'src mut OH::SrcHandle, mut OutputHandlePtr: MaybeUninit<OH>) -> (OH, SQLRETURN)  {
    unsafe{
        let ret = AllocHandle(OH::identifier(), InputHandle.as_SQLHANDLE(), OutputHandlePtr.as_mut_ptr().cast());
        (OutputHandlePtr.assume_init(), ret)
    }
}
