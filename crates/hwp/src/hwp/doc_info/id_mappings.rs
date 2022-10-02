use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord},
    version::Version,
};

#[derive(Debug)]
pub struct IDMappings {
    /// 바이너리 데이터
    pub binary_data: i32,
    /// 한글 글꼴
    pub korean_fonts: i32,
    /// 영어 글꼴
    pub english_fonts: i32,
    /// 한자 글꼴
    pub chinese_characters_fonts: i32,
    /// 일어 글꼴
    pub japanese_fonts: i32,
    /// 기타 글꼴
    pub etc_fonts: i32,
    /// 기호 글꼴
    pub symbol_fonts: i32,
    /// 사용자 글꼴
    pub user_fonts: i32,
    /// 테두리/배경
    pub border_fils: i32,
    /// 글자 모양
    pub char_shapes: i32,
    /// 탭 정의
    pub tab_definitions: i32,
    /// 문단 번호
    pub numbering: i32,
    /// 글머리표
    pub bullets: i32,
    /// 문단 모양
    pub paragraph_shapes: i32,
    /// 스타일(문단 스타일)
    pub styles: i32,
    /// 메모 모양 (5.0.2.1 이상)
    pub memo_shapes: i32,
    /// 변경추적 (5.0.3.2 이상)
    pub change_trackings: i32,
    /// 변경추적 사용자 (5.0.3.2 이상)
    pub change_tracking_users: i32,
}

impl IDMappings {
    pub fn from_reader<T: Read>(stream: &mut T, version: &Version) -> IDMappings {
        let (tag_id, _, size, mut data) = stream.read_record::<LittleEndian>().unwrap();
        if tag_id != DocInfoRecord::HWPTAG_ID_MAPPINGS as u32 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

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

        if size != target_size {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        let binary_data = data.read_i32::<LittleEndian>().unwrap();
        let korean_fonts = data.read_i32::<LittleEndian>().unwrap();
        let english_fonts = data.read_i32::<LittleEndian>().unwrap();
        let chinese_characters_fonts = data.read_i32::<LittleEndian>().unwrap();
        let japanese_fonts = data.read_i32::<LittleEndian>().unwrap();
        let etc_fonts = data.read_i32::<LittleEndian>().unwrap();
        let symbol_fonts = data.read_i32::<LittleEndian>().unwrap();
        let user_fonts = data.read_i32::<LittleEndian>().unwrap();
        let border_fils = data.read_i32::<LittleEndian>().unwrap();
        let char_shapes = data.read_i32::<LittleEndian>().unwrap();
        let tab_definitions = data.read_i32::<LittleEndian>().unwrap();
        let numbering = data.read_i32::<LittleEndian>().unwrap();
        let bullets = data.read_i32::<LittleEndian>().unwrap();
        let paragraph_shapes = data.read_i32::<LittleEndian>().unwrap();
        let styles = data.read_i32::<LittleEndian>().unwrap();

        let memo_shapes = if *version >= memo_supported_version {
            data.read_i32::<LittleEndian>().unwrap()
        } else {
            0
        };

        let change_trackings = if version.ge(&tracking_supported_version) {
            data.read_i32::<LittleEndian>().unwrap()
        } else {
            0
        };

        let change_tracking_users = if version.ge(&tracking_supported_version) {
            data.read_i32::<LittleEndian>().unwrap()
        } else {
            0
        };

        IDMappings {
            binary_data,
            korean_fonts,
            english_fonts,
            chinese_characters_fonts,
            japanese_fonts,
            etc_fonts,
            symbol_fonts,
            user_fonts,
            border_fils,
            char_shapes,
            tab_definitions,
            numbering,
            bullets,
            paragraph_shapes,
            styles,
            memo_shapes,
            change_trackings,
            change_tracking_users,
        }
    }

    pub fn fonts_count(&self) -> i32 {
        let count = self.korean_fonts
            + self.english_fonts
            + self.chinese_characters_fonts
            + self.japanese_fonts
            + self.etc_fonts
            + self.user_fonts
            + self.symbol_fonts;

        count
    }
}
