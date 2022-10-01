use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{reader::RecordReader, tags::BodyTextRecord};

#[derive(Debug)]
pub struct PageDefinition {}

impl PageDefinition {
    pub fn from_reader<T: Read>(reader: &mut T) -> PageDefinition {
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();

        if tag_id != BodyTextRecord::HWPTAG_PAGE_DEF as u32 {
            panic!("잘못된 레코드 입니다");
        }

        // HWPUNIT 4 용지 가로 크기
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 용지 세로 크기
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 용지 왼쪽 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 오른쪽 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 위 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 아래 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 머리말 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 꼬리말 여백
        record.read_u32::<LittleEndian>().unwrap();
        // HWPUNIT 4 제본 여백
        record.read_u32::<LittleEndian>().unwrap();
        // UINT32 4 속성(표 132 참조)
        record.read_u32::<LittleEndian>().unwrap();

        PageDefinition {}
    }
}
