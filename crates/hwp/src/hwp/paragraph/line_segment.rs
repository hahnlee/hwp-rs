use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::utils::bits::get_flag;

#[derive(Debug, Clone)]
pub struct LineSegment {
    /// 텍스트 시작 위치
    pub start_position: u32,
    /// 줄의 세로 위치
    pub vertical_position: i32,
    /// 줄의 높이
    pub line_height: i32,
    /// 텍스트 부분의 높이
    pub text_height: i32,
    /// 줄의 세로 위치에서 베이스라인까지 거리
    pub base_line_gap: i32,
    /// 줄간격
    pub line_spacing: i32,
    /// 컬럼에서의 시작 위치
    pub start_position_in_column: i32,
    /// 세그먼트의 폭
    pub width: i32,
    /// 페이지의 첫 줄인지 여부
    pub is_first_line_in_page: bool,
    /// 컬럼의 첫 줄인지 여부
    pub is_first_line_in_column: bool,
    /// 텍스트가 배열되지 않은 빈 세그먼트인지 여부
    pub is_empty: bool,
    /// 줄의 첫 세그먼트인지 여부
    pub is_first: bool,
    /// 줄의 마지막 세그먼트인지 여부
    pub is_last: bool,
    /// 줄의 마지막에 auto-hyphenation이 수행되었는지 여부.
    pub auto_hyphenated: bool,
    /// indentation 적용
    pub indented: bool,
    /// 문단 머리 모양 적용
    pub use_heading: bool,
}

impl LineSegment {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        let start_position = reader.read_u32::<LittleEndian>().unwrap();
        let vertical_position = reader.read_i32::<LittleEndian>().unwrap();
        let line_height = reader.read_i32::<LittleEndian>().unwrap();
        let text_height = reader.read_i32::<LittleEndian>().unwrap();
        let base_line_gap = reader.read_i32::<LittleEndian>().unwrap();
        let line_spacing = reader.read_i32::<LittleEndian>().unwrap();
        let start_position_in_column = reader.read_i32::<LittleEndian>().unwrap();
        let width = reader.read_i32::<LittleEndian>().unwrap();

        let tag = reader.read_u32::<LittleEndian>().unwrap();
        let is_first_line_in_page = get_flag(tag, 0);
        let is_first_line_in_column = get_flag(tag, 1);
        let is_empty = get_flag(tag, 16);
        let is_first = get_flag(tag, 17);
        let is_last = get_flag(tag, 18);
        let auto_hyphenated = get_flag(tag, 19);
        let indented = get_flag(tag, 20);
        let use_heading = get_flag(tag, 21);

        Self {
            start_position,
            vertical_position,
            line_height,
            text_height,
            base_line_gap,
            line_spacing,
            start_position_in_column,
            width,
            is_first_line_in_page,
            is_first_line_in_column,
            is_empty,
            is_first,
            is_last,
            auto_hyphenated,
            indented,
            use_heading,
        }
    }
}
