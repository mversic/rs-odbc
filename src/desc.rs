use crate::{Attr, AttrWrite, Ident, SQLHDESC, SQLINTEGER, IntoSQLPOINTER, AttrLen};

pub trait WriteDescField<DT, A: Ident>: Attr<A> + AttrLen<<Self as Attr<A>>::DefinedBy, <Self as Attr<A>>::NonBinary, SQLINTEGER> + IntoSQLPOINTER {
    // TODO: Implement for buffers to bind their lifetimes
    fn update_handle(&self, _: &SQLHDESC<DT>) {}
}

pub trait DescField<A: crate::Ident>: Attr<A> + AttrLen<<Self as Attr<A>>::DefinedBy, <Self as Attr<A>>::NonBinary, SQLINTEGER> {}

//    pub enum DescFieldIdentifier {
//        // Header fields
//        SQL_DESC_ALLOC_TYPE = 1099,
//        SQL_DESC_ARRAY_SIZE = 20,
//        SQL_DESC_ARRAY_STATUS_PTR = 21,
//        SQL_DESC_BIND_OFFSET_PTR = 24,
//        SQL_DESC_BIND_TYPE = 25,
//        SQL_DESC_ROWS_PROCESSED_PTR = 34,
//
//        // Record fields
//        SQL_DESC_DATA_PTR = 1010,
//        SQL_DESC_DATETIME_INTERVAL_CODE = 1007,
//        SQL_DESC_DATETIME_INTERVAL_PRECISION = 26,
//        SQL_DESC_INDICATOR_PTR = 1009,
//        SQL_DESC_OCTET_LENGTH_PTR = 1004,
//        SQL_DESC_PARAMETER_TYPE = 33,
//        #[cfg(feature = "v3_5")]
//        SQL_DESC_ROWVER = 35,
//        nQL_DESC_UNNAMED = 1012,
//
//        // TODO: Not mentioned anywhere in the documentation
//        // SQL_DESC_MAXIMUM_SCALE = 30,
//        // SQL_DESC_MINIMUM_SCALE = 31,
//        #[cfg(feature = "v4")]
//        SQL_DESC_CHARACTER_SET_CATALOG = 1018,
//        #[cfg(feature = "v4")]
//        SQL_DESC_CHARACTER_SET_SCHEMA = 1019,
//        #[cfg(feature = "v4")]
//        SQL_DESC_CHARACTER_SET_NAME = 1020,
//        #[cfg(feature = "v4")]
//        SQL_DESC_COLLATION_CATALOG = 1015,
//        #[cfg(feature = "v4")]
//        SQL_DESC_COLLATION_SCHEMA = 1016,
//        #[cfg(feature = "v4")]
//        SQL_DESC_COLLATION_NAME = 1017,
//        #[cfg(feature = "v4")]
//        SQL_DESC_USER_DEFINED_TYPE_CATALOG = 1026,
//        #[cfg(feature = "v4")]
//        SQL_DESC_USER_DEFINED_TYPE_SCHEMA = 1027,
//        #[cfg(feature = "v4")]
//        SQL_DESC_USER_DEFINED_TYPE_NAME = 1028,
//        #[cfg(feature = "v4")]
//        SQL_DESC_MIME_TYPE = 36,
//    }
//
//    pub enum SQL_DESC_ALLOC_TYPE {
//        SQL_DESC_ALLOC_AUTO = 1,
//        SQL_DESC_ALLOC_USER = 2,
//    }
//
//    pub enum SQL_DESC_ARRAY_STATUS_PTR {
//        SQL_PARAM_SUCCESS = 0,
//        SQL_PARAM_SUCCESS_WITH_INFO = 6,
//        SQL_PARAM_ERROR = 5,
//        SQL_PARAM_UNUSED = 7,
//        SQL_PARAM_DIAG_UNAVAILABLE = 1,
//        // TODO: What are these?
//        //SQL_PARAM_PROCEED = 0,
//        //SQL_PARAM_IGNORE = 1,
//    }

//impl<'a, H: Handle, D: Ident> DescField<H, D> for &'a [SQLWCHAR]
//where
//    &'a [SQLCHAR]: DescField<H, D>,
//    // TODO: This seems superflous
//    &'a [SQLWCHAR]: AttrLen<<Self as Attr<D>>::DefinedBy, <Self as Attr<D>>::NonBinary, SQLSMALLINT>,
//{
//}
//
//impl<H: Handle, D: Ident, T: Ident> DescField<H, D> for std::mem::MaybeUninit<T>
//where
//    T: DescField<H, D>,
//    Self: Attr<D> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
//{
//}
//impl<'a, H: Handle, D: Ident, T> DescField<H, D> for &'a [std::mem::MaybeUninit<T>]
//where
//    &'a [T]: DescField<H, D>,
//    Self: Attr<D> + AttrLen<Self::DefinedBy, Self::NonBinary, SQLSMALLINT>,
//{
//}
