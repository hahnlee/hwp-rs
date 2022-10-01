use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{tags::BodyTextRecord, Record};

#[derive(Debug)]
pub struct PageDefinition {}

impl PageDefinition {
    pub fn from_record(record: Record) -> PageDefinition {
        if record.tag_id != BodyTextRecord::HWPTAG_PAGE_DEF as u32 {
            panic!("잘못된 레코드 입니다 {}", record.tag_id);
        }

        let mut reader = record.get_data_reader();

        // HWPUNIT 4 용지 가로 크기
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 용지 세로 크기
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 용지 왼쪽 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 오른쪽 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 위 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 아래 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 머리말 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 꼬리말 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 제본 여백
        reader.read_u32::<LittleEndian>().unwrap();
        // UINT32 4 속성(표 132 참조)
        reader.read_u32::<LittleEndian>().unwrap();

        PageDefinition {}
    }
}
