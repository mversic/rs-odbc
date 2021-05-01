use crate::handle::{AppDesc, AsSQLHANDLE, ImplDesc, SQLHDESC};
use crate::{
    extern_api, AsMutPtr, AsMutSQLPOINTER, Attr, AttrLen, AttrRead, AttrWrite, Ident, OdbcDefined,
    True, SQLHSTMT, SQLINTEGER, SQLPOINTER, SQLRETURN, SQLULEN, SQL_SUCCESS,
};
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;
use std::ops::Deref;

pub trait StmtAttr<'stmt, 'data, A: Ident>:
    Attr<A> + AttrLen<<Self as Attr<A>>::DefinedBy, <Self as Attr<A>>::NonBinary, SQLINTEGER>
{
    // TODO: Can I use here descriptor and statement defined on different connections??? This
    // should not be allowed If this is true, then I need to use unelided lifetime 'conn to
    // tie the lifetimes. Will that solve the problem?
    fn update_handle(&self, _: &SQLHSTMT<'_, 'stmt, 'data>)
    where
        Self: AttrWrite<A>,
    {
    }

    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN
    where
        MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        Self: AttrRead<A> + crate::AnsiType,
        A: Ident<Type = SQLINTEGER>,
    {
        let ValuePtrLen = self.len();

        unsafe {
            extern_api::SQLGetStmtAttrA(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                self.as_mut_SQLPOINTER(),
                ValuePtrLen,
                <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
            )
        }
    }

    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN
    where
        MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        Self: AttrRead<A> + crate::UnicodeType,
        A: Ident<Type = SQLINTEGER>,
    {
        let ValuePtrLen = self.len();

        unsafe {
            extern_api::SQLGetStmtAttrA(
                StatementHandle.as_SQLHANDLE(),
                A::IDENTIFIER,
                self.as_mut_SQLPOINTER(),
                ValuePtrLen,
                <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
            )
        }
    }
}

pub struct RefSQLHDESC<'stmt, T>(&'stmt SQLHDESC<'stmt, T>);
impl<'stmt, T> Ident for RefSQLHDESC<'stmt, T> {
    type Type = <&'stmt SQLHDESC<'stmt, T> as Ident>::Type;
    const IDENTIFIER: Self::Type = <&SQLHDESC<T>>::IDENTIFIER;
}
unsafe impl<'data, T: crate::handle::DescType<'data>> AsMutSQLPOINTER for MaybeUninit<RefSQLHDESC<'_, T>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        unimplemented!()
    }
}
impl<T> crate::AnsiType for MaybeUninit<RefSQLHDESC<'_, T>> {}
impl<T> crate::UnicodeType for MaybeUninit<RefSQLHDESC<'_, T>> {}
impl<'stmt, T> Deref for RefSQLHDESC<'stmt, T> {
    type Target = SQLHDESC<'stmt, T>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// TODO: These seem to be from v2.0
#[deprecated]
#[allow(non_camel_case_types)]
enum StmtOption {
    SQL_ROWSET_SIZE = 9,
    SQL_GET_BOOKMARK = 13,
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;
unsafe impl Attr<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl Attr<SQL_ATTR_MAX_ROWS> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_MAX_ROWS> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl Attr<SQL_ATTR_NOSCAN> for Noscan {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_NOSCAN> for Noscan {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;
unsafe impl Attr<SQL_ATTR_MAX_LENGTH> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_MAX_LENGTH> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
unsafe impl Attr<SQL_ATTR_CURSOR_TYPE> for CursorType {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_CURSOR_TYPE> for CursorType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
unsafe impl Attr<SQL_ATTR_CONCURRENCY> for Concurrency {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_CONCURRENCY> for Concurrency {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;
unsafe impl Attr<SQL_ATTR_KEYSET_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_KEYSET_SIZE> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl Attr<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl Attr<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_RETRIEVE_DATA> for RetrieveData {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl Attr<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl StmtAttr<'_, '_, SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {}

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
unsafe impl Attr<SQL_ATTR_APP_ROW_DESC> for &SQLHDESC<'_, AppDesc<'_>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
// TODO: I think this should be implemented only for MaybeUninit<RefSQLHDESC>
unsafe impl<'stmt, 'data> Attr<SQL_ATTR_APP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'data>>> {
    type DefinedBy =
        <&'stmt SQLHDESC<'stmt, AppDesc<'data>> as Attr<SQL_ATTR_APP_ROW_DESC>>::DefinedBy;
    type NonBinary =
        <&'stmt SQLHDESC<'stmt, AppDesc<'data>> as Attr<SQL_ATTR_APP_ROW_DESC>>::NonBinary;
}
impl<'stmt, 'data> StmtAttr<'stmt, 'data, SQL_ATTR_APP_ROW_DESC>
    for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'data>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
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
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        if let Some(explicit_ard) = StatementHandle.explicit_ard.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_ard));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ard));
        }

        SQL_SUCCESS
    }
}
impl<'stmt, 'data> StmtAttr<'stmt, 'data, SQL_ATTR_APP_ROW_DESC>
    for &'stmt SQLHDESC<'_, AppDesc<'data>>
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &SQLHSTMT<'_, 'stmt, 'data>) {
        StatementHandle.explicit_ard.set(Some(self));
    }
}
unsafe impl AttrRead<SQL_ATTR_APP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'_, AppDesc<'_>>> {}
unsafe impl AttrWrite<SQL_ATTR_APP_ROW_DESC> for &SQLHDESC<'_, AppDesc<'_>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10011)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl Attr<SQL_ATTR_APP_PARAM_DESC> for &SQLHDESC<'_, AppDesc<'_>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
unsafe impl<'stmt, 'data> Attr<SQL_ATTR_APP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'data>>> {
    type DefinedBy =
        <&'stmt SQLHDESC<'stmt, AppDesc<'data>> as Attr<SQL_ATTR_APP_PARAM_DESC>>::DefinedBy;
    type NonBinary =
        <&'stmt SQLHDESC<'stmt, AppDesc<'data>> as Attr<SQL_ATTR_APP_PARAM_DESC>>::NonBinary;
}
impl<'stmt, 'data> StmtAttr<'stmt, 'data, SQL_ATTR_APP_PARAM_DESC>
    for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'data>>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
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
        StatementHandle: &'stmt SQLHSTMT<'_, '_, 'data>,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        if let Some(explicit_apd) = StatementHandle.explicit_apd.get() {
            *self = MaybeUninit::new(RefSQLHDESC(explicit_apd));
        } else {
            *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.apd));
        }
        SQL_SUCCESS
    }
}
impl<'stmt, 'data> StmtAttr<'stmt, 'data, SQL_ATTR_APP_PARAM_DESC>
    for &'stmt SQLHDESC<'_, AppDesc<'data>>
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &SQLHSTMT<'_, 'stmt, 'data>) {
        StatementHandle.explicit_apd.set(Some(self));
    }
}
unsafe impl AttrRead<SQL_ATTR_APP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'_, AppDesc<'_>>> {}
unsafe impl AttrWrite<SQL_ATTR_APP_PARAM_DESC> for &SQLHDESC<'_, AppDesc<'_>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10012)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl Attr<SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'_, ImplDesc>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<'stmt> StmtAttr<'stmt, '_, SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'stmt, ImplDesc>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ird));
        SQL_SUCCESS
    }
    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ird));
        SQL_SUCCESS
    }
}
unsafe impl AttrRead<SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<RefSQLHDESC<'_, ImplDesc>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10013)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl Attr<SQL_ATTR_IMP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'_, ImplDesc>> {
    type DefinedBy = OdbcDefined;
    type NonBinary = True;
}
impl<'stmt> StmtAttr<'stmt, '_, SQL_ATTR_IMP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'stmt, ImplDesc>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ipd));
        SQL_SUCCESS
    }
    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT,
        StringLengthPtr: &mut MaybeUninit<Self::StrLen>,
    ) -> SQLRETURN {
        *self = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ipd));
        SQL_SUCCESS
    }
}
unsafe impl AttrRead<SQL_ATTR_IMP_PARAM_DESC> for MaybeUninit<RefSQLHDESC<'_, ImplDesc>> {}

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
//impl AttrRead<MaybeUninit<SQLUINTEGER>> for SQL_ATTR_METADATA_ID {}
//
#[derive(Ident)]
#[identifier(SQLINTEGER, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
//impl AttrRead<MaybeUninit<SQLULEN>> for SQL_ATTR_ASYNC_ENABLE {}

#[odbc_type(SQLULEN)]
pub struct Noscan;
pub const SQL_NOSCAN_OFF: Noscan = Noscan(0);
pub const SQL_NOSCAN_ON: Noscan = Noscan(1);
pub use SQL_NOSCAN_OFF as SQL_NOSCAN_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct CursorType;
pub const SQL_CURSOR_FORWARD_ONLY: CursorType = CursorType(0);
pub const SQL_CURSOR_KEYSET_DRIVEN: CursorType = CursorType(1);
pub const SQL_CURSOR_DYNAMIC: CursorType = CursorType(2);
pub const SQL_CURSOR_STATIC: CursorType = CursorType(3);
pub use SQL_CURSOR_FORWARD_ONLY as SQL_CURSOR_TYPE_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct Concurrency;
pub const SQL_CONCUR_READ_ONLY: Concurrency = Concurrency(1);
pub const SQL_CONCUR_LOCK: Concurrency = Concurrency(2);
pub const SQL_CONCUR_ROWVER: Concurrency = Concurrency(3);
pub const SQL_CONCUR_VALUES: Concurrency = Concurrency(4);
pub use SQL_CONCUR_READ_ONLY as SQL_CONCUR_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct SimulateCursor;
pub const SQL_SC_NON_UNIQUE: SimulateCursor = SimulateCursor(0);
pub const SQL_SC_TRY_UNIQUE: SimulateCursor = SimulateCursor(1);
pub const SQL_SC_UNIQUE: SimulateCursor = SimulateCursor(2);

#[odbc_type(SQLULEN)]
pub struct RetrieveData;
pub const SQL_RD_OFF: RetrieveData = RetrieveData(0);
pub const SQL_RD_ON: RetrieveData = RetrieveData(1);
pub use SQL_RD_ON as SQL_RD_DEFAULT;

#[odbc_type(SQLULEN)]
pub struct UseBookmarks;
pub const SQL_UB_OFF: UseBookmarks = UseBookmarks(0);
pub const SQL_UB_ON: UseBookmarks = UseBookmarks(1);
pub use SQL_UB_OFF as SQL_UB_DEFAULT;

impl<'stmt, 'data, A: Ident, T: Ident> StmtAttr<'stmt, 'data, A> for MaybeUninit<T>
where
    T: StmtAttr<'stmt, 'data, A>,
    Self: Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
impl<'stmt, 'data, 'a, A: Ident, T> StmtAttr<'stmt, 'data, A> for &'a [MaybeUninit<T>]
where
    &'a [T]: StmtAttr<'stmt, 'data, A>,
    Self: Attr<A> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLINTEGER>,
{
}
