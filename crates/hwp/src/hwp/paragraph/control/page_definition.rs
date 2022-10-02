use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{tags::BodyTextRecord, Record};

/// 페이지 정의
#[derive(Debug)]
pub struct PageDefinition {}

impl PageDefinition {
    pub fn from_record(record: Record) -> PageDefinition {
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_PAGE_DEF as u32,
            "잘못된 레코드 입니다"
        );

        let mut reader = record.get_data_reader();

        // 용지 가로 크기
        reader.read_u32::<LittleEndian>().unwrap();
        // 용지 세로 크기
        reader.read_u32::<LittleEndian>().unwrap();
        // 용지 왼쪽 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 오른쪽 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 위 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 아래 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 머리말 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 꼬리말 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 제본 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // 속성(표 132 참조)
        reader.read_u32::<LittleEndian>().unwrap();

        PageDefinition {}
    }
}
