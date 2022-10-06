use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{record::Record, utils::bits::get_value_range};

/// 페이지 번호 위치
#[derive(Debug, Clone)]
pub struct PageNumberPosition {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 번호의 표시 위치
    pub position: DisplayPosition,
    /// 사용자 기호
    pub user_char: char,
    /// 앞 장식 문자
    pub prefix_char: char,
    /// 뒤 장식 문자
    pub suffix_char: char,
}

impl PageNumberPosition {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 번호모양
        get_value_range(properties, 0, 7);
        let position = DisplayPosition::from_u32(get_value_range(properties, 8, 11)).unwrap();

        let user_char = char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let prefix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let suffix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        Self {
            ctrl_id,
            position,
            user_char,
            prefix_char,
            suffix_char,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum DisplayPosition {
    /// 쪽 번호 없음
    None,
    /// 왼쪽 위
    TopLeft,
    //가운데 위
    TopMiddle,
    /// 오른쪽 위
    TopRight,
    /// 왼쪽 아래
    BottomLeft,
    /// 가운데 아래
    BottomMiddle,
    /// 오른쪽 아래
    BottomRight,
    /// 바깥쪽 위
    OuterTop,
    /// 바깥쪽 아래
    OuterBottom,
    /// 안쪽 위
    InnerTop,
    /// 안쪽 아래
    InnerBottom,
}
