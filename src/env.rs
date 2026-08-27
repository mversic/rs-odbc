use co3::ReprC;
use rust_spec::RustSpec;

use crate::{Defined, OdbcDefined, attr::*};

/// Marks an environment attribute whose value can be retrieved.
///
/// # Safety
/// `Buffer` must have the representation and initialization requirements prescribed by ODBC.
#[sealed::sealed]
pub unsafe trait EnvAttrGet<V: OdbcVersion>: Defined<By = OdbcDefined> {
    type Buffer<C: crate::str::OdbcChar>: ?Sized;
}

/// Marks an environment attribute whose value can be supplied.
///
/// # Safety
/// `Value` must lower to the representation prescribed by ODBC.
#[sealed::sealed]
pub unsafe trait EnvAttrSet<V: OdbcVersion>: Defined<By = OdbcDefined> {
    type Value<'a, C: crate::str::OdbcChar + 'a>;
}

macro_rules! inherit_env_attr {
    (get $from:ty => $to:ty) => {
        #[sealed::sealed]
        unsafe impl<T: EnvAttrGet<$from>> EnvAttrGet<$to> for T {
            type Buffer<C: crate::str::OdbcChar> = <T as EnvAttrGet<$from>>::Buffer<C>;
        }
    };
    (set $from:ty => $to:ty) => {
        #[sealed::sealed]
        unsafe impl<T: EnvAttrSet<$from>> EnvAttrSet<$to> for T {
            type Value<'a, C: crate::str::OdbcChar + 'a> = <T as EnvAttrSet<$from>>::Value<'a, C>;
        }
    };
}

inherit_env_attr!(get OV_ODBC3 => OV_ODBC3_80);
inherit_env_attr!(get OV_ODBC3_80 => OV_ODBC4);
inherit_env_attr!(set OV_ODBC3 => OV_ODBC3_80);
inherit_env_attr!(set OV_ODBC3_80 => OV_ODBC4);

#[derive(RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub(crate) struct VersionValue(pub(crate) u32);

impl_attr!(Env, set, OV_ODBC3, ODBC_VERSION => VersionValue);
impl_attr!(Env, get set, OV_ODBC3, CP_MATCH => CpMatch);
impl_attr!(Env, get set, OV_ODBC3_80, CONNECTION_POOLING => ConnectionPooling);

macro_rules! impl_env_scalar_attr_unpack {
    ($($value:ty => $repr:ty),+ $(,)?) => {$(
        impl co3::slice::Unpack2<
            <$crate::data::POINTER as co3::ExternC>::CType,
            $crate::data::INTEGER,
        > for $value {
            type Error = core::convert::Infallible;

            fn unpack(value: Self::CType) -> Result<(
                <$crate::data::POINTER as co3::ExternC>::CType,
                $crate::data::INTEGER,
            ), Self::Error> {
                const {
                    assert!(core::mem::size_of::<<$value as co3::ExternC>::CType>()
                        == core::mem::size_of::<$repr>());
                }
                let value: $repr = unsafe { core::mem::transmute_copy(&value) };
                Ok((co3::encode($crate::data::POINTER::from(value)), $crate::data::INTEGER::new(0)))
            }
        }
    )+};
}

impl_env_scalar_attr_unpack!(
    VersionValue => u32,
    CpMatch => u32,
    ConnectionPooling => u32,
);

//=====================================================================================//
//-------------------------------------Attributes--------------------------------------//

// TODO: Consider using const generics for OdbcVersion once it's available on stable,
// otherwise don't expose this attribute unless there is a valid use-case
//=====================================================================================//

#[sealed::sealed]
pub trait OdbcVersion {
    const ID: u32;
}
#[derive(Debug)]
#[expect(non_camel_case_types)]
pub enum OV_ODBC3 {}
#[sealed::sealed]
impl OdbcVersion for OV_ODBC3 {
    const ID: u32 = 3;
}
#[derive(Debug)]
#[expect(non_camel_case_types)]
pub enum OV_ODBC3_80 {}
#[sealed::sealed]
impl OdbcVersion for OV_ODBC3_80 {
    const ID: u32 = 380;
}
#[derive(Debug)]
#[expect(non_camel_case_types)]
pub enum OV_ODBC4 {}
#[sealed::sealed]
impl OdbcVersion for OV_ODBC4 {
    const ID: u32 = 400;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, rust_spec::RustSpec, co3::ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum CpMatch {
    CP_STRICT_MATCH,
    CP_RELAXED_MATCH,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, rust_spec::RustSpec, co3::ReprC)]
#[expect(non_camel_case_types)]
#[repr(u32)]
pub enum ConnectionPooling {
    CP_OFF,
    CP_ONE_PER_DRIVER,
    CP_ONE_PER_HENV,
    CP_DRIVER_AWARE,
}

/// Selects where `SQLDataSources` starts or continues enumerating data sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rust_spec::RustSpec, co3::ReprC)]
#[expect(non_camel_case_types)]
#[repr(u16)]
pub enum DataSourceDirection {
    FETCH_NEXT = 1,
    FETCH_FIRST = 2,
    FETCH_FIRST_USER = 31,
    FETCH_FIRST_SYSTEM = 32,
}
