use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::{get_flag, get_value, get_value_range},
    version::Version,
};

#[derive(Debug)]
pub struct Numbering {
    /// 시작 번호
    pub start: u16,
    /// 문단 모양들
    pub paragraph_heads: Vec<ParagraphHead>,
}

impl FromRecordCursor for Numbering {
    fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_NUMBERING as u32);

        let mut reader = record.get_data_reader();
        let mut paragraph_heads = vec![];

        for _ in 0..7 {
            paragraph_heads.push(ParagraphHead::from_reader(&mut reader, true));
        }

        let start = reader.read_u16::<LittleEndian>().unwrap();

        if *version >= Version::from_str("5.0.2.5") {
            for i in 0..7 {
                paragraph_heads[i].start_number = Some(reader.read_u32::<LittleEndian>().unwrap());
            }
        }

        if reader.position() < record.size.into() {
            for _ in 7..10 {
                paragraph_heads.push(ParagraphHead::from_reader(&mut reader, true));
            }

            if *version >= Version::from_str("5.1.0.0") {
                for i in 7..10 {
                    paragraph_heads[i].start_number =
                        Some(reader.read_u32::<LittleEndian>().unwrap());
                }
            }
        }

        assert_eq!(reader.position(), record.size.into());

        Self {
            start,
            paragraph_heads,
        }
    }
}

#[derive(Debug)]
pub struct ParagraphHead {
    /// 문단의 정렬 종류
    pub align: ParagraphHeadAlign,
    /// 번호 너비를 실제 인스턴스
    /// 문자열의 너비에 따를지 여부
    pub use_instance_width: bool,
    /// 자동 내어 쓰기 여부
    pub auto_indent: bool,
    /// 수준별 본문과의 거리 종류
    pub text_offset_kind: TextOffsetKind,
    /// 너비 보정값
    pub width_adjust: i16,
    /// 본문과의 거리
    pub text_offset: i16,
    /// 글자 모양 아이디 참조
    pub char_shape_id: u32,
    /// 번호 형식
    pub number_format: String,
    /// 수준별 시작번호
    /// - level 1~7: 5.0.2.5 이상
    /// - level 8~10: 5.1.0.0 이상
    pub start_number: Option<u32>,
}

impl ParagraphHead {
    pub fn from_reader<T: Read>(reader: &mut T, numbering: bool) -> Self {
        // 속성(표 40 참조)
        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let align = ParagraphHeadAlign::from_u32(get_value_range(attribute, 0, 1)).unwrap();
        let use_instance_width = get_flag(attribute, 2);
        let auto_indent = get_flag(attribute, 3);
        let text_offset_kind = TextOffsetKind::from_u32(get_value(attribute, 4)).unwrap();

        let width_adjust = reader.read_i16::<LittleEndian>().unwrap();
        let text_offset = reader.read_i16::<LittleEndian>().unwrap();
        let char_shape_id = reader.read_u32::<LittleEndian>().unwrap();
        let number_format = if numbering {
            reader.read_string::<LittleEndian>().unwrap()
        } else {
            format!("")
        };

        let start_number = None;

        Self {
            align,
            use_instance_width,
            auto_indent,
            text_offset_kind,
            width_adjust,
            text_offset,
            char_shape_id,
            number_format,
            start_number,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum ParagraphHeadAlign {
    Left,
    Center,
    Right,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum TextOffsetKind {
    /// 글자 크기에 대한 상대 비율
    Percent,
    /// 값
    HWPUnit,
}
