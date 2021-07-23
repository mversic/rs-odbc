use crate::c_types::{ScalarCType, StrLenOrInd};
use crate::str::{Ansi, OdbcChar, OdbcStr, Unicode};
use crate::{
    slice_len, Ident, Void, SQLCHAR, SQLINTEGER, SQLLEN, SQLPOINTER, SQLSMALLINT, SQLUINTEGER,
    SQLULEN, SQLUSMALLINT, SQLWCHAR,
};
use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem::MaybeUninit;

/// Used to do a cheap mutable reference-to-raw pointer conversion.
/// Implementing types must support all possible values for T because
/// any valid T value can be written to the obtained raw mut pointer
pub unsafe trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

/// A value-to-SQLPOINTER conversion that consumes the input value.
/// Invariant: SQLPOINTER obtained through this trait is never written to
pub unsafe trait IntoSQLPOINTER: Copy {
    #[allow(non_snake_case)]
    fn into_SQLPOINTER(self) -> SQLPOINTER;
}

/// Used to do a cheap reference-to-SQLPOINTER conversion.
/// Invariant: SQLPOINTER obtained through this trait is never written to
pub unsafe trait AsSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_SQLPOINTER(&self) -> SQLPOINTER;
}

/// Used to do a cheap mutable reference-to-SQLPOINTER conversion.
pub unsafe trait AsMutSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER;
}

/// Used to do a cheap slice-to-raw slice conversion.
/// Invariant: raw pointer obtained through this trait is never written to
pub(crate) unsafe trait AsRawSlice<CH, LEN: Copy> {
    fn as_raw_slice(&self) -> (*const CH, LEN);
}

/// Used to do a cheap mutable slice-to-raw slice conversion.
pub(crate) unsafe trait AsMutRawSlice<CH, LEN: Copy> {
    fn as_mut_raw_slice(&mut self) -> (*mut CH, LEN);
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

unsafe impl<T> AsMutPtr<T> for MaybeUninit<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
}
unsafe impl<T: Ident> AsMutPtr<T> for MaybeUninit<Void> {
    fn as_mut_ptr(&mut self) -> *mut T {
        // SAFETY:
        // Acording to the ODBC specification returning `self.as_mut_ptr().cast()` here
        // should be fine. However non-compliant implementations might try to write
        // to non-null pointers obtained through this method which would cause UB
        std::ptr::null_mut()
    }
}
unsafe impl<T: ScalarCType> IntoSQLPOINTER for &UnsafeCell<T> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.get().cast()
    }
}
unsafe impl<T> IntoSQLPOINTER for &[T] {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is safe because of the invariant
        // that SQLPOINTER obtained through IntoSQLPOINTER will never be written to
        (self.as_ptr() as *mut T).cast()
    }
}
unsafe impl<CH: OdbcChar> IntoSQLPOINTER for &OdbcStr<CH> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is safe because of the invariant
        // that SQLPOINTER obtained through IntoSQLPOINTER will never be written to
        (self.as_ptr() as *mut CH).cast()
    }
}
unsafe impl<T: ScalarCType> AsSQLPOINTER for T {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // ScalarCType is guaranteed to have SQLPOINTER representation
        self as *const _ as SQLPOINTER
    }
}
unsafe impl<T: ScalarCType> AsSQLPOINTER for UnsafeCell<T> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // ScalarCType is guaranteed to have SQLPOINTER representation
        self.get().cast()
    }
}
unsafe impl<T> AsSQLPOINTER for [T] {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is ok because of the invariant
        // that SQLPOINTER obtained through AsSQLPOINTER will never be written to
        (self.as_ptr() as *mut T).cast()
    }
}
unsafe impl<T> AsSQLPOINTER for OdbcStr<T> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is ok because of the invariant
        // that SQLPOINTER obtained through AsSQLPOINTER will never be written to
        (self.as_ptr() as *mut T).cast()
    }
}

unsafe impl<T: ScalarCType> AsMutSQLPOINTER for T {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // ScalarCType is guaranteed to have SQLPOINTER representation
        (self as *mut Self).cast()
    }
}
unsafe impl<T: ScalarCType> AsMutSQLPOINTER for MaybeUninit<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // ScalarCType is guaranteed to have SQLPOINTER representation
        self.as_mut_ptr().cast()
    }
}
unsafe impl<T> AsMutSQLPOINTER for [T] {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}
unsafe impl<T> AsMutSQLPOINTER for OdbcStr<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        <[T] as AsMutSQLPOINTER>::as_mut_SQLPOINTER(self)
    }
}
unsafe impl<T: AsMutSQLPOINTER> AsMutSQLPOINTER for UnsafeCell<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.get_mut().as_mut_SQLPOINTER()
    }
}

unsafe impl<CH, LEN: TryFrom<usize>> AsRawSlice<CH, LEN> for OdbcStr<CH>
where
    LEN: Copy,
    LEN::Error: Debug,
{
    fn as_raw_slice(&self) -> (*const CH, LEN) {
        (self.as_ptr(), slice_len(self))
    }
}

unsafe impl<CH, LEN: TryFrom<usize>> AsMutRawSlice<SQLCHAR, LEN> for OdbcStr<CH>
where
    LEN: Copy,
    LEN::Error: Debug,
    OdbcStr<CH>: Ansi,
{
    fn as_mut_raw_slice(&mut self) -> (*mut SQLCHAR, LEN) {
        (self.as_mut_ptr().cast(), slice_len(self))
    }
}
unsafe impl<CH, LEN: TryFrom<usize>> AsMutRawSlice<SQLWCHAR, LEN> for OdbcStr<CH>
where
    LEN: Copy,
    LEN::Error: Debug,
    OdbcStr<CH>: Unicode,
{
    fn as_mut_raw_slice(&mut self) -> (*mut SQLWCHAR, LEN) {
        (self.as_mut_ptr().cast(), slice_len(self))
    }
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

// TODO: Why is this needed?
unsafe impl AsMutPtr<SQLLEN> for MaybeUninit<StrLenOrInd> {
    fn as_mut_ptr(&mut self) -> *mut SQLLEN {
        self.as_mut_ptr().cast()
    }
}
unsafe impl AsMutPtr<SQLLEN> for UnsafeCell<StrLenOrInd> {
    fn as_mut_ptr(&mut self) -> *mut SQLLEN {
        self.get().cast()
    }
}

unsafe impl IntoSQLPOINTER for SQLSMALLINT {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
unsafe impl IntoSQLPOINTER for SQLUSMALLINT {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
unsafe impl IntoSQLPOINTER for SQLINTEGER {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
unsafe impl IntoSQLPOINTER for SQLUINTEGER {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
unsafe impl IntoSQLPOINTER for SQLLEN {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
unsafe impl IntoSQLPOINTER for SQLULEN {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as SQLPOINTER
    }
}
#[cfg(feature = "raw_api")]
unsafe impl AsSQLPOINTER for (SQLPOINTER, SQLLEN) {
    fn into_SQLPOINTER(&self) -> SQLPOINTER {
        self.0
    }
}

unsafe impl AsMutSQLPOINTER for SQLLEN {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        (self as *mut Self).cast()
    }
}
unsafe impl AsMutSQLPOINTER for SQLULEN {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        (self as *mut Self).cast()
    }
}
