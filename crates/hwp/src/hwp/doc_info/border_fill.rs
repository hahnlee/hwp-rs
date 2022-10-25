use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    record::{tags::DocInfoRecord, FromRecord, Record},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct BorderFill {
    /// 선 정보
    pub borders: [Border; 4],
    /// 대각선
    pub diagonal_border: Border,
    /// 채우기 종류
    pub fill_kind: FillKind,
    /// 채우기 정보
    pub fill_content: FillContent,
}

impl FromRecord for BorderFill {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_BORDER_FILL as u32);

        let mut reader = record.get_data_reader();

        // TODO: (@hahnlee) 속성(표 24 참조)
        reader.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 공식문서와 순서가 다르다
        let borders = [
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
        ];

        let diagonal_border = Border::from_reader(&mut reader);

        let fill_kind = FillKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        let fill_content = match fill_kind {
            FillKind::Color => FillContent::Color(ColorFill::from_reader(&mut reader)),
            _ => FillContent::None(()),
        };

        Self {
            borders,
            diagonal_border,
            fill_kind,
            fill_content,
        }
    }
}

#[derive(Debug)]
pub struct Border {
    pub width: u8,
    pub kind: u8,
    pub color: ColorRef,
}

impl Border {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            kind: reader.read_u8().unwrap(),
            width: reader.read_u8().unwrap(),
            color: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
        }
    }
}

impl BorderFill {
    pub fn as_color_fill(&self) -> &ColorFill {
        match &self.fill_content {
            FillContent::Color(color) => color,
            _ => panic!("color_fill이 아닙니다"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, FromPrimitive, PartialEq)]
pub enum FillKind {
    /// 채우기 없음
    None = 0x00000000,
    /// 단색 채우기
    Color = 0x00000001,
    /// 이미지 채우기
    Image = 0x00000002,
    /// 그라데이션 채우기
    Gradation = 0x00000004,
}

#[derive(Debug)]
pub enum FillContent {
    None(()),
    Color(ColorFill),
}

#[derive(Debug)]
pub struct ColorFill {
    pub background_color: ColorRef,
    pub pattern_color: ColorRef,
}

impl ColorFill {
    fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            background_color: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
            pattern_color: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
        }
    }
}
