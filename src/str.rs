use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use co3::ReprC;
use rust_spec::RustSpec;

use crate::data::{CHAR, WCHAR};

#[sealed::sealed]
pub trait OdbcChar:
    RustSpec<Layout = rust_spec::Stable> + co3::transmute::CheckedTransmute<CType: Sized>
{
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, RustSpec, ReprC)]
#[repr(transparent)]
pub struct OdbcStr<T>([T]);

#[sealed::sealed]
impl OdbcChar for CHAR {}
#[sealed::sealed]
impl OdbcChar for WCHAR {}

impl<T> Deref for OdbcStr<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for OdbcStr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl AsRef<OdbcStr<CHAR>> for str {
    fn as_ref(&self) -> &OdbcStr<CHAR> {
        self.as_bytes().as_ref()
    }
}
impl AsMut<OdbcStr<CHAR>> for str {
    fn as_mut(&mut self) -> &mut OdbcStr<CHAR> {
        unsafe { self.as_bytes_mut().as_mut() }
    }
}
impl AsRef<OdbcStr<CHAR>> for [CHAR] {
    fn as_ref(&self) -> &OdbcStr<CHAR> {
        unsafe { &*(self as *const [CHAR] as *const OdbcStr<CHAR>) }
    }
}
impl AsMut<OdbcStr<CHAR>> for [CHAR] {
    fn as_mut(&mut self) -> &mut OdbcStr<CHAR> {
        unsafe { &mut *(self as *mut [CHAR] as *mut OdbcStr<CHAR>) }
    }
}
impl AsRef<OdbcStr<WCHAR>> for [WCHAR] {
    fn as_ref(&self) -> &OdbcStr<WCHAR> {
        unsafe { &*(self as *const [WCHAR] as *const OdbcStr<WCHAR>) }
    }
}
impl AsMut<OdbcStr<WCHAR>> for [WCHAR] {
    fn as_mut(&mut self) -> &mut OdbcStr<WCHAR> {
        unsafe { &mut *(self as *mut [WCHAR] as *mut OdbcStr<WCHAR>) }
    }
}
impl AsRef<OdbcStr<UnsafeCell<CHAR>>> for UnsafeCell<str> {
    fn as_ref(&self) -> &OdbcStr<UnsafeCell<CHAR>> {
        // FIXME: Relies on the implicit assumption that str is transmutable into [u8]
        // SAFETY: Types are transparent
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<OdbcStr<UnsafeCell<CHAR>>> for UnsafeCell<[CHAR]> {
    fn as_ref(&self) -> &OdbcStr<UnsafeCell<CHAR>> {
        // SAFETY: Types are transparent
        unsafe { core::mem::transmute(self) }
    }
}
impl AsRef<OdbcStr<UnsafeCell<WCHAR>>> for UnsafeCell<[WCHAR]> {
    fn as_ref(&self) -> &OdbcStr<UnsafeCell<WCHAR>> {
        // SAFETY: Types are transparent
        unsafe { core::mem::transmute(self) }
    }
}
impl AsMut<OdbcStr<MaybeUninit<CHAR>>> for [MaybeUninit<CHAR>]
where
    [CHAR]: AsMut<OdbcStr<CHAR>>,
{
    fn as_mut(&mut self) -> &mut OdbcStr<MaybeUninit<CHAR>> {
        unsafe { &mut *(self as *mut [MaybeUninit<CHAR>] as *mut OdbcStr<MaybeUninit<CHAR>>) }
    }
}
impl AsMut<OdbcStr<MaybeUninit<WCHAR>>> for [MaybeUninit<WCHAR>]
where
    [WCHAR]: AsMut<OdbcStr<WCHAR>>,
{
    fn as_mut(&mut self) -> &mut OdbcStr<MaybeUninit<WCHAR>> {
        unsafe { &mut *(self as *mut [MaybeUninit<WCHAR>] as *mut OdbcStr<MaybeUninit<WCHAR>>) }
    }
}
