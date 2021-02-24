use crate::{ReadAttr, SQLLEN, SQLSMALLINT, OdbcAttr};
use rs_odbc_derive::{DiagField, Identifier};

pub trait DiagField<H: crate::handle::HandleIdentifier>: crate::Identifier<IdentType = SQLSMALLINT> {
    type AttrType;
}

// Header fields -----------------------------------------------------------------
#[derive(Identifier)]
#[identifier(SQLSMALLINT, -1249)]
#[allow(non_camel_case_types)]
pub struct SQL_DIAG_CURSOR_ROW_COUNT;
impl DiagField<crate::handle::SQL_HANDLE_STMT> for SQL_DIAG_CURSOR_ROW_COUNT {
    type AttrType = OdbcAttr;
}
unsafe impl<C> ReadAttr<C, SQLLEN> for SQL_DIAG_CURSOR_ROW_COUNT {}

//#[identifier(7)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_DYNAMIC_FUNCTION;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_DYNAMIC_FUNCTION {}
//
//#[identifier(12)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_DYNAMIC_FUNCTION_CODE;
//impl ReadAttr<SQLINTEGER> for SQL_DIAG_DYNAMIC_FUNCTION_CODE {}
//
//#[identifier(2)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_NUMBER;
//impl ReadAttr<SQLINTEGER> for SQL_DIAG_NUMBER {}
//
//#[identifier(1)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_RETURNCODE;
//impl ReadAttr<SQLRETURN> for SQL_DIAG_RETURNCODE {}
//
//#[identifier(3)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_ROW_COUNT;
//impl ReadAttr<SQLLEN> for SQL_DIAG_ROW_COUNT {}
//
//// Record fields ---------------------------------------------------------------
//#[identifier(8)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_CLASS_ORIGIN;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_CLASS_ORIGIN {}
//
//#[identifier(-1247)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_COLUMN_NUMBER;
//impl ReadAttr<SQLINTEGER> for SQL_DIAG_COLUMN_NUMBER {}
//
////#[repr(SQLINTEGER)]
////pub enum ColumnNumber {
////    SQL_NO_COLUMN_NUMBER = -1,
////    SQL_COLUMN_NUMBER_UNKNOWN = -2,
////}
//
//#[identifier(10)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_CONNECTION_NAME;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_CONNECTION_NAME {}
//
//#[identifier(6)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_MESSAGE_TEXT;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_MESSAGE_TEXT {}
//
//#[identifier(5)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_NATIVE;
//impl ReadAttr<SQLINTEGER> for SQL_DIAG_NATIVE {}
//
//#[identifier(-1248)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_ROW_NUMBER;
//impl ReadAttr<SQLLEN> for SQL_DIAG_ROW_NUMBER {}
//
//pub enum RowNumber {
//    SQL_NO_ROW_NUMBER = -1,
//    SQL_ROW_NUMBER_UNKNOWN = -2,
//}
//
//#[identifier(11)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_SERVER_NAME;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_SERVER_NAME {}
//
//#[identifier(4)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_SQLSTATE;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_SQLSTATE {}
//
//#[identifier(9)]
//#[derive(DiagField)]
//pub struct SQL_DIAG_SUBCLASS_ORIGIN;
//impl<T: AsMutRawCharSlice> ReadAttr<T> for SQL_DIAG_SUBCLASS_ORIGIN {}
