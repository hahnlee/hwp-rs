use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    paragraph::Paragraph, record::RecordCursor, utils::bits::get_value_range, version::Version,
};

/// 문단 리스트
#[derive(Debug, Clone)]
pub struct ParagraphList {
    pub header: ParagraphListHeader,
    pub paragraphs: Vec<Paragraph>,
}

impl ParagraphList {
    pub fn from_reader<T: Read>(
        reader: &mut T,
        cursor: &mut RecordCursor,
        version: &Version,
    ) -> Self {
        let header = ParagraphListHeader::from_reader(reader);

        // NOTE: 나머지 속성은 사용처에서 파싱해야함
        let mut paragraphs = Vec::new();
        for _ in 0..header.count {
            let paragraph = Paragraph::from_record_cursor(cursor, version);
            paragraphs.push(paragraph);
        }

        Self { header, paragraphs }
    }
}

/// 문단 해더
#[derive(Debug, Clone)]
pub struct ParagraphListHeader {
    /// 문단 수
    pub count: u32,
    /// 방향
    pub direction: Direction,
    /// 문단의 줄바꿈
    pub line_break: LineBreak,
    /// 세로 정렬
    pub vertical_align: VerticalAlign,
}

impl ParagraphListHeader {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        // NOTE: (@hahnlee) 문서에는 2바이트로 나와있으나, 실제론 4바이트를 읽어야함
        let count = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let direction = Direction::from_u32(get_value_range(attribute, 0, 2)).unwrap();
        let line_break = LineBreak::from_u32(get_value_range(attribute, 3, 4)).unwrap();
        let vertical_align = VerticalAlign::from_u32(get_value_range(attribute, 5, 6)).unwrap();

        Self {
            count,
            direction,
            line_break,
            vertical_align,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum LineBreak {
    /// 일반적인 줄바꿈
    Normal,
    /// 자간을 조종하여 한 줄을 유지
    SingleLine,
    /// 내용에 따라 폭이 늘어남
    Dynamic,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}
