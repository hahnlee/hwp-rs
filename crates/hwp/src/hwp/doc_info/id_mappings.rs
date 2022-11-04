use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{read_items, tags::DocInfoRecord, RecordCursor},
    version::Version,
};

use super::{
    bin_data::BinData, border_fill::BorderFill, bullet::Bullet, change_tracking::ChangeTracking,
    change_tracking_author::ChangeTrackingAuthor, char_shape::CharShape, font::Font,
    memo_shape::MemoShape, numbering::Numbering, paragraph_shape::ParagraphShape, style::Style,
    tab_definition::TabDefinition,
};

#[derive(Debug)]
pub struct IDMappings {
    /// 바이너리 데이터
    pub binary_data: Vec<BinData>,
    /// 한글 글꼴
    pub korean_fonts: Vec<Font>,
    /// 영어 글꼴
    pub english_fonts: Vec<Font>,
    /// 한자 글꼴
    pub chinese_characters_fonts: Vec<Font>,
    /// 일어 글꼴
    pub japanese_fonts: Vec<Font>,
    /// 기타 글꼴
    pub etc_fonts: Vec<Font>,
    /// 기호 글꼴
    pub symbol_fonts: Vec<Font>,
    /// 사용자 글꼴
    pub user_fonts: Vec<Font>,
    /// 테두리/배경
    pub border_fills: Vec<BorderFill>,
    /// 글자 모양
    pub char_shapes: Vec<CharShape>,
    /// 탭 정의
    pub tab_definitions: Vec<TabDefinition>,
    /// 문단 번호
    pub numberings: Vec<Numbering>,
    /// 글머리표
    pub bullets: Vec<Bullet>,
    /// 문단 모양
    pub paragraph_shapes: Vec<ParagraphShape>,
    /// 스타일(문단 스타일)
    pub styles: Vec<Style>,
    /// 메모 모양 (5.0.2.1 이상)
    pub memo_shapes: Vec<MemoShape>,
    /// 변경추적 (5.0.3.2 이상)
    pub change_trackings: Vec<ChangeTracking>,
    /// 변경추적 사용자 (5.0.3.2 이상)
    pub change_tracking_authors: Vec<ChangeTrackingAuthor>,
}

impl IDMappings {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_ID_MAPPINGS as u32,
            "올바르지 않은 정보"
        );

        let memo_supported_version = Version::from_str("5.0.2.1");
        let tracking_supported_version = Version::from_str("5.0.3.2");

        let target_size: u32 = {
            if version.lt(&memo_supported_version) {
                60
            } else if version.lt(&tracking_supported_version) {
                64
            } else {
                72
            }
        };

        assert_eq!(record.size, target_size, "올바르지 않은 정보");

        let mut reader = record.get_data_reader();

        let binary_data = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let korean_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let english_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let chinese_characters_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let japanese_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let etc_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let symbol_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let user_fonts = reader.read_i32::<LittleEndian>().unwrap() as usize;

        let border_fills = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let char_shapes = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let tab_definitions = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let numberings = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let bullets = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let paragraph_shapes = reader.read_i32::<LittleEndian>().unwrap() as usize;
        let styles = reader.read_i32::<LittleEndian>().unwrap() as usize;

        let memo_shapes = if *version >= memo_supported_version {
            reader.read_i32::<LittleEndian>().unwrap() as usize
        } else {
            0
        };

        let change_trackings = if version.ge(&tracking_supported_version) {
            reader.read_i32::<LittleEndian>().unwrap() as usize
        } else {
            0
        };

        let change_tracking_authors = if version.ge(&tracking_supported_version) {
            reader.read_i32::<LittleEndian>().unwrap() as usize
        } else {
            0
        };

        Self {
            binary_data: read_items(cursor, version, binary_data),
            korean_fonts: read_items(cursor, version, korean_fonts),
            english_fonts: read_items(cursor, version, english_fonts),
            chinese_characters_fonts: read_items(cursor, version, chinese_characters_fonts),
            japanese_fonts: read_items(cursor, version, japanese_fonts),
            etc_fonts: read_items(cursor, version, etc_fonts),
            symbol_fonts: read_items(cursor, version, symbol_fonts),
            user_fonts: read_items(cursor, version, user_fonts),
            border_fills: read_items(cursor, version, border_fills),
            char_shapes: read_items(cursor, version, char_shapes),
            tab_definitions: read_items(cursor, version, tab_definitions),
            numberings: read_items(cursor, version, numberings),
            bullets: read_items(cursor, version, bullets),
            paragraph_shapes: read_items(cursor, version, paragraph_shapes),
            styles: read_items(cursor, version, styles),
            memo_shapes: read_items(cursor, version, memo_shapes),
            change_trackings: read_items(cursor, version, change_trackings),
            change_tracking_authors: read_items(cursor, version, change_tracking_authors),
        }
    }
}
