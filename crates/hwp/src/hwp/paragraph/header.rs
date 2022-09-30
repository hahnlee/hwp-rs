use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{reader::RecordReader, tags::BodyTextRecord},
    version::Version,
};

#[derive(Debug)]
pub struct ParagraphHeader {
    /// 글자수
    pub chars: u32,
    /// 글자 모양 정보 수
    pub char_shapes: u16,
    /// 각 줄에 대한 align에 대한 정보 수
    pub aligns: u16,
    // range tag 정보 수
    pub ranges: u16,
}

impl ParagraphHeader {
    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Self {
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_HEADER as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            panic!("잘못된 정보입니다");
        }

        let mut chars = record.read_u32::<LittleEndian>().unwrap();

        if (chars & 0x80000000) == 0x80000000 {
            chars &= 0x7fffffff;
        }

        // ctrl mask
        record.read_u32::<LittleEndian>().unwrap();

        // 문단 모양 아이디 참조값
        record.read_u16::<LittleEndian>().unwrap();

        // 문단 스타일 아이디 참조값
        record.read_u8().unwrap();

        // 단 나누기 종류(표 59 참조)
        record.read_u8().unwrap();

        let char_shapes = record.read_u16::<LittleEndian>().unwrap();

        let ranges = record.read_u16::<LittleEndian>().unwrap();

        let aligns = record.read_u16::<LittleEndian>().unwrap();

        // UINT32 4 문단 Instance ID (unique ID)
        record.read_u32::<LittleEndian>().unwrap();

        // 변경추적 병합 문단여부. (5.0.3.2 버전 이상)
        if *version >= Version::from_str("5.0.3.2") {
            record.read_u16::<LittleEndian>().unwrap();
        }

        Self {
            chars,
            char_shapes,
            aligns,
            ranges,
        }
    }
}
