use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::Record,
    utils::bits::{get_flag, get_value_range},
};

use super::section::{NumberShape, format_number_shape};

/// 번호 종류
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum NumberKind {
    /// 쪽 번호
    Page,
    /// 각주 번호
    Footnote,
    /// 미주 번호
    Endnote,
    /// 그림 번호
    Picture,
    /// 표 번호
    Table,
    /// 수식 번호
    Equation,
}

/// 자동 번호
#[derive(Debug, Clone)]
pub struct AutoNumber {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 번호 종류
    pub kind: NumberKind,
    // 번호 모양
    pub number_shape: NumberShape,
    /// 각주에서만 사용된다.
    /// 각주 내용 중 번호 코드의 모양을 위 첨자 형식으로 할지 여부.
    pub superscript: bool,
    /// 번호
    pub number: u16,
    /// 사용자 기호
    pub user_char: char,
    /// 앞 장식 문자
    pub prefix_char: char,
    /// 뒤 장식 문자
    pub suffix_char: char,
}

impl AutoNumber {
    pub fn from_record(record: &mut Record) -> Self {
        let mut reader = record.get_data_reader();
        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        let kind = NumberKind::from_u32(get_value_range(properties, 0, 3)).unwrap();
        let number_shape = NumberShape::from_u32(get_value_range(properties, 4, 11)).unwrap();
        let superscript = get_flag(properties, 12);

        let number = reader.read_u16::<LittleEndian>().unwrap();

        let user_char = char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let prefix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let suffix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        Self {
            ctrl_id,
            kind,
            number_shape,
            superscript,
            number,
            user_char,
            prefix_char,
            suffix_char,
        }
    }

    pub fn to_string(&self) -> String {
        format_number_shape(&self.number_shape, self.number)
    }
}

/// 새 번호 지정
#[derive(Debug, Clone)]
pub struct NewNumber {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 번호 종류
    pub kind: NumberKind,
    /// 번호
    pub number: u16,
}

impl NewNumber {
    pub fn from_record(record: &mut Record) -> Self {
        let mut reader = record.get_data_reader();
        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        let kind = NumberKind::from_u32(get_value_range(properties, 0, 3)).unwrap();

        let number = reader.read_u16::<LittleEndian>().unwrap();

        Self {
            ctrl_id,
            kind,
            number,
        }
    }
}
