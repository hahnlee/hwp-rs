use std::io::Seek;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::BodyTextRecord, Record, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

use super::paragraph_list::ParagraphList;

/// 개체 공통 속성
#[derive(Debug, Clone)]
pub struct CommonProperties {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 글자처럼 취급 여부
    pub treat_as_char: bool,
    /// 줄 간격에 영향을 줄지 여부
    /// `treat_as_char` 속성이 true일 때에만 사용한다
    pub affect_letter_spacing: bool,
    /// 세로 위치의 기준
    /// `treat_as_char` 속성이 true일 때에만 사용한다
    pub vertical_relative_to: VerticalRelativeTo,
    /// 세로 위치의 기준에 대한 상대적인 배열 방식
    /// `treat_as_char` 속성이 true일 때에만 사용한다
    pub vertical_align: Option<Align>,
    /// 가로 위치의 기준
    /// `treat_as_char` 속성이 true일 때에만 사용한다
    pub horizontal_relative_to: HorizontalRelativeTo,
    /// 가로 위치의 기준에 대한 상대적인 배열 방식
    /// `treat_as_char` 속성이 true일 때에만 사용한다
    pub horizontal_align: Option<Align>,
    /// 오브젝트의 세로 위치를 본문 영역으로 제한할지 여부
    /// `vertical_relative_to` 속성이 Paragraph 일때만 사용한다
    pub flow_with_text: Option<bool>,
    /// 다른 오브젝트와 겹치는 것을 허용할지 여부
    /// `treat_as_char` 속성이 false일 때에만 사용한다
    /// `flow_with_text` 속성이 true면 무조건 false로 간주함
    pub allow_overlap: Option<bool>,
    /// 오브젝트 폭의 기준
    pub width_relative_to: WidthRelativeTo,
    /// 오브젝트 높이의 기준
    pub height_relative_to: HeightRelativeTo,
    /// 크기 보호 여부
    /// `vertical_relative_to` 속성이 Paragraph 일때만 사용한다
    pub protect: Option<bool>,
    /// 오브젝트 주위를 텍스트가 어떻게 흘러갈지 지정하는 옵션
    /// `treat_as_char` 속성이 false일 때에만 사용한다
    pub text_wrap: Option<TextWrap>,
    /// 오브젝트의 좌/우 어느 쪽에 글을 배치할지 지정하는 옵션
    /// `text_wrap` 속성이 Square, Tight, Through 일때 사용한다
    pub text_flow: Option<TextFlow>,
    /// 이 개체가 속하는 번호 범주
    pub numbering_kind: NumberingKind,
    /// 오프셋
    pub offset: Offset,
    /// 오브젝트의 폭
    pub width: u32,
    /// 오브젝트의 높이
    pub height: u32,
    /// z-order
    pub z_order: i32,
    /// 오브젝트의 바깥 4방향 여백
    pub margin: [i16; 4],
    /// 문서 내 각 개체에 대한 고유 아이디
    pub instance_id: u32,
    /// 쪽 나눔 방지
    pub prevent_page_break: bool,
    /// 개체 설명문
    pub description: String,
    /// 캡션
    pub caption: Option<Caption>,
}

impl CommonProperties {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let size = record.data.len() as u64;
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let treat_as_char = get_flag(attribute, 0);
        let affect_letter_spacing = get_flag(attribute, 2);
        let vertical_relative_to =
            VerticalRelativeTo::from_u32(get_value_range(attribute, 3, 4)).unwrap();
        let vertical_align = if treat_as_char {
            Some(map_to_align(
                get_value_range(attribute, 5, 7),
                vertical_relative_to.clone() as u8,
            ))
        } else {
            None
        };
        let horizontal_relative_to =
            HorizontalRelativeTo::from_u32(get_value_range(attribute, 8, 9)).unwrap();
        let horizontal_align = if treat_as_char {
            Some(map_to_align(
                get_value_range(attribute, 10, 12),
                horizontal_relative_to.clone() as u8,
            ))
        } else {
            None
        };
        let flow_with_text = if vertical_relative_to == VerticalRelativeTo::Paragraph {
            Some(get_flag(attribute, 13))
        } else {
            None
        };
        let allow_overlap = if treat_as_char {
            None
        } else if flow_with_text.is_some() && flow_with_text.unwrap() == true {
            Some(false)
        } else {
            Some(get_flag(attribute, 14))
        };
        let width_relative_to =
            WidthRelativeTo::from_u32(get_value_range(attribute, 15, 17)).unwrap();
        let height_relative_to =
            HeightRelativeTo::from_u32(get_value_range(attribute, 18, 19)).unwrap();
        let protect = if vertical_relative_to == VerticalRelativeTo::Paragraph {
            Some(get_flag(attribute, 20))
        } else {
            None
        };
        let text_wrap = if treat_as_char {
            None
        } else {
            Some(TextWrap::from_u32(get_value_range(attribute, 21, 23)).unwrap())
        };
        let text_flow = match text_wrap {
            Some(TextWrap::Square) | Some(TextWrap::Tight) | Some(TextWrap::Through) => {
                Some(TextFlow::from_u32(get_value_range(attribute, 24, 25)).unwrap())
            }
            _ => None,
        };

        // NOTE: (@hahnlee) 배포용 문서에서 넘어가는 경우가 있음. 확인한 문서에서는 mod 값과 동일함
        let numbering_kind =
            NumberingKind::from_u32(get_value_range(attribute, 26, 28) % 4).unwrap();

        let offset = Offset {
            vertical: reader.read_u32::<LittleEndian>().unwrap(),
            horizontal: reader.read_u32::<LittleEndian>().unwrap(),
        };

        let width = reader.read_u32::<LittleEndian>().unwrap();
        let height = reader.read_u32::<LittleEndian>().unwrap();
        let z_order = reader.read_i32::<LittleEndian>().unwrap();

        let margin = [
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
            reader.read_i16::<LittleEndian>().unwrap(),
        ];

        let instance_id = reader.read_u32::<LittleEndian>().unwrap();

        let prevent_page_break = reader.read_i32::<LittleEndian>().unwrap() == 0;

        // NOTE: (@hahnlee) len이 0이 아니라 아예 값이 없을 수도 있다
        let description = if reader.stream_position().unwrap() < size {
            reader.read_string::<LittleEndian>().unwrap()
        } else {
            format!("")
        };

        assert_eq!(
            reader.stream_position().unwrap(),
            size as u64,
            "안읽은 바이트가 있습니다"
        );

        let caption = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(Caption::from_record_cursor(cursor, version))
        } else {
            None
        };

        Self {
            ctrl_id,
            treat_as_char,
            affect_letter_spacing,
            vertical_relative_to,
            vertical_align,
            horizontal_relative_to,
            horizontal_align,
            flow_with_text,
            allow_overlap,
            width_relative_to,
            height_relative_to,
            protect,
            text_wrap,
            text_flow,
            numbering_kind,
            offset,
            width,
            height,
            z_order,
            margin,
            instance_id,
            prevent_page_break,
            description,
            caption,
        }
    }
}

/// 세로 위치의 기준
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum VerticalRelativeTo {
    Paper,
    Page,
    Paragraph,
}

/// 배열 방식
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Align {
    Top,
    Center,
    Bottom,
    Left,
    Right,
    Inside,
    Outside,
}

fn map_to_align(value: u32, rel_to: u8) -> Align {
    if rel_to == 0 || rel_to == 1 {
        match value {
            0 => Align::Top,
            1 => Align::Center,
            2 => Align::Bottom,
            3 => Align::Inside,
            4 => Align::Outside,
            _ => panic!("잘못된 값입니다."),
        }
    } else {
        match value {
            0 => Align::Left,
            2 => Align::Right,
            _ => panic!("잘못된 값입니다."),
        }
    }
}

/// 가로 배열 방식
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum HorizontalRelativeTo {
    Paper,
    Page,
    Column,
    Paragraph,
}

/// 오브젝트 폭의 기준
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum WidthRelativeTo {
    Paper,
    Page,
    Column,
    Paragraph,
    Absolute,
}

/// 오브젝트 높이의 기준
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum HeightRelativeTo {
    Paper,
    Page,
    Absolute,
}

/// 오브젝트 주위를 텍스트가 어떻게 흘러갈지 지정하는 옵션
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum TextWrap {
    /// bound rect를 따라
    Square,
    /// 오브젝트의 outline을 따라
    Tight,
    /// 오브젝트 내부의 빈 공간까지
    Through,
    /// 좌, 우에는 텍스트를 배치하지 않음
    TopAndBottom,
    /// 글과 겹치게 하여 글 뒤로
    BehindText,
    /// 글과 겹치게 하여 글 앞으로
    InFrontOfText,
}

/// 오브젝트의 좌/우 어느 쪽에 글을 배치할지
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum TextFlow {
    BothSides,
    LeftOnly,
    RightOnly,
    LargestOnly,
}

/// 이 개체가 속하는 번호 범주
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum NumberingKind {
    None,
    Figure,
    Table,
    Equation,
}

#[derive(Debug, Clone)]
pub struct Offset {
    pub vertical: u32,
    pub horizontal: u32,
}

#[derive(Debug, Clone)]
pub struct Caption {
    /// 문단 리스트
    pub paragraph_list: ParagraphList,
    /// 방향
    pub align: CaptionAlign,
    /// 캡션 폭에 마진을 포함할 지 여부 (가로 방향일 때만 사용)
    pub full_size: bool,
    /// 캡션 폭(세로 방향일 때만 사용)
    pub width: u32,
    /// 캡션과 틀 사이 간격
    pub gap: i16,
    /// 텍스트의 최대 길이(=개체의 폭)
    pub last_width: u32,
}

impl Caption {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();

        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_LIST_HEADER as u32,
            "다른 레코드 입니다"
        );

        let mut reader = record.get_data_reader();

        let paragraph_list = ParagraphList::from_reader(&mut reader, cursor, version);

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let align = CaptionAlign::from_u32(get_value_range(attribute, 0, 1)).unwrap();
        let full_size = get_flag(attribute, 2);

        let width = reader.read_u32::<LittleEndian>().unwrap();
        let gap = reader.read_i16::<LittleEndian>().unwrap();
        let last_width = reader.read_u32::<LittleEndian>().unwrap();

        Self {
            paragraph_list,
            align,
            full_size,
            width,
            gap,
            last_width,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum CaptionAlign {
    Left,
    Right,
    Top,
    Bottom,
}
