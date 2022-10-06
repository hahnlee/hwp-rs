use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::record::{reader::RecordReader, Record};

/// 덧말
#[derive(Debug, Clone)]
pub struct SubText {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 본문
    pub main_text: String,
    /// 덧말
    pub sub_text: String,
    /// 덧말의 위치
    pub position: SubTextPosition,
    pub f_size_ratio: u8,
    pub option: u8,
    pub style_number: u8,
    pub align: SubTextAlign,
}

impl SubText {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let main_text = reader.read_string::<LittleEndian>().unwrap();
        let sub_text = reader.read_string::<LittleEndian>().unwrap();
        let position = SubTextPosition::from_u8(reader.read_u8().unwrap()).unwrap();
        let f_size_ratio = reader.read_u8().unwrap();
        let option = reader.read_u8().unwrap();
        let style_number = reader.read_u8().unwrap();
        let align = SubTextAlign::from_u8(reader.read_u8().unwrap()).unwrap();

        Self {
            ctrl_id,
            main_text,
            sub_text,
            position,
            f_size_ratio,
            option,
            style_number,
            align,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum SubTextPosition {
    Top,
    Bottom,
    Middle,
}

#[repr(u8)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum SubTextAlign {
    /// 양쪽 정렬
    Justify,
    Left,
    Right,
    Center,
    /// 배분 정렬
    Distribution,
    /// 나눔 정렬
    Divide,
}
