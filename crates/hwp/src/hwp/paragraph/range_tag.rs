use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct RangeTag {
    /// 영역 시작
    pub start_position: u32,
    /// 영역 끝
    pub end_position: u32,
    /// 태그(종류 + 데이터)
    ///
    /// 상위 8비트가 종류를 하위 24비트가 종류별로 다른
    /// 설명을 부여할 수 있는 임의의 데이터를 나타낸다.
    pub tag: u32,
}

impl RangeTag {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let start_position = reader.read_u32::<LittleEndian>().unwrap();
        let end_position = reader.read_u32::<LittleEndian>().unwrap();
        let tag = reader.read_u32::<LittleEndian>().unwrap();

        Self {
            start_position,
            end_position,
            tag,
        }
    }
}
