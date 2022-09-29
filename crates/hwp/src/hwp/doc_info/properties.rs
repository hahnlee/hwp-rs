use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{reader::RecordReader, tags::DocInfoRecord};

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
    pub fn from_reader<T: Read>(reader: &mut T) -> Properties {
        let (tag, _, size, mut data) = reader.read_record::<LittleEndian>().unwrap();

        if tag != DocInfoRecord::HWPTAG_DOCUMENT_PROPERTIES as u32 || size != 26 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        Properties {
            sections: data.read_u16::<LittleEndian>().unwrap(),
            page_start_number: data.read_u16::<LittleEndian>().unwrap(),
            footnote_start_number: data.read_u16::<LittleEndian>().unwrap(),
            endnote_start_number: data.read_u16::<LittleEndian>().unwrap(),
            picture_start_number: data.read_u16::<LittleEndian>().unwrap(),
            table_start_number: data.read_u16::<LittleEndian>().unwrap(),
            formula_start_number: data.read_u16::<LittleEndian>().unwrap(),
            list_id: data.read_u32::<LittleEndian>().unwrap(),
            paragraph_id: data.read_u32::<LittleEndian>().unwrap(),
            character_in_paragraph: data.read_u32::<LittleEndian>().unwrap(),
        }
    }
}
