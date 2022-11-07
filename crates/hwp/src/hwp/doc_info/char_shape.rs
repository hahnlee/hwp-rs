use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    color_ref::ColorRef,
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

use super::border_fill::BorderKind;

#[derive(Debug)]
pub struct CharShape {
    /// 언어별 글꼴 ID(FaceID) 참조 값
    pub font_ids: [u16; 7],
    /// 언어별 장평, 50%～200%
    pub font_scales: [u8; 7],
    /// 언어별 자간
    pub font_spacings: [i8; 7],
    /// 언어별 상대 크기, 10%～250%
    pub font_sizes: [u8; 7],
    /// 언어별 글자 위치, -100%～100%
    pub font_positions: [i8; 7],
    /// 기준 크기, 0pt～4096pt
    pub base_size: i32,
    /// 기울임 여부
    pub italic: bool,
    /// 진하게 여부
    pub bold: bool,
    /// 밑줄 종류
    pub underline_kind: UnderlineKind,
    /// 밑줄 모양
    pub underline_shape: BorderKind,
    /// 외곽선 종류
    pub outline_kind: OutlineKind,
    /// 그림자 종류
    pub shadow_kind: ShadowKind,
    /// 양각여부
    pub emboss: bool,
    /// 음각여부
    pub engrave: bool,
    /// 위 첨자 여부
    pub supscript: bool,
    /// 아래 첨자 여부
    pub subscript: bool,
    /// 취소선 여부
    pub strike: bool,
    /// 강조점 종류
    pub sym_mark: SymMark,
    /// 글꼴에 어울리는 빈칸 사용 여부
    pub use_font_space: bool,
    /// 취소선 모양
    pub strike_shape: BorderKind,
    /// Kerning 여부
    pub use_kerning: bool,
    /// 그림자 간격 X, -100%～100%
    pub shadow_offset_x: u8,
    /// 그림자 간격 Y, -100%～100%
    pub shadow_offset_y: u8,
    /// 글자 색
    pub color: ColorRef,
    /// 밑줄 색
    pub underline_color: ColorRef,
    /// 음영 색
    pub shade_color: ColorRef,
    /// 그림자 색
    pub shadow_color: ColorRef,
    /// 글자 테두리/배경 ID 참조 값 (5.0.2.1 이상)
    pub border_fill_id: Option<u16>,
    /// 취소선 색 (5.0.3.0 이상)
    pub strike_color: Option<ColorRef>,
}

impl FromRecordCursor for CharShape {
    fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_CHAR_SHAPE as u32,
            "올바르지 않은 정보"
        );

        let mut reader = record.get_data_reader();

        let font_ids = [
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
            reader.read_u16::<LittleEndian>().unwrap(),
        ];

        let font_scales = [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];

        let font_spacings = [
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
        ];

        let font_sizes = [
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
            reader.read_u8().unwrap(),
        ];

        let font_positions = [
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
            reader.read_i8().unwrap(),
        ];

        let base_size = reader.read_i32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let italic = get_flag(attribute, 0);
        let bold = get_flag(attribute, 1);
        let underline_kind = UnderlineKind::from_u32(get_value_range(attribute, 2, 3)).unwrap();
        let underline_shape = BorderKind::from_u32(get_value_range(attribute, 4, 7)).unwrap();
        let outline_kind = OutlineKind::from_u32(get_value_range(attribute, 8, 10)).unwrap();
        let shadow_kind = ShadowKind::from_u32(get_value_range(attribute, 11, 12)).unwrap();
        let emboss = get_flag(attribute, 13);
        let engrave = get_flag(attribute, 14);
        let supscript = get_flag(attribute, 15);
        let subscript = get_flag(attribute, 16);
        let strike = get_value_range(attribute, 18, 20) > 0;
        let sym_mark = SymMark::from_u32(get_value_range(attribute, 21, 24)).unwrap();
        let use_font_space = get_flag(attribute, 25);
        let strike_shape = BorderKind::from_u32(get_value_range(attribute, 26, 29)).unwrap();
        let use_kerning = get_flag(attribute, 30);

        let shadow_offset_x = reader.read_u8().unwrap();
        let shadow_offset_y = reader.read_u8().unwrap();

        let color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let underline_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let shade_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());
        let shadow_color = ColorRef::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        let border_fill_id = if *version >= Version::from_str("5.0.2.1") {
            Some(reader.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        let strike_color = if *version >= Version::from_str("5.0.3.0") {
            Some(ColorRef::from_u32(
                reader.read_u32::<LittleEndian>().unwrap(),
            ))
        } else {
            None
        };

        Self {
            font_ids,
            font_scales,
            font_spacings,
            font_sizes,
            font_positions,
            base_size,
            italic,
            bold,
            underline_kind,
            underline_shape,
            outline_kind,
            shadow_kind,
            emboss,
            engrave,
            supscript,
            subscript,
            strike,
            sym_mark,
            use_font_space,
            strike_shape,
            use_kerning,
            shadow_offset_x,
            shadow_offset_y,
            color,
            underline_color,
            shade_color,
            shadow_color,
            border_fill_id,
            strike_color,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum UnderlineKind {
    None,
    Bottom,
    Top,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum OutlineKind {
    /// 없음
    None,
    /// 실선
    Solid,
    /// 점선
    Dot,
    /// 굵은 실선(두꺼운 선)
    Tick,
    /// 파선(긴 점선)
    Dash,
    /// 일점쇄선 (-.-.-.-.)
    DashDot,
    /// 이점쇄선 (-..-..-..)
    DashDotDot,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum ShadowKind {
    /// 없음
    None,
    /// 비연속
    Drop,
    /// 연속
    Continuous,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum SymMark {
    /// 없음
    None,
    /// 검정 동그라미 강조점
    DotAbove,
    /// 속 빈 동그라미 강조점̊̊̊̊̊̊̊̊̊
    RingAbove,
    /// ˇ
    Caron,
    ///  ̃
    Tilde,
    /// ･
    DotMiddle,
    /// :
    Colon,
}
