pub mod api;
pub mod c_types;
pub mod col;
pub mod conn;
pub mod desc;
pub mod diag;
pub mod env;
pub(crate) mod extern_api;
pub mod handle;
pub mod info;
pub mod sql_types;
pub mod sqlreturn;
pub mod stmt;

use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem::MaybeUninit;

pub use conn::{
    SQL_ASYNC_DBC_ENABLE_DEFAULT, SQL_ATTR_ACCESS_MODE, SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE,
    SQL_ATTR_AUTOCOMMIT, SQL_ATTR_AUTO_IPD, SQL_ATTR_CONNECTION_DEAD, SQL_ATTR_CONNECTION_TIMEOUT,
    SQL_ATTR_CURRENT_CATALOG, SQL_ATTR_LOGIN_TIMEOUT, SQL_ATTR_PACKET_SIZE, SQL_ATTR_TRACE,
    SQL_ATTR_TRACEFILE, SQL_ATTR_TRANSLATE_LIB, SQL_AUTOCOMMIT_DEFAULT, SQL_MODE_DEFAULT,
    SQL_OPT_TRACE_DEFAULT,
};
pub use env::{
    SQL_ATTR_CONNECTION_POOLING, SQL_ATTR_CP_MATCH, SQL_ATTR_ODBC_VERSION, SQL_CP_DEFAULT,
    SQL_CP_DRIVER_AWARE, SQL_CP_MATCH_DEFAULT, SQL_CP_OFF, SQL_CP_ONE_PER_DRIVER,
    SQL_CP_ONE_PER_HENV, SQL_CP_RELAXED_MATCH, SQL_CP_STRICT_MATCH, SQL_OV_ODBC3, SQL_OV_ODBC3_80,
};
pub use handle::{
    SQLHANDLE, SQLHDBC, SQLHDESC, SQLHENV, SQLHSTMT, SQLHWND, SQL_HANDLE_DBC, SQL_HANDLE_DESC,
    SQL_HANDLE_ENV, SQL_HANDLE_STMT, SQL_NULL_HANDLE,
}; // TODO: SQLHWND
pub use rs_odbc_derive::odbc_type;
pub use sql_types::*;
pub use sqlreturn::{
    SQLRETURN, SQL_ERROR, SQL_INVALID_HANDLE, SQL_NEED_DATA, SQL_NO_DATA, SQL_PARAM_DATA_AVAILABLE,
    SQL_STILL_EXECUTING, SQL_SUCCEEDED, SQL_SUCCESS, SQL_SUCCESS_WITH_INFO,
};
pub use DriverCompletion::*;
pub use {api::*, c_types::*, sql_types::*};

// TODO: Add support for mingw-x64 on x86 platform

pub type SQLSMALLINT = i16;
pub type SQLUSMALLINT = u16;

pub type SQLINTEGER = i32;
pub type SQLUINTEGER = u32;

pub type SQLREAL = f32;
pub type SQLDOUBLE = f64;
pub use SQLDOUBLE as SQLFLOAT;

/// ASCII encoded character
pub type SQLCHAR = u8;
pub type SQLSCHAR = i8;

/// UCS-2 encoded character
pub type SQLWCHAR = u16;

pub type SQLBIGINT = i64;
pub type SQLUBIGINT = u64;

pub type SQLLEN = isize;
pub type SQLULEN = usize;

pub type RETCODE = i16;

#[cfg(target_pointer_width = "32")]
pub type SQLSETPOSIROW = SQLUSMALLINT;
#[cfg(target_pointer_width = "64")]
pub type SQLSETPOSIROW = u64;

pub struct SqlStateA([SQLCHAR; 6]);
pub struct SqlStateW([SQLWCHAR; 6]);

type UWORD = u16;
type SQLPOINTER = *mut std::ffi::c_void;

#[derive(Clone, Copy)]
// TODO: https://github.com/rust-lang/rust/issues/35121
// Use uninhabited type ! when it is available in std
pub enum Void {}

const SQL_IS_POINTER: SQLSMALLINT = -4;
const SQL_IS_UINTEGER: SQLSMALLINT = -5;
const SQL_IS_INTEGER: SQLSMALLINT = -6;
const SQL_IS_USMALLINT: SQLSMALLINT = -7;
const SQL_IS_SMALLINT: SQLSMALLINT = -8;

// TODO: Remove these and implement traits directly on
// SQLLEN/SQLULEN like `impl Attr<A> for SQLLEN`
// WARNING: These are not mentioned in ODBC
const SQL_IS_LEN: SQLSMALLINT = SQL_IS_INTEGER;
const SQL_IS_ULEN: SQLSMALLINT = SQL_IS_UINTEGER;

const SLICE_LEN_TOO_LARGE_MSG: &str = "Slice len too large";

/// Implementing types must support all possible values for T because
/// any valid T value can be written to the obtained raw mut pointer
pub unsafe trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

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
unsafe impl AsMutPtr<SQLLEN> for MaybeUninit<StrLenOrInd> {
    fn as_mut_ptr(&mut self) -> *mut SQLLEN {
        self.as_mut_ptr().cast()
    }
}
unsafe impl AsMutPtr<SQLLEN> for std::cell::UnsafeCell<StrLenOrInd> {
    fn as_mut_ptr(&mut self) -> *mut SQLLEN {
        self.get().cast()
    }
}

/// Invariant: SQLPOINTER obtained through this trait is never written to
pub unsafe trait AsSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_SQLPOINTER(&self) -> SQLPOINTER;
}
unsafe impl<T> AsSQLPOINTER for [T] {
    fn as_SQLPOINTER(&self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is ok because of the invariant
        // that SQLPOINTER obtained through AsSQLPOINTER will never be written to
        (self.as_ptr() as *mut T).cast()
    }
}

/// Invariant: SQLPOINTER obtained through this trait is never written to
pub unsafe trait IntoSQLPOINTER: Copy {
    #[allow(non_snake_case)]
    fn into_SQLPOINTER(self) -> SQLPOINTER;
}
unsafe impl IntoSQLPOINTER for SQLSMALLINT {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        SQLINTEGER::into_SQLPOINTER(self as SQLINTEGER)
    }
}
unsafe impl IntoSQLPOINTER for SQLUSMALLINT {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        SQLUINTEGER::into_SQLPOINTER(self as SQLUINTEGER)
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
unsafe impl<T> IntoSQLPOINTER for &[T] {
    fn into_SQLPOINTER(self) -> SQLPOINTER {
        // Casting from const to mutable raw pointer is safe because of the invariant
        // that SQLPOINTER obtained through IntoSQLPOINTER will never be written to
        (self.as_ptr() as *mut T).cast()
    }
}

/// If type implementing this trait is a reference allocated inside Driver Manager, then
/// it must be constrained by the given lifetime parameter 'a. Such references are never
/// owned (and therefore never dropped) by the Rust code
pub unsafe trait AsMutSQLPOINTER {
    #[allow(non_snake_case)]
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER;
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
unsafe impl<T> AsMutSQLPOINTER for [T] {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}

/// Implementing type must have the same representation as SQLPOINTER
pub trait Ident {
    type Type: Copy;
    const IDENTIFIER: Self::Type;
}
impl Ident for SQLSMALLINT {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_SMALLINT;
}
impl Ident for SQLUSMALLINT {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_USMALLINT;
}
impl Ident for SQLINTEGER {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_INTEGER;
}
impl Ident for SQLUINTEGER {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_UINTEGER;
}
impl Ident for SQLLEN {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_LEN;
}
impl Ident for SQLULEN {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_ULEN;
}
impl<T> Ident for &T {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_POINTER;
}
impl<T> Ident for &mut T {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_POINTER;
}
impl<T> Ident for Option<&T> {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_POINTER;
}
impl<T> Ident for Option<&mut T> {
    type Type = SQLSMALLINT;

    const IDENTIFIER: SQLSMALLINT = SQL_IS_POINTER;
}

pub unsafe trait Attr<A: Ident> {
    type DefinedBy: Def;

    // Documentation says that binary buffers are allowed as ValuePtr arguments
    // and in the case of driver-defined attributes size of such values is specially defined.
    // Since SQLCHAR and binary are both represented with u8, this type is used
    // in order to disambiguate between [SQLCHAR] and binary buffers
    type NonBinary: Bool;
}
unsafe impl<A: Ident> Attr<A> for [SQLWCHAR]
where
    [SQLCHAR]: Attr<A, NonBinary=True>,
{
    type DefinedBy = <[SQLCHAR] as Attr<A>>::DefinedBy;
    type NonBinary = <[SQLCHAR] as Attr<A>>::NonBinary;
}
unsafe impl<A: Ident, T: Ident> Attr<A> for MaybeUninit<T>
where
    T: Attr<A>,
{
    type DefinedBy = T::DefinedBy;
    type NonBinary = T::NonBinary;
}
unsafe impl<A: Ident> Attr<A> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: Attr<A>,
{
    type DefinedBy = <[SQLCHAR] as Attr<A>>::DefinedBy;
    type NonBinary = <[SQLCHAR] as Attr<A>>::NonBinary;
}
unsafe impl<A: Ident> Attr<A> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: Attr<A>,
{
    type DefinedBy = <[SQLWCHAR] as Attr<A>>::DefinedBy;
    type NonBinary = <[SQLWCHAR] as Attr<A>>::NonBinary;
}
unsafe impl<A: Ident> Attr<A> for &[SQLCHAR]
where
    [SQLCHAR]: Attr<A>,
{
    type DefinedBy = <[SQLCHAR] as Attr<A>>::DefinedBy;
    type NonBinary = <[SQLCHAR] as Attr<A>>::NonBinary;
}
unsafe impl<A: Ident> Attr<A> for &[SQLWCHAR]
where
    [SQLWCHAR]: Attr<A>,
{
    type DefinedBy = <[SQLWCHAR] as Attr<A>>::DefinedBy;
    type NonBinary = <[SQLWCHAR] as Attr<A>>::NonBinary;
}

// TODO: https://github.com/rust-lang/rust/issues/20400
// Once this problem is resolved, it would be possible to modify AttrLen<AD, NB, LEN>
// into AttrLen<A, LEN> and do more precise blanket implementations like
// impl<T: Attr<A>, LEN> AttrLen<A, LEN> for T {}
pub unsafe trait AttrLen<AD: Def, NB: Bool, LEN: Copy> {
    /// Invariant: StrLen can only be LEN(for slices) or uninhabited type(for scalar types)
    /// It is assumed that ODBC driver will never write to StrLen pointer for scalar types
    type StrLen: Copy;

    fn len(&self) -> LEN;
}
unsafe impl<AD: Def, NB: Bool, LEN: Copy, T: Ident> AttrLen<AD, NB, LEN> for T
where
    MaybeUninit<T>: AttrLen<AD, NB, LEN>,
    LEN: From<u8>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <MaybeUninit<T> as AttrLen<AD, NB, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<NB: Bool, LEN: Copy, T: Ident> AttrLen<OdbcDefined, NB, LEN> for MaybeUninit<T>
where
    LEN: From<u8>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        LEN::from(0)
    }
}
unsafe impl<NB: Bool, LEN: Copy, T: Ident> AttrLen<DriverDefined, NB, LEN> for MaybeUninit<T>
where
    T: AttrLen<OdbcDefined, NB, LEN>,
    LEN: From<T::Type>,
{
    type StrLen = Void;

    fn len(&self) -> LEN {
        LEN::from(T::IDENTIFIER)
    }
}
unsafe impl<AD: Def, NB: Bool, LEN: Copy> AttrLen<AD, NB, LEN> for [SQLCHAR]
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
    [MaybeUninit<SQLCHAR>]: AttrLen<AD, NB, LEN>,
{
    type StrLen = <[MaybeUninit<SQLCHAR>] as AttrLen<AD, NB, LEN>>::StrLen;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <[MaybeUninit<SQLCHAR>] as AttrLen<AD, NB, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<AD: Def, LEN: Copy> AttrLen<AD, True, LEN> for [SQLWCHAR]
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
    [MaybeUninit<SQLWCHAR>]: AttrLen<AD, True, LEN>,
{
    type StrLen = <[MaybeUninit<SQLWCHAR>] as AttrLen<AD, True, LEN>>::StrLen;

    fn len(&self) -> LEN {
        // Transmute is safe because MaybeUninit<T> has the same size and alignment as T
        <[MaybeUninit<SQLWCHAR>] as AttrLen<AD, True, LEN>>::len(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<AD: Def, LEN: Copy> AttrLen<AD, True, LEN> for [MaybeUninit<SQLCHAR>]
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        slice_len(self)
    }
}
unsafe impl<LEN: Copy> AttrLen<OdbcDefined, False, LEN> for [MaybeUninit<SQLCHAR>]
where
    LEN: TryFrom<usize>,
    LEN::Error: Debug,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        slice_len(self)
    }
}
unsafe impl<LEN: Copy> AttrLen<DriverDefined, False, LEN> for [MaybeUninit<SQLCHAR>]
where
    LEN: TryFrom<usize>,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        unimplemented!();
    }
}
unsafe impl<AD: Def, LEN: Copy> AttrLen<AD, True, LEN> for [MaybeUninit<SQLWCHAR>]
where
    LEN: TryFrom<usize> + std::ops::Mul<Output = LEN>,
    LEN::Error: Debug,
{
    type StrLen = LEN;

    fn len(&self) -> LEN {
        // TODO: Check for multiplication overflow
        slice_len::<_, LEN>(self)
            * LEN::try_from(std::mem::size_of::<SQLWCHAR>()).expect(SLICE_LEN_TOO_LARGE_MSG)
    }
}
unsafe impl<AD: Def, NB: Bool, LEN: Copy, T> AttrLen<AD, NB, LEN> for &[T]
where
    [T]: AttrLen<AD, NB, LEN>,
{
    type StrLen = <[T] as AttrLen<AD, NB, LEN>>::StrLen;

    fn len(&self) -> LEN {
        AttrLen::len(*self)
    }
}

pub unsafe trait AsRawSlice<T, LEN: Copy> {
    fn as_raw_slice(&self) -> (*const T, LEN);
}
unsafe impl<C, LEN: TryFrom<usize>> AsRawSlice<C, LEN> for [C]
where
    LEN: Copy,
    LEN::Error: Debug,
{
    fn as_raw_slice(&self) -> (*const C, LEN) {
        (self.as_ptr(), slice_len(self))
    }
}

pub unsafe trait AsMutRawSlice<T, LEN: Copy> {
    fn as_mut_raw_slice(&mut self) -> (*mut T, LEN);
}
unsafe impl<C, LEN: TryFrom<usize>> AsMutRawSlice<C, LEN> for [C]
where
    LEN: Copy,
    LEN::Error: Debug,
{
    fn as_mut_raw_slice(&mut self) -> (*mut C, LEN) {
        // Transmute is safe because MaybeUninit<C> has the same size and alignment as C
        <[MaybeUninit<C>]>::as_mut_raw_slice(unsafe { std::mem::transmute(self) })
    }
}
unsafe impl<C, LEN: TryFrom<usize>> AsMutRawSlice<C, LEN> for [MaybeUninit<C>]
where
    LEN: Copy,
    LEN::Error: Debug,
{
    fn as_mut_raw_slice(&mut self) -> (*mut C, LEN) {
        (self.as_mut_ptr().cast(), slice_len(self))
    }
}

pub trait Bool {}
pub struct True {}
impl Bool for True {}
pub struct False {}
impl Bool for False {}
pub trait Def {}
pub enum OdbcDefined {}
impl Def for OdbcDefined {}
pub enum DriverDefined {}
impl Def for DriverDefined {}

pub unsafe trait AttrRead<A>: AsMutSQLPOINTER {}
pub unsafe trait AttrWrite<A>: IntoSQLPOINTER {}

unsafe impl<A> AttrRead<A> for [SQLWCHAR] where [SQLCHAR]: AttrRead<A> {}
unsafe impl<'a, A> AttrWrite<A> for &'a [SQLWCHAR] where &'a [SQLCHAR]: AttrWrite<A> {}
unsafe impl<A: Ident, T: Ident> AttrWrite<A> for MaybeUninit<T>
where
    Self: IntoSQLPOINTER,
    T: AttrWrite<A>,
{
}
unsafe impl<A: Ident, T: Ident> AttrRead<A> for MaybeUninit<T>
where
    Self: AsMutSQLPOINTER,
    T: AttrRead<A>,
{
}
unsafe impl<A: Ident> AttrRead<A> for [MaybeUninit<SQLCHAR>] where [SQLCHAR]: AttrRead<A> {}
unsafe impl<A: Ident> AttrRead<A> for [MaybeUninit<SQLWCHAR>] where [SQLWCHAR]: AttrRead<A> {}

pub trait AnsiType {}
pub trait UnicodeType {}
impl<T: Ident> AnsiType for T {}
impl<T: Ident> UnicodeType for T {}
impl AnsiType for [SQLCHAR] {}
impl UnicodeType for [SQLWCHAR] {}
impl AnsiType for [MaybeUninit<SQLCHAR>] where [SQLCHAR]: AnsiType {}
impl UnicodeType for [MaybeUninit<SQLWCHAR>] where [SQLWCHAR]: UnicodeType {}

impl AnsiType for &[SQLCHAR] {}
impl UnicodeType for &[SQLWCHAR] {}
impl<'a> AnsiType for &'a [MaybeUninit<SQLCHAR>] where &'a [SQLCHAR]: AnsiType {}
impl<'a> UnicodeType for &'a [MaybeUninit<SQLWCHAR>] where &'a [SQLWCHAR]: UnicodeType {}

// TODO: Comapare attribute types: <attribute>(type, default)
// SQL_ATTR_OUTPUT_NTS(u32, true), SQL_ATTR_AUTO_IPD(u32, _)
#[odbc_type(SQLUINTEGER)]
#[allow(non_camel_case_types)]
pub struct OdbcBool;
pub const SQL_FALSE: OdbcBool = OdbcBool(0);
pub const SQL_TRUE: OdbcBool = OdbcBool(1);

// TODO
//pub use SQL_COLUMN_SEARCHABLE::SQL_SEARCHABLE as SQL_PRED_SEARCHABLE;
// Special return values for SQLGetData
// SQL_NO_TOTAL = -4,

#[odbc_type(SQLSMALLINT)]
// TODO: See how to name this struct
pub struct NullAllowed;
pub const SQL_NO_NULLS: NullAllowed = NullAllowed(0);
pub const SQL_NULLABLE: NullAllowed = NullAllowed(1);
// TODO: This value should not be used with SQLSpecialColumns
pub const SQL_NULLABLE_UNKNOWN: NullAllowed = NullAllowed(2);

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum DriverCompletion {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum IdentifierType {
    SQL_BEST_ROWID = 1,
    SQL_ROWVER = 2,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum BulkOperation {
    SQL_ADD = 4,
    SQL_UPDATE_BY_BOOKMARK = 5,
    SQL_DELETE_BY_BOOKMARK = 6,
    SQL_FETCH_BY_BOOKMARK = 7,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Operation {
    SQL_POSITION = 0,
    SQL_REFRESH = 1,
    SQL_UPDATE = 2,
    SQL_DELETE = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum LockType {
    SQL_LOCK_NO_CHANGE = 0,
    SQL_LOCK_EXCLUSIVE = 1,
    SQL_LOCK_UNLOCK = 2,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum CompletionType {
    SQL_COMMIT = 0,
    SQL_ROLLBACK = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum FreeStmtOption {
    SQL_CLOSE = 0,
    SQL_UNBIND = 2,
    SQL_RESET_PARAMS = 3,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Reserved {
    SQL_QUICK = 0,
    SQL_ENSURE = 1,
}

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Unique {
    SQL_INDEX_UNIQUE = 0,
    SQL_INDEX_ALL = 1,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
pub enum Scope {
    SQL_SCOPE_CURROW = 0,
    SQL_SCOPE_TRANSACTION = 1,
    SQL_SCOPE_SESSION = 2,
}

#[odbc_type(SQLSMALLINT)]
#[allow(non_camel_case_types)]
// TODO: Think about splitting for IO
pub enum ParameterType {
    SQL_PARAM_INPUT = 1,
    SQL_PARAM_INPUT_OUTPUT = 2,
    SQL_PARAM_OUTPUT = 4,

    SQL_PARAM_INPUT_OUTPUT_STREAM = 8,
    SQL_PARAM_OUTPUT_STREAM = 16,

    SQL_PARAM_TYPE_UNKNOWN = 0,
    SQL_RESULT_COL = 3,
    SQL_RETURN_VALUE = 5,
}

//pub const fn SQL_LEN_BINARY_ATTR<LEN>(length: LEN) {
//    const SQL_LEN_BINARY_ATTR_OFFSET: LEN = -100;
//    -length + SQL_LEN_BINARY_ATTR_OFFSET
//}

//#[derive(Ident)]
//#[ident(SQLSMALLINT, -99)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ARD_TYPE;
//impl<T> GetData<T> for SQL_ARD_TYPE {}
//
//#[derive(Ident)]
//#[ident(SQLSMALLINT, -100)]
//#[allow(non_camel_case_types)]
//pub struct SQL_APD_TYPE;
//impl<T> GetData<T> for SQL_APD_TYPE {}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES

// TODO: and what about SQLCHAR vs SQLWCHAR?
pub const SQL_ALL_CATALOGS: &str = "%";
pub const SQL_ALL_SCHEMAS: &str = "%";
pub const SQL_ALL_TABLE_TYPES: &str = "%";

#[odbc_type(SQLUSMALLINT)]
#[allow(non_camel_case_types)]
pub enum FunctionId {
    SQL_API_ODBC3_ALL_FUNCTIONS = 999,
    SQL_API_SQLALLOCCONNECT = 1,
    SQL_API_SQLALLOCENV = 2,
    SQL_API_SQLALLOCHANDLE = 1001,
    SQL_API_SQLALLOCSTMT = 3,
    SQL_API_SQLBINDCOL = 4,
    SQL_API_SQLBINDPARAM = 1002,
    SQL_API_SQLCANCEL = 5,
    SQL_API_SQLCLOSECURSOR = 1003,
    SQL_API_SQLCOLATTRIBUTE = 6,
    SQL_API_SQLCOLUMNS = 40,
    SQL_API_SQLCONNECT = 7,
    SQL_API_SQLCOPYDESC = 1004,
    SQL_API_SQLDATASOURCES = 57,
    SQL_API_SQLDESCRIBECOL = 8,
    SQL_API_SQLDISCONNECT = 9,
    SQL_API_SQLENDTRAN = 1005,
    SQL_API_SQLERROR = 10,
    SQL_API_SQLEXECDIRECT = 11,
    SQL_API_SQLEXECUTE = 12,
    SQL_API_SQLFETCH = 13,
    SQL_API_SQLFETCHSCROLL = 1021,
    SQL_API_SQLFREECONNECT = 14,
    SQL_API_SQLFREEENV = 15,
    SQL_API_SQLFREEHANDLE = 1006,
    SQL_API_SQLFREESTMT = 16,
    SQL_API_SQLGETCONNECTATTR = 1007,
    SQL_API_SQLGETCONNECTOPTION = 42,
    SQL_API_SQLGETCURSORNAME = 17,
    SQL_API_SQLGETDATA = 43,
    SQL_API_SQLGETDESCFIELD = 1008,
    SQL_API_SQLGETDESCREC = 1009,
    SQL_API_SQLGETDIAGFIELD = 1010,
    SQL_API_SQLGETDIAGREC = 1011,
    SQL_API_SQLGETENVATTR = 1012,
    SQL_API_SQLGETFUNCTIONS = 44,
    SQL_API_SQLGETINFO = 45,
    SQL_API_SQLGETSTMTATTR = 1014,
    SQL_API_SQLGETSTMTOPTION = 46,
    SQL_API_SQLGETTYPEINFO = 47,
    SQL_API_SQLNUMRESULTCOLS = 18,
    SQL_API_SQLPARAMDATA = 48,
    SQL_API_SQLPREPARE = 19,
    SQL_API_SQLPUTDATA = 49,
    SQL_API_SQLROWCOUNT = 20,
    SQL_API_SQLSETCONNECTATTR = 1016,
    SQL_API_SQLSETCONNECTOPTION = 50,
    SQL_API_SQLSETCURSORNAME = 21,
    SQL_API_SQLSETDESCFIELD = 1017,
    SQL_API_SQLSETDESCREC = 1018,
    SQL_API_SQLSETENVATTR = 1019,
    SQL_API_SQLSETPARAM = 22,
    SQL_API_SQLSETSTMTATTR = 1020,
    SQL_API_SQLSETSTMTOPTION = 51,
    SQL_API_SQLSPECIALCOLUMNS = 52,
    SQL_API_SQLSTATISTICS = 53,
    SQL_API_SQLTABLES = 54,
    SQL_API_SQLTRANSACT = 23,
    SQL_API_SQLCANCELHANDLE = 1550,
    SQL_API_SQLCOMPLETEASYNC = 1551,
}

//pub const fn SQL_FUNC_EXISTS(pfExists: SQLUSMALLINT, uwAPI: SQLUSMALLINT) -> OdbcBool {
//    if *((pfExists as *const UWORD).offset((uwAPI >> 4) as isize)) & (1 << (uwAPI & 0x000F)) {
//        return SQL_TRUE;
//    }
//
//    SQL_FALSE
//}

// TODO: Make it const
fn slice_len<T, LEN: TryFrom<usize>>(slice: &[T]) -> LEN
where
    LEN::Error: Debug,
{
    LEN::try_from(slice.len()).expect(SLICE_LEN_TOO_LARGE_MSG)
}

// TODO: Instead of implementing traits for every Option<T>, consider making a blanket impl for all T
