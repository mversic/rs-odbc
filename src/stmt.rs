use crate::handle::{AppDesc, AsSQLHANDLE, ImplDesc, SQLHDESC};
use crate::{
    AsMutPtr, AsMutSQLPOINTER, Len, OdbcAttr, ReadAttr, WriteAttr, SQLHSTMT, SQLINTEGER,
    SQLPOINTER, SQLRETURN, SQLSMALLINT, SQLULEN, SQL_SUCCESS,
};
use rs_odbc_derive::{odbc_type, Identifier, StmtAttr};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;

pub trait WriteStmtAttr<'conn, T, C>: WriteAttr<T, C> + StmtAttr {
    fn update_handle(_: &SQLHSTMT<'conn>, _: &T) {}
}

pub trait ReadStmtAttr<'stmt, T, C>: ReadAttr<T, C> + StmtAttr
where
    T: Len<Self::AttrType, SQLINTEGER>,
    MaybeUninit<T::StrLen>: AsMutPtr<SQLINTEGER>,
{
    #[allow(non_snake_case)]
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut T,
        StringLengthPtr: &mut MaybeUninit<T::StrLen>,
    ) -> SQLRETURN;

    #[allow(non_snake_case)]
    unsafe fn read_as_SQLPOINTER(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut T,
        StringLengthPtr: &mut MaybeUninit<T::StrLen>,
    ) -> SQLRETURN
    where
        T: AsMutSQLPOINTER,
    {
        let ValuePtrLen = ValuePtr.len();

        crate::extern_api::SQLGetStmtAttrA(
            StatementHandle.as_SQLHANDLE(),
            Self::IDENTIFIER,
            ValuePtr.as_mut_SQLPOINTER(),
            ValuePtrLen,
            <MaybeUninit<_> as AsMutPtr<_>>::as_mut_ptr(StringLengthPtr),
        )
    }
}

pub trait StmtAttr: crate::Identifier<IdentType = SQLINTEGER> {
    type AttrType;
}

pub enum RefSQLHDESC<'stmt> {
    // TODO: Use SQLHDESC<'conn, AppDesc> if SQLHDESC is to be used after
    // statement is dropped, i.e. if unwrap method() is added on this struct
    Implicit(&'stmt SQLHDESC<'stmt, AppDesc>),
    Explicit(Rc<SQLHDESC<'stmt, AppDesc>>),
}
impl<'stmt> Deref for RefSQLHDESC<'stmt> {
    type Target = SQLHDESC<'stmt, AppDesc>;

    fn deref(&self) -> &Self::Target {
        match self {
            RefSQLHDESC::Implicit(descriptor) => descriptor,
            RefSQLHDESC::Explicit(descriptor) => descriptor,
        }
    }
}
unsafe impl AsMutSQLPOINTER for MaybeUninit<RefSQLHDESC<'_>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        unimplemented!("")
    }
}
unsafe impl<LEN: Copy> Len<OdbcAttr, LEN> for MaybeUninit<RefSQLHDESC<'_>>
where
    LEN: From<SQLSMALLINT>,
{
    type StrLen = ();
    fn len(&self) -> LEN {
        LEN::from(crate::SQL_IS_POINTER)
    }
}

// TODO: These seem to be from v2.0
#[deprecated]
#[allow(non_camel_case_types)]
enum StmtOption {
    SQL_ROWSET_SIZE = 9,
    SQL_GET_BOOKMARK = 13,
}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<MaybeUninit<SQLULEN>, C> for SQL_ATTR_QUERY_TIMEOUT {}
unsafe impl<C> WriteAttr<SQLULEN, C> for SQL_ATTR_QUERY_TIMEOUT {}
impl<'stmt, C> ReadStmtAttr<'stmt, MaybeUninit<SQLULEN>, C> for SQL_ATTR_QUERY_TIMEOUT {
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut MaybeUninit<SQLULEN>,
        StringLengthPtr: &mut MaybeUninit<
            <MaybeUninit<SQLULEN> as Len<Self::AttrType, SQLINTEGER>>::StrLen,
        >,
    ) -> SQLRETURN {
        SQL_SUCCESS
        //ReadStmtAttr::read_as_SQLPOINTER(StatementHandle, ValuePtr, StringLengthPtr)
    }
}
impl<'conn, C> WriteStmtAttr<'conn, SQLULEN, C> for SQL_ATTR_QUERY_TIMEOUT {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl<C> ReadAttr<MaybeUninit<SQLULEN>, C> for SQL_ATTR_MAX_ROWS {}
unsafe impl<C> WriteAttr<SQLULEN, C> for SQL_ATTR_MAX_ROWS {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl<C> ReadAttr<MaybeUninit<Noscan>, C> for SQL_ATTR_NOSCAN {}
unsafe impl<C> WriteAttr<Noscan, C> for SQL_ATTR_NOSCAN {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<MaybeUninit<SQLULEN>, C> for SQL_ATTR_MAX_LENGTH {}
unsafe impl<C> WriteAttr<SQLULEN, C> for SQL_ATTR_MAX_LENGTH {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
unsafe impl<C> ReadAttr<MaybeUninit<CursorType>, C> for SQL_ATTR_CURSOR_TYPE {}
unsafe impl<C> WriteAttr<CursorType, C> for SQL_ATTR_CURSOR_TYPE {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
unsafe impl<C> ReadAttr<MaybeUninit<Concurrency>, C> for SQL_ATTR_CONCURRENCY {}
unsafe impl<C> WriteAttr<Concurrency, C> for SQL_ATTR_CONCURRENCY {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;
unsafe impl<C> ReadAttr<MaybeUninit<SQLULEN>, C> for SQL_ATTR_KEYSET_SIZE {}
unsafe impl<C> WriteAttr<SQLULEN, C> for SQL_ATTR_KEYSET_SIZE {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl<C> ReadAttr<MaybeUninit<SimulateCursor>, C> for SQL_ATTR_SIMULATE_CURSOR {}
unsafe impl<C> WriteAttr<SimulateCursor, C> for SQL_ATTR_SIMULATE_CURSOR {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl<C> ReadAttr<MaybeUninit<RetrieveData>, C> for SQL_ATTR_RETRIEVE_DATA {}
unsafe impl<C> WriteAttr<RetrieveData, C> for SQL_ATTR_RETRIEVE_DATA {}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl<C> ReadAttr<MaybeUninit<UseBookmarks>, C> for SQL_ATTR_USE_BOOKMARKS {}
unsafe impl<C> WriteAttr<UseBookmarks, C> for SQL_ATTR_USE_BOOKMARKS {}

//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 15)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ENABLE_AUTO_IPD;
//
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 16)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;
//
//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 5)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 17)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 18)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 20)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 21)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 22)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 23)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 24)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 25)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 26)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 27)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(SQLINTEGER, 29)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v3_8")]
//#[allow(non_camel_case_types)]
// TODO: This type MUST be Rc or similar
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(SQLINTEGER, 30)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(SQLINTEGER, 31)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(SQLINTEGER, 32)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 33)]
//#[derive(Identifier, StmtAttr)]
//#[cfg(feature = "v4")]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;
//
//// TODO: Write-only - Cannot be used with SetIdentifier, StmtAttr
//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, 14)]
//#[allow(non_camel_case_types)]
//// This is read-only attribute
//pub struct SQL_ATTR_ROW_NUMBER;

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10010)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_ROW_DESC;
// Explicit descriptors must not be dropped, I think they can be shared by other
// means and I don't think this is required
unsafe impl<C> ReadAttr<MaybeUninit<RefSQLHDESC<'_>>, C> for SQL_ATTR_APP_ROW_DESC {}
impl<'stmt, C> ReadStmtAttr<'stmt, MaybeUninit<RefSQLHDESC<'stmt>>, C> for SQL_ATTR_APP_ROW_DESC {
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut MaybeUninit<RefSQLHDESC<'stmt>>,
        StringLengthPtr: &mut MaybeUninit<
            <MaybeUninit<RefSQLHDESC<'_>> as Len<Self::AttrType, SQLINTEGER>>::StrLen,
        >,
    ) -> SQLRETURN {
        let explicit_ard = StatementHandle.explicit_ard.take();

        std::ptr::write(
            ValuePtr.as_mut_ptr(),
            if let Some(explicit_ard) = &explicit_ard {
                RefSQLHDESC::Explicit(Rc::clone(&explicit_ard))
            } else {
                RefSQLHDESC::Implicit(&StatementHandle.ard)
            },
        );

        StatementHandle.explicit_ard.set(explicit_ard);
        SQL_SUCCESS
    }
}
unsafe impl<C> WriteAttr<Rc<SQLHDESC<'_, AppDesc>>, C> for SQL_ATTR_APP_ROW_DESC {}
impl<'conn, C> WriteStmtAttr<'conn, Rc<SQLHDESC<'conn, AppDesc>>, C> for SQL_ATTR_APP_ROW_DESC {
    fn update_handle(stmt: &SQLHSTMT<'conn>, val: &Rc<SQLHDESC<'conn, AppDesc>>) {
        stmt.explicit_ard.replace(Some(Rc::clone(val)));
    }
}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10011)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl<C> ReadAttr<MaybeUninit<RefSQLHDESC<'_>>, C> for SQL_ATTR_APP_PARAM_DESC {}
impl<'stmt, C> ReadStmtAttr<'stmt, MaybeUninit<RefSQLHDESC<'stmt>>, C> for SQL_ATTR_APP_PARAM_DESC {
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut MaybeUninit<RefSQLHDESC<'stmt>>,
        StringLengthPtr: &mut MaybeUninit<
            <MaybeUninit<RefSQLHDESC<'_>> as Len<Self::AttrType, SQLINTEGER>>::StrLen,
        >,
    ) -> SQLRETURN {
        let explicit_apd = StatementHandle.explicit_apd.take();

        std::ptr::write(
            ValuePtr.as_mut_ptr(),
            if let Some(explicit_apd) = &explicit_apd {
                RefSQLHDESC::Explicit(Rc::clone(&explicit_apd))
            } else {
                RefSQLHDESC::Implicit(&StatementHandle.apd)
            },
        );

        StatementHandle.explicit_apd.set(explicit_apd);
        SQL_SUCCESS
    }
}
unsafe impl<C> WriteAttr<Rc<SQLHDESC<'_, AppDesc>>, C> for SQL_ATTR_APP_PARAM_DESC {}
impl<'conn, C> WriteStmtAttr<'conn, Rc<SQLHDESC<'conn, AppDesc>>, C> for SQL_ATTR_APP_PARAM_DESC {
    fn update_handle(stmt: &SQLHSTMT<'conn>, val: &Rc<SQLHDESC<'conn, AppDesc>>) {
        stmt.explicit_apd.replace(Some(Rc::clone(val)));
    }
}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10012)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl<C> ReadAttr<MaybeUninit<&SQLHDESC<'_, ImplDesc>>, C> for SQL_ATTR_IMP_ROW_DESC {}
impl<'stmt, C> ReadStmtAttr<'stmt, MaybeUninit<&SQLHDESC<'stmt, ImplDesc>>, C>
    for SQL_ATTR_IMP_ROW_DESC
{
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut MaybeUninit<&SQLHDESC<'stmt, ImplDesc>>,
        StringLengthPtr: &mut MaybeUninit<
            <MaybeUninit<&SQLHDESC<'_, ImplDesc>> as Len<Self::AttrType, SQLINTEGER>>::StrLen,
        >,
    ) -> SQLRETURN {
        std::ptr::write(ValuePtr.as_mut_ptr(), &StatementHandle.ird);

        SQL_SUCCESS
    }
}

#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10013)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl<C> ReadAttr<MaybeUninit<&SQLHDESC<'_, ImplDesc>>, C> for SQL_ATTR_IMP_PARAM_DESC {}
impl<'stmt, C> ReadStmtAttr<'stmt, MaybeUninit<&SQLHDESC<'stmt, ImplDesc>>, C>
    for SQL_ATTR_IMP_PARAM_DESC
{
    unsafe fn read(
        StatementHandle: &'stmt SQLHSTMT,
        ValuePtr: &mut MaybeUninit<&SQLHDESC<'stmt, ImplDesc>>,
        StringLengthPtr: &mut MaybeUninit<
            <MaybeUninit<&SQLHDESC<'_, ImplDesc>> as Len<Self::AttrType, SQLINTEGER>>::StrLen,
        >,
    ) -> SQLRETURN {
        std::ptr::write(ValuePtr.as_mut_ptr(), &StatementHandle.ipd);

        SQL_SUCCESS
    }
}

//#[derive(Identifier, StmtAttr)]
//#[identifier(SQLINTEGER, -1)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_CURSOR_SCROLLABLE;
//
//#[derive(Identifier, StmtAttr)]
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
#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 10014)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
//impl<C> ReadAttr<MaybeUninit<SQLUINTEGER>, C> for SQL_ATTR_METADATA_ID {}
//
#[derive(Identifier, StmtAttr)]
#[identifier(SQLINTEGER, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
//impl<C> ReadAttr<MaybeUninit<SQLULEN>, C> for SQL_ATTR_ASYNC_ENABLE {}

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
