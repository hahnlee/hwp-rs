use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{utils::bits::get_flag, version::Version};

#[derive(Debug, Clone)]
pub struct ParagraphHeader {
    /// control mask
    pub ctrl_mask: CtrlMask,
    /// 문단 모양 아이디 참조값
    pub paragraph_shape_id: u16,
    /// 문단 스타일 아이디 참조값
    pub style_id: u8,
    /// 구역 나누기
    pub section_break: bool,
    /// 다단 나누기
    pub columns_break: bool,
    /// 쪽 나누기
    pub page_break: bool,
    /// 단 나누기
    pub column_break: bool,
    /// 글자수
    pub chars: u32,
    /// 글자 모양 정보 수
    pub char_shapes: u16,
    /// range tag 정보 수
    pub ranges: u16,
    /// 각 줄에 대한 align에 대한 정보 수
    pub aligns: u16,
    pub instance_id: u32,
    /// 변경추적 병합 문단여부. (5.0.3.2 버전 이상)
    pub tracking_change_merged: Option<u16>,
}

impl ParagraphHeader {
    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Self {
        let mut chars = reader.read_u32::<LittleEndian>().unwrap();

        if (chars & 0x80000000) == 0x80000000 {
            chars &= 0x7fffffff;
        }

        let ctrl_mask = CtrlMask::from_u32(reader.read_u32::<LittleEndian>().unwrap());

        let paragraph_shape_id = reader.read_u16::<LittleEndian>().unwrap();

        let style_id = reader.read_u8().unwrap();

        let break_options = reader.read_u8().unwrap();
        let section_break = get_flag(break_options, 0);
        let columns_break = get_flag(break_options, 1);
        let page_break = get_flag(break_options, 2);
        let column_break = get_flag(break_options, 3);

        let char_shapes = reader.read_u16::<LittleEndian>().unwrap();

        let ranges = reader.read_u16::<LittleEndian>().unwrap();

        let aligns = reader.read_u16::<LittleEndian>().unwrap();

        let instance_id = reader.read_u32::<LittleEndian>().unwrap();

        let tracking_change_merged = if *version >= Version::from_str("5.0.3.2") {
            Some(reader.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        Self {
            ctrl_mask,
            paragraph_shape_id,
            style_id,
            section_break,
            columns_break,
            page_break,
            column_break,
            chars,
            char_shapes,
            ranges,
            aligns,
            instance_id,
            tracking_change_merged,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CtrlMask {
    /// 구역/단 정의
    pub section_column_definition: bool,
    /// 필드시작
    pub field_start: bool,
    /// 필드끝
    pub field_end: bool,
    /// 탭
    pub tab: bool,
    /// 강제 줄 나눔
    pub line_break: bool,
    /// 그리기 개체 / 표
    pub shape_object_table: bool,
    /// 문단 나누기
    pub paragraph_break: bool,
    /// 주석
    pub hidden_comment: bool,
    /// 머리말 / 꼬리말 존재 여부
    pub header_footer: bool,
    /// 각주 / 미주
    pub headnote_footnote: bool,
    /// 자동 번호
    pub auto_number: bool,
    /// 쪽바뀜
    pub page_break: bool,
    /// 책갈피 / 찾아보기 표시
    pub book_mark_index_mark: bool,
    /// 덧말 / 글자 겹침
    pub sub_text: bool,
    /// 하이픈
    pub hyphen: bool,
    /// 묶음 빈칸
    pub keep_word_space: bool,
    /// 고정 폭 빈칸
    pub fixed_width_space: bool,
}

impl CtrlMask {
    pub fn from_u32(bits: u32) -> Self {
        Self {
            section_column_definition: get_flag(bits, 2),
            field_start: get_flag(bits, 3),
            field_end: get_flag(bits, 4),
            tab: get_flag(bits, 9),
            line_break: get_flag(bits, 10),
            shape_object_table: get_flag(bits, 11),
            paragraph_break: get_flag(bits, 13),
            hidden_comment: get_flag(bits, 15),
            header_footer: get_flag(bits, 16),
            headnote_footnote: get_flag(bits, 17),
            auto_number: get_flag(bits, 18),
            page_break: get_flag(bits, 21),
            book_mark_index_mark: get_flag(bits, 22),
            sub_text: get_flag(bits, 23),
            hyphen: get_flag(bits, 24),
            keep_word_space: get_flag(bits, 30),
            fixed_width_space: get_flag(bits, 31),
        }
    }
}
