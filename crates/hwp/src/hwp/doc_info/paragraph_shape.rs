use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::{get_flag, get_value, get_value_range},
    version::Version,
};

#[derive(Debug)]
pub struct ParagraphShape {
    /// 줄 간격 종류. 한/글 2007 이하 버전에서 사용.
    pub line_space_kind_old: LineSpacingKind,
    /// 정렬 방식
    pub align: Align,
    /// 라틴 문자의 줄나눔 단위
    pub break_latin_word: BreakLatinWord,
    /// 라틴 문자 이외의 줄나눔 단위
    pub break_non_latin_word: BreakNonLatinWord,
    /// 편집 용지의 줄 격자 사용 여부
    pub snap_to_grid: bool,
    /// 공백 최소값
    pub condense: u32,
    /// 외톨이줄 보호 여부
    pub widow_orphan: bool,
    /// 다음 문단과 함께 여부
    pub keep_with_next: bool,
    /// 문단 보호 여부
    pub keep_lines: bool,
    /// 문단 앞에서 항상 쪽 나눔 여부
    pub page_break_before: bool,
    /// 세로 정렬
    pub vertical_align: VerticalAlign,
    /// 글꼴에 어울리는 줄 높이 여부
    pub font_line_height: bool,
    /// 문단 머리 모양 종류
    pub heading_kind: ParagraphHeadingKind,
    /// 문단 수준
    pub heading_level: u8,
    /// 문단 테두리 연결 여부
    pub border_connect: bool,
    /// 문단 여백 무시 여부
    pub border_ignore_margin: bool,
    /// 문단 꼬리 모양
    pub tailing: u8,
    /// 왼쪽 여백
    pub padding_left: i32,
    /// 오른쪽 여백
    pub padding_right: i32,
    /// 들여 쓰기/내어 쓰기
    pub indent: i32,
    /// 문단 간격 위
    pub margin_top: i32,
    /// 문단 간격 아래
    pub margin_bottom: i32,
    /// 줄 간격. 한글 2007 이하 버전(5.0.2.5 버전 미만)에서 사용.
    pub line_space_old: i32,
    /// 탭 정의 아이디(TabDef ID) 참조 값
    pub tab_definition_id: u16,
    /// 번호 문단 ID(Numbering ID) 또는 글머리표 문단 모양 ID(Bullet ID) 참조 값
    pub numbering_bullet_id: u16,
    /// 테두리/배경 모양 ID(BorderFill ID) 참조 값
    pub border_fill_id: u16,
    /// 문단 테두리 왼쪽 간격
    pub border_offset_left: i16,
    /// 문단 테두리 오른쪽 간격
    pub border_offset_right: i16,
    /// 문단 테두리 위쪽 간격
    pub border_offset_top: i16,
    /// 문단 테두리 아래쪽 간격
    pub border_offset_bottom: i16,
    /// 한 줄로 입력 여부 (5.0.1.7 버전 이상)
    pub single_line: Option<bool>,
    /// 한글과 영어 간격을 자동 조절 여부 (5.0.1.7 버전 이상)
    pub auto_spacing_kr_eng: Option<bool>,
    /// 한글과 숫자 간격을 자동 조절 여부 (5.0.1.7 버전 이상)
    pub auto_spacing_kr_num: Option<bool>,
    /// 줄 간격 종류 (5.0.2.5 버전 이상)
    pub line_spacing_kind: Option<LineSpacingKind>,
    /// 줄 간격 (5.0.2.5 버전 이상)
    pub line_spacing: Option<u32>,
}

impl FromRecordCursor for ParagraphShape {
    fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_PARA_SHAPE as u32);

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let line_space_kind_old =
            LineSpacingKind::from_u32(get_value_range(attribute, 0, 1)).unwrap();
        let align = Align::from_u32(get_value_range(attribute, 2, 4)).unwrap();
        let break_latin_word = BreakLatinWord::from_u32(get_value_range(attribute, 5, 6)).unwrap();
        let break_non_latin_word = BreakNonLatinWord::from_u32(get_value(attribute, 7)).unwrap();
        let snap_to_grid = get_flag(attribute, 8);
        let condense = get_value_range(attribute, 9, 15);
        let widow_orphan = get_flag(attribute, 16);
        let keep_with_next = get_flag(attribute, 17);
        let keep_lines = get_flag(attribute, 18);
        let page_break_before = get_flag(attribute, 19);
        let vertical_align = VerticalAlign::from_u32(get_value_range(attribute, 20, 21)).unwrap();
        let font_line_height = get_flag(attribute, 22);
        let heading_kind =
            ParagraphHeadingKind::from_u32(get_value_range(attribute, 23, 24)).unwrap();
        let heading_level = get_value_range(attribute, 25, 27) as u8;
        let border_connect = get_flag(attribute, 28);
        let border_ignore_margin = get_flag(attribute, 29);
        let tailing = get_value(attribute, 30) as u8;

        let padding_left = reader.read_i32::<LittleEndian>().unwrap();
        let padding_right = reader.read_i32::<LittleEndian>().unwrap();

        let indent = reader.read_i32::<LittleEndian>().unwrap();

        let margin_top = reader.read_i32::<LittleEndian>().unwrap();
        let margin_bottom = reader.read_i32::<LittleEndian>().unwrap();

        let line_space_old = reader.read_i32::<LittleEndian>().unwrap();

        let tab_definition_id = reader.read_u16::<LittleEndian>().unwrap();
        let numbering_bullet_id = reader.read_u16::<LittleEndian>().unwrap();
        let border_fill_id = reader.read_u16::<LittleEndian>().unwrap();

        let border_offset_left = reader.read_i16::<LittleEndian>().unwrap();
        let border_offset_right = reader.read_i16::<LittleEndian>().unwrap();
        let border_offset_top = reader.read_i16::<LittleEndian>().unwrap();
        let border_offset_bottom = reader.read_i16::<LittleEndian>().unwrap();

        let attribute_v2 = if *version >= Version::from_str("5.0.1.7") {
            reader.read_u32::<LittleEndian>().unwrap()
        } else {
            0
        };
        let single_line = if *version >= Version::from_str("5.0.1.7") {
            Some(get_value_range(attribute_v2, 0, 1) > 0)
        } else {
            None
        };
        let auto_spacing_kr_eng = if *version >= Version::from_str("5.0.1.7") {
            Some(get_flag(attribute_v2, 4))
        } else {
            None
        };
        let auto_spacing_kr_num = if *version >= Version::from_str("5.0.1.7") {
            Some(get_flag(attribute_v2, 5))
        } else {
            None
        };

        let attribute_v3 = if *version >= Version::from_str("5.0.2.5") {
            reader.read_u32::<LittleEndian>().unwrap()
        } else {
            0
        };
        let line_spacing_kind = if *version >= Version::from_str("5.0.2.5") {
            Some(LineSpacingKind::from_u32(get_value_range(attribute_v3, 0, 4)).unwrap())
        } else {
            None
        };

        let line_spacing = if *version >= Version::from_str("5.0.2.5") {
            Some(reader.read_u32::<LittleEndian>().unwrap())
        } else {
            None
        };

        Self {
            align,
            line_space_kind_old,
            break_latin_word,
            break_non_latin_word,
            snap_to_grid,
            condense,
            widow_orphan,
            keep_with_next,
            keep_lines,
            page_break_before,
            vertical_align,
            font_line_height,
            heading_kind,
            heading_level,
            border_connect,
            border_ignore_margin,
            tailing,
            padding_left,
            padding_right,
            indent,
            margin_top,
            margin_bottom,
            line_space_old,
            tab_definition_id,
            numbering_bullet_id,
            border_fill_id,
            border_offset_left,
            border_offset_right,
            border_offset_top,
            border_offset_bottom,
            single_line,
            auto_spacing_kr_eng,
            auto_spacing_kr_num,
            line_spacing_kind,
            line_spacing,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum Align {
    /// 양쪽 정렬
    Justify,
    /// 왼쪽 정렬
    Left,
    /// 오른쪽 정렬
    Right,
    /// 가운데 정렬
    Center,
    /// 배분 정렬
    Distributive,
    /// 나눔 정렬
    DistributiveSpace,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum BreakLatinWord {
    /// 단어
    KeepWord,
    /// 하이픈
    Hyphenation,
    /// 글자
    BreakWord,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum BreakNonLatinWord {
    /// 단어
    KeepWord,
    /// 글자
    BreakWord,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum VerticalAlign {
    /// 글꼴기준
    Baseline,
    /// 위쪽
    Top,
    /// 가운데
    Center,
    /// 아래
    Bottom,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum ParagraphHeadingKind {
    /// 없음
    None,
    /// 개요
    Outline,
    /// 번호
    Number,
    /// 글머리표
    Bullet,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum LineSpacingKind {
    /// 글자에 따라 (%)
    Percent,
    /// 고정값
    Fixed,
    /// 여백만 지정
    BetweenLine,
    /// 최소 (5.0.2.5 버전 이상)
    AtLeast,
}
