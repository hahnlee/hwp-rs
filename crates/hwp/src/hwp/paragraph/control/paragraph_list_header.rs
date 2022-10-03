use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::utils::bits::get_value_range;

// TODO: (@hahnlee) 제너릭으로 만드는것 검토
#[derive(Debug)]
pub struct ParagraphListHeader {
    /// 문단 수
    pub count: u32,
    /// 방향
    pub direction: Direction,
}

impl ParagraphListHeader {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        // NOTE: (@hahnlee) 문서에는 2바이트로 나와있으나, 실제론 4바이트를 읽어야함
        let count = reader.read_u32::<LittleEndian>().unwrap();

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        let direction = Direction::from_u32(get_value_range(properties, 0, 2)).unwrap();

        // TODO: (@hahnlee) 나머지 속성 추가

        // 이후 속성은 레코드에 따라 다름
        Self { count, direction }
    }
}

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum Direction {
    Horizontal,
    Vertical,
}
