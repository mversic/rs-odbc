use crate::{
    c_types::CScalar,
    conn::ConnState,
    desc::DescType,
    env::OdbcVersion,
    handle::{
        RefSQLHDESC, RefUnsafeSQLHDESC, UnsafeSQLHDESC, UnsafeSQLHSTMT, SQLHANDLE, SQLHDBC,
        SQLHDESC, SQLHENV, SQLHSTMT, SQL_NULL_HANDLE,
    },
    slice_len,
    str::{Ansi, OdbcChar, OdbcStr, Unicode},
    Scalar, SQLCHAR, SQLINTEGER, SQLLEN, SQLPOINTER, SQLSMALLINT, SQLUINTEGER, SQLULEN,
    SQLUSMALLINT, SQLWCHAR,
};
use core::{cell::UnsafeCell, fmt::Debug, mem::MaybeUninit};

/// Used to do a cheap mutable reference-to-raw pointer conversion.
///
/// # Invariant
///
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

pub unsafe trait AsSQLHANDLE {
    #[allow(non_snake_case)]
    fn as_SQLHANDLE(&self) -> SQLHANDLE;
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

unsafe impl<T: Scalar> AsMutPtr<T> for T {
    fn as_mut_ptr(&mut self) -> *mut T {
        self
    }
}
unsafe impl<T: AsMutPtr<T>> AsMutPtr<T> for MaybeUninit<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
}
unsafe impl<T: Scalar> IntoSQLPOINTER for &T {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self as *const _ as SQLPOINTER
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
unsafe impl<T: CScalar> IntoSQLPOINTER for &UnsafeCell<T> {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.get().cast()
    }
}
unsafe impl<T: CScalar> AsSQLPOINTER for T {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // CScalar is guaranteed to have SQLPOINTER representation
        self as *const _ as SQLPOINTER
    }
}
unsafe impl<T: CScalar> AsSQLPOINTER for UnsafeCell<T> {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // CScalar is guaranteed to have SQLPOINTER representation
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

unsafe impl<T: CScalar> AsMutSQLPOINTER for T {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // CScalar is guaranteed to have SQLPOINTER representation
        (self as *mut Self).cast()
    }
}
unsafe impl<T: CScalar> AsMutSQLPOINTER for MaybeUninit<T> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // CScalar is guaranteed to have SQLPOINTER representation
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
    Self: Ansi,
{
    fn as_mut_raw_slice(&mut self) -> (*mut SQLCHAR, LEN) {
        (self.as_mut_ptr().cast(), slice_len(self))
    }
}
unsafe impl<CH, LEN: TryFrom<usize>> AsMutRawSlice<SQLWCHAR, LEN> for OdbcStr<CH>
where
    LEN: Copy,
    LEN::Error: Debug,
    Self: Unicode,
{
    fn as_mut_raw_slice(&mut self) -> (*mut SQLWCHAR, LEN) {
        (self.as_mut_ptr().cast(), slice_len(self))
    }
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

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
unsafe impl<'buf, V: OdbcVersion, T: DescType<'buf>> IntoSQLPOINTER
    for Option<&SQLHDESC<'_, T, V>>
{
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.map_or_else(core::ptr::null_mut, |handle| {
            Some(&handle.0).into_SQLPOINTER()
        })
    }
}
unsafe impl<'buf, V: OdbcVersion, T: DescType<'buf>> IntoSQLPOINTER
    for Option<&UnsafeSQLHDESC<'_, T, V>>
{
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        self.map_or_else(core::ptr::null_mut, |handle| handle.as_SQLHANDLE().cast())
    }
}

unsafe impl AsSQLPOINTER for (SQLPOINTER, SQLLEN) {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
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
unsafe impl<DT, V: OdbcVersion> AsMutSQLPOINTER for MaybeUninit<RefUnsafeSQLHDESC<'_, DT, V>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        if cfg!(feature = "odbc_debug") {
            // SQLHDESC is not transparent
            unimplemented!("This method should never be called")
        }

        // SQLHDESC is transparent
        self.as_mut_ptr().cast()
    }
}
unsafe impl<'conn, 'desc, DT, V: OdbcVersion> AsMutSQLPOINTER
    for MaybeUninit<RefSQLHDESC<'conn, DT, V>>
{
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        // Valid because RefSQLHDESC is a transparent newtype wrapper over RefUnsafeSQLHDESC
        unsafe {
            core::mem::transmute::<_, &mut MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>>>(self)
        }
        .as_mut_SQLPOINTER()
    }
}

unsafe impl AsSQLHANDLE for SQL_NULL_HANDLE {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        core::ptr::null_mut()
    }
}
unsafe impl<V: OdbcVersion> AsSQLHANDLE for SQLHENV<V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
unsafe impl<C: ConnState, V: OdbcVersion> AsSQLHANDLE for SQLHDBC<'_, C, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
unsafe impl<V: OdbcVersion> AsSQLHANDLE for SQLHSTMT<'_, '_, '_, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.0.as_SQLHANDLE()
    }
}
unsafe impl<V: OdbcVersion> AsSQLHANDLE for UnsafeSQLHSTMT<'_, '_, '_, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
unsafe impl<V: OdbcVersion, T> AsSQLHANDLE for SQLHDESC<'_, T, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.0.as_SQLHANDLE()
    }
}
unsafe impl<V: OdbcVersion, T> AsSQLHANDLE for UnsafeSQLHDESC<'_, T, V> {
    fn as_SQLHANDLE(&self) -> SQLHANDLE {
        self.handle
    }
}
