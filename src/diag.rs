use crate::api::Handle;
use crate::attr::{Attr, AttrGet, AttrLen, AttrZeroAssert};
use crate::convert::AsMutSQLPOINTER;
use crate::env::OdbcVersion;
use crate::handle::SQLHSTMT;
use crate::str::{OdbcChar, OdbcStr};
use crate::{
    sqlreturn::SQLRETURN, Def, Ident, OdbcDefined, Scalar, Void, SQLCHAR, SQLINTEGER, SQLLEN,
    SQLPOINTER, SQLSMALLINT, SQLWCHAR,
};
use core::mem::MaybeUninit;
use rs_odbc_derive::{odbc_type, Ident};

pub trait DiagField<H: Handle, D: Ident>: Attr<D> + AttrLen<Self::DefinedBy, SQLSMALLINT> {
    // TODO: These could be checked by the type system
    // SQL_DIAG_CURSOR_ROW_COUNT -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // SQL_DIAG_DYNAMIC_FUNCTION -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // SQL_DIAG_DYNAMIC_FUNCTION_CODE -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // SQL_DIAG_ROW_COUNT -> SQLExecute, SQLExecDirect, SQLBulkOperations, or SQLSetPos
}

pub const SQLSTATE_SIZE: usize = 5;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SQLSTATE<C: OdbcChar>([C; SQLSTATE_SIZE + 1]);
impl SQLSTATE<SQLCHAR> {
    pub fn new(init: &str) -> SQLSTATE<SQLCHAR> {
        let bytes = init.as_bytes();

        assert_eq!(
            SQLSTATE_SIZE,
            bytes.len(),
            "SQLSTATE({}) len != {}",
            init,
            SQLSTATE_SIZE
        );

        let mut sqlstate = [SQLCHAR::default(); SQLSTATE_SIZE + 1];
        for (s, i) in sqlstate.iter_mut().zip(bytes.iter()) {
            *s = *i;
        }

        Self(sqlstate)
    }
}
impl SQLSTATE<SQLWCHAR> {
    pub fn new(init: &str) -> SQLSTATE<SQLWCHAR> {
        let bytes = init.as_bytes();

        assert_eq!(
            SQLSTATE_SIZE,
            bytes.len(),
            "SQLSTATE({}) len != {}",
            init,
            SQLSTATE_SIZE
        );

        let mut sqlstate = [SQLWCHAR::default(); SQLSTATE_SIZE + 1];
        for (s, i) in sqlstate.iter_mut().zip(bytes.iter()) {
            *s = *i as u16;
        }

        Self(sqlstate)
    }
}
unsafe impl<C: OdbcChar> AsMutSQLPOINTER for SQLSTATE<C> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        (self as *mut Self).cast()
    }
}
unsafe impl<C: OdbcChar> AsMutSQLPOINTER for MaybeUninit<SQLSTATE<C>> {
    fn as_mut_SQLPOINTER(&mut self) -> SQLPOINTER {
        self.as_mut_ptr().cast()
    }
}
impl PartialEq<&str> for SQLSTATE<SQLCHAR> {
    fn eq(&self, other: &&str) -> bool {
        *self == SQLSTATE::<SQLCHAR>::new(other)
    }
}
impl PartialEq<&str> for SQLSTATE<SQLWCHAR> {
    fn eq(&self, other: &&str) -> bool {
        *self == SQLSTATE::<SQLWCHAR>::new(other)
    }
}
impl<'a, C: OdbcChar> PartialEq<SQLSTATE<C>> for &'a str
where
    SQLSTATE<C>: PartialEq<&'a str>,
{
    fn eq(&self, other: &SQLSTATE<C>) -> bool {
        other == self
    }
}
impl<C: OdbcChar> AttrZeroAssert for SQLSTATE<C> {
    // This is character field and doesn't have to be zero checked
}
unsafe impl<C: OdbcChar> AttrLen<OdbcDefined, SQLSMALLINT> for SQLSTATE<C>
where
    MaybeUninit<SQLSTATE<C>>: AttrLen<OdbcDefined, SQLSMALLINT>,
{
    type StrLen = Void;

    fn len(&self) -> SQLSMALLINT {
        // This is ok because MaybeUninit<T> has the same memory layout as T
        <MaybeUninit<SQLSTATE<C>>>::len(unsafe { core::mem::transmute(self) })
    }
}
unsafe impl<AD: Def> AttrLen<AD, SQLSMALLINT> for MaybeUninit<SQLSTATE<SQLCHAR>> {
    type StrLen = Void;

    fn len(&self) -> SQLSMALLINT {
        (SQLSTATE_SIZE + 1) as SQLSMALLINT
    }
}
unsafe impl<AD: Def> AttrLen<AD, SQLSMALLINT> for MaybeUninit<SQLSTATE<SQLWCHAR>> {
    type StrLen = Void;

    fn len(&self) -> SQLSMALLINT {
        (core::mem::size_of::<SQLWCHAR>() * (SQLSTATE_SIZE + 1)) as SQLSMALLINT
    }
}

// Implement DiagField for uninitialized diagnostic attributes
impl<D: Ident, T: Scalar, H: Handle> DiagField<H, D> for MaybeUninit<T>
where
    T: DiagField<H, D> + AttrGet<D>,
    Self: AttrLen<Self::DefinedBy, SQLSMALLINT>,
{
}

impl<D: Ident, T: Scalar, H: Handle> DiagField<H, D> for [MaybeUninit<T>]
where
    [T]: DiagField<H, D> + AttrGet<D>,
    Self: AttrLen<Self::DefinedBy, SQLSMALLINT>,
{
}

impl<D: Ident, H: Handle> DiagField<H, D> for OdbcStr<MaybeUninit<SQLCHAR>> where
    OdbcStr<SQLCHAR>: DiagField<H, D> + AttrGet<D>
{
}
impl<D: Ident, H: Handle> DiagField<H, D> for OdbcStr<MaybeUninit<SQLWCHAR>> where
    OdbcStr<SQLWCHAR>: DiagField<H, D> + AttrGet<D>
{
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Header fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, -1249)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_CURSOR_ROW_COUNT;
unsafe impl Attr<SQL_DIAG_CURSOR_ROW_COUNT> for SQLLEN {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_CURSOR_ROW_COUNT> for SQLLEN {}
unsafe impl AttrGet<SQL_DIAG_CURSOR_ROW_COUNT> for SQLLEN {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 7)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_DYNAMIC_FUNCTION;
unsafe impl Attr<SQL_DIAG_DYNAMIC_FUNCTION> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_DYNAMIC_FUNCTION>
    for OdbcStr<SQLCHAR>
{
}
unsafe impl AttrGet<SQL_DIAG_DYNAMIC_FUNCTION> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 12)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_DYNAMIC_FUNCTION_CODE;
unsafe impl Attr<SQL_DIAG_DYNAMIC_FUNCTION_CODE> for DiagDynamicFunctionCode {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_DYNAMIC_FUNCTION_CODE>
    for DiagDynamicFunctionCode
{
}
unsafe impl AttrGet<SQL_DIAG_DYNAMIC_FUNCTION_CODE> for DiagDynamicFunctionCode {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 2)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_NUMBER;
unsafe impl Attr<SQL_DIAG_NUMBER> for SQLINTEGER {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_NUMBER> for SQLINTEGER {}
unsafe impl AttrGet<SQL_DIAG_NUMBER> for SQLINTEGER {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 1)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_RETURNCODE;
unsafe impl Attr<SQL_DIAG_RETURNCODE> for SQLRETURN {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_RETURNCODE> for SQLRETURN {}
unsafe impl AttrGet<SQL_DIAG_RETURNCODE> for SQLRETURN {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 3)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_ROW_COUNT;
unsafe impl Attr<SQL_DIAG_ROW_COUNT> for SQLLEN {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_ROW_COUNT> for SQLLEN {}
unsafe impl AttrGet<SQL_DIAG_ROW_COUNT> for SQLLEN {}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Record fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Ident)]
#[identifier(SQLSMALLINT, 8)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_CLASS_ORIGIN;
unsafe impl Attr<SQL_DIAG_CLASS_ORIGIN> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_CLASS_ORIGIN> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DIAG_CLASS_ORIGIN> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, -1247)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_COLUMN_NUMBER;
unsafe impl Attr<SQL_DIAG_COLUMN_NUMBER> for DiagColumnNumber {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_COLUMN_NUMBER>
    for DiagColumnNumber
{
}
unsafe impl AttrGet<SQL_DIAG_COLUMN_NUMBER> for DiagColumnNumber {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 10)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_CONNECTION_NAME;
unsafe impl Attr<SQL_DIAG_CONNECTION_NAME> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_CONNECTION_NAME> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DIAG_CONNECTION_NAME> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 6)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_MESSAGE_TEXT;
unsafe impl Attr<SQL_DIAG_MESSAGE_TEXT> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_MESSAGE_TEXT> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DIAG_MESSAGE_TEXT> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 5)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_NATIVE;
unsafe impl Attr<SQL_DIAG_NATIVE> for SQLINTEGER {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_NATIVE> for SQLINTEGER {}
unsafe impl AttrGet<SQL_DIAG_NATIVE> for SQLINTEGER {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, -1248)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_ROW_NUMBER;
unsafe impl Attr<SQL_DIAG_ROW_NUMBER> for DiagRowNumber {
    type DefinedBy = OdbcDefined;
}
impl<V: OdbcVersion> DiagField<SQLHSTMT<'_, '_, '_, V>, SQL_DIAG_ROW_NUMBER> for DiagRowNumber {}
unsafe impl AttrGet<SQL_DIAG_ROW_NUMBER> for DiagRowNumber {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 11)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_SERVER_NAME;
unsafe impl Attr<SQL_DIAG_SERVER_NAME> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_SERVER_NAME> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DIAG_SERVER_NAME> for OdbcStr<SQLCHAR> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 4)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_SQLSTATE;
unsafe impl<C: OdbcChar> Attr<SQL_DIAG_SQLSTATE> for SQLSTATE<C> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_SQLSTATE> for SQLSTATE<SQLCHAR> {}
impl<H: Handle> DiagField<H, SQL_DIAG_SQLSTATE> for SQLSTATE<SQLWCHAR> {}
unsafe impl<C: OdbcChar> AttrGet<SQL_DIAG_SQLSTATE> for SQLSTATE<C> {}

#[derive(Ident)]
#[identifier(SQLSMALLINT, 9)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_SUBCLASS_ORIGIN;
unsafe impl Attr<SQL_DIAG_SUBCLASS_ORIGIN> for OdbcStr<SQLCHAR> {
    type DefinedBy = OdbcDefined;
}
impl<H: Handle> DiagField<H, SQL_DIAG_SUBCLASS_ORIGIN> for OdbcStr<SQLCHAR> {}
unsafe impl AttrGet<SQL_DIAG_SUBCLASS_ORIGIN> for OdbcStr<SQLCHAR> {}

//=====================================================================================//

#[odbc_type(SQLINTEGER)]
pub struct DiagDynamicFunctionCode;
pub const SQL_DIAG_ALTER_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(3);
pub const SQL_DIAG_ALTER_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(4);
pub const SQL_DIAG_CREATE_ASSERTION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(6);
pub const SQL_DIAG_CREATE_CHARACTER_SET: DiagDynamicFunctionCode = DiagDynamicFunctionCode(8);
pub const SQL_DIAG_CREATE_COLLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(10);
pub const SQL_DIAG_CREATE_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(23);
pub const SQL_DIAG_CREATE_INDEX: DiagDynamicFunctionCode = DiagDynamicFunctionCode(-1);
pub const SQL_DIAG_CREATE_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(77);
pub const SQL_DIAG_CREATE_VIEW: DiagDynamicFunctionCode = DiagDynamicFunctionCode(84);
pub const SQL_DIAG_SELECT_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(85);
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(38);
pub const SQL_DIAG_DELETE_WHERE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(19);
pub const SQL_DIAG_DROP_ASSERTION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(24);
pub const SQL_DIAG_DROP_CHARACTER_SET: DiagDynamicFunctionCode = DiagDynamicFunctionCode(25);
pub const SQL_DIAG_DROP_COLLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(26);
pub const SQL_DIAG_DROP_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(27);
pub const SQL_DIAG_DROP_INDEX: DiagDynamicFunctionCode = DiagDynamicFunctionCode(-2);
pub const SQL_DIAG_DROP_SCHEMA: DiagDynamicFunctionCode = DiagDynamicFunctionCode(31);
pub const SQL_DIAG_DROP_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(32);
pub const SQL_DIAG_DROP_TRANSLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(33);
pub const SQL_DIAG_DROP_VIEW: DiagDynamicFunctionCode = DiagDynamicFunctionCode(36);
pub const SQL_DIAG_GRANT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(48);
pub const SQL_DIAG_INSERT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(50);
pub const SQL_DIAG_CALL: DiagDynamicFunctionCode = DiagDynamicFunctionCode(7);
pub const SQL_DIAG_REVOKE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(59);
pub const SQL_DIAG_CREATE_SCHEMA: DiagDynamicFunctionCode = DiagDynamicFunctionCode(64);
pub const SQL_DIAG_CREATE_TRANSLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(79);
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(81);
pub const SQL_DIAG_UPDATE_WHERE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(82);
pub const SQL_DIAG_UNKNOWN_STATEMENT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(0);

#[odbc_type(SQLINTEGER)]
pub struct DiagColumnNumber;
pub const SQL_NO_COLUMN_NUMBER: DiagColumnNumber = DiagColumnNumber(-1);
pub const SQL_COLUMN_NUMBER_UNKNOWN: DiagColumnNumber = DiagColumnNumber(-2);

#[odbc_type(SQLLEN)]
pub struct DiagRowNumber;
pub const SQL_NO_ROW_NUMBER: DiagRowNumber = DiagRowNumber(-1);
pub const SQL_ROW_NUMBER_UNKNOWN: DiagRowNumber = DiagRowNumber(-2);

//=====================================================================================//
//----------------------------------------Tests----------------------------------------//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn new_sqlstate_SQLCHAR() {
        let sqlstate = SQLSTATE::<SQLCHAR>::new("12345");

        assert_eq!(6, sqlstate.len());
        assert_eq!([49, 50, 51, 52, 53, 0].as_ref(), sqlstate.0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn new_sqlstate_SQLWCHAR() {
        let sqlstate = SQLSTATE::<SQLWCHAR>::new("12345");

        assert_eq!(12, sqlstate.len());
        assert_eq!([49, 50, 51, 52, 53, 0].as_ref(), sqlstate.0);
    }

    #[test]
    #[should_panic]
    #[allow(non_snake_case)]
    fn new_sqlstate_SQLCHAR_size_4() {
        SQLSTATE::<SQLCHAR>::new("0000");
    }

    #[test]
    #[should_panic]
    #[allow(non_snake_case)]
    fn new_sqlstate_SQLWCHAR_size_4() {
        SQLSTATE::<SQLWCHAR>::new("0000");
    }

    #[test]
    #[allow(non_snake_case)]
    fn sqlstate_SQLCHAR_cmp() {
        let sqlstate = SQLSTATE::<SQLCHAR>::new("12345");

        assert_eq!("12345", sqlstate);
        assert_eq!(sqlstate, "12345");
    }

    #[test]
    #[allow(non_snake_case)]
    fn sqlstate_SQLWCHAR_cmp() {
        let sqlstate = SQLSTATE::<SQLWCHAR>::new("12345");

        assert_eq!("12345", sqlstate);
        assert_eq!(sqlstate, "12345");
    }
}
