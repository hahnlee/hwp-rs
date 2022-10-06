use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{tags::BodyTextRecord, Record};

/// 각주 / 미주
#[derive(Debug, Clone)]
pub struct FootnoteShape {
    /// 사용자 기호
    pub user_char: char,
    /// 앞 장식 문자
    pub prefix_char: char,
    /// 뒤 장식 문자
    pub suffix_char: char,
    /// 시작 번호
    pub start_number: u16,

    /// 구분선 길이
    ///
    /// NOTE: 공식 문서와 다르게 실제로는 4바이트다
    pub divide_line_length: u32,
}

impl FootnoteShape {
    pub fn from_record(record: Record) -> FootnoteShape {
        if record.tag_id != BodyTextRecord::HWPTAG_FOOTNOTE_SHAPE as u32 {
            // TODO: (@hahnlee) Result 타입으로 바꾸는것 검토
            panic!("다른 레코드 입니다 {}", record.tag_id);
        }

        let mut reader = record.get_data_reader();

        // TODO: (@hahnlee) 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let user_char = char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let prefix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let suffix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        let start_number = reader.read_u16::<LittleEndian>().unwrap();

        let divide_line_length = reader.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 위 여백
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 구분선 아래 여백
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 주석 사이 여백
        reader.read_i16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 종류
        reader.read_u8().unwrap();
        // TODO: (@hahnlee) 구분선 굵기
        reader.read_u8().unwrap();

        // TODO: (@hahnlee) 구분선 색상
        reader.read_u32::<LittleEndian>().unwrap();

        FootnoteShape {
            user_char,
            prefix_char,
            suffix_char,
            start_number,
            divide_line_length,
        }
    }
}
