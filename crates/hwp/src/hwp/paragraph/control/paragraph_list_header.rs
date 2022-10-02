use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ParagraphListHeader {
    pub count: i16,
}

impl ParagraphListHeader {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let count = reader.read_i16::<LittleEndian>().unwrap();

        // 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let mut unknown = Vec::new();
        reader.read_to_end(&mut unknown).unwrap();

        Self { count }
    }
}
