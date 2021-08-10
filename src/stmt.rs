#[double]
use crate::api::ffi;
use crate::api::Statement;
use crate::attr::{Attr, AttrGet, AttrLen, AttrSet};
use crate::convert::AsMutPtr;
use crate::desc::{AppDesc, ImplDesc, IRD, IPD};
use crate::env::{OdbcVersion, SQL_OV_ODBC3, SQL_OV_ODBC3_80, SQL_OV_ODBC4};
use crate::handle::{UnsafeSQLHDESC, SQLHDESC};
use crate::handle::{UnsafeSQLHSTMT, SQLHSTMT, RefSQLHDESC, RefUnsafeSQLHDESC};
use crate::str::{Ansi, OdbcChar, OdbcStr, Unicode};
use crate::{
    sqlreturn::SQLRETURN, Ident, OdbcBool, OdbcDefined, SQLCHAR, SQLINTEGER, SQLULEN,
    SQLWCHAR,
};
use mockall_double::double;
use rs_odbc_derive::{odbc_type, Ident};
use std::mem::MaybeUninit;

pub(crate) mod private {
    use super::*;

    pub trait BaseStmtAttr<'stmt, 'desc, 'buf, A: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion>:
        Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER>
    {
        fn update_handle(&self, _: &S)
        where
            Self: AttrSet<A>,
        {
        }

        #[allow(non_snake_case)]
        fn readA(
            &mut self,
            StatementHandle: &'stmt S,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Ansi,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        {
            let ValuePtrLen = self.len();

            unsafe {
                ffi::SQLGetStmtAttrA(
                    StatementHandle.as_SQLHANDLE(),
                    A::IDENTIFIER,
                    self.as_mut_SQLPOINTER(),
                    ValuePtrLen,
                    StringLengthPtr.map_or_else(std::ptr::null_mut, AsMutPtr::as_mut_ptr),
                )
            }
        }

        #[allow(non_snake_case)]
        fn readW(
            &mut self,
            StatementHandle: &'stmt S,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Unicode,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        {
            let ValuePtrLen = self.len();

            unsafe {
                ffi::SQLGetStmtAttrW(
                    StatementHandle.as_SQLHANDLE(),
                    A::IDENTIFIER,
                    self.as_mut_SQLPOINTER(),
                    ValuePtrLen,
                    StringLengthPtr.map_or_else(std::ptr::null_mut, AsMutPtr::as_mut_ptr),
                )
            }
        }
    }

    impl<'desc, 'buf, A: Ident, T: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion> BaseStmtAttr<'_, 'desc, 'buf, A, S, V> for T where Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {}
    impl<'desc, 'buf, A: Ident, CH: OdbcChar, S: Statement<'desc, 'buf, V>, V: OdbcVersion> BaseStmtAttr<'_, 'desc, 'buf, A, S, V> for OdbcStr<CH> where Self: Attr<A> {}

    // Implement BaseStmtAttr for uninitialized statement attributes
    impl<'stmt, 'desc, 'buf, A: Ident, T: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V> for MaybeUninit<T> where T: BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V>, Self: AttrLen<Self::DefinedBy, SQLINTEGER> {}
    impl<'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar, S: Statement<'desc, 'buf, V>, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V> for OdbcStr<MaybeUninit<CH>> where OdbcStr<CH>: BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V>, Self: Attr<A> {}

    impl<'conn, 'stmt, 'desc, 'buf, A: Ident, DT, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, V>, V> for Option<&'desc SQLHDESC<'conn, DT, V>>
    where Option<&'desc UnsafeSQLHDESC<'conn, DT, V>>: BaseStmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, V> + AttrSet<A>, Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {
        // TODO: Can I use here descriptor and statement defined on different connections??? This
        // should not be allowed If this is true, then I need to use unelided lifetime 'conn to
        // tie the lifetimes. Will that solve the problem?
        fn update_handle(&self, StatementHandle: &SQLHSTMT<'conn, 'desc, 'buf, V>)
        where
            Self: AttrSet<A>,
        {
            unsafe {std::mem::transmute::<_, Option<&'desc UnsafeSQLHDESC<'conn, DT, V>>>(self)}.update_handle(&StatementHandle.0)
        }
    }

    impl<'conn, 'stmt, 'desc, 'buf, A: Ident, DT, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, V>, V> for MaybeUninit<&'desc SQLHDESC<'stmt, DT, V>>
    where MaybeUninit<&'desc UnsafeSQLHDESC<'stmt, DT, V>>: BaseStmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, V> + AttrGet<A>, Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {
        // TODO: feature not required here
        #[cfg(feature = "odbc_debug")]
        fn readA(
            &mut self,
            StatementHandle: &'desc SQLHSTMT<'conn, 'desc, 'buf, V>,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Ansi,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>
        {
           unsafe {std::mem::transmute::<_, MaybeUninit<UnsafeSQLHDESC<'stmt, DT, V>>>(self)}.readA(&StatementHandle.0, StringLengthPtr)
        }

        // TODO: feature not required here
        #[cfg(feature = "odbc_debug")]
        fn readW(
            &mut self,
            StatementHandle: &'desc SQLHSTMT<'conn, 'desc, 'buf, V>,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Unicode,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        {
            unsafe {std::mem::transmute::<_, MaybeUninit<UnsafeSQLHDESC<'stmt, DT, V>>>(self)}.readW(&StatementHandle.0, StringLengthPtr)
        }
    }

    impl<'conn, 'stmt, 'desc, 'buf, A: Ident, DT, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, V>, V> for MaybeUninit<RefSQLHDESC<'stmt, DT, V>>
    where MaybeUninit<RefUnsafeSQLHDESC<'stmt, DT, V>>: BaseStmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, V> + AttrGet<A>, Self: Attr<A> + AttrLen<Self::DefinedBy, SQLINTEGER> {
        // TODO: feature not required here
        #[cfg(feature = "odbc_debug")]
        fn readA(
            &mut self,
            StatementHandle: &SQLHSTMT<'conn, 'desc, 'buf, V>,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Ansi,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>
        {
           unsafe {std::mem::transmute::<_, MaybeUninit<RefUnsafeSQLHDESC<'stmt, DT, V>>>(self)}.readA(&StatementHandle.0, StringLengthPtr)
        }

        // TODO: feature not required here
        #[cfg(feature = "odbc_debug")]
        fn readW(
            &mut self,
            StatementHandle: &SQLHSTMT<'conn, 'desc, 'buf, V>,
            StringLengthPtr: Option<&mut MaybeUninit<Self::StrLen>>,
        ) -> SQLRETURN
        where
            A: Ident<Type = SQLINTEGER>,
            Self: AttrGet<A> + Unicode,
            MaybeUninit<Self::StrLen>: AsMutPtr<SQLINTEGER>,
        {
            unsafe {std::mem::transmute::<_, MaybeUninit<RefUnsafeSQLHDESC<'stmt, DT, V>>>(self)}.readW(&StatementHandle.0, StringLengthPtr)
        }
    }

    // Implement BaseStmtAttr for references to character statement attributes (used by AttrSet)
    impl<'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar, S: Statement<'desc, 'buf, V>, V: OdbcVersion> BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V> for &OdbcStr<CH>
    where OdbcStr<CH>: BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V>, Self: AttrSet<A> {}
}

pub trait StmtAttr<'stmt, 'desc, 'buf, A: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion>: private::BaseStmtAttr<'stmt, 'desc, 'buf, A, S, V> {}

//#[cfg(feature = "odbc_debug")]
//fn get_ard<'desc, 'buf, V: OdbcVersion>(
//    desc: &mut MaybeUninit<RefSQLHDESC<'desc, AppDesc<'buf>, V>>,
//    StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
//) -> SQLRETURN {
//    if let Some(explicit_ard) = StatementHandle.explicit_ard.get() {
//        *desc = MaybeUninit::new(RefSQLHDESC(explicit_ard));
//    } else {
//        *desc = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ard));
//    }
//
//    SQL_SUCCESS
//}
//
//#[cfg(feature = "odbc_debug")]
//fn get_apd<'desc, 'buf, V: OdbcVersion>(
//    desc: &mut MaybeUninit<RefSQLHDESC<'desc, AppDesc<'buf>, V>>,
//    StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
//) -> SQLRETURN {
//    if let Some(explicit_apd) = StatementHandle.explicit_apd.get() {
//        *desc = MaybeUninit::new(RefSQLHDESC(explicit_apd));
//    } else {
//        *desc = MaybeUninit::new(RefSQLHDESC(&StatementHandle.apd));
//    }
//
//    SQL_SUCCESS
//}
//
//#[cfg(feature = "odbc_debug")]
//fn get_ird<'desc, V: OdbcVersion>(
//    desc: &mut MaybeUninit<RefSQLHDESC<'desc, ImplDesc<IRD>, V>>,
//    StatementHandle: &'desc UnsafeSQLHSTMT<V>,
//) -> SQLRETURN {
//    *desc = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ird));
//    SQL_SUCCESS
//}
//
//#[cfg(feature = "odbc_debug")]
//fn get_ipd<'desc, V: OdbcVersion>(
//    desc: &mut MaybeUninit<RefSQLHDESC<'desc, ImplDesc<IPD>, V>>,
//    StatementHandle: &'desc UnsafeSQLHSTMT<V>,
//) -> SQLRETURN {
//    *desc = MaybeUninit::new(RefSQLHDESC(&StatementHandle.ipd));
//    SQL_SUCCESS
//}

// Implement StmtAttr for all versions of SQLHSTMT statement attributes
impl<'conn, 'stmt, 'desc, 'buf, A: Ident, T: Ident> StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, SQL_OV_ODBC3_80> for T
where T: StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, T: Ident> StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, SQL_OV_ODBC4> for T
where T: StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar> StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, SQL_OV_ODBC3_80> for OdbcStr<CH>
where OdbcStr<CH>: StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar> StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, SQL_OV_ODBC4> for OdbcStr<CH>
where OdbcStr<CH>: StmtAttr<'stmt, 'desc, 'buf, A, SQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion> {}

// Implement StmtAttr for all versions of UnsafeSQLHSTMT statement attributes
impl<'conn, 'stmt, 'desc, 'buf, A: Ident, T: Ident> StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, SQL_OV_ODBC3_80> for T
where T: StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, T: Ident> StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, SQL_OV_ODBC4> for T
where T: StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar> StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC3_80>, SQL_OV_ODBC3_80> for OdbcStr<CH>
where OdbcStr<CH>: StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC3_80 as OdbcVersion>::PrevVersion> {}

impl<'conn, 'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar> StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, SQL_OV_ODBC4>, SQL_OV_ODBC4> for OdbcStr<CH>
where OdbcStr<CH>: StmtAttr<'stmt, 'desc, 'buf, A, UnsafeSQLHSTMT<'conn, 'desc, 'buf, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion>, <SQL_OV_ODBC4 as OdbcVersion>::PrevVersion> {}

// Implement StmtAttr for uninitialized statement attributes
impl<'stmt, 'desc, 'buf, A: Ident, T: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, A, S, V> for MaybeUninit<T> where T: StmtAttr<'stmt, 'desc, 'buf, A, S, V> + AttrGet<A>, Self: AttrLen<Self::DefinedBy, SQLINTEGER> {}
impl<'stmt, 'desc, 'buf, A: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, A, S, V> for OdbcStr<MaybeUninit<SQLCHAR>> where OdbcStr<SQLCHAR>: StmtAttr<'stmt, 'desc, 'buf, A, S, V> {}
impl<'stmt, 'desc, 'buf, A: Ident, S: Statement<'desc, 'buf, V>, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, A, S, V> for OdbcStr<MaybeUninit<SQLWCHAR>> where OdbcStr<SQLWCHAR>: StmtAttr<'stmt, 'desc, 'buf, A, S, V> { }

// Implement StmtAttr for references to character statement attributes (used by AttrSet)
impl<'stmt, 'desc, 'buf, A: Ident, CH: OdbcChar, S: Statement<'desc, 'buf, V>, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, A, S, V> for &OdbcStr<CH>
where OdbcStr<CH>: StmtAttr<'stmt, 'desc, 'buf, A, S, V>, Self: AttrSet<A> {}

// Implement methods for setting and getting descriptor handles
unsafe impl<'conn, 'desc, 'buf, A: Ident, V: OdbcVersion> Attr<A> for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>>
where Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>: Attr<A> {
    type DefinedBy = <Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, 'buf, A: Ident, DT, V: OdbcVersion> Attr<A> for MaybeUninit<RefSQLHDESC<'conn, DT, V>>
where MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>>: Attr<A> {
    type DefinedBy = <MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, 'desc, 'buf, A: Ident, DT, V: OdbcVersion> Attr<A> for MaybeUninit<&'desc SQLHDESC<'conn, DT, V>>
where MaybeUninit<&'desc UnsafeSQLHDESC<'conn, DT, V>>: Attr<A> {
    type DefinedBy = <MaybeUninit<&'desc UnsafeSQLHDESC<'conn, DT, V>> as Attr<A>>::DefinedBy;
}
unsafe impl<'conn, 'buf, A: Ident, DT, V: OdbcVersion> AttrGet<A> for MaybeUninit<RefSQLHDESC<'conn, DT, V>> where MaybeUninit<RefUnsafeSQLHDESC<'conn, DT, V>>: AttrGet<A> {}
unsafe impl<'desc, 'conn, 'buf, A: Ident, V: OdbcVersion> AttrSet<A> for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>> where Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>>: AttrSet<A> {}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

#[derive(Ident)]
#[identifier(SQLINTEGER, 0)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_QUERY_TIMEOUT;
unsafe impl Attr<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_QUERY_TIMEOUT, S, SQL_OV_ODBC3> for SQLULEN {}
unsafe impl AttrGet<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_QUERY_TIMEOUT> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_ROWS;
unsafe impl Attr<SQL_ATTR_MAX_ROWS> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_MAX_ROWS, S, SQL_OV_ODBC3> for SQLULEN {}
unsafe impl AttrGet<SQL_ATTR_MAX_ROWS> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_MAX_ROWS> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_NOSCAN;
unsafe impl Attr<SQL_ATTR_NOSCAN> for Noscan {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_NOSCAN, S, SQL_OV_ODBC3> for Noscan {}
unsafe impl AttrGet<SQL_ATTR_NOSCAN> for Noscan {}
unsafe impl AttrSet<SQL_ATTR_NOSCAN> for Noscan {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_MAX_LENGTH;
unsafe impl Attr<SQL_ATTR_MAX_LENGTH> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_MAX_LENGTH, S, SQL_OV_ODBC3> for SQLULEN {}
unsafe impl AttrGet<SQL_ATTR_MAX_LENGTH> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_MAX_LENGTH> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_TYPE;
// TODO: This attribute cannot be specified after the SQL statement has been prepared.
unsafe impl Attr<SQL_ATTR_CURSOR_TYPE> for CursorType {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_CURSOR_TYPE, S, SQL_OV_ODBC3> for CursorType {}
unsafe impl AttrGet<SQL_ATTR_CURSOR_TYPE> for CursorType {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_TYPE> for CursorType {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CONCURRENCY;
// TODO: This attribute cannot be specified for an open cursor
unsafe impl Attr<SQL_ATTR_CONCURRENCY> for Concurrency {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_CONCURRENCY, S, SQL_OV_ODBC3> for Concurrency {}
unsafe impl AttrGet<SQL_ATTR_CONCURRENCY> for Concurrency {}
unsafe impl AttrSet<SQL_ATTR_CONCURRENCY> for Concurrency {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_KEYSET_SIZE;
unsafe impl Attr<SQL_ATTR_KEYSET_SIZE> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_KEYSET_SIZE, S, SQL_OV_ODBC3> for SQLULEN {}
unsafe impl AttrGet<SQL_ATTR_KEYSET_SIZE> for SQLULEN {}
unsafe impl AttrSet<SQL_ATTR_KEYSET_SIZE> for SQLULEN {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_SIMULATE_CURSOR;
unsafe impl Attr<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_SIMULATE_CURSOR, S, SQL_OV_ODBC3> for SimulateCursor {}
unsafe impl AttrGet<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {}
unsafe impl AttrSet<SQL_ATTR_SIMULATE_CURSOR> for SimulateCursor {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_RETRIEVE_DATA;
unsafe impl Attr<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_RETRIEVE_DATA, S, SQL_OV_ODBC3> for RetrieveData {}
unsafe impl AttrGet<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {}
unsafe impl AttrSet<SQL_ATTR_RETRIEVE_DATA> for RetrieveData {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_USE_BOOKMARKS;
unsafe impl Attr<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_USE_BOOKMARKS, S, SQL_OV_ODBC3> for UseBookmarks {}
unsafe impl AttrGet<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {}
unsafe impl AttrSet<SQL_ATTR_USE_BOOKMARKS> for UseBookmarks {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 15)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ENABLE_AUTO_IPD;
unsafe impl Attr<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_ENABLE_AUTO_IPD, S, SQL_OV_ODBC3> for OdbcBool {}
unsafe impl AttrGet<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {}
unsafe impl AttrSet<SQL_ATTR_ENABLE_AUTO_IPD> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 14)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_ROW_NUMBER;
unsafe impl Attr<SQL_ATTR_ROW_NUMBER> for SQLULEN {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_ROW_NUMBER, S, SQL_OV_ODBC3> for SQLULEN {}
unsafe impl AttrGet<SQL_ATTR_ROW_NUMBER> for SQLULEN {}

// TODO:
//#[derive(Ident)]
//#[identifier(SQLINTEGER, 16)]
//#[allow(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;

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

#[derive(Ident)]
#[identifier(SQLINTEGER, 10010)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_ROW_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_ROW_DESC> for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>> {
    type DefinedBy = OdbcDefined;
}
unsafe impl<'conn, 'buf, V: OdbcVersion> Attr<SQL_ATTR_APP_ROW_DESC> for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {
    type DefinedBy = <Option<&'conn UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> as Attr<SQL_ATTR_APP_ROW_DESC>>::DefinedBy;
}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<RefUnsafeSQLHDESC<'stmt, AppDesc<'buf>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &UnsafeSQLHSTMT<'_, 'desc, 'buf, V>) {
        StatementHandle.explicit_ard.set(*self);
    }
}
impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, V> for MaybeUninit<RefUnsafeSQLHDESC<'stmt, AppDesc<'buf>, V>> {}
impl<'conn, 'desc, 'buf, V: OdbcVersion> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, UnsafeSQLHSTMT<'conn, 'desc, 'buf, V>, V> for Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'buf>, V>> {}
impl<'conn, 'desc, 'buf, V: OdbcVersion> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_ROW_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>> {}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_APP_ROW_DESC> for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>> {}
unsafe impl<V: OdbcVersion> AttrSet<SQL_ATTR_APP_ROW_DESC> for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10011)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_APP_PARAM_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_APP_PARAM_DESC> for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>> {
    type DefinedBy = OdbcDefined;
}
unsafe impl<'conn, 'buf, V: OdbcVersion> Attr<SQL_ATTR_APP_PARAM_DESC> for MaybeUninit<RefUnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {
    type DefinedBy = <Option<&'conn UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> as Attr<SQL_ATTR_APP_PARAM_DESC>>::DefinedBy;
}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<RefUnsafeSQLHDESC<'stmt, AppDesc<'buf>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn update_handle(&self, StatementHandle: &UnsafeSQLHSTMT<'_, 'desc, 'buf, V>) {
        StatementHandle.explicit_ard.set(*self);
    }
}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<RefUnsafeSQLHDESC<'stmt, AppDesc<'buf>, V>> {}
impl<'conn, 'desc, 'buf, V: OdbcVersion> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for Option<&'desc UnsafeSQLHDESC<'conn, AppDesc<'buf>, V>> {}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<RefSQLHDESC<'stmt, AppDesc<'buf>, V>> {}
impl<'conn, 'desc, 'buf, V: OdbcVersion> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_APP_PARAM_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for Option<&'desc SQLHDESC<'conn, AppDesc<'buf>, V>> {}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_APP_PARAM_DESC> for MaybeUninit<RefUnsafeSQLHDESC<'_, AppDesc<'_>, V>> {}
unsafe impl<V: OdbcVersion> AttrSet<SQL_ATTR_APP_PARAM_DESC> for Option<&UnsafeSQLHDESC<'_, AppDesc<'_>, V>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10012)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_ROW_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<&UnsafeSQLHDESC<'_, ImplDesc<IRD>, V>> {
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_ROW_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc UnsafeSQLHDESC<'stmt, ImplDesc<IRD>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_ROW_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc UnsafeSQLHDESC<'stmt, ImplDesc<IRD>, V>> {}
impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_ROW_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc SQLHDESC<'stmt, ImplDesc<IRD>, V>> {}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_IMP_ROW_DESC> for MaybeUninit<&UnsafeSQLHDESC<'_, ImplDesc<IRD>, V>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10013)]
#[allow(non_camel_case_types)]
// This is read-only attribute
pub struct SQL_ATTR_IMP_PARAM_DESC;
unsafe impl<V: OdbcVersion> Attr<SQL_ATTR_IMP_PARAM_DESC> for MaybeUninit<&UnsafeSQLHDESC<'_, ImplDesc<IPD>, V>> {
    type DefinedBy = OdbcDefined;
}

impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> private::BaseStmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc UnsafeSQLHDESC<'stmt, ImplDesc<IPD>, V>> {
    #[cfg(feature = "odbc_debug")]
    fn readA(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }

    #[cfg(feature = "odbc_debug")]
    fn readW(
        &mut self,
        StatementHandle: &'desc UnsafeSQLHSTMT<'_, '_, 'buf, V>,
        _: Option<&mut MaybeUninit<Self::StrLen>>,
    ) -> SQLRETURN {
        get_ard(self, StatementHandle)
    }
}
impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_PARAM_DESC, UnsafeSQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc UnsafeSQLHDESC<'stmt, ImplDesc<IPD>, V>> {}
impl<'conn, 'stmt, 'desc, 'buf, V: OdbcVersion> StmtAttr<'stmt, 'desc, 'buf, SQL_ATTR_IMP_PARAM_DESC, SQLHSTMT<'conn ,'desc, 'buf, V>, V> for MaybeUninit<&'desc SQLHDESC<'stmt, ImplDesc<IPD>, V>> {}

unsafe impl<V: OdbcVersion> AttrGet<SQL_ATTR_IMP_PARAM_DESC> for MaybeUninit<&UnsafeSQLHDESC<'_, ImplDesc<IPD>, V>> {}

#[derive(Ident)]
#[identifier(SQLINTEGER, -1)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_SCROLLABLE;
unsafe impl Attr<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_CURSOR_SCROLLABLE, S, SQL_OV_ODBC3> for CursorScrollable {}
unsafe impl AttrGet<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_SCROLLABLE> for CursorScrollable {}

#[derive(Ident)]
#[identifier(SQLINTEGER, -2)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_CURSOR_SENSITIVITY;
unsafe impl Attr<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_CURSOR_SENSITIVITY, S, SQL_OV_ODBC3> for CursorSensitivity {}
unsafe impl AttrGet<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {}
unsafe impl AttrSet<SQL_ATTR_CURSOR_SENSITIVITY> for CursorSensitivity {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 10014)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_METADATA_ID;
unsafe impl Attr<SQL_ATTR_METADATA_ID> for OdbcBool {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_METADATA_ID, S, SQL_OV_ODBC3> for OdbcBool {}
unsafe impl AttrGet<SQL_ATTR_METADATA_ID> for OdbcBool {}
unsafe impl AttrSet<SQL_ATTR_METADATA_ID> for OdbcBool {}

#[derive(Ident)]
#[identifier(SQLINTEGER, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_ATTR_ASYNC_ENABLE;
// TODO: For drivers with statement level asynchronous execution support,
// the statement attribute SQL_ATTR_ASYNC_ENABLE is read only
unsafe impl Attr<SQL_ATTR_ASYNC_ENABLE> for AsyncEnable {
    type DefinedBy = OdbcDefined;
}
impl<'desc, 'buf, S: Statement<'desc, 'buf, SQL_OV_ODBC3>> StmtAttr<'_, 'desc, 'buf, SQL_ATTR_ASYNC_ENABLE, S, SQL_OV_ODBC3> for AsyncEnable {}
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
