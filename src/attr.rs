use core::{cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit};

use co3::{ExternC, ReprC, slice::Unpack2};
use rust_spec::RustSpec;

use crate::{
    Definition, DriverDefined, OdbcDefined,
    data::{CHAR, INTEGER, LEN, POINTER, SMALLINT, WCHAR},
    str::OdbcStr,
};

macro_rules! define_i32_attributes {
    ($($visibility:vis $name:ident = $id:expr;)+) => {$(
        #[derive(co3::Tag)]
        #[tag(i32, unsafe($id))]
        #[allow(non_camel_case_types)]
        $visibility enum $name {}
    )+};
}

define_i32_attributes! {
    pub(crate) ODBC_VERSION = 200;
    pub CP_MATCH = 202;
    pub CONNECTION_POOLING = 201;
    pub ACCESS_MODE = 101;
    pub AUTOCOMMIT = 102;
    pub CONNECTION_TIMEOUT = 113;
    pub CURRENT_CATALOG = 109;
    pub LOGIN_TIMEOUT = 103;
    pub PACKET_SIZE = 112;
    pub TRACE = 104;
    pub TRACEFILE = 105;
    pub TRANSLATE_LIB = 106;
    pub TRANSLATE_OPTION = 107;
    pub TXN_ISOLATION = 108;
    pub AUTO_IPD = 10001;
    pub ASYNC_DBC_FUNCTIONS_ENABLE = 117;
    pub CONNECTION_DEAD = 1209;
    pub REFRESH_CONNECTION = 123;
    // pub CREDENTIALS = 122;

    pub QUERY_TIMEOUT = 0;
    pub MAX_ROWS = 1;
    pub NOSCAN = 2;
    pub MAX_LENGTH = 3;
    pub ASYNC_ENABLE = 4;
    // pub ROW_BIND_TYPE = 5;
    pub CURSOR_TYPE = 6;
    pub CONCURRENCY = 7;
    pub KEYSET_SIZE = 8;
    pub SIMULATE_CURSOR = 10;
    pub RETRIEVE_DATA = 11;
    pub USE_BOOKMARKS = 12;
    pub ROW_NUMBER = 14;
    pub ENABLE_AUTO_IPD = 15;
    // pub PARAM_BIND_TYPE = 18;
    // pub PARAMSET_SIZE = 22;
    // pub ROW_ARRAY_SIZE = 27;
    // pub DYNAMIC_COLUMNS = 31;
    // pub TYPE_EXCEPTION_BEHAVIOR = 32;
    // pub LENGTH_EXCEPTION_BEHAVIOR = 33;
    pub CURSOR_SCROLLABLE = -1;
    pub CURSOR_SENSITIVITY = -2;
    pub APP_ROW_DESC = 10010;
    pub APP_PARAM_DESC = 10011;
    pub IMP_ROW_DESC = 10012;
    pub IMP_PARAM_DESC = 10013;
    pub METADATA_ID = 10014;
}

macro_rules! impl_attr {
    (Env, set, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        #[sealed::sealed]
        unsafe impl EnvAttrSet<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar + 'a> = $value;
        }
    };
    (Env, get set, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        #[sealed::sealed]
        unsafe impl EnvAttrGet<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = core::mem::MaybeUninit<$value>;
        }
        #[sealed::sealed]
        unsafe impl EnvAttrSet<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar + 'a> = $value;
        }
    };
    (Env, set, $($rest:tt)*) => {
        impl_attr!(Env, set EnvAttrSet, $($rest)*);
    };
    (Env, get set, $($rest:tt)*) => {
        impl_attr!(Env, get EnvAttrGet set EnvAttrSet, $($rest)*);
    };
    (Conn, get, $($rest:tt)*) => {
        impl_attr!(Conn, get ConnAttrGet, $($rest)*);
    };
    (Conn, get set, $($rest:tt)*) => {
        impl_attr!(Conn, get ConnAttrGet set ConnAttrSet, $($rest)*);
    };
    (Stmt, get, $($rest:tt)*) => {
        impl_attr!(Stmt, get StmtAttrGet, $($rest)*);
    };
    (Stmt, get set, $($rest:tt)*) => {
        impl_attr!(Stmt, get StmtAttrGet set StmtAttrSet, $($rest)*);
    };
    (Col, get, $version:ty, $attr:ty => &'a OdbcStr<C>) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl ColAttrGet<$version> for $attr {
            type CharacterBuffer<C: $crate::str::OdbcChar> =
                $crate::str::OdbcStr<core::mem::MaybeUninit<C>>;
            type NumericBuffer = $crate::col::Unavailable<$crate::data::LEN>;
        }
    };
    (Col, get, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl ColAttrGet<$version> for $attr {
            type CharacterBuffer<C: $crate::str::OdbcChar> = $crate::col::Unavailable<C>;
            type NumericBuffer = core::mem::MaybeUninit<$value>;
        }
    };
    ($kind:ident, get $get_kind:ident, $version:ty, $attr:ty => &'a OdbcStr<C>) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $get_kind<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = $crate::str::OdbcStr<core::mem::MaybeUninit<C>>;
        }
    };
    ($kind:ident, get $get_kind:ident set $set_kind:ident, $version:ty, $attr:ty => &'a OdbcStr<C>) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $get_kind<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = $crate::str::OdbcStr<core::mem::MaybeUninit<C>>;
        }
        unsafe impl $set_kind<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar>
                = &'a $crate::str::OdbcStr<C>
            where
                C: 'a;
        }
    };
    ($kind:ident, get $get_kind:ident, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $get_kind<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = core::mem::MaybeUninit<$value>;
        }
    };
    ($kind:ident, set $set_kind:ident, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $set_kind<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar>
                = $value
            where
                C: 'a;
        }
    };
    ($kind:ident, get $get_kind:ident set $set_kind:ident, $version:ty, $attr:ty => $value:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $get_kind<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = core::mem::MaybeUninit<$value>;
        }
        unsafe impl $set_kind<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar>
                = $value
            where
                C: 'a;
        }
    };
    ($kind:ident, get $get_kind:ident set $set_kind:ident, $version:ty, $attr:ty => get $get:ty, set $set:ty) => {
        impl $crate::Defined for $attr {
            type By = $crate::OdbcDefined;
        }
        unsafe impl $get_kind<$version> for $attr {
            type Buffer<C: $crate::str::OdbcChar> = core::mem::MaybeUninit<$get>;
        }
        unsafe impl $set_kind<$version> for $attr {
            type Value<'a, C: $crate::str::OdbcChar>
                = $set
            where
                C: 'a;
        }
    };
}
pub(crate) use impl_attr;

macro_rules! inherit_attr {
    (get $trait:ident, $from:ty => $to:ty) => {
        unsafe impl<T: $trait<$from>> $trait<$to> for T {
            type Buffer<C: $crate::str::OdbcChar> = <T as $trait<$from>>::Buffer<C>;
        }
    };
    (get $trait:ident, $from:ty => $to:ty, state) => {
        unsafe impl<T, S: $crate::conn::ConnState> $trait<$to, S> for T
        where
            T: $trait<$from, S>,
        {
            type Buffer<C: $crate::str::OdbcChar> = <T as $trait<$from, S>>::Buffer<C>;
        }
    };
    (get $trait:ident, $from:ty => $to:ty, desc) => {
        unsafe impl<T, DT: $crate::desc::DescType> $trait<$to, DT> for T
        where
            T: $trait<$from, DT>,
        {
            type Buffer<C: $crate::str::OdbcChar> = <T as $trait<$from, DT>>::Buffer<C>;
        }
    };
    (get $trait:ident, $from:ty => $to:ty, col) => {
        unsafe impl<T: $trait<$from>> $trait<$to> for T {
            type CharacterBuffer<C: $crate::str::OdbcChar> =
                <T as $trait<$from>>::CharacterBuffer<C>;
            type NumericBuffer = <T as $trait<$from>>::NumericBuffer;
        }
    };
    (set $trait:ident, $from:ty => $to:ty) => {
        unsafe impl<T: $trait<$from>> $trait<$to> for T {
            type Value<'a, C: $crate::str::OdbcChar + 'a> = <T as $trait<$from>>::Value<'a, C>;
        }
    };
    (set $trait:ident, $from:ty => $to:ty, state) => {
        unsafe impl<T, S: $crate::conn::ConnState> $trait<$to, S> for T
        where
            T: $trait<$from, S>,
        {
            type Value<'a, C: $crate::str::OdbcChar + 'a> = <T as $trait<$from, S>>::Value<'a, C>;
        }
    };
    (set $trait:ident, $from:ty => $to:ty, desc) => {
        unsafe impl<T, DT: $crate::desc::DescType> $trait<$to, DT> for T
        where
            T: $trait<$from, DT>,
        {
            type Value<C: $crate::str::OdbcChar> = <T as $trait<$from, DT>>::Value<C>;
        }
    };
}
pub(crate) use inherit_attr;

#[derive(RustSpec, ReprC)]
#[reprC(identity)]
#[repr(transparent)]
pub struct AttrLen<D: Definition, L>(L, PhantomData<fn() -> D>);

impl<D: Definition, L: Copy> Copy for AttrLen<D, L> {}

impl<D: Definition, L: Clone> Clone for AttrLen<D, L> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<D: Definition, L> AttrLen<D, L> {
    pub(crate) const fn new(length: L) -> Self {
        Self(length, PhantomData)
    }
}

/// `StringLength`/`BufferLength` markers for driver-defined attributes and fields.
pub const IS_POINTER: i32 = -4;
pub const IS_UINTEGER: i32 = -5;
pub const IS_INTEGER: i32 = -6;
pub const IS_USMALLINT: i32 = -7;
pub const IS_SMALLINT: i32 = -8;

const LEN_BINARY_ATTR_OFFSET: i32 = -100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrLenError {
    Overflow,
    TooLarge(core::num::TryFromIntError),
}

macro_rules! impl_string_attr_unpack {
    ($defined_by:ty, $character:ty, $length:ty) => {
        impl<'a>
            Unpack2<<POINTER as ExternC>::CType, <AttrLen<$defined_by, $length> as ExternC>::CType>
            for &'a OdbcStr<$character>
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<$defined_by, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, elements) =
                    <&[$character] as Unpack2<*const $character, usize>>::unpack(value).unwrap();
                let bytes = elements
                    .checked_mul(core::mem::size_of::<$character>())
                    .ok_or(AttrLenError::Overflow)?;
                let bytes = <$length>::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(bytes)),
                ))
            }
        }

        impl<'a>
            Unpack2<<POINTER as ExternC>::CType, <AttrLen<$defined_by, $length> as ExternC>::CType>
            for &'a mut OdbcStr<MaybeUninit<$character>>
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<$defined_by, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, elements) = <&mut [MaybeUninit<$character>] as Unpack2<
                    *mut MaybeUninit<$character>,
                    usize,
                >>::unpack(value)
                .unwrap();
                let bytes = elements
                    .checked_mul(core::mem::size_of::<$character>())
                    .ok_or(AttrLenError::Overflow)?;
                let bytes = <$length>::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(bytes)),
                ))
            }
        }
    };
}

impl_string_attr_unpack!(OdbcDefined, CHAR, i32);
impl_string_attr_unpack!(OdbcDefined, WCHAR, i32);
impl_string_attr_unpack!(DriverDefined, CHAR, i32);
impl_string_attr_unpack!(DriverDefined, WCHAR, i32);
impl_string_attr_unpack!(OdbcDefined, CHAR, i16);
impl_string_attr_unpack!(OdbcDefined, WCHAR, i16);
impl_string_attr_unpack!(DriverDefined, CHAR, i16);
impl_string_attr_unpack!(DriverDefined, WCHAR, i16);

macro_rules! impl_odbc_binary_attr_unpack {
    ($length:ty) => {
        impl<'a>
            Unpack2<<POINTER as ExternC>::CType, <AttrLen<OdbcDefined, $length> as ExternC>::CType>
            for &'a [u8]
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<OdbcDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, bytes) = <&[u8] as Unpack2<*const u8, usize>>::unpack(value).unwrap();
                let bytes = <$length>::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(bytes)),
                ))
            }
        }

        impl<'a>
            Unpack2<<POINTER as ExternC>::CType, <AttrLen<OdbcDefined, $length> as ExternC>::CType>
            for &'a mut [MaybeUninit<u8>]
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<OdbcDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, bytes) =
                    <&mut [MaybeUninit<u8>] as Unpack2<*mut MaybeUninit<u8>, usize>>::unpack(value)
                        .unwrap();
                let bytes = <$length>::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(bytes)),
                ))
            }
        }
    };
}

macro_rules! impl_driver_binary_attr_unpack {
    ($length:ty) => {
        impl<'a>
            Unpack2<
                <POINTER as ExternC>::CType,
                <AttrLen<DriverDefined, $length> as ExternC>::CType,
            > for &'a [u8]
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<DriverDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, bytes) = <&[u8] as Unpack2<*const u8, usize>>::unpack(value).unwrap();
                let bytes = i32::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                let length = bytes
                    .checked_neg()
                    .and_then(|bytes| bytes.checked_add(LEN_BINARY_ATTR_OFFSET))
                    .ok_or(AttrLenError::Overflow)?;
                let length = <$length>::try_from(length).map_err(|_| AttrLenError::Overflow)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(length)),
                ))
            }
        }

        impl<'a>
            Unpack2<
                <POINTER as ExternC>::CType,
                <AttrLen<DriverDefined, $length> as ExternC>::CType,
            > for &'a mut [MaybeUninit<u8>]
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<DriverDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                let (data, bytes) =
                    <&mut [MaybeUninit<u8>] as Unpack2<*mut MaybeUninit<u8>, usize>>::unpack(value)
                        .unwrap();
                let bytes = i32::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                let length = bytes
                    .checked_neg()
                    .and_then(|bytes| bytes.checked_add(LEN_BINARY_ATTR_OFFSET))
                    .ok_or(AttrLenError::Overflow)?;
                let length = <$length>::try_from(length).map_err(|_| AttrLenError::Overflow)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(AttrLen::new(length)),
                ))
            }
        }
    };
}

impl_odbc_binary_attr_unpack!(i32);
impl_driver_binary_attr_unpack!(i32);
impl_odbc_binary_attr_unpack!(i16);
impl_driver_binary_attr_unpack!(i16);

macro_rules! impl_scalar_output_attr_unpack {
    ($length:ty) => {
        impl<'a, T>
            Unpack2<<POINTER as ExternC>::CType, <AttrLen<OdbcDefined, $length> as ExternC>::CType>
            for &'a mut MaybeUninit<T>
        where
            Self: ExternC<CType: Sized>,
            POINTER: From<Self::CType>,
        {
            type Error = core::convert::Infallible;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<OdbcDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                Ok((
                    co3::encode(POINTER::from(value)),
                    co3::encode(AttrLen::new(0 as $length)),
                ))
            }
        }
    };
}

impl_scalar_output_attr_unpack!(i32);
impl_scalar_output_attr_unpack!(i16);

macro_rules! impl_driver_scalar_attr_unpack {
    ($value:ty, $marker:expr, $length:ty) => {
        impl
            Unpack2<
                <POINTER as ExternC>::CType,
                <AttrLen<DriverDefined, $length> as ExternC>::CType,
            > for $value
        {
            type Error = core::convert::Infallible;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<DriverDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                Ok((
                    co3::encode(POINTER::from(value)),
                    co3::encode(AttrLen::new($marker as $length)),
                ))
            }
        }

        impl<'a>
            Unpack2<
                <POINTER as ExternC>::CType,
                <AttrLen<DriverDefined, $length> as ExternC>::CType,
            > for &'a mut MaybeUninit<$value>
        {
            type Error = core::convert::Infallible;

            fn unpack(
                value: Self::CType,
            ) -> Result<
                (
                    <POINTER as ExternC>::CType,
                    <AttrLen<DriverDefined, $length> as ExternC>::CType,
                ),
                Self::Error,
            > {
                Ok((
                    co3::encode(POINTER::from(value)),
                    co3::encode(AttrLen::new($marker as $length)),
                ))
            }
        }
    };
}

impl_driver_scalar_attr_unpack!(i32, IS_INTEGER, i32);
impl_driver_scalar_attr_unpack!(u32, IS_UINTEGER, i32);
impl_driver_scalar_attr_unpack!(i16, IS_SMALLINT, i32);
impl_driver_scalar_attr_unpack!(u16, IS_USMALLINT, i32);
impl_driver_scalar_attr_unpack!(i32, IS_INTEGER, i16);
impl_driver_scalar_attr_unpack!(u32, IS_UINTEGER, i16);
impl_driver_scalar_attr_unpack!(i16, IS_SMALLINT, i16);
impl_driver_scalar_attr_unpack!(u16, IS_USMALLINT, i16);

macro_rules! impl_info_string_pointer_unpack {
    ($character:ty) => {
        impl<'a> Unpack2<<POINTER as ExternC>::CType, SMALLINT>
            for &'a mut OdbcStr<MaybeUninit<$character>>
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<(<POINTER as ExternC>::CType, SMALLINT), Self::Error> {
                let (data, elements) = <&mut [MaybeUninit<$character>] as Unpack2<
                    *mut MaybeUninit<$character>,
                    usize,
                >>::unpack(value)
                .unwrap();
                let bytes = elements
                    .checked_mul(core::mem::size_of::<$character>())
                    .ok_or(AttrLenError::Overflow)?;
                let bytes = i16::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((co3::encode(POINTER::from(data)), SMALLINT::new(bytes)))
            }
        }
    };
}

impl_info_string_pointer_unpack!(CHAR);
impl_info_string_pointer_unpack!(WCHAR);

macro_rules! impl_bind_col_string_unpack {
    ($character:ty) => {
        impl<'a> Unpack2<<POINTER as ExternC>::CType, <LEN as ExternC>::CType>
            for &'a UnsafeCell<OdbcStr<MaybeUninit<$character>>>
        {
            type Error = AttrLenError;

            fn unpack(
                value: Self::CType,
            ) -> Result<(<POINTER as ExternC>::CType, <LEN as ExternC>::CType), Self::Error> {
                let (data, elements) = <&UnsafeCell<OdbcStr<MaybeUninit<$character>>> as Unpack2<
                    *mut MaybeUninit<$character>,
                    usize,
                >>::unpack(value)
                .unwrap();
                let bytes = elements
                    .checked_mul(core::mem::size_of::<$character>())
                    .ok_or(AttrLenError::Overflow)?;
                let bytes = isize::try_from(bytes).map_err(AttrLenError::TooLarge)?;
                Ok((
                    co3::encode(POINTER::from(data)),
                    co3::encode(LEN::new(bytes)),
                ))
            }
        }
    };
}

impl_bind_col_string_unpack!(CHAR);
impl_bind_col_string_unpack!(WCHAR);

impl<T> Unpack2<<POINTER as ExternC>::CType, <LEN as ExternC>::CType>
    for &UnsafeCell<MaybeUninit<T>>
where
    Self: ExternC<CType: Sized>,
    POINTER: From<Self::CType>,
{
    type Error = core::convert::Infallible;

    fn unpack(
        value: Self::CType,
    ) -> Result<(<POINTER as ExternC>::CType, <LEN as ExternC>::CType), Self::Error> {
        Ok((co3::encode(POINTER::from(value)), co3::encode(LEN::new(0))))
    }
}

impl<T> Unpack2<<POINTER as ExternC>::CType, <AttrLen<OdbcDefined, i32> as ExternC>::CType>
    for &UnsafeCell<MaybeUninit<T>>
where
    Self: ExternC<CType: Sized>,
    POINTER: From<Self::CType>,
{
    type Error = core::convert::Infallible;

    fn unpack(
        value: Self::CType,
    ) -> Result<
        (
            <POINTER as ExternC>::CType,
            <AttrLen<OdbcDefined, i32> as ExternC>::CType,
        ),
        Self::Error,
    > {
        Ok((
            co3::encode(POINTER::from(value)),
            co3::encode(AttrLen::new(IS_POINTER)),
        ))
    }
}

impl<T> Unpack2<<POINTER as ExternC>::CType, SMALLINT> for &mut MaybeUninit<T>
where
    Self: ExternC<CType: Sized>,
    POINTER: From<Self::CType>,
{
    type Error = core::convert::Infallible;

    fn unpack(value: Self::CType) -> Result<(<POINTER as ExternC>::CType, SMALLINT), Self::Error> {
        Ok((co3::encode(POINTER::from(value)), SMALLINT::new(0)))
    }
}

macro_rules! impl_odbc_scalar_attr_unpack {
    ($($value:ty => $repr:ty),+ $(,)?) => {$(
        impl co3::slice::Unpack2<
            <$crate::data::POINTER as co3::ExternC>::CType,
            <$crate::attr::AttrLen<$crate::OdbcDefined, i32>
                as co3::ExternC>::CType,
        > for $value {
            type Error = core::convert::Infallible;

            fn unpack(value: Self::CType) -> Result<(
                <$crate::data::POINTER as co3::ExternC>::CType,
                <$crate::attr::AttrLen<$crate::OdbcDefined, i32>
                    as co3::ExternC>::CType,
            ), Self::Error> {
                const {
                    assert!(core::mem::size_of::<<$value as co3::ExternC>::CType>()
                        == core::mem::size_of::<$repr>());
                }
                let value: $repr = unsafe { core::mem::transmute_copy(&value) };
                Ok((
                    co3::encode($crate::data::POINTER::from(value)),
                    co3::encode($crate::attr::AttrLen::new(0)),
                ))
            }

        }

    )+};
}
pub(crate) use impl_odbc_scalar_attr_unpack;

impl<T> Unpack2<<POINTER as ExternC>::CType, <INTEGER as ExternC>::CType> for &mut MaybeUninit<T>
where
    Self: ExternC<CType: Sized>,
    POINTER: From<Self::CType>,
{
    type Error = core::convert::Infallible;

    fn unpack(value: Self::CType) -> Result<(<POINTER as ExternC>::CType, INTEGER), Self::Error> {
        Ok((co3::encode(POINTER::from(value)), INTEGER::new(0)))
    }
}

macro_rules! impl_odbc_primitive_attr_unpack {
    ($($value:ty),+ $(,)?) => {$(
        impl co3::slice::Unpack2<
            <$crate::data::POINTER as co3::ExternC>::CType,
            <$crate::attr::AttrLen<$crate::OdbcDefined, i32>
                as co3::ExternC>::CType,
        > for $value {
            type Error = core::convert::Infallible;

            fn unpack(value: Self::CType) -> Result<(
                <$crate::data::POINTER as co3::ExternC>::CType,
                <$crate::attr::AttrLen<$crate::OdbcDefined, i32>
                    as co3::ExternC>::CType,
            ), Self::Error> {
                Ok((
                    co3::encode($crate::data::POINTER::from(value)),
                    co3::encode($crate::attr::AttrLen::new(0)),
                ))
            }

        }

    )+};
}

impl_odbc_primitive_attr_unpack!(u32, usize);

#[cfg(test)]
mod unpack_tests {
    use super::*;

    type PointerPart = <POINTER as ExternC>::CType;
    type OdbcIntegerLength = <AttrLen<OdbcDefined, i32> as ExternC>::CType;
    type DriverIntegerLength = <AttrLen<DriverDefined, i32> as ExternC>::CType;
    type DriverSmallIntLength = <AttrLen<DriverDefined, i16> as ExternC>::CType;

    fn odbc_length(value: OdbcIntegerLength) -> i32 {
        unsafe { co3::decode::<AttrLen<OdbcDefined, i32>>(value) }
            .expect("AttrLen has no invalid representation")
            .0
    }

    fn driver_length(value: DriverIntegerLength) -> i32 {
        unsafe { co3::decode::<AttrLen<DriverDefined, i32>>(value) }
            .expect("AttrLen has no invalid representation")
            .0
    }

    fn driver_smallint_length(value: DriverSmallIntLength) -> i16 {
        unsafe { co3::decode::<AttrLen<DriverDefined, i16>>(value) }
            .expect("AttrLen has no invalid representation")
            .0
    }

    #[test]
    fn driver_fixed_values_use_their_type_markers() {
        let (_, length) = <i32 as Unpack2<PointerPart, DriverIntegerLength>>::unpack(1).unwrap();
        assert_eq!(driver_length(length), IS_INTEGER);

        let (_, length) = <u32 as Unpack2<PointerPart, DriverIntegerLength>>::unpack(1).unwrap();
        assert_eq!(driver_length(length), IS_UINTEGER);

        let (_, length) = <i16 as Unpack2<PointerPart, DriverIntegerLength>>::unpack(1).unwrap();
        assert_eq!(driver_length(length), IS_SMALLINT);

        let (_, length) = <u16 as Unpack2<PointerPart, DriverIntegerLength>>::unpack(1).unwrap();
        assert_eq!(driver_length(length), IS_USMALLINT);
    }

    #[test]
    fn driver_fixed_output_buffers_use_their_type_markers() {
        let mut value = MaybeUninit::new(u32::default());
        let encoded = co3::encode(Some(&mut value));
        let (_, length) = <Option<&mut MaybeUninit<u32>> as Unpack2<
            PointerPart,
            DriverIntegerLength,
        >>::unpack(encoded)
        .unwrap();

        assert_eq!(driver_length(length), IS_UINTEGER);
    }

    #[test]
    fn driver_column_output_buffers_use_smallint_type_markers() {
        let mut value = MaybeUninit::new(u32::default());
        let encoded = co3::encode(Some(&mut value));
        let (_, length) = <Option<&mut MaybeUninit<u32>> as Unpack2<
            PointerPart,
            DriverSmallIntLength,
        >>::unpack(encoded)
        .unwrap();

        assert_eq!(driver_smallint_length(length), IS_UINTEGER as i16);
    }

    #[test]
    fn driver_binary_lengths_use_sql_len_binary_attr() {
        let bytes = [1_u8, 2, 3];
        let encoded = co3::encode(bytes.as_slice());
        let (_, length) =
            <&[CHAR] as Unpack2<PointerPart, DriverIntegerLength>>::unpack(encoded).unwrap();

        assert_eq!(driver_length(length), -103);
    }

    #[test]
    fn odbc_binary_lengths_are_positive_byte_counts() {
        let bytes = [1_u8, 2, 3];
        let encoded = co3::encode(bytes.as_slice());
        let (_, length) =
            <&[CHAR] as Unpack2<PointerPart, OdbcIntegerLength>>::unpack(encoded).unwrap();

        assert_eq!(odbc_length(length), 3);
    }

    #[test]
    fn wide_string_lengths_are_measured_in_bytes() {
        let characters = [1_u16, 2, 3];
        let value: &OdbcStr<WCHAR> = characters.as_slice().as_ref();
        let encoded = co3::encode(value);
        let (_, length) =
            <&OdbcStr<WCHAR> as Unpack2<PointerPart, DriverIntegerLength>>::unpack(encoded)
                .unwrap();

        assert_eq!(driver_length(length), 6);
    }
}
