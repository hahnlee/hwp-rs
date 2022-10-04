use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct CharShape {
    /// 글자 모양이 바뀌는 시작 위치
    pub start_position: u32,
    /// 글자 모양 ID
    pub shape_id: u32,
}

impl CharShape {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let start_position = reader.read_u32::<LittleEndian>().unwrap();
        let shape_id = reader.read_u32::<LittleEndian>().unwrap();

        Self {
            start_position,
            shape_id,
        }
    }
}
