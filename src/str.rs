use crate::{
    env::OdbcVersion,
    handle::{RefSQLHDESC, RefUnsafeSQLHDESC, UnsafeSQLHDESC, SQLHDESC},
    Ident, SQLCHAR, SQLWCHAR,
};
use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

pub trait OdbcChar {}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OdbcStr<T>([T]);

pub trait Ansi {}
pub trait Unicode {}

impl OdbcChar for SQLCHAR {}
impl OdbcChar for SQLWCHAR {}

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
impl AsRef<OdbcStr<SQLCHAR>> for str {
    fn as_ref(&self) -> &OdbcStr<SQLCHAR> {
        self.as_bytes().as_ref()
    }
}
impl AsMut<OdbcStr<SQLCHAR>> for str {
    fn as_mut(&mut self) -> &mut OdbcStr<SQLCHAR> {
        unsafe { self.as_bytes_mut().as_mut() }
    }
}
impl AsRef<OdbcStr<SQLCHAR>> for [SQLCHAR] {
    fn as_ref(&self) -> &OdbcStr<SQLCHAR> {
        unsafe { &*(self as *const [SQLCHAR] as *const OdbcStr<SQLCHAR>) }
    }
}
impl AsMut<OdbcStr<SQLCHAR>> for [SQLCHAR] {
    fn as_mut(&mut self) -> &mut OdbcStr<SQLCHAR> {
        unsafe { &mut *(self as *mut [SQLCHAR] as *mut OdbcStr<SQLCHAR>) }
    }
}
impl AsRef<OdbcStr<SQLWCHAR>> for [SQLWCHAR] {
    fn as_ref(&self) -> &OdbcStr<SQLWCHAR> {
        unsafe { &*(self as *const [SQLWCHAR] as *const OdbcStr<SQLWCHAR>) }
    }
}
impl AsMut<OdbcStr<SQLWCHAR>> for [SQLWCHAR] {
    fn as_mut(&mut self) -> &mut OdbcStr<SQLWCHAR> {
        unsafe { &mut *(self as *mut [SQLWCHAR] as *mut OdbcStr<SQLWCHAR>) }
    }
}
impl AsMut<OdbcStr<MaybeUninit<SQLCHAR>>> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: AsMut<OdbcStr<SQLCHAR>>,
{
    fn as_mut(&mut self) -> &mut OdbcStr<MaybeUninit<SQLCHAR>> {
        unsafe { &mut *(self as *mut [MaybeUninit<SQLCHAR>] as *mut OdbcStr<MaybeUninit<SQLCHAR>>) }
    }
}
impl AsMut<OdbcStr<MaybeUninit<SQLWCHAR>>> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: AsMut<OdbcStr<SQLWCHAR>>,
{
    fn as_mut(&mut self) -> &mut OdbcStr<MaybeUninit<SQLWCHAR>> {
        unsafe {
            &mut *(self as *mut [MaybeUninit<SQLWCHAR>] as *mut OdbcStr<MaybeUninit<SQLWCHAR>>)
        }
    }
}

impl<T: Ident> Ansi for T {} // TODO: This coincidentally implements it for SQLWCHAR as well. May not be a problem?
impl<T: Ident> Unicode for T {} // TODO: This coincidentally implements it for SQLWCHAR as well. May not be a problem?

impl<T: Ident> Ansi for MaybeUninit<T> where T: Ansi {}
impl<T: Ident> Unicode for MaybeUninit<T> where T: Unicode {}

impl Ansi for OdbcStr<SQLCHAR> {}
impl Unicode for OdbcStr<SQLWCHAR> {}

impl Ansi for OdbcStr<MaybeUninit<SQLCHAR>> {}
impl Unicode for OdbcStr<MaybeUninit<SQLWCHAR>> {}

impl<CH: OdbcChar> Ansi for &OdbcStr<CH> where OdbcStr<CH>: Ansi {}
impl<CH: OdbcChar> Unicode for &OdbcStr<CH> where OdbcStr<CH>: Unicode {}

impl<DT, V: OdbcVersion> Ansi for MaybeUninit<RefSQLHDESC<'_, DT, V>> {}
impl<DT, V: OdbcVersion> Unicode for MaybeUninit<RefSQLHDESC<'_, DT, V>> {}

impl<DT, V: OdbcVersion> Ansi for MaybeUninit<RefUnsafeSQLHDESC<'_, DT, V>> {}
impl<DT, V: OdbcVersion> Unicode for MaybeUninit<RefUnsafeSQLHDESC<'_, DT, V>> {}

impl<DT, V: OdbcVersion> Ansi for Option<&SQLHDESC<'_, DT, V>> {}
impl<DT, V: OdbcVersion> Unicode for Option<&SQLHDESC<'_, DT, V>> {}

impl<DT, V: OdbcVersion> Ansi for Option<&UnsafeSQLHDESC<'_, DT, V>> {}
impl<DT, V: OdbcVersion> Unicode for Option<&UnsafeSQLHDESC<'_, DT, V>> {}
