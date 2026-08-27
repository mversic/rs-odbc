use core::mem::MaybeUninit;

use rs_odbc::{
    DriverDefined,
    c_type::CType,
    col::ColAttrGet,
    desc::{DescFieldGet, IRD},
    env::OV_ODBC3_80,
    sql_type::SqlType,
};

enum DriverCType {}

impl co3::tag::TagFamily for DriverCType {
    type Kind = i16;
}

unsafe impl co3::tag::Tagged for DriverCType {
    const TAG: i16 = 0x4001;
}

unsafe impl CType<OV_ODBC3_80> for DriverCType {
    type Value = MaybeUninit<[u8; 16]>;
}

enum DriverSqlType {}

impl co3::tag::TagFamily for DriverSqlType {
    type Kind = i16;
}

unsafe impl co3::tag::Tagged for DriverSqlType {
    const TAG: i16 = 0x4002;
}

impl SqlType<OV_ODBC3_80> for DriverSqlType {}

enum DriverDescField {}

impl co3::tag::TagFamily for DriverDescField {
    type Kind = i16;
}

unsafe impl co3::tag::Tagged for DriverDescField {
    const TAG: i16 = 0x4003;
}

impl rs_odbc::Defined for DriverDescField {
    type By = DriverDefined;
}

unsafe impl DescFieldGet<OV_ODBC3_80, IRD> for DriverDescField {
    type Buffer<C: rs_odbc::str::OdbcChar> = MaybeUninit<u32>;
}

enum DriverColField {}

impl co3::tag::TagFamily for DriverColField {
    type Kind = u16;
}

unsafe impl co3::tag::Tagged for DriverColField {
    const TAG: u16 = 0x4004;
}

impl rs_odbc::Defined for DriverColField {
    type By = DriverDefined;
}

unsafe impl ColAttrGet<OV_ODBC3_80> for DriverColField {
    type CharacterBuffer<C: rs_odbc::str::OdbcChar> = MaybeUninit<u32>;
    type NumericBuffer = rs_odbc::col::Unavailable<rs_odbc::data::LEN>;
}

#[test]
fn driver_types_can_extend_the_type_sets() {
    fn accepts_c_type<T: CType<OV_ODBC3_80>>() {}
    fn accepts_sql_type<T: SqlType<OV_ODBC3_80>>() {}
    fn accepts_desc_field<T: DescFieldGet<OV_ODBC3_80, IRD>>() {}
    fn accepts_col_field<T: ColAttrGet<OV_ODBC3_80>>() {}

    accepts_c_type::<DriverCType>();
    accepts_sql_type::<DriverSqlType>();
    accepts_desc_field::<DriverDescField>();
    accepts_col_field::<DriverColField>();
}
