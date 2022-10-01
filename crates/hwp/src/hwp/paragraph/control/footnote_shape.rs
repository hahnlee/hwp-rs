use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{reader::RecordReader, tags::BodyTextRecord};

#[derive(Debug)]
pub struct FootnoteShape {
    /// 사용자 기호
    pub user_char: String,
    /// 앞 장식 문자
    pub prefix_char: String,
    /// 뒤 장식 문자
    pub suffix_char: String,
    /// 시작 번호
    pub start_number: u16,

    /// 구분선 길이
    ///
    /// NOTE: 공식 문서와 다르게 실제로는 4바이트다
    pub divide_line_length: u32,
}

impl FootnoteShape {
    pub fn from_reader<T: Read>(reader: &mut T) -> FootnoteShape {
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();

        if tag_id != BodyTextRecord::HWPTAG_FOOTNOTE_SHAPE as u32 {
            // TODO: (@hahnlee) Result 타입으로 바꾸는것 검토
            panic!("다른 레코드 입니다");
        }

        // TODO: (@hahnlee) 속성
        record.read_u32::<LittleEndian>().unwrap();

        let user_char = String::from_utf16(&[record.read_u16::<LittleEndian>().unwrap()]).unwrap();
        let prefix_char =
            String::from_utf16(&[record.read_u16::<LittleEndian>().unwrap()]).unwrap();
        let suffix_char =
            String::from_utf16(&[record.read_u16::<LittleEndian>().unwrap()]).unwrap();

        let start_number = record.read_u16::<LittleEndian>().unwrap();

        let divide_line_length = record.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 위 여백
        record.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 구분선 아래 여백
        record.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 주석 사이 여백
        record.read_i16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 종류
        record.read_u8().unwrap();
        // TODO: (@hahnlee) 구분선 굵기
        record.read_u8().unwrap();

        // TODO: (@hahnlee) 구분선 색상
        record.read_u32::<LittleEndian>().unwrap();

        FootnoteShape {
            user_char,
            prefix_char,
            suffix_char,
            start_number,
            divide_line_length,
        }
    }
}
