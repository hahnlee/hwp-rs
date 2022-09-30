use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
#[allow(dead_code)]
pub struct SectionControl {
    unknown: Vec<u8>,
}

impl SectionControl {
    pub fn from_reader<T: Read>(reader: &mut T, size: u32) -> SectionControl {
        let mut record = reader.take(size.into());

        record.read_u32::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        record.read_u32::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        // 버전 분기 추가
        record.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 표준 문서에 작성된 내용과 다르게 실제로는 더 많은 바이트가 있다.
        let mut unknown = Vec::new();
        record.read_to_end(&mut unknown).unwrap();

        SectionControl { unknown }
    }
}
