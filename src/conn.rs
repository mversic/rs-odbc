use co3::ReprC;
use rust_spec::RustSpec;

use crate::{
    Defined,
    attr::{impl_odbc_scalar_attr_unpack, *},
    env::{OV_ODBC3, OV_ODBC3_80, OV_ODBC4},
    info::TxnIsolation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum DriverCompletion {
    DRIVER_NOPROMPT = 0,
    DRIVER_COMPLETE = 1,
    DRIVER_PROMPT = 2,
    DRIVER_COMPLETE_REQUIRED = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[repr(i16)]
pub enum CompletionType {
    COMMIT = 0,
    ROLLBACK = 1,
}

/// Marks a connection attribute whose value can be retrieved while the connection is in `S`.
///
/// # Safety
/// `Buffer` must have the representation and initialization requirements prescribed by ODBC or
/// by the driver specification.
pub unsafe trait ConnAttrGet<V: crate::env::OdbcVersion, S: ConnState>: Defined {
    type Buffer<C: crate::str::OdbcChar>: ?Sized;
}

/// Marks a connection attribute whose value can be supplied while the connection is in `S`.
///
/// # Safety
/// `Value` must lower to the representation prescribed by ODBC or by the driver specification.
pub unsafe trait ConnAttrSet<V: crate::env::OdbcVersion, S: ConnState>: Defined {
    type Value<'a, C: crate::str::OdbcChar + 'a>;
}

inherit_attr!(get ConnAttrGet, OV_ODBC3 => OV_ODBC3_80, state);
inherit_attr!(get ConnAttrGet, OV_ODBC3_80 => OV_ODBC4, state);
inherit_attr!(set ConnAttrSet, OV_ODBC3 => OV_ODBC3_80, state);
inherit_attr!(set ConnAttrSet, OV_ODBC3_80 => OV_ODBC4, state);

/// Sealed marker for the connection states supported by this crate.
#[sealed::sealed]
pub trait ConnState {}

/// Sealed marker for states in which browsing is valid.
#[sealed::sealed]
pub trait BrowseConnect: ConnState {}
/// Sealed marker for states in which disconnection is valid.
#[sealed::sealed]
pub trait Disconnect: ConnState {}

/// Allocated
#[derive(Debug)]
pub enum C2 {}

/// Need data
#[derive(Debug)]
pub enum C3 {}

/// Connected
#[derive(Debug)]
pub enum C4 {}

#[sealed::sealed]
impl ConnState for C2 {}
#[sealed::sealed]
impl ConnState for C3 {}
#[sealed::sealed]
impl ConnState for C4 {}

#[sealed::sealed]
impl BrowseConnect for C2 {}
#[sealed::sealed]
impl BrowseConnect for C3 {}

#[sealed::sealed]
impl Disconnect for C3 {}
#[sealed::sealed]
impl Disconnect for C4 {}

macro_rules! impl_conn_attr_set_state {
    ($version:ty, $attr:ty, $value:ty; $($state:ty),+ $(,)?) => {$(
        unsafe impl ConnAttrSet<$version, $state> for $attr {
            type Value<'a, C: crate::str::OdbcChar + 'a> = $value;
        }
    )+};
}

macro_rules! impl_conn_attr_get_states {
    ($version:ty, $attr:ty, $value:ty; $($state:ty),+ $(,)?) => {
        impl Defined for $attr {
            type By = crate::OdbcDefined;
        }
        $(unsafe impl ConnAttrGet<$version, $state> for $attr {
            type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<$value>;
        })+
    };
}

macro_rules! impl_conn_string_attr_get_states {
    ($version:ty, $attr:ty; $($state:ty),+ $(,)?) => {
        impl Defined for $attr {
            type By = crate::OdbcDefined;
        }
        $(unsafe impl ConnAttrGet<$version, $state> for $attr {
            type Buffer<C: crate::str::OdbcChar> =
                crate::str::OdbcStr<core::mem::MaybeUninit<C>>;
        })+
    };
}

macro_rules! impl_conn_string_attr_set_state {
    ($version:ty, $attr:ty; $($state:ty),+ $(,)?) => {$(
        unsafe impl ConnAttrSet<$version, $state> for $attr {
            type Value<'a, C: crate::str::OdbcChar + 'a> = &'a crate::str::OdbcStr<C>;
        }
    )+};
}

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// TODO: Spec says this is 3.5, but it is not 3.5 in implementation ???
// but it says that drivers conforming to earlier versions can support this field
//#[derive(Tag)]
//#[tag(i32, unsafe(107))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_TRANSLATE_OPTION;
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &OwnedHDBC<SQL_OV_ODBC3>) {
//        ConnectionHandle.assert_connected();
//    }
//}

//#[derive(Tag)]
//#[tag(i32, unsafe(118))]
//// This is set-only attribute
//pub struct SQL_ATTR_DBC_INFO_TOKEN;
//    #[cfg(feature = "odbc_debug")]
//    fn check_attr(&self, ConnectionHandle: &OwnedHDBC<SQL_OV_ODBC3_80>) {
//        assert_connected(ConnectionHandle);
//    }

//#[derive(Tag)]
//#[tag(i32, unsafe(119))]
//pub struct SQL_ATTR_ASYNC_DBC_EVENT;
//// TODO: It's an Event handle. Should probably implement event handle

//#[derive(Tag)]
//#[tag(i32, unsafe(111))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_QUIET_MODE;

// TODO: Not found in documentation, only in implementation
//#[derive(Tag)]
//#[tag(i32, unsafe(114))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_DISCONNECT_BEHAVIOR;

//#[expect(non_camel_case_types)]
//pub struct DisconnectBehavior;
//pub const SQL_DB_RETURN_TO_POOL: DisconnectBehavior = DisconnectBehavior(0);
//pub const SQL_DB_DISCONNECT: DisconnectBehavior = DisconnectBehavior(1);

//*  ODBC Driver Manager sets this connection attribute to a unicode driver
//    (which supports SQLConnectW) when the application is an ANSI application
//    (which calls SQLConnect, SQLDriverConnect, or SQLBrowseConnect).
//    This is SetConnectAttr only and application does not set this attribute
//    This attribute was introduced because some unicode driver's some APIs may
//    need to behave differently on ANSI or Unicode applications. A unicode
//    driver, which  has same behavior for both ANSI or Unicode applications,
//    should return ERROR when the driver manager sets this connection
//    attribute. When a unicode driver returns SUCCESS on this attribute,
//    the driver manager treates ANSI and Unicode connections differently in
//    connection pooling.
//*/
//// TODO: These 4 are not in Documentation??
//#[derive(Tag)]
//#[tag(i32, unsafe(115))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_ANSI_APP;

//pub enum AnsiApp {
//    SQL_AA_TRUE = 1,  /* the application is an ANSI app */
//    SQL_AA_FALSE = 0,  /* the application is a Unicode app */
//}

//#[derive(Tag)]
//#[tag(i32, unsafe(116))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_RESET_CONNECTION;

//#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//pub enum ResetConnection {
//    SQL_RESET_CONNECTION_YES = 1,
//}

//#[derive(Tag)]
//#[tag(i32, unsafe(122))]
//#[expect(non_camel_case_types)]
//pub struct SQL_ATTR_CREDENTIALS;

impl_conn_attr_get_states!(OV_ODBC3, ACCESS_MODE, AccessMode; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, ACCESS_MODE, AccessMode; C2, C4);
impl_conn_attr_get_states!(OV_ODBC3, AUTOCOMMIT, AutoCommit; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, AUTOCOMMIT, AutoCommit; C2, C4);
impl_conn_attr_get_states!(OV_ODBC3, CONNECTION_TIMEOUT, u32; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, CONNECTION_TIMEOUT, u32; C2, C4);
impl_conn_string_attr_get_states!(OV_ODBC3, CURRENT_CATALOG; C2, C4);
impl_conn_string_attr_set_state!(OV_ODBC3, CURRENT_CATALOG; C2, C4);
impl_conn_attr_get_states!(OV_ODBC3, LOGIN_TIMEOUT, u32; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, LOGIN_TIMEOUT, u32; C2);
impl_conn_attr_get_states!(OV_ODBC3, PACKET_SIZE, u32; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, PACKET_SIZE, u32; C2);
impl_conn_attr_get_states!(OV_ODBC3, TRACE, Trace; C2, C4);
impl_conn_attr_set_state!(OV_ODBC3, TRACE, Trace; C2, C4);
impl_conn_string_attr_get_states!(OV_ODBC3, TRACEFILE; C2, C4);
impl_conn_string_attr_set_state!(OV_ODBC3, TRACEFILE; C2, C4);
impl_conn_string_attr_get_states!(OV_ODBC3, TRANSLATE_LIB; C4);
impl_conn_string_attr_set_state!(OV_ODBC3, TRANSLATE_LIB; C4);
impl_conn_attr_get_states!(OV_ODBC3, TRANSLATE_OPTION, u32; C4);
impl_conn_attr_set_state!(OV_ODBC3, TRANSLATE_OPTION, u32; C4);
impl_conn_attr_get_states!(OV_ODBC3, AUTO_IPD, AutoIpd; C4);
impl_conn_attr_get_states!(
    OV_ODBC3_80,
    ASYNC_DBC_FUNCTIONS_ENABLE,
    AsyncDbcFunctionsEnable;
    C2,
    C4
);
impl_conn_attr_set_state!(
    OV_ODBC3_80,
    ASYNC_DBC_FUNCTIONS_ENABLE,
    AsyncDbcFunctionsEnable;
    C2,
    C4
);
impl_conn_attr_get_states!(OV_ODBC3_80, CONNECTION_DEAD, ConnectionDead; C4);
unsafe impl ConnAttrGet<OV_ODBC3, C2> for METADATA_ID {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<MetadataId>;
}
unsafe impl ConnAttrGet<OV_ODBC3, C4> for METADATA_ID {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<MetadataId>;
}
unsafe impl ConnAttrSet<OV_ODBC3, C2> for METADATA_ID {
    type Value<'a, C: crate::str::OdbcChar + 'a> = MetadataId;
}
unsafe impl ConnAttrSet<OV_ODBC3, C4> for METADATA_ID {
    type Value<'a, C: crate::str::OdbcChar + 'a> = MetadataId;
}

macro_rules! conn_bool {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
        #[repr(u32)]
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

        impl_odbc_scalar_attr_unpack!($name => u32);
    )+};
}

conn_bool!(AutoIpd, MetadataId);

unsafe impl ConnAttrGet<OV_ODBC3, C2> for ASYNC_ENABLE {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<crate::stmt::AsyncEnable>;
}
unsafe impl ConnAttrGet<OV_ODBC3, C4> for ASYNC_ENABLE {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<crate::stmt::AsyncEnable>;
}
unsafe impl ConnAttrSet<OV_ODBC3, C2> for ASYNC_ENABLE {
    type Value<'a, C: crate::str::OdbcChar + 'a> = crate::stmt::AsyncEnable;
}
unsafe impl ConnAttrSet<OV_ODBC3, C4> for ASYNC_ENABLE {
    type Value<'a, C: crate::str::OdbcChar + 'a> = crate::stmt::AsyncEnable;
}
impl Defined for TXN_ISOLATION {
    type By = crate::OdbcDefined;
}
unsafe impl ConnAttrGet<OV_ODBC3, C4> for TXN_ISOLATION {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<Option<TxnIsolation>>;
}
impl_conn_attr_set_state!(OV_ODBC3, TXN_ISOLATION, TxnIsolation; C4);
unsafe impl ConnAttrGet<OV_ODBC4, C4> for REFRESH_CONNECTION {
    type Buffer<C: crate::str::OdbcChar> = core::mem::MaybeUninit<RefreshConnection>;
}
impl Defined for REFRESH_CONNECTION {
    type By = crate::OdbcDefined;
}
impl_conn_attr_set_state!(OV_ODBC4, REFRESH_CONNECTION, RefreshConnection; C4);
// impl_conn_string_attr_get_states!(OV_ODBC4, CREDENTIALS; C2, C4);
// impl_conn_string_attr_set_state!(OV_ODBC4, CREDENTIALS; C2);

//=====================================================================================//

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum AccessMode {
    MODE_READ_WRITE,
    MODE_READ_ONLY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum AutoCommit {
    AUTOCOMMIT_OFF,
    AUTOCOMMIT_ON,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum Trace {
    OPT_TRACE_OFF,
    OPT_TRACE_ON,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum AsyncDbcFunctionsEnable {
    ASYNC_DBC_ENABLE_OFF,
    ASYNC_DBC_ENABLE_ON,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum ConnectionDead {
    CD_FALSE,
    CD_TRUE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustSpec, ReprC)]
#[expect(non_camel_case_types)]
#[repr(i32)]
pub enum RefreshConnection {
    REFRESH_NOW = -1,
    REFRESH_AUTO = 0,
    REFRESH_MANUAL = 1,
}

impl_odbc_scalar_attr_unpack!(
    AccessMode => u32,
    AutoCommit => u32,
    Trace => u32,
    AsyncDbcFunctionsEnable => u32,
    ConnectionDead => u32,
    RefreshConnection => i32,
);

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::*;

    fn assert_get<V, S, A>()
    where
        V: crate::env::OdbcVersion,
        S: ConnState,
        A: ConnAttrGet<V, S>,
    {
    }

    fn assert_set<V, S, A>()
    where
        V: crate::env::OdbcVersion,
        S: ConnState,
        A: ConnAttrSet<V, S>,
    {
    }

    #[test]
    fn connection_attribute_states_and_versions_are_composable() {
        // Before-only setters.
        assert_set::<OV_ODBC3, C2, LOGIN_TIMEOUT>();
        assert_set::<OV_ODBC3_80, C2, PACKET_SIZE>();

        // Connected-only access.
        assert_get::<OV_ODBC3, C4, AUTO_IPD>();
        assert_get::<OV_ODBC3_80, C4, CONNECTION_DEAD>();
        assert_set::<OV_ODBC4, C4, REFRESH_CONNECTION>();

        // Attributes valid on both sides of connecting.
        assert_get::<OV_ODBC3, C2, AUTOCOMMIT>();
        assert_get::<OV_ODBC3, C4, AUTOCOMMIT>();
        assert_set::<OV_ODBC3, C2, AUTOCOMMIT>();
        assert_set::<OV_ODBC3, C4, AUTOCOMMIT>();

        // State-aware implementations still inherit into newer ODBC versions.
        assert_get::<OV_ODBC4, C2, ACCESS_MODE>();
        assert_set::<OV_ODBC4, C4, ACCESS_MODE>();
    }

    //#[test]
    //fn test_SQL_ATTR_METADATA_ID_is_ConnAttr() {
    //    let SQLSetConnectAttr_ctx = ffi::SQLSetConnectAttrA_context();
    //    SQLSetConnectAttr_ctx.expect().once().return_const(SQL_SUCCESS);
    //    let SQLFreeHandle_ctx = ffi::SQLFreeHandle_context();
    //    SQLFreeHandle_ctx.expect().once()0return_const(SQL_SUCCESS);

    //    let handle = unsafe { SQLHDBC::<C2, SQL_OV_ODBC3_80>::from_raw(13 as SQLHANDLE)};
    //    assert_eq!(SQL_SUCCESS, handle.SQLSetConnectAttrA(SQL_ATTR_METADATA_ID, SQL_TRUE));
    //}

    //#[test]
    //fn test_SQL_ATTR_ASYNC_ENABLE_is_ConnAttr() {
    //    let SQLSetConnectAttr_ctx = ffi::SQLSetConnectAttrA_context();
    //    SQLSetConnectAttr_ctx.expect().once().return_const(SQL_SUCCESS);

    //    let handle = unsafe { SQLHDBC::<C2, SQL_OV_ODBC3_80>::from_raw(13 as SQLHANDLE)};
    //    assert_eq!(SQL_SUCCESS, handle.SQLSetConnectAttrA(SQL_ATTR_ASYNC_ENABLE, SQL_ASYNC_ENABLE_OFF));
    //}
}
