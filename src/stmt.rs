use crate::handle::{AppDesc, AsSQLHANDLE, ImplDesc, ParamDesc, RowDesc, SQLHDESC};
use crate::{
    extern_api, handle::SQLHSTMT, sqlreturn::SQLRETURN, AsMutPtr, AsMutSQLPOINTER, Attr, AttrLen,
    AttrRead, AttrWrite, Ident, OdbcBool, OdbcDefined, True, Version, SQLCHAR, SQLINTEGER,
    SQLPOINTER, SQLULEN, SQLWCHAR, V3, V3_8, V4,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;
use std::ops::Deref;

#[cfg(feature = "odbc_debug")]
use crate::sqlreturn::SQL_SUCCESS;

pub trait StmtAttr<'stmt, 'buf, A: Ident, V: Version>:
    Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>
{
    // TODO: Can I use here descriptor and statement defined on different connections??? This
    // should not be allowed If this is true, then I need to use unelided lifetime 'conn to
    // tie the lifetimes. Will that solve the problem?
    fn update_handle(&self, _: &SQLHSTMT<'_, 'stmt, 'buf, V>)
    where
        Self: AttrWrite<A>,
    {
    }

    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf, V>,
        StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN
    where
        A: Ident<Type = SQLINTEGER>,
        Self: AttrRead<A> + crate::AnsiType,
        MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
    {
        let ValuePtrLen = self.len();

        unsafe {
            extern_api::SQLGetStmtAttrA(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                self.as_mut_SQLPOINTER(),
                ValuePtrLen,
                StringLengthPtr.map_or_else(std::ptr::null_mut, AsMutPtr::as_mut_ptr),
            )
        }
    }

    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf, V>,
        StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN
    where
        A: Ident<Type = SQLINTEGER>,
        Self: AttrRead<A> + crate::UnicodeType,
        MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
    {
        let ValuePtrLen = self.len();

        unsafe {
            extern_api::SQLGetStmtAttrW(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                self.as_mut_SQLPOINTER(),
                ValuePtrLen,
                StringLengthPtr.map_or_else(std::ptr::null_mut, AsMutPtr::as_mut_ptr),
            )
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct RefSQLHDESC<'stmt, V, T>(&'stmt SQLHDESC<'stmt, V, T>);
impl<'stmt, V, T> Ident for RefSQLHDESC<'stmt, V, T> {
    type Type = <Option<&'stmt SQLHDESC<'stmt, V, T>> as Ident>::Type;
    const IDENTIFIER: Self::Type = <Option<&SQLHDESC<V, T>>>::IDENTIFIER;
}
unsafe impl<'buf, V, T: crate::handle::DescType<'buf>> AsMutSQLPOINTER
    for MaybeUninit<RefSQLHDESC<'_, V, T>>
{
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        if cfg!(feature = "odbc_debug") {
            // SQLHDESC is not transparent
            unimplemented!("This method should never be called")
        } else {
            // SQLHDESC is transparent
            self.as_mut_ptr().cast()
        }
    }
}
impl<'stmt, V, T> Deref for RefSQLHDESC<'stmt, V, T> {
    type Target = SQLHDESC<'stmt, V, T>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// TODO: Implement functions in these blanket implementations
impl<'stmt, 'buf, A: Ident> StmtAttr<'stmt, 'buf, A, V3> for &[SQLCHAR] where
    [SQLCHAR]: StmtAttr<'stmt, 'buf, A, V3>
{
}
impl<'stmt, 'buf, A: Ident> StmtAttr<'stmt, 'buf, A, V3> for &[SQLWCHAR] where
    [SQLWCHAR]: StmtAttr<'stmt, 'buf, A, V3>
{
}

impl<'stmt, 'buf, A: Ident> StmtAttr<'stmt, 'buf, A, V3> for [SQLWCHAR]
where
    [SQLCHAR]: StmtAttr<'stmt, 'buf, A, V3, NonBinary = True>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}

impl<'stmt, 'buf, A: Ident> StmtAttr<'stmt, 'buf, A, V3> for [MaybeUninit<SQLCHAR>]
where
    [SQLCHAR]: StmtAttr<'stmt, 'buf, A, V3>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
impl<'stmt, 'buf, A: Ident> StmtAttr<'stmt, 'buf, A, V3> for [MaybeUninit<SQLWCHAR>]
where
    [SQLWCHAR]: StmtAttr<'stmt, 'buf, A, V3>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
impl<'stmt, 'buf, A: Ident, T: Ident> StmtAttr<'stmt, 'buf, A, V3> for MaybeUninit<T>
where
    T: StmtAttr<'stmt, 'buf, A, V3>,
    Self: AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}

impl<'stmt, 'buf, A: Ident, T: StmtAttr<'stmt, 'buf, A, V3>> StmtAttr<'stmt, 'buf, A, V3_8> for T where T: ?Sized {
}
impl<'stmt, 'buf, A: Ident, T: StmtAttr<'stmt, 'buf, A, V3_8>> StmtAttr<'stmt, 'buf, A, V4> for T where T: ?Sized {
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
unsafe impl Attr<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_QUERY_TIMEOUT, V3> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl Attr<SQL_ATTR_MAX_ROWS> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_MAX_ROWS, V3> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl Attr<SQL_ATTR_NOSCAN> for Noscan {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_NOSCAN, V3> for Noscan {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
unsafe impl Attr<SQL_ATTR_MAX_LENGTH> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_MAX_LENGTH, V3> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
unsafe impl Attr<SQL_ATTR_CURSOR_TYPE> for CursorType {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_CURSOR_TYPE, V3> for CursorType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
unsafe impl Attr<SQL_ATTR_CONCURRENCY> for Concurrency {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_CONCURRENCY, V3> for Concurrency {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
unsafe impl Attr<SQL_ATTR_KEYSET_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_KEYSET_SIZE, V3> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl Attr<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_SIMULATE_CURSOR, V3> for SimulateCursor {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl Attr<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_RETRIEVE_DATA, V3> for RetrieveData {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl Attr<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_USE_BOOKMARKS, V3> for UseBookmarks {}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, 15)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ENABLE_AUTO_IPD;
//
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 16)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
//
//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 5)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 17)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 20)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 21)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 22)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//

// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 23)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 24)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 25)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 26)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 27)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(SQLINTEGER, 29)]
//#[derive(Ident)]
//#[cfg(feature = "v3_8")]
//#[allow(non_camel_case_types)]
// TODO: This type MUST be Rc or similar
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(SQLINTEGER, 30)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(SQLINTEGER, 31)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(SQLINTEGER, 32)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 33)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
//
//// TODO: Write-only - Cannot be used with Setident
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 14)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_ROW_NUMBER;

#[derive(Ident)]
#[identifier(SQLINTEGER, 10010)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_ROW_DESC;
unsafe impl<V> Attr<SQL_ATTR_APP_ROW_DESC> for Option<&SQLHDESC<'_, V, AppDesc<'_>>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl<'stmt, 'buf, V> Attr<SQL_ATTR_APP_ROW_DESC>
    for MaybeUninit<RefSQLHDESC<'stmt, V, AppDesc<'buf>>>
{
    type DefinedBy = <Option<&'stmt SQLHDESC<'stmt, V, AppDesc<'buf>>> as Attr<
        SQL_ATTR_APP_ROW_DESC,
    >>::DefinedBy;
    type NonBinary = <Option<&'stmt SQLHDESC<'stmt, V, AppDesc<'buf>>> as Attr<
        SQL_ATTR_APP_ROW_DESC,
    >>::NonBinary;
}
impl<'stmt, 'buf, V> StmtAttr<'stmt, 'buf, SQL_ATTR_APP_ROW_DESC, V3>
    for MaybeUninit<RefSQLHDESC<'stmt, V, AppDesc<'buf>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        if let Some(explicit_ard) = StatementHandle.explicit_ard.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_ard));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ard));
        }

        SQL_SUCCESS
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        if let Some(explicit_ard) = StatementHandle.explicit_ard.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_ard));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ard));
        }

        SQL_SUCCESS
    }
}
impl<'stmt, 'buf, V> StmtAttr<'stmt, 'buf, SQL_ATTR_APP_ROW_DESC, V3>
    for Option<&'stmt SQLHDESC<'_, V, AppDesc<'buf>>>
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &SQLHSTMT<'_, 'stmt, 'buf>) {
        StatementHandle.explicit_ard.set(*self);
    }
}
unsafe impl<V> AttrRead<SQL_ATTR_APP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'_, V, AppDesc<'_>>> {}
unsafe impl<V> AttrWrite<SQL_ATTR_APP_ROW_DESC> for Option<&SQLHDESC<'_, V, AppDesc<'_>>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10011)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl<V> Attr<SQL_ATTR_APP_PARAM_DESC> for Option<&SQLHDESC<'_, V, AppDesc<'_>>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl<'stmt, 'buf, V> Attr<SQL_ATTR_APP_PARAM_DESC>
    for MaybeUninit<RefSQLHDESC<'stmt, V, AppDesc<'buf>>>
{
    type DefinedBy = <Option<&'stmt SQLHDESC<'stmt, V, AppDesc<'buf>>> as Attr<
        SQL_ATTR_APP_PARAM_DESC,
    >>::DefinedBy;
    type NonBinary = <Option<&'stmt SQLHDESC<'stmt, V, AppDesc<'buf>>> as Attr<
        SQL_ATTR_APP_PARAM_DESC,
    >>::NonBinary;
}
impl<'stmt, 'buf, V> StmtAttr<'stmt, 'buf, SQL_ATTR_APP_PARAM_DESC, V3>
    for MaybeUninit<RefSQLHDESC<'stmt, V, AppDesc<'buf>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        if let Some(explicit_apd) = StatementHandle.explicit_apd.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_apd));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.apd));
        }

        SQL_SUCCESS
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'buf>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        if let Some(explicit_apd) = StatementHandle.explicit_apd.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_apd));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.apd));
        }

        SQL_SUCCESS
    }
}
impl<'stmt, 'buf, V> StmtAttr<'stmt, 'buf, SQL_ATTR_APP_PARAM_DESC, V3>
    for Option<&'stmt SQLHDESC<'_, V, AppDesc<'buf>>>
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &SQLHSTMT<'_, 'stmt, 'buf>) {
        StatementHandle.explicit_apd.set(*self);
    }
}
unsafe impl<V> AttrRead<SQL_ATTR_APP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'_, V, AppDesc<'_>>> {}
unsafe impl<V> AttrWrite<SQL_ATTR_APP_PARAM_DESC> for Option<&SQLHDESC<'_, V, AppDesc<'_>>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10012)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl<V> Attr<SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'_, V, ImplDesc<RowDesc>>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<'stmt, V> StmtAttr<'stmt, '_, SQL_ATTR_IMP_ROW_DESC, V3>
    for MaybeUninit<RefSQLHDESC<'stmt, V, ImplDesc<RowDesc>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ird));
        SQL_SUCCESS
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ird));
        SQL_SUCCESS
    }
}
unsafe impl<V> AttrRead<SQL_ATTR_IMP_ROW_DESC>
    for MaybeUninit<RefSQLHDESC<'_, V, ImplDesc<RowDesc>>>
{
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10013)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl<V> Attr<SQL_ATTR_IMP_PARAM_DESC>
    for MaybeUninit<RefSQLHDESC<'_, V, ImplDesc<ParamDesc>>>
{
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<'stmt, V> StmtAttr<'stmt, '_, SQL_ATTR_IMP_PARAM_DESC, V3>
    for MaybeUninit<RefSQLHDESC<'stmt, V, ImplDesc<ParamDesc>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ipd));
        SQL_SUCCESS
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ipd));
        SQL_SUCCESS
    }
}
unsafe impl<V> AttrRead<SQL_ATTR_IMP_PARAM_DESC>
    for MaybeUninit<RefSQLHDESC<'_, V, ImplDesc<ParamDesc>>>
{
}

//#[derive(Ident)]
//#[identifier(SQLINTEGER, -1)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SCROLLABLE;
//
//#[derive(Ident)]
//#[identifier(SQLINTEGER, -2)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SENSITIVITY;
//
//// TODO: Not found in implementation
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCALLBACK
//// #[cfg(feature = "v3_8")]
//// SQL_ATTR_ASYNC_STMT_PCONTEXT
//
#[derive(Ident)]
#[identifier(SQLINTEGER, 10014)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
unsafe impl Attr<SQL_ATTR_METADATA_ID> for OdbcBool {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
//impl StmtAttr<SQL_ATTR_METADATA_ID> for OdbcBool {}
//unsafe impl AttrRead<SQL_ATTR_METADATA_ID> for OdbcBool {}
//unsafe impl AttrWrite<SQL_ATTR_METADATA_ID> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
unsafe impl Attr<SQL_ATTR_ASYNC_ENABLE> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
//impl StmtAttr<SQL_ATTR_ASYNC_ENABLE> for SQLULEN {}
//unsafe impl AttrRead<SQL_ATTR_ASYNC_ENABLE> for SQLULEN {}
//unsafe impl AttrWrite<SQL_ATTR_ASYNC_ENABLE> for SQLULEN {}

//=====================================================================================//

#[odbc_type(SQLULEN)]
pub struct Noscan;
pub const SQL_NOSCAN_OFF: Noscan = Noscan(0);
pub const SQL_NOSCAN_ON: Noscan = Noscan(1);

#[odbc_type(SQLULEN)]
pub struct CursorType;
pub const SQL_CURSOR_FORWARD_ONLY: CursorType = CursorType(0);
pub const SQL_CURSOR_KEYSET_DRIVEN: CursorType = CursorType(1);
pub const SQL_CURSOR_DYNAMIC: CursorType = CursorType(2);
pub const SQL_CURSOR_STATIC: CursorType = CursorType(3);

#[odbc_type(SQLULEN)]
pub struct Concurrency;
pub const SQL_CONCUR_READ_ONLY: Concurrency = Concurrency(1);
pub const SQL_CONCUR_LOCK: Concurrency = Concurrency(2);
pub const SQL_CONCUR_ROWVER: Concurrency = Concurrency(3);
pub const SQL_CONCUR_VALUES: Concurrency = Concurrency(4);

#[odbc_type(SQLULEN)]
pub struct SimulateCursor;
pub const SQL_SC_NON_UNIQUE: SimulateCursor = SimulateCursor(0);
pub const SQL_SC_TRY_UNIQUE: SimulateCursor = SimulateCursor(1);
pub const SQL_SC_UNIQUE: SimulateCursor = SimulateCursor(2);

#[odbc_type(SQLULEN)]
pub struct RetrieveData;
pub const SQL_RD_OFF: RetrieveData = RetrieveData(0);
pub const SQL_RD_ON: RetrieveData = RetrieveData(1);

#[odbc_type(SQLULEN)]
pub struct UseBookmarks;
pub const SQL_UB_OFF: UseBookmarks = UseBookmarks(0);
pub const SQL_UB_ON: UseBookmarks = UseBookmarks(1);

// TODO: These seem to be from v2.0
#[deprecated]
#[allow(non_camel_case_types)]
enum StmtOption {
    SQL_ROWSET_SIZE = 9,
    SQL_GET_BOOKMARK = 13,
}
