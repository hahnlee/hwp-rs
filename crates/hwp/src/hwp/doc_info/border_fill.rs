use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    record::{tags::DocInfoRecord, FromRecord, Record},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

#[derive(Debug)]
pub struct BorderFill {
    /// 3D 효과의 유무
    pub effect_3d: bool,
    /// 그림자 효과의 유무
    pub effect_shadow: bool,
    /// Slash 대각선 모양
    pub slash_diagonal_shape: SlashDiagonalShape,
    /// BackSlash 대각선 모양
    pub back_slash_diagonal_shape: BackSlashDiagonalShape,
    /// Slash 대각선 꺾은선 여부
    pub broken_slash_diagonal_line: bool,
    /// BackSlash 대각선 꺾은선 여부
    pub broken_back_slash_diagonal_line: bool,
    /// Slash 대각선 모양 180도 회전 여부
    pub slack_diagonal_line_rotated: bool,
    /// BackSlash 대각선 모양 180도 회전 여부
    pub back_slack_diagonal_line_rotated: bool,
    /// 중심선 유무
    pub center_line: bool,
    /// 선 정보
    pub borders: [Border; 4],
    /// 대각선
    pub diagonal_border: Border,
    /// 채우기 정보
    pub fill: Fill,
}

impl FromRecord for BorderFill {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_BORDER_FILL as u32);

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u16::<LittleEndian>().unwrap();
        let effect_3d = get_flag(attribute, 0);
        let effect_shadow = get_flag(attribute, 1);
        let slash_diagonal_shape =
            SlashDiagonalShape::from_u16(get_value_range(attribute, 2, 4)).unwrap();
        let back_slash_diagonal_shape =
            BackSlashDiagonalShape::from_u16(get_value_range(attribute, 5, 7)).unwrap();
        let broken_slash_diagonal_line = if get_value_range(attribute, 8, 9) > 0 {
            true
        } else {
            false
        };
        let broken_back_slash_diagonal_line = get_flag(attribute, 10);
        let slack_diagonal_line_rotated = get_flag(attribute, 11);
        let back_slack_diagonal_line_rotated = get_flag(attribute, 11);
        let center_line = get_flag(attribute, 13);

        // NOTE: (@hahnlee) 공식문서와 순서가 다르다
        let borders = [
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
            Border::from_reader(&mut reader),
        ];

        let diagonal_border = Border::from_reader(&mut reader);

        let fill = Fill::from_reader(&mut reader);

        Self {
            effect_3d,
            effect_shadow,
            slash_diagonal_shape,
            back_slash_diagonal_shape,
            broken_slash_diagonal_line,
            broken_back_slash_diagonal_line,
            slack_diagonal_line_rotated,
            back_slack_diagonal_line_rotated,
            center_line,
            borders,
            diagonal_border,
            fill,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum SlashDiagonalShape {
    None = 0b000,
    Slash = 0b010,
    LeftTopToBottomEdge = 0b011,
    LeftTopToRightEdge = 0b110,
    LeftTopToBottomRightEdge = 0b111,
}

#[repr(u8)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum BackSlashDiagonalShape {
    None = 0b000,
    BackSlash = 0b010,
    RightTopToBottomEdge = 0b011,
    RightTopToLeftEdge = 0b110,
    RightTopToBottomLeftEdge = 0b111,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Fill {
    /// 채우기 종류
    pub kind: FillKind,
    /// 채우기 내용
    pub content: FillContent,
}

impl Fill {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let kind = FillKind::from_u32(reader.read_u32::<LittleEndian>().unwrap()).unwrap();

        // TODO: (@hahnlee) 나머지 채우기
        let content = match kind {
            FillKind::Color => FillContent::Color(ColorFill::from_reader(reader)),
            _ => FillContent::None(()),
        };

        Self { kind, content }
    }

    pub fn as_color_fill(&self) -> &ColorFill {
        match &self.content {
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
    pub background: ColorRef,
    pub pattern: ColorRef,
}

impl ColorFill {
    fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            background: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
            pattern: ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap()),
        }
    }
}
