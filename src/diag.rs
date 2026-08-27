use core::mem::MaybeUninit;

use co3::{ReprC, Tag};
use rust_spec::RustSpec;

use crate::{
    data::{CHAR, WCHAR},
    env::OdbcVersion,
    handle::HSTMT,
    sqlreturn::RETURN,
    str::{OdbcChar, OdbcStr},
};

/// A diagnostic field supported by handle `H`.
///
/// This trait is open so drivers can define fields beginning at
/// `DRIVER_START` and provide their corresponding value type.
pub trait DiagField<H> {
    type Value<C: OdbcChar>: ?Sized;

    // TODO: These could be checked by the type system
    // CURSOR_ROW_COUNT -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // DYNAMIC_FUNCTION -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // DYNAMIC_FUNCTION_CODE -> The contents of this field are defined only after SQLExecute, SQLExecDirect, or SQLMoreResults
    // ROW_COUNT -> SQLExecute, SQLExecDirect, SQLBulkOperations, or SQLSetPos
}

pub const STATE_SIZE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct STATE<C: OdbcChar>([C; STATE_SIZE + 1]);
impl<C: OdbcChar> STATE<C> {
    pub const fn len(&self) -> usize {
        core::mem::size_of_val(&self.0)
    }

    pub const fn is_empty(&self) -> bool {
        false
    }
}
impl STATE<CHAR> {
    pub fn new(init: &str) -> STATE<CHAR> {
        let bytes = init.as_bytes();

        assert_eq!(
            STATE_SIZE,
            bytes.len(),
            "STATE({}) len != {}",
            init,
            STATE_SIZE
        );

        let mut sqlstate = [CHAR::default(); STATE_SIZE + 1];
        for (s, i) in sqlstate.iter_mut().zip(bytes.iter()) {
            *s = *i;
        }

        Self(sqlstate)
    }
}
impl STATE<WCHAR> {
    pub fn new(init: &str) -> STATE<WCHAR> {
        let bytes = init.as_bytes();

        assert_eq!(
            STATE_SIZE,
            bytes.len(),
            "STATE({}) len != {}",
            init,
            STATE_SIZE
        );

        let mut sqlstate = [WCHAR::default(); STATE_SIZE + 1];
        for (s, i) in sqlstate.iter_mut().zip(bytes.iter()) {
            *s = *i as u16;
        }

        Self(sqlstate)
    }
}
impl PartialEq<&str> for STATE<CHAR> {
    fn eq(&self, other: &&str) -> bool {
        *self == STATE::<CHAR>::new(other)
    }
}
impl PartialEq<&str> for STATE<WCHAR> {
    fn eq(&self, other: &&str) -> bool {
        *self == STATE::<WCHAR>::new(other)
    }
}
impl<'a, C: OdbcChar> PartialEq<STATE<C>> for &'a str
where
    STATE<C>: PartialEq<&'a str>,
{
    fn eq(&self, other: &STATE<C>) -> bool {
        other == self
    }
}
//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Header fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

macro_rules! define_diag_attributes {
    ($($name:ident = $id:expr;)+) => {$(
        #[derive(Tag)]
        #[tag(i16, unsafe($id))]
        #[allow(non_camel_case_types)]
        pub enum $name {}
    )+};
}

define_diag_attributes! {
    CURSOR_ROW_COUNT = -1249;
    DYNAMIC_FUNCTION = 7;
    DYNAMIC_FUNCTION_CODE = 12;
    NUMBER = 2;
    RETURNCODE = 1;
    ROW_COUNT = 3;
    CLASS_ORIGIN = 8;
    COLUMN_NUMBER = -1247;
    CONNECTION_NAME = 10;
    MESSAGE_TEXT = 6;
    NATIVE = 5;
    ROW_NUMBER = -1248;
    SERVER_NAME = 11;
    SQLSTATE = 4;
    SUBCLASS_ORIGIN = 9;
}

impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for CURSOR_ROW_COUNT {
    type Value<C: OdbcChar> = MaybeUninit<isize>;
}
impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for DYNAMIC_FUNCTION {
    type Value<C: OdbcChar> = OdbcStr<MaybeUninit<C>>;
}
impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for DYNAMIC_FUNCTION_CODE {
    type Value<C: OdbcChar> = MaybeUninit<DiagDynamicFunctionCode>;
}
impl<H> DiagField<H> for NUMBER {
    type Value<C: OdbcChar> = MaybeUninit<i32>;
}
impl<H> DiagField<H> for RETURNCODE {
    type Value<C: OdbcChar> = MaybeUninit<RETURN>;
}
impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for ROW_COUNT {
    type Value<C: OdbcChar> = MaybeUninit<isize>;
}

/////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// Record fields ////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_string_diag_field {
    ($($field:ty),+ $(,)?) => {$(
        impl<H> DiagField<H> for $field {
            type Value<C: OdbcChar> = OdbcStr<MaybeUninit<C>>;
        }
    )+};
}

impl_string_diag_field!(
    CLASS_ORIGIN,
    CONNECTION_NAME,
    MESSAGE_TEXT,
    SERVER_NAME,
    SUBCLASS_ORIGIN,
);

impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for COLUMN_NUMBER {
    type Value<C: OdbcChar> = MaybeUninit<DiagColumnNumber>;
}
impl<H> DiagField<H> for NATIVE {
    type Value<C: OdbcChar> = MaybeUninit<i32>;
}
impl<V: OdbcVersion> DiagField<HSTMT<'_, '_, '_, V>> for ROW_NUMBER {
    type Value<C: OdbcChar> = MaybeUninit<DiagRowNumber>;
}
impl<H> DiagField<H> for SQLSTATE {
    type Value<C: OdbcChar> = MaybeUninit<STATE<C>>;
}

//=====================================================================================//

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct DiagDynamicFunctionCode(pub(crate) i32);
pub const ALTER_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(3);
pub const ALTER_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(4);
pub const CREATE_ASSERTION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(6);
pub const CREATE_CHARACTER_SET: DiagDynamicFunctionCode = DiagDynamicFunctionCode(8);
pub const CREATE_COLLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(10);
pub const CREATE_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(23);
pub const CREATE_INDEX: DiagDynamicFunctionCode = DiagDynamicFunctionCode(-1);
pub const CREATE_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(77);
pub const CREATE_VIEW: DiagDynamicFunctionCode = DiagDynamicFunctionCode(84);
pub const SELECT_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(85);
pub const DYNAMIC_DELETE_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(38);
pub const DELETE_WHERE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(19);
pub const DROP_ASSERTION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(24);
pub const DROP_CHARACTER_SET: DiagDynamicFunctionCode = DiagDynamicFunctionCode(25);
pub const DROP_COLLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(26);
pub const DROP_DOMAIN: DiagDynamicFunctionCode = DiagDynamicFunctionCode(27);
pub const DROP_INDEX: DiagDynamicFunctionCode = DiagDynamicFunctionCode(-2);
pub const DROP_SCHEMA: DiagDynamicFunctionCode = DiagDynamicFunctionCode(31);
pub const DROP_TABLE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(32);
pub const DROP_TRANSLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(33);
pub const DROP_VIEW: DiagDynamicFunctionCode = DiagDynamicFunctionCode(36);
pub const GRANT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(48);
pub const INSERT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(50);
pub const CALL: DiagDynamicFunctionCode = DiagDynamicFunctionCode(7);
pub const REVOKE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(59);
pub const CREATE_SCHEMA: DiagDynamicFunctionCode = DiagDynamicFunctionCode(64);
pub const CREATE_TRANSLATION: DiagDynamicFunctionCode = DiagDynamicFunctionCode(79);
pub const DYNAMIC_UPDATE_CURSOR: DiagDynamicFunctionCode = DiagDynamicFunctionCode(81);
pub const UPDATE_WHERE: DiagDynamicFunctionCode = DiagDynamicFunctionCode(82);
pub const UNKNOWN_STATEMENT: DiagDynamicFunctionCode = DiagDynamicFunctionCode(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct DiagColumnNumber(pub(crate) i32);
pub const NO_COLUMN_NUMBER: DiagColumnNumber = DiagColumnNumber(-1);
pub const COLUMN_NUMBER_UNKNOWN: DiagColumnNumber = DiagColumnNumber(-2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct DiagRowNumber(pub(crate) isize);
pub const NO_ROW_NUMBER: DiagRowNumber = DiagRowNumber(-1);
pub const ROW_NUMBER_UNKNOWN: DiagRowNumber = DiagRowNumber(-2);

//=====================================================================================//
//----------------------------------------Tests----------------------------------------//

#[cfg(test)]
mod test {
    #![expect(non_snake_case)]

    use super::*;

    #[test]
    fn new_sqlstate_SQLCHAR() {
        let sqlstate = STATE::<CHAR>::new("12345");

        assert_eq!(6, sqlstate.len());
        assert_eq!([49, 50, 51, 52, 53, 0].as_ref(), sqlstate.0);
    }

    #[test]
    fn new_sqlstate_SQLWCHAR() {
        let sqlstate = STATE::<WCHAR>::new("12345");

        assert_eq!(12, sqlstate.len());
        assert_eq!([49, 50, 51, 52, 53, 0].as_ref(), sqlstate.0);
    }

    #[test]
    #[should_panic]
    fn new_sqlstate_SQLCHAR_size_4() {
        STATE::<CHAR>::new("0000");
    }

    #[test]
    #[should_panic]
    fn new_sqlstate_SQLWCHAR_size_4() {
        STATE::<WCHAR>::new("0000");
    }

    #[test]
    fn sqlstate_SQLCHAR_cmp() {
        let sqlstate = STATE::<CHAR>::new("12345");

        assert_eq!("12345", sqlstate);
        assert_eq!(sqlstate, "12345");
    }

    #[test]
    fn sqlstate_SQLWCHAR_cmp() {
        let sqlstate = STATE::<WCHAR>::new("12345");

        assert_eq!("12345", sqlstate);
        assert_eq!(sqlstate, "12345");
    }
}
