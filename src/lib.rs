#![cfg_attr(not(feature = "std"), no_std)]

use core::mem::MaybeUninit;

pub mod attr;
pub mod c_type;
pub mod col;
pub mod conn;
pub mod data;
pub mod desc;
pub mod diag;
pub mod env;
pub mod handle;
pub mod info;
pub mod sql_type;
pub mod sqlreturn;
pub mod stmt;
pub mod str;

pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: usize = 250;
pub const SQL_API_ALL_FUNCTIONS_SIZE: usize = 100;

/// Identifies an ODBC API function handle and its `SQLGetFunctions` result type.
///
/// This trait is sealed because the supported function-handle set is defined
/// by this crate.
#[sealed::sealed]
pub trait Function {
    type Supported;
}

/// Identifies who defines an ODBC property and, consequently, how its value is encoded.
///
/// This trait is sealed because the set of supported encoding policies is closed.
#[sealed::sealed]
pub trait Definition {}
pub enum OdbcDefined {}
pub enum DriverDefined {}

/// Classifies an identifier as ODBC-defined or driver-defined.
pub trait Defined {
    type By: Definition;
}
#[sealed::sealed]
impl Definition for OdbcDefined {}
#[sealed::sealed]
impl Definition for DriverDefined {}

// /// Specifies how many active connections a particular driver supports.
//#define SQL_MAX_DRIVER_CONNECTIONS          0
//#define SQL_MAXIMUM_DRIVER_CONNECTIONS      SQL_MAX_DRIVER_CONNECTIONS
///// Some drivers limit the number of active statements they support; the SQL_MAX_CONCURRENT_ACTIVITIES option in SQLGetInfo specifies how many active statements a driver supports on a single connection.
//#define SQL_MAX_CONCURRENT_ACTIVITIES       1
//#define SQL_MAXIMUM_CONCURRENT_ACTIVITIES   SQL_MAX_CONCURRENT_ACTIVITIES

// TODO: and what about CHAR vs WCHAR?
pub const ALL_CATALOGS: &str = "%";
pub const ALL_SCHEMAS: &str = "%";
pub const ALL_TABLE_TYPES: &str = "%";

macro_rules! function_handle {
    ($name:ident = $id:expr) => {
        #[derive(co3::Tag)]
        #[tag(u16, unsafe($id))]
        #[expect(non_camel_case_types)]
        pub enum $name {}

        #[sealed::sealed]
        impl Function for $name {
            type Supported = MaybeUninit<u16>;
        }
    };
    ($name:ident = $id:expr, all($size:expr)) => {
        #[derive(co3::Tag)]
        #[tag(u16, unsafe($id))]
        #[expect(non_camel_case_types)]
        pub enum $name {}

        #[sealed::sealed]
        impl Function for $name {
            type Supported = [MaybeUninit<u16>; $size];
        }
    };
}

function_handle!(API_ALL_FUNCTIONS = 0, all(SQL_API_ALL_FUNCTIONS_SIZE));
function_handle!(
    API_ODBC3_ALL_FUNCTIONS = 999,
    all(SQL_API_ODBC3_ALL_FUNCTIONS_SIZE)
);
function_handle!(API_SQLALLOCCONNECT = 1);
function_handle!(API_SQLALLOCENV = 2);
function_handle!(API_SQLALLOCHANDLE = 1001);
function_handle!(API_SQLALLOCSTMT = 3);
function_handle!(API_SQLBINDCOL = 4);
function_handle!(API_SQLBINDPARAM = 1002);
function_handle!(API_SQLCANCEL = 5);
function_handle!(API_SQLCLOSECURSOR = 1003);
function_handle!(API_SQLCOLATTRIBUTE = 6);
function_handle!(API_SQLCOLUMNS = 40);
function_handle!(API_SQLCONNECT = 7);
function_handle!(API_SQLCOPYDESC = 1004);
function_handle!(API_SQLDATASOURCES = 57);
function_handle!(API_SQLDESCRIBECOL = 8);
function_handle!(API_SQLDISCONNECT = 9);
function_handle!(API_SQLENDTRAN = 1005);
function_handle!(API_SQLERROR = 10);
function_handle!(API_SQLEXECDIRECT = 11);
function_handle!(API_SQLEXECUTE = 12);
function_handle!(API_SQLFETCH = 13);
function_handle!(API_SQLFETCHSCROLL = 1021);
function_handle!(API_SQLFREECONNECT = 14);
function_handle!(API_SQLFREEENV = 15);
function_handle!(API_SQLFREEHANDLE = 1006);
function_handle!(API_SQLFREESTMT = 16);
function_handle!(API_SQLGETCONNECTATTR = 1007);
function_handle!(API_SQLGETCONNECTOPTION = 42);
function_handle!(API_SQLGETCURSORNAME = 17);
function_handle!(API_SQLGETDATA = 43);
function_handle!(API_SQLGETDESCFIELD = 1008);
function_handle!(API_SQLGETDESCREC = 1009);
function_handle!(API_SQLGETDIAGFIELD = 1010);
function_handle!(API_SQLGETDIAGREC = 1011);
function_handle!(API_SQLGETENVATTR = 1012);
function_handle!(API_SQLGETFUNCTIONS = 44);
function_handle!(API_SQLGETINFO = 45);
function_handle!(API_SQLGETSTMTATTR = 1014);
function_handle!(API_SQLGETSTMTOPTION = 46);
function_handle!(API_SQLGETTYPEINFO = 47);
function_handle!(API_SQLNUMRESULTCOLS = 18);
function_handle!(API_SQLPARAMDATA = 48);
function_handle!(API_SQLPREPARE = 19);
function_handle!(API_SQLPUTDATA = 49);
function_handle!(API_SQLROWCOUNT = 20);
function_handle!(API_SQLSETCONNECTATTR = 1016);
function_handle!(API_SQLSETCONNECTOPTION = 50);
function_handle!(API_SQLSETCURSORNAME = 21);
function_handle!(API_SQLSETDESCFIELD = 1017);
function_handle!(API_SQLSETDESCREC = 1018);
function_handle!(API_SQLSETENVATTR = 1019);
function_handle!(API_SQLSETPARAM = 22);
function_handle!(API_SQLSETSTMTATTR = 1020);
function_handle!(API_SQLSETSTMTOPTION = 51);
function_handle!(API_SQLSPECIALCOLUMNS = 52);
function_handle!(API_SQLSTATISTICS = 53);
function_handle!(API_SQLTABLES = 54);
function_handle!(API_SQLTRANSACT = 23);
function_handle!(API_SQLCANCELHANDLE = 1550);
function_handle!(API_SQLCOMPLETEASYNC = 1551);

//pub const fn SQL_FUNC_EXISTS(pfExists: u16, uwAPI: u16) -> bool {
//    *((pfExists as *const UWORD).offset((uwAPI >> 4) as isize)) & (1 << (uwAPI & 0x000F))
//}

//const SQL_LEN_DATA_AT_EXEC_OFFSET: usize = -100;
//pub const fn SQL_LEN_DATA_AT_EXEC<isize>(length: isize) {
//    (-length).checked_add(SQL_LEN_DATA_AT_EXEC_OFFSET).expect()
//}

// TODO: Instead of implementing traits for every Option<T>, consider making a blanket impl for all T
