use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord, Record, FromRecord},
    utils::bits::get_flag, version::Version,
};

#[derive(Debug)]
pub struct Font {
    /// 글꼴 이름
    pub name: String,
    /// 기본 글꼴 이름
    pub default_font_name: Option<String>,
    // TODO: (@hahnlee) panose 자료형 만들기
    /// 글꼴유형정보
    pub panose: Option<[u8; 10]>,
    // TODO: (@hahnlee) 자료형 만드는것 검토, enum화
    /// 대체 글꼴 유형
    pub alternative_type: Option<u8>,
    /// 대체 글꼴 이름
    pub alternative_font_name: Option<String>,
}

impl FromRecord for Font {
    fn from_record(record: &mut Record, _: &Version) -> Font {
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_FACE_NAME as u32,
            "올바르지 않은 정보"
        );

        let mut data = record.get_data_reader();

        let properties = data.read_u8().unwrap();
        let name = data.read_string::<LittleEndian>().unwrap();

        let has_alternative = get_flag(properties, 7);
        let has_panose = get_flag(properties, 6);
        let has_default_font = get_flag(properties, 5);

        let alternative_type = if has_alternative {
            Some(data.read_u8().unwrap())
        } else {
            None
        };
        let alternative_font_name = if has_alternative {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let panose = if has_panose {
            let mut panose = [0u8; 10];
            data.read_exact(&mut panose).unwrap();
            Some(panose)
        } else {
            None
        };

        let default_font_name = if has_default_font {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        Font {
            name,
            default_font_name,
            panose,
            alternative_type,
            alternative_font_name,
        }
    }
}
