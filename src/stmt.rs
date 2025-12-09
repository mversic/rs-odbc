use crate::api::Statement;
#[double]
use crate::api::ffi;
use crate::attr::{Attr, AttrGet, AttrLen, AttrSet, StrLen};
use crate::desc::{AppDesc, IPD, IRD};
use crate::env::{OdbcVersion, SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::handle::{RefSQLHDESC, RefUnsafeSQLHDESC, SQLHSTMT, UnsafeSQLHSTMT};
use crate::handle::{SQLHDESC, UnsafeSQLHDESC};
use crate::str::{Ansi, OdbcChar, OdbcStr, Unicode};
use crate::{
    Ident, OdbcBool, OdbcDefined, Ref, SQLCHAR, SQLINTEGER, SQLULEN, SQLWCHAR, Scalar,
    sqlreturn::SQLRETURN,
};
use core::mem::MaybeUninit;
use mockall_double::double;
use rs_odbc_derive::{Ident, odbc_type};

pub(crate) mod private {
    use super::*;

    #[expect(non_snake_case)]
    pub trait BaseStmtAttr<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, V: OdbcVersion>:
        Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER>
    {
        fn update_handle(&self, _: &S)
        where
            Self: AttrSet<A>,
        {
        }

        fn readA<'stmt>(
            &mut self,
            StatementHandle: &'stmt S,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Ansi + Ref<'stmt>,
            MaybeUninit<Self::StrLen>: StrLen<SQLINTEGER>,
        {
            let ValuePtrLen = self.len();

            unsafe {
                ffi::SQLGetStmtAttrA(
                    StatementHandle.as_SQLHANDLE(),
                    A::IDENTIFIER,
                    self.as_mut_SQLPOINTER(),
                    ValuePtrLen,
                    StringLengthPtr.map_or_else(core::ptr::null_mut, StrLen::as_mut_ptr),
                )
            }
        }

        fn readW<'stmt>(
            &mut self,
            StatementHandle: &'stmt S,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Unicode + Ref<'stmt>,
            MaybeUninit<Self::StrLen>: StrLen<SQLINTEGER>,
        {
            let ValuePtrLen = self.len();

            unsafe {
                ffi::SQLGetStmtAttrW(
                    StatementHandle.as_SQLHANDLE(),
                    A::IDENTIFIER,
                    self.as_mut_SQLPOINTER(),
                    ValuePtrLen,
                    StringLengthPtr.map_or_else(core::ptr::null_mut, StrLen::as_mut_ptr),
                )
            }
        }
    }

    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for T
    where
        Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER>,
    {
    }
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for [T]
    where
        Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER>,
    {
    }
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, CH: OdbcChar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for OdbcStr<CH>
    where
        Self: Attr<A>,
    {
    }

    // Implement BaseStmtAttr for uninitialized statement attributes
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for MaybeUninit<T>
    where
        T: BaseStmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
        Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
    {
    }
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for [MaybeUninit<T>]
    where
        [T]: BaseStmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
        Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
    {
    }
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, CH: OdbcChar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for OdbcStr<MaybeUninit<CH>>
    where
        OdbcStr<CH>: BaseStmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
        Self: Attr<A>,
    {
    }

    // Implement BaseStmtAttr for references to unsized (used by AttrSet)
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for &[T]
    where
        [T]: StmtAttr<'desc, 'buf, S, A, V>,
        Self: AttrSet<A>,
    {
    }
    impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, CH: OdbcChar, V: OdbcVersion>
        BaseStmtAttr<'desc, 'buf, S, A, V> for &OdbcStr<CH>
    where
        OdbcStr<CH>: BaseStmtAttr<'desc, 'buf, S, A, V>,
        Self: AttrSet<A>,
    {
    }
}

pub trait StmtAttr<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, V: OdbcVersion>:
    private::BaseStmtAttr<'desc, 'buf, S, A, V>
{
}

// Implement StmtAttr for all versions of SQLHSTMT statement attributes
impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80> for T
where
    T: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4> for T
where
    T: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80>
    for [T]
where
    [T]: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4> for [T]
where
    [T]: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, CH: OdbcChar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80>
    for OdbcStr<CH>
where
    OdbcStr<CH>: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, CH: OdbcChar>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4>
    for OdbcStr<CH>
where
    OdbcStr<CH>: StmtAttr<
            'desc,
            'buf,
            SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

// Implement StmtAttr for all versions of UnsafeSQLHSTMT statement attributes
impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80>
    for T
where
    T: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4> for T
where
    T: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80>
    for [T]
where
    [T]: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, T: Scalar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4>
    for [T]
where
    [T]: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, CH: OdbcChar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, A, SQL_OV_ODBC3_80>
    for OdbcStr<CH>
where
    OdbcStr<CH>: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion,
        >,
{
}

impl<'conn, 'desc, 'buf, A: Ident, CH: OdbcChar>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, A, SQL_OV_ODBC4>
    for OdbcStr<CH>
where
    OdbcStr<CH>: StmtAttr<
            'desc,
            'buf,
            UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>,
            A,
            <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion,
        >,
{
}

// Implement StmtAttr for uninitialized statement attributes
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for MaybeUninit<T>
where
    T: StmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
    Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for [MaybeUninit<T>]
where
    [T]: StmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
    Self: AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for OdbcStr<MaybeUninit<SQLCHAR>>
where
    OdbcStr<SQLCHAR>: StmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for OdbcStr<MaybeUninit<SQLWCHAR>>
where
    OdbcStr<SQLWCHAR>: StmtAttr<'desc, 'buf, S, A, V> + AttrGet<A>,
{
}

// Implement StmtAttr for references to unsized (used by AttrSet)
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, T: Scalar, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for &[T]
where
    [T]: StmtAttr<'desc, 'buf, S, A, V>,
    Self: AttrSet<A>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, A: Ident, CH: OdbcChar, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, A, V> for &OdbcStr<CH>
where
    OdbcStr<CH>: StmtAttr<'desc, 'buf, S, A, V>,
    Self: AttrSet<A>,
{
}

// Implement methods for setting and getting descriptor handles
unsafe impl<'conn, 'desc, 'buf, A: Ident, V: OdbcVersion> Attr<A>
    for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>: Attr<A>,
{
    type DefinedBy = <Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, A: Ident, DT, V: OdbcVersion> Attr<A> for MaybeUninit<RefSQLHDESC<'conn, DT, V>>
where
    MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>>: Attr<A>,
{
    type DefinedBy = <MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, 'desc, A: Ident, DT, V: OdbcVersion> Attr<A>
    for MaybeUninit<&'desc SQLHDESC<'conn, DT, V>>
where
    MaybeUninit<&'desc UnsafeSQLHDESC<'conn, DT, V>>: Attr<A>,
{
    type DefinedBy = <MaybeUninit<&'desc UnsafeSQLHDESC<'conn, DT, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, DT, A: Ident, V: OdbcVersion> AttrGet<A>
    for MaybeUninit<RefSQLHDESC<'conn, DT, V>>
where
    MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>>: AttrGet<A>,
{
}
unsafe impl<'conn, 'desc, 'buf, A: Ident, V: OdbcVersion> AttrSet<A>
    for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>: AttrSet<A>,
{
}

impl<T: Scalar> Ref<'_> for T {}
impl<T: Scalar> Ref<'_> for [T] {}

impl<CH> Ref<'_> for OdbcStr<CH> {}

impl<T: Scalar> Ref<'_> for MaybeUninit<T> {}
impl<T: Scalar> Ref<'_> for [MaybeUninit<T>] {}

impl<'stmt, DT, V: OdbcVersion> Ref<'stmt> for MaybeUninit<RefSQLHDESC<'stmt, DT, V>> {}
impl<'stmt, DT, V: OdbcVersion> Ref<'stmt> for MaybeUninit<RefUnsafeSQLHDESC<'stmt, DT, V>> {}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 0)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
unsafe impl Attr<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_QUERY_TIMEOUT, SQL_OV_ODBC3> for SQLULEN
{
}
unsafe impl AttrGet<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 1)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl Attr<SQL_ATTR_MAX_ROWS> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_MAX_ROWS, SQL_OV_ODBC3> for SQLULEN
{
}
unsafe impl AttrGet<SQL_ATTR_MAX_ROWS> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_MAX_ROWS> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 2)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl Attr<SQL_ATTR_NOSCAN> for Noscan {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_NOSCAN, SQL_OV_ODBC3> for Noscan
{
}
unsafe impl AttrGet<SQL_ATTR_NOSCAN> for Noscan {}
unsafe impl AttrSet<SQL_ATTR_NOSCAN> for Noscan {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 3)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
unsafe impl Attr<SQL_ATTR_MAX_LENGTH> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_MAX_LENGTH, SQL_OV_ODBC3> for SQLULEN
{
}
unsafe impl AttrGet<SQL_ATTR_MAX_LENGTH> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_MAX_LENGTH> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 6)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
// TODO: This attribute cannot be specified after the SQL statement has been prepared.
unsafe impl Attr<SQL_ATTR_CURSOR_TYPE> for CursorType {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_CURSOR_TYPE, SQL_OV_ODBC3> for CursorType
{
}
unsafe impl AttrGet<SQL_ATTR_CURSOR_TYPE> for CursorType {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_TYPE> for CursorType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 7)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
// TODO: This attribute cannot be specified for an open cursor
unsafe impl Attr<SQL_ATTR_CONCURRENCY> for Concurrency {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_CONCURRENCY, SQL_OV_ODBC3> for Concurrency
{
}
unsafe impl AttrGet<SQL_ATTR_CONCURRENCY> for Concurrency {}
unsafe impl AttrSet<SQL_ATTR_CONCURRENCY> for Concurrency {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 8)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
unsafe impl Attr<SQL_ATTR_KEYSET_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_KEYSET_SIZE, SQL_OV_ODBC3> for SQLULEN
{
}
unsafe impl AttrGet<SQL_ATTR_KEYSET_SIZE> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_KEYSET_SIZE> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl Attr<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_SIMULATE_CURSOR, SQL_OV_ODBC3> for SimulateCursor
{
}
unsafe impl AttrGet<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {}
unsafe impl AttrSet<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 11)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl Attr<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_RETRIEVE_DATA, SQL_OV_ODBC3> for RetrieveData
{
}
unsafe impl AttrGet<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {}
unsafe impl AttrSet<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 12)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl Attr<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_USE_BOOKMARKS, SQL_OV_ODBC3> for UseBookmarks
{
}
unsafe impl AttrGet<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {}
unsafe impl AttrSet<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 15)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_ENABLE_AUTO_IPD;
unsafe impl Attr<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_ENABLE_AUTO_IPD, SQL_OV_ODBC3> for OdbcBool
{
}
unsafe impl AttrGet<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {}
unsafe impl AttrSet<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 14)]
#[expect(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_ROW_NUMBER;
unsafe impl Attr<SQL_ATTR_ROW_NUMBER> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_ROW_NUMBER, SQL_OV_ODBC3> for SQLULEN
{
}
unsafe impl AttrGet<SQL_ATTR_ROW_NUMBER> for SQLULEN {}

// TODO:
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 16)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;

//// The following are Header fields--------------------------------
//
//// TODO: This one could be special??
//// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 5)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
//// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 17)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
//// Corresponds to APD SQL_DESC_BIND_TYPE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 18)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
//// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 18)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
//// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 20)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
//// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 21)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
//// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 22)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//

// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
//// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 23)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 24)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
//// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 25)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
//// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 26)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
//// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 27)]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[identifier(SQLINTEGER, 29)]
//#[derive(Ident)]
//#[cfg(feature = "v3_8")]
//#[expect(non_camel_case_types)]
// TODO: This type MUST be Rc or similar
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[identifier(SQLINTEGER, 30)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[identifier(SQLINTEGER, 31)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[identifier(SQLINTEGER, 32)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[identifier(SQLINTEGER, 33)]
//#[derive(Ident)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;

#[derive(Ident)]
#[identifier(SQLINTEGER, 10010)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_APP_ROW_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_ROW_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
    type DefinedBy = OdbcDefined;
}
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_ROW_DESC>
    for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<
        'desc,
        'buf,
        UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        SQL_ATTR_APP_ROW_DESC,
        V,
    > for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_ROW_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, S, SQL_ATTR_APP_ROW_DESC, V>
    for Option<&'desc S::ExplicitARD>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &UnsafeSQLHSTMT<'_, 'desc, 'buf, V>) {
        StatementHandle.explicit_ard.set(*self);
    }
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_ROW_DESC, V>
    for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_ROW_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_APP_ROW_DESC, V> for Option<&'desc S::ExplicitARD>
where
    Self: Attr<SQL_ATTR_APP_ROW_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_APP_ROW_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
}
unsafe impl<V: OdbcVersion> AttrSet<SQL_ATTR_APP_ROW_DESC>
    for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10011)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_PARAM_DESC>
    for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
    type DefinedBy = OdbcDefined;
}
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_PARAM_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<
        'desc,
        'buf,
        UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        SQL_ATTR_APP_PARAM_DESC,
        V,
    > for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_PARAM_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, AppDesc<'buf>, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, S, SQL_ATTR_APP_PARAM_DESC, V>
    for Option<&'desc S::ExplicitAPD>
where
    Self: Attr<SQL_ATTR_APP_PARAM_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &UnsafeSQLHSTMT<'_, 'desc, 'buf, V>) {
        StatementHandle.explicit_ard.set(*self);
    }
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_PARAM_DESC, V>
    for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_PARAM_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_APP_PARAM_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, AppDesc<'buf>, V>>
where
    Self: Attr<SQL_ATTR_APP_PARAM_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, V>, V: OdbcVersion>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_APP_PARAM_DESC, V> for Option<&'desc S::ExplicitAPD>
where
    Self: Attr<SQL_ATTR_APP_PARAM_DESC> + AttrLen<Self::DefinedBy, SQLINTEGER>,
{
}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_APP_PARAM_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
}
unsafe impl<V: OdbcVersion> AttrSet<SQL_ATTR_APP_PARAM_DESC>
    for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>>
{
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10012)]
#[expect(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_IMP_ROW_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, IRD, V>>
{
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<
        'desc,
        'buf,
        UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        SQL_ATTR_IMP_ROW_DESC,
        V,
    > for MaybeUninit<RefUnsafeSQLHDESC<'conn, IRD, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_ROW_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, IRD, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_ROW_DESC, V>
    for MaybeUninit<RefUnsafeSQLHDESC<'conn, IRD, V>>
{
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_ROW_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, IRD, V>>
{
}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_IMP_ROW_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, IRD, V>>
{
}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10013)]
#[expect(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_IMP_PARAM_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, IPD, V>>
{
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<
        'desc,
        'buf,
        UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        SQL_ATTR_IMP_PARAM_DESC,
        V,
    > for MaybeUninit<RefUnsafeSQLHDESC<'conn, IPD, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    private::BaseStmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_PARAM_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, IPD, V>>
{
    #[cfg(feature = "odbc_debug")]
    fn readA<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW<'stmt>(
        &mut self,
        StatementHandle: &'stmt SQLHSTMT<'conn, 'desc, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_PARAM_DESC, V>
    for MaybeUninit<RefUnsafeSQLHDESC<'conn, IPD, V>>
{
}
impl<'conn, 'desc, 'buf, V: OdbcVersion>
    StmtAttr<'desc, 'buf, SQLHSTMT<'conn, 'desc, 'buf, V>, SQL_ATTR_IMP_PARAM_DESC, V>
    for MaybeUninit<RefSQLHDESC<'conn, IPD, V>>
{
}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_IMP_PARAM_DESC>
    for MaybeUninit<RefUnsafeSQLHDESC<'_, IPD, V>>
{
}

#[derive(Ident)]
#[identifier(SQLINTEGER, -1)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_SCROLLABLE;
unsafe impl Attr<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_CURSOR_SCROLLABLE, SQL_OV_ODBC3> for CursorScrollable
{
}
unsafe impl AttrGet<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {}

#[derive(Ident)]
#[identifier(SQLINTEGER, -2)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_SENSITIVITY;
unsafe impl Attr<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_CURSOR_SENSITIVITY, SQL_OV_ODBC3> for CursorSensitivity
{
}
unsafe impl AttrGet<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10014)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
unsafe impl Attr<SQL_ATTR_METADATA_ID> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_METADATA_ID, SQL_OV_ODBC3> for OdbcBool
{
}
unsafe impl AttrGet<SQL_ATTR_METADATA_ID> for OdbcBool {}
unsafe impl AttrSet<SQL_ATTR_METADATA_ID> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 4)]
#[expect(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
// TODO: For drivers with statement level asynchronous execution support,
// the statement attribute SQL_ATTR_ASYNC_ENABLE is read only
// This attribute is reexported in conn.rs
unsafe impl Attr<SQL_ATTR_ASYNC_ENABLE> for AsyncEnable {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>>
    StmtAttr<'desc, 'buf, S, SQL_ATTR_ASYNC_ENABLE, SQL_OV_ODBC3> for AsyncEnable
{
}
unsafe impl AttrGet<SQL_ATTR_ASYNC_ENABLE> for AsyncEnable {}
unsafe impl AttrSet<SQL_ATTR_ASYNC_ENABLE> for AsyncEnable {}

// TODO: Not found in implementation
// #[cfg(feature = "v3_8")]
// SQL_ATTR_ASYNC_STMT_PCALLBACK
// #[cfg(feature = "v3_8")]
// SQL_ATTR_ASYNC_STMT_PCONTEXT

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

#[odbc_type(SQLULEN)]
pub struct AsyncEnable;
pub const SQL_ASYNC_ENABLE_OFF: AsyncEnable = AsyncEnable(0);
pub const SQL_ASYNC_ENABLE_ON: AsyncEnable = AsyncEnable(1);

#[odbc_type(SQLULEN)]
pub struct CursorScrollable;
pub const SQL_NONSCROLLABLE: CursorScrollable = CursorScrollable(0);
pub const SQL_SCROLLABLE: CursorScrollable = CursorScrollable(1);

#[odbc_type(SQLULEN)]
pub struct CursorSensitivity;
pub const SQL_UNSPECIFIED: CursorSensitivity = CursorSensitivity(0);
pub const SQL_INSENSITIVE: CursorSensitivity = CursorSensitivity(1);
pub const SQL_SENSITIVE: CursorSensitivity = CursorSensitivity(2);
