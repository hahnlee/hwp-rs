use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{tags::DocInfoRecord, Record};

#[derive(Debug)]
pub struct Properties {
    /// 구역 개수
    pub sections: u16,
    /// 페이지 시작 번호
    pub page_start_number: u16,
    /// 각주 시작 번호
    pub footnote_start_number: u16,
    /// 미주 시작 번호
    pub endnote_start_number: u16,
    /// 그림 시작 번호
    pub picture_start_number: u16,
    /// 표 시작 번호
    pub table_start_number: u16,
    /// 수식 시작 번호
    pub formula_start_number: u16,
    /// 리스트 아이디
    pub list_id: u32,
    /// 문단 아이디
    pub paragraph_id: u32,
    /// 문단 내에서의 글자 단위 위치
    pub character_in_paragraph: u32,
}

impl Properties {
    pub fn from_record(record: &mut Record) -> Self {
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_DOCUMENT_PROPERTIES as u32,
            "올바르지 않은 레코드 입니다"
        );

        let mut reader = record.get_data_reader();

        Self {
            sections: reader.read_u16::<LittleEndian>().unwrap(),
            page_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            footnote_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            endnote_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            picture_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            table_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            formula_start_number: reader.read_u16::<LittleEndian>().unwrap(),
            list_id: reader.read_u32::<LittleEndian>().unwrap(),
            paragraph_id: reader.read_u32::<LittleEndian>().unwrap(),
            character_in_paragraph: reader.read_u32::<LittleEndian>().unwrap(),
        }
    }
}
