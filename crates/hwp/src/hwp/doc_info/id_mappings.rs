use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{tags::DocInfoRecord, Record, read_items},
    version::Version,
};

use super::{bin_data::BinData, font::Font};

#[derive(Debug)]
pub struct IDMappings {
    /// 바이너리 데이터
    pub binary_data: Vec<BinData>,
    // /// 한글 글꼴
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
    // /// 테두리/배경
    // pub border_fils: i32,
    // /// 글자 모양
    // pub char_shapes: i32,
    // /// 탭 정의
    // pub tab_definitions: i32,
    // /// 문단 번호
    // pub numbering: i32,
    // /// 글머리표
    // pub bullets: i32,
    // /// 문단 모양
    // pub paragraph_shapes: i32,
    // /// 스타일(문단 스타일)
    // pub styles: i32,
    // /// 메모 모양 (5.0.2.1 이상)
    // pub memo_shapes: i32,
    // /// 변경추적 (5.0.3.2 이상)
    // pub change_trackings: i32,
    // /// 변경추적 사용자 (5.0.3.2 이상)
    // pub change_tracking_users: i32,
}

impl IDMappings {
    pub fn from_record(record: &mut Record, version: &Version) -> IDMappings {
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

        let binary_data_len = reader.read_i32::<LittleEndian>().unwrap();
        let korean_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let english_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let chinese_characters_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let japanese_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let etc_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let symbol_fonts_len = reader.read_i32::<LittleEndian>().unwrap();
        let user_fonts_len = reader.read_i32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee)
        // let border_fils = reader.read_i32::<LittleEndian>().unwrap();
        // let char_shapes = reader.read_i32::<LittleEndian>().unwrap();
        // let tab_definitions = reader.read_i32::<LittleEndian>().unwrap();
        // let numbering = reader.read_i32::<LittleEndian>().unwrap();
        // let bullets = reader.read_i32::<LittleEndian>().unwrap();
        // let paragraph_shapes = reader.read_i32::<LittleEndian>().unwrap();
        // let styles = reader.read_i32::<LittleEndian>().unwrap();

        // let memo_shapes = if *version >= memo_supported_version {
        //     reader.read_i32::<LittleEndian>().unwrap()
        // } else {
        //     0
        // };

        // let change_trackings = if version.ge(&tracking_supported_version) {
        //     reader.read_i32::<LittleEndian>().unwrap()
        // } else {
        //     0
        // };

        // let change_tracking_users = if version.ge(&tracking_supported_version) {
        //     reader.read_i32::<LittleEndian>().unwrap()
        // } else {
        //     0
        // };

        let binary_data = read_items::<BinData>(record,  version, binary_data_len as usize);

        let korean_fonts = read_items::<Font>(record, version, korean_fonts_len as usize);
        let english_fonts = read_items::<Font>(record, version, english_fonts_len as usize);
        let chinese_characters_fonts = read_items::<Font>(record, version, chinese_characters_fonts_len as usize);
        let japanese_fonts = read_items::<Font>(record, version, japanese_fonts_len as usize);
        let etc_fonts = read_items::<Font>(record, version, etc_fonts_len as usize);
        let symbol_fonts = read_items::<Font>(record, version, symbol_fonts_len as usize);
        let user_fonts = read_items::<Font>(record, version, user_fonts_len as usize);

        IDMappings {
            binary_data,
            korean_fonts,
            english_fonts,
            chinese_characters_fonts,
            japanese_fonts,
            etc_fonts,
            symbol_fonts,
            user_fonts,
            // border_fils,
            // char_shapes,
            // tab_definitions,
            // numbering,
            // bullets,
            // paragraph_shapes,
            // styles,
            // memo_shapes,
            // change_trackings,
            // change_tracking_users,
        }
    }
}
