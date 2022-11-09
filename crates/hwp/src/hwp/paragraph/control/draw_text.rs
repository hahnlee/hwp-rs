use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::paragraph_list::ParagraphList,
    record::{tags::BodyTextRecord, RecordCursor},
    version::Version,
};

/// 글상자
#[derive(Debug, Clone)]
pub struct DrawText {
    /// 문단 리스트
    pub paragraph_list: ParagraphList,
    /// 글상자 텍스트 왼쪽 여백
    pub margin_left: i16,
    // 글상자 텍스트 오른쪽 여백
    pub margin_right: i16,
    /// 글상자 텍스트 위쪽 여백
    pub margin_top: i16,
    /// 글상자 텍스트 아래쪽 여백
    pub margin_bottom: i16,
    /// 텍스트 문자열의 최대 폭
    pub last_width: u32,
    /// 스펙에 정의되지 않은 바이트
    pub unknown: Vec<u8>,
}

impl DrawText {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, BodyTextRecord::HWPTAG_LIST_HEADER as u32);

        let mut reader = record.get_data_reader();
        let paragraph_list = ParagraphList::from_reader(&mut reader, cursor, version);

        let margin_left = reader.read_i16::<LittleEndian>().unwrap();
        let margin_right = reader.read_i16::<LittleEndian>().unwrap();
        let margin_top = reader.read_i16::<LittleEndian>().unwrap();
        let margin_bottom = reader.read_i16::<LittleEndian>().unwrap();

        let last_width = reader.read_u32::<LittleEndian>().unwrap();

        let mut unknown = vec![];
        reader.read_to_end(&mut unknown).unwrap();

        Self {
            paragraph_list,
            margin_left,
            margin_right,
            margin_top,
            margin_bottom,
            last_width,
            unknown,
        }
    }
}
