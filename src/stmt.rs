use co3::ReprC;
use rust_spec::RustSpec;

use crate::{
    Defined,
    attr::{impl_odbc_scalar_attr_unpack, *},
    env::OV_ODBC3,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum NullAllowed {
    NO_NULLS = 0,
    NULLABLE = 1,
    NULLABLE_UNKNOWN = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum IdentifierType {
    BEST_ROWID = 1,
    ROWVER = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum BulkOperation {
    ADD = 4,
    UPDATE_BY_BOOKMARK = 5,
    DELETE_BY_BOOKMARK = 6,
    FETCH_BY_BOOKMARK = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(u16)]
pub enum Operation {
    POSITION = 0,
    REFRESH = 1,
    UPDATE = 2,
    DELETE = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum LockType {
    LOCK_NO_CHANGE = 0,
    LOCK_EXCLUSIVE = 1,
    LOCK_UNLOCK = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum FreeStmtOption {
    CLOSE = 0,
    UNBIND = 2,
    RESET_PARAMS = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(u16)]
pub enum Reserved {
    QUICK = 0,
    ENSURE = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum Unique {
    INDEX_UNIQUE = 0,
    INDEX_ALL = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum Scope {
    SCOPE_CURROW = 0,
    SCOPE_TRANSACTION = 1,
    SCOPE_SESSION = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i16)]
pub enum IOType {
    PARAM_INPUT = 1,
    PARAM_INPUT_OUTPUT = 2,
    PARAM_OUTPUT = 4,
    PARAM_INPUT_OUTPUT_STREAM = 8,
    PARAM_OUTPUT_STREAM = 16,
    PARAM_TYPE_UNKNOWN = 0,
    RESULT_COL = 3,
    RETURN_VALUE = 5,
}

/// Marks a statement attribute whose value can be retrieved.
///
/// # Safety
/// `Buffer` must have the representation and initialization requirements prescribed by ODBC or
/// by the driver specification.
pub unsafe trait StmtAttrGet<V: crate::env::OdbcVersion>: Defined {
    type Buffer<C: crate::str::OdbcChar>: ?Sized;
}

/// Marks a statement attribute whose value can be supplied.
///
/// # Safety
/// `Value` must lower to the representation prescribed by ODBC or by the driver specification.
pub unsafe trait StmtAttrSet<V: crate::env::OdbcVersion>: Defined {
    type Value<'a, C: crate::str::OdbcChar + 'a>;
}

inherit_attr!(get StmtAttrGet, OV_ODBC3 => crate::env::OV_ODBC3_80);
inherit_attr!(get StmtAttrGet, crate::env::OV_ODBC3_80 => crate::env::OV_ODBC4);
inherit_attr!(set StmtAttrSet, OV_ODBC3 => crate::env::OV_ODBC3_80);
inherit_attr!(set StmtAttrSet, crate::env::OV_ODBC3_80 => crate::env::OV_ODBC4);

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// TODO: This attribute cannot be specified after the SQL statement has been prepared.

// TODO: This attribute cannot be specified for an open cursor

// TODO:
//#[derive(Tag)]
//#[tag(i32, unsafe(16))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_FETCH_BOOKMARK_PTR;

// The following are Header fields--------------------------------
//
// TODO: This one could be special??
// Corresponds to ARD SQL_DESC_BIND_TYPE
//#[derive(Tag)]
//#[tag(i32, unsafe(5))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_TYPE;
//
// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
// Corresponds to APD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(17))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_OFFSET_PTR;
//
// Corresponds to APD SQL_DESC_BIND_TYPE
//#[derive(Tag)]
//#[tag(i32, unsafe(18))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_BIND_TYPE;
//
// Corresponds to APD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(19))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_OPERATION_PTR;
//
// Corresponds to IPD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(20))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAM_STATUS_PTR;
//
// Corresponds to IPD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(21))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMS_PROCESSED_PTR;
//
// Corresponds to APD SQL_DESC_ARRAY_SIZE
//#[derive(Tag)]
//#[tag(i32, unsafe(22))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_PARAMSET_SIZE;
//

// TODO: This cannot be supported until SQL_DESC_BIND_OFFSET_PTR is supported in descriptors
// Corresponds to ARD SQL_DESC_BIND_OFFSET_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(23))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_BIND_OFFSET_PTR;
//
// Corresponds to ARD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(24))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_OPERATION_PTR;
//
// Corresponds to IRD SQL_DESC_ARRAY_STATUS_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(25))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_STATUS_PTR;
//
// Corresponds to IRD SQL_DESC_ROWS_PROCESSED_PTR
//#[derive(Tag)]
//#[tag(i32, unsafe(26))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROWS_FETCHED_PTR;
//
// Corresponds to ARD SQL_DESC_ARRAY_SIZE
//#[derive(Tag)]
//#[tag(i32, unsafe(27))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ROW_ARRAY_SIZE;
//
//#[tag(i32, unsafe(29))]
//#[derive(Tag)]
//#[cfg(feature = "v3_8")]
//#[expect(non_camel_case_types)]
// TODO: This type MUST be Rc or similar
//pub struct SQL_ATTR_ASYNC_STMT_EVENT;
//
//#[tag(i32, unsafe(30))]
//#[derive(Tag)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_SAMPLE_SIZE;
//
//#[tag(i32, unsafe(31))]
//#[derive(Tag)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_DYNAMIC_COLUMNS;
//
//#[tag(i32, unsafe(32))]
//#[derive(Tag)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_TYPE_EXCEPTION_BEHAVIOR;
//
//#[tag(i32, unsafe(33))]
//#[derive(Tag)]
//#[cfg(feature = "v4")]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_LENGTH_EXCEPTION_BEHAVIOR;

impl_attr!(Stmt, get set, OV_ODBC3, QUERY_TIMEOUT => usize);
impl_attr!(Stmt, get set, OV_ODBC3, MAX_ROWS => usize);
impl_attr!(Stmt, get set, OV_ODBC3, NOSCAN => Noscan);
impl_attr!(Stmt, get set, OV_ODBC3, MAX_LENGTH => usize);
impl_attr!(Stmt, get set, OV_ODBC3, CURSOR_TYPE => CursorType);
impl_attr!(Stmt, get set, OV_ODBC3, CONCURRENCY => Concurrency);
impl_attr!(Stmt, get set, OV_ODBC3, KEYSET_SIZE => usize);
impl_attr!(Stmt, get set, OV_ODBC3, SIMULATE_CURSOR => SimulateCursor);
impl_attr!(Stmt, get set, OV_ODBC3, RETRIEVE_DATA => RetrieveData);
impl_attr!(Stmt, get set, OV_ODBC3, USE_BOOKMARKS => UseBookmarks);
impl_attr!(Stmt, get set, OV_ODBC3, ENABLE_AUTO_IPD => EnableAutoIpd);
impl_attr!(Stmt, get, OV_ODBC3, ROW_NUMBER => usize);
impl_attr!(Stmt, get set, OV_ODBC3, CURSOR_SCROLLABLE => CursorScrollable);
impl_attr!(Stmt, get set, OV_ODBC3, CURSOR_SENSITIVITY => CursorSensitivity);
impl_attr!(Stmt, get set, OV_ODBC3, METADATA_ID => MetadataId);
// TODO: For drivers with statement level asynchronous execution support,
impl_attr!(Stmt, get set, OV_ODBC3, ASYNC_ENABLE => AsyncEnable);

// impl_attr!(Stmt, get set, OV_ODBC3, ROW_BIND_TYPE => usize);
// impl_attr!(Stmt, get set, OV_ODBC3, PARAM_BIND_TYPE => usize);
// impl_attr!(Stmt, get set, OV_ODBC3, PARAMSET_SIZE => usize);
// impl_attr!(Stmt, get set, OV_ODBC3, ROW_ARRAY_SIZE => usize);
// impl_attr!(Stmt, get set, crate::env::OV_ODBC4, DYNAMIC_COLUMNS => DynamicColumns);
// impl_attr!(Stmt, get set, crate::env::OV_ODBC4, TYPE_EXCEPTION_BEHAVIOR => TypeExceptionBehavior);
// impl_attr!(Stmt, get set, crate::env::OV_ODBC4, LENGTH_EXCEPTION_BEHAVIOR => LengthExceptionBehavior);

// TODO: Not found in implementation
// #[cfg(feature = "v3_8")]
// SQL_ATTR_ASYNC_STMT_PCALLBACK
// #[cfg(feature = "v3_8")]
// SQL_ATTR_ASYNC_STMT_PCONTEXT

//=====================================================================================//

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum Noscan {
    NOSCAN_OFF,
    NOSCAN_ON,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum CursorType {
    CURSOR_FORWARD_ONLY,
    CURSOR_KEYSET_DRIVEN,
    CURSOR_DYNAMIC,
    CURSOR_STATIC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum Concurrency {
    CONCUR_READ_ONLY = 1,
    CONCUR_LOCK = 2,
    CONCUR_ROWVER = 3,
    CONCUR_VALUES = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum SimulateCursor {
    SC_NON_UNIQUE,
    SC_TRY_UNIQUE,
    SC_UNIQUE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum RetrieveData {
    RD_OFF,
    RD_ON,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum UseBookmarks {
    UB_OFF,
    UB_ON,
    UB_VARIABLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(usize)]
pub enum AsyncEnable {
    ASYNC_ENABLE_OFF,
    ASYNC_ENABLE_ON,
}

macro_rules! stmt_bool {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
        #[repr(usize)]
        pub enum $name {
            FALSE = 0,
            TRUE = 1,
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                if value { Self::TRUE } else { Self::FALSE }
            }
        }

        impl From<$name> for bool {
            fn from(value: $name) -> Self {
                value == $name::TRUE
            }
        }
    )+};
}

stmt_bool!(EnableAutoIpd, MetadataId);

// #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
// #[repr(u16)]
// pub enum TypeExceptionBehavior {
//     TE_ERROR = 1,
//     TE_CONTINUE = 2,
//     TE_REPORT = 3,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
// #[repr(u16)]
// pub enum LengthExceptionBehavior {
//     LE_CONTINUE = 1,
//     LE_REPORT = 2,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(usize)]
pub enum CursorScrollable {
    NONSCROLLABLE,
    SCROLLABLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(usize)]
pub enum CursorSensitivity {
    UNSPECIFIED,
    INSENSITIVE,
    SENSITIVE,
}

impl_odbc_scalar_attr_unpack!(
    Noscan => usize,
    CursorType => usize,
    Concurrency => usize,
    SimulateCursor => usize,
    RetrieveData => usize,
    UseBookmarks => usize,
    AsyncEnable => usize,
    EnableAutoIpd => usize,
    MetadataId => usize,
    // TypeExceptionBehavior => u16,
    // LengthExceptionBehavior => u16,
    CursorScrollable => usize,
    CursorSensitivity => usize,
);
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_attribute_representations_match_odbc_widths() {
        assert_eq!(
            core::mem::size_of::<EnableAutoIpd>(),
            core::mem::size_of::<usize>()
        );
        assert_eq!(
            core::mem::size_of::<MetadataId>(),
            core::mem::size_of::<usize>()
        );
    }

    #[test]
    fn variable_bookmarks_have_the_standard_value() {
        assert_eq!(UseBookmarks::UB_VARIABLE as usize, 2);
    }
}
