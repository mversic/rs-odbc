use crate::c_types::ScalarCType;
use crate::convert::{AsMutSQLPOINTER, IntoSQLPOINTER};
use crate::str::{OdbcChar, OdbcStr};
use crate::{
    slice_len, Def, DriverDefined, Ident, OdbcDefined, Void, SQLCHAR, SQLINTEGER, SQLLEN,
    SQLSMALLINT, SQLUINTEGER, SQLULEN, SQLUSMALLINT, SQLWCHAR,
};
use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem::MaybeUninit;

pub unsafe trait Attr<A: Ident> {
    type DefinedBy: Def;
}
pub unsafe trait AttrGet<A>: AsMutSQLPOINTER + AttrZeroAssert {}
pub unsafe trait AttrSet<A>: IntoSQLPOINTER {}

// TODO: https://github.com/rust-lang/rust/issues/20400
// Once this problem is resolved, it would be possible to modify AttrLen<AD, LEN>
// into AttrLen<A, LEN> and do more precise blanket implementations like
// impl<T: Attr<A>, LEN> AttrLen<A, LEN> for T {}
pub unsafe trait AttrLen<AD: Def, LEN: Copy> {
    /// Invariant: StrLen can only be LEN(for slices) or uninhabited type(for scalar types)
    /// It is assumed that ODBC driver will never write to StrLen pointer for scalar types
    type StrLen: Copy;

    fn len(&self) -> LEN;
}

pub trait AttrZeroAssert {
    fn assert_zeroed(&self) {}
}

// TODO: Implement and use for binary strings AttrLen
//pub const fn SQL_LEN_BINARY_ATTR<LEN: OdbcInt>(length: LEN) {
//    let SQL_LEN_BINARY_ATTR_OFFSET: LEN::new(-100);
//    (-length).checked_add(SQL_LEN_BINARY_ATTR_OFFSET).expect()
//}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

unsafe impl<A: Ident, T: Ident> Attr<A> for MaybeUninit<T>
where
    T: Attr<A>,
{
    type DefinedBy = T::DefinedBy;
}
unsafe impl<A: Ident> Attr<A> for OdbcStr<MaybeUninit<SQLCHAR>>
where
    OdbcStr<SQLCHAR>: Attr<A>,
{
    type DefinedBy = <OdbcStr<SQLCHAR> as Attr<A>>::DefinedBy;
}
unsafe impl<A: Ident> Attr<A> for OdbcStr<MaybeUninit<SQLWCHAR>>
where
    OdbcStr<SQLWCHAR>: Attr<A>,
{
    type DefinedBy = <OdbcStr<SQLWCHAR> as Attr<A>>::DefinedBy;
}
unsafe impl<A: Ident, CH: OdbcChar> Attr<A> for &OdbcStr<CH>
where
    OdbcStr<CH>: Attr<A>,
{
    type DefinedBy = <OdbcStr<CH> as Attr<A>>::DefinedBy;
}

unsafe impl<A: Ident, T: Ident> AttrGet<A> for MaybeUninit<T>
where
    Self: AsMutSQLPOINTER,
    T: AttrGet<A>,
{
}
unsafe impl<A: Ident> AttrGet<A> for OdbcStr<MaybeUninit<SQLCHAR>> where OdbcStr<SQLCHAR>: AttrGet<A>
{}
unsafe impl<A: Ident> AttrGet<A> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: AttrGet<A>
{
}

unsafe impl<A: Ident, T: Ident> AttrSet<A> for MaybeUninit<T>
where
    Self: IntoSQLPOINTER,
    T: AttrSet<A>,
{
}

unsafe impl<AD: Def, LEN: Copy, T: Ident> AttrLen<AD, LEN> for T
where
    MaybeUninit<T>: AttrLen<AD, LEN>,
    LEN: From<u8>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <MaybeUninit<_> as AttrLen<AD, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<LEN: Copy, T: Ident> AttrLen<OdbcDefined, LEN> for MaybeUninit<T>
where
    LEN: From<u8>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        LEN::from(0)
    }
}
unsafe impl<LEN: Copy, T: Ident> AttrLen<DriverDefined, LEN> for MaybeUninit<T>
where
    T: AttrLen<OdbcDefined, LEN>,
    LEN: From<T::Type>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        LEN::from(T::IDENTIFIER)
    }
}
unsafe impl<AD: Def, CH: OdbcChar, LEN: Copy> AttrLen<AD, LEN> for OdbcStr<CH>
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
    OdbcStr<MaybeUninit<CH>>: AttrLen<AD, LEN>,
{
    type StrLen = <OdbcStr<MaybeUninit<CH>> as AttrLen<AD, LEN>>::StrLen;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <OdbcStr<MaybeUninit<CH>> as AttrLen<AD, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<AD: Def, CH: OdbcChar, LEN: Copy> AttrLen<AD, LEN> for OdbcStr<MaybeUninit<CH>>
where
    LEN: TryFrom<usize> + std::ops::Mul<Output = LEN>,
    LEN::Error: Debug,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        // TODO: Check for multiplication overflow with checked_mul
        slice_len::<_, LEN>(self) * LEN::try_from(std::mem::size_of::<CH>()).unwrap()
    }
}
// TODO: If this is a deferred buffer, then I believe len should be 0
// This can be resolved with specialization by having special implementation for SQL_DESC_DATA_PTR
// and alike if there are other attributes that correspond to deferred buffers
unsafe impl<LEN: Copy> AttrLen<OdbcDefined, LEN> for [MaybeUninit<SQLCHAR>]
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        slice_len(self)
    }
}
// TODO: What if this is a deferred buffer, then I believe len should be 0
// This can be resolved with specialization by having special implementation for SQL_DESC_DATA_PTR
// and alike if there are other attributes that correspond to deferred buffers
unsafe impl<LEN: Copy> AttrLen<DriverDefined, LEN> for [MaybeUninit<SQLCHAR>] {
    type StrLen = LEN;

    fn len(&self) -> LEN {
        // TODO: Should be a negative value
        unimplemented!();
    }
}
unsafe impl<AD: Def, LEN: Copy> AttrLen<AD, LEN> for [SQLCHAR]
where
    [MaybeUninit<SQLCHAR>]: AttrLen<AD, LEN>,
{
    type StrLen = <[MaybeUninit<SQLCHAR>] as AttrLen<AD, LEN>>::StrLen;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <[MaybeUninit<SQLCHAR>] as AttrLen<AD, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<AD: Def, LEN: Copy, T: Ident> AttrLen<AD, LEN> for [T]
where
    LEN: From<u8>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        LEN::from(0)
    }
}
unsafe impl<AD: Def, LEN: Copy, CH: OdbcChar> AttrLen<AD, LEN> for &OdbcStr<CH>
where
    OdbcStr<CH>: AttrLen<AD, LEN>,
{
    type StrLen = <OdbcStr<CH> as AttrLen<AD, LEN>>::StrLen;

    fn len(&self) -> LEN {
        AttrLen::len(*self)
    }
}
unsafe impl<AD: Def, LEN: Copy, T> AttrLen<AD, LEN> for &[T]
where
    [T]: AttrLen<AD, LEN>,
{
    type StrLen = <[T] as AttrLen<AD, LEN>>::StrLen;

    fn len(&self) -> LEN {
        AttrLen::len(*self)
    }
}
// Deferred buffers are used only through SQLSetDescAttr and SQLGetDescAttr
unsafe impl<AD: Def, T: ScalarCType> AttrLen<AD, SQLINTEGER> for UnsafeCell<T> {
    type StrLen = Void;

    fn len(&self) -> SQLINTEGER {
        0
    }
}
// Deferred buffers are used only through SQLSetDescAttr and SQLGetDescAttr
unsafe impl<AD: Def, T> AttrLen<AD, SQLINTEGER> for [UnsafeCell<T>] {
    type StrLen = Void;

    fn len(&self) -> SQLINTEGER {
        0 // Length is not used for deferred buffers
    }
}

impl<T> AttrZeroAssert for MaybeUninit<T> {
    // MaybeUninit must not be read
}
impl<T> AttrZeroAssert for [T] {}
impl<T> AttrZeroAssert for OdbcStr<T> {}
impl<T: ScalarCType> AttrZeroAssert for UnsafeCell<T> {
    // Deferred buffers don't need to be zeroed
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

impl AttrZeroAssert for SQLSMALLINT {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}
impl AttrZeroAssert for SQLUSMALLINT {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}
impl AttrZeroAssert for SQLINTEGER {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}
impl AttrZeroAssert for SQLUINTEGER {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}
impl AttrZeroAssert for SQLLEN {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}
impl AttrZeroAssert for SQLULEN {
    fn assert_zeroed(&self) {
        // TODO: Add custom message
        assert_eq!(0, *self);
    }
}