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
    pub fill_kind: BorderFillKind,
    pub fill_content: FillContent,
}

impl FromRecord for BorderFill {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_BORDER_FILL as u32
        );

        let mut reader = record.get_data_reader();

        // TODO: (@hahnlee) 속성(표 24 참조)
        reader.read_u16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 4방향 테두리선 종류(표 25 참조)
        [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];
        // TODO: (@hahnlee) 4방향 테두리선 굵기(표 26 참조)
        [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];
        // TODO: (@hahnlee) 4방향 테두리선 색상.
        [
            ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
            ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
            ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
            ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
        ];
        // TODO: (@hahnlee) 대각선 종류
        reader.read_u8().unwrap();
        // TODO: (@hahnlee) 대각선 굵기
        reader.read_u8().unwrap();
        // TODO: (@hahnlee) 대각선 색깔
        ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        let fill_kind =
            BorderFillKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        let fill_content = match fill_kind {
            BorderFillKind::Color => FillContent::Color(ColorFill::from_reader(&mut reader)),
            _ => FillContent::None(()),
        };

        Self {
            fill_kind,
            fill_content,
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
pub enum BorderFillKind {
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
