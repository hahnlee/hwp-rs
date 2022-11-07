use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::get_flag,
    version::Version,
};

#[derive(Debug)]
pub struct Font {
    /// 글꼴 이름
    pub name: String,
    /// 기본 글꼴 이름
    pub default_font_name: Option<String>,
    /// 글꼴유형정보
    pub panose: Option<Panose>,
    /// 대체 글꼴 유형
    pub alternative_kind: Option<AlternativeKind>,
    /// 대체 글꼴 이름
    pub alternative_font_name: Option<String>,
}

impl FromRecordCursor for Font {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();

        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_FACE_NAME as u32,
            "올바르지 않은 정보"
        );

        let mut reader = record.get_data_reader();

        let properties = reader.read_u8().unwrap();
        let name = reader.read_string::<LittleEndian>().unwrap();

        let has_alternative = get_flag(properties, 7);
        let has_panose = get_flag(properties, 6);
        let has_default_font = get_flag(properties, 5);

        let alternative_kind = if has_alternative {
            Some(AlternativeKind::from_u8(reader.read_u8().unwrap()).unwrap())
        } else {
            None
        };
        let alternative_font_name = if has_alternative {
            Some(reader.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let panose = if has_panose {
            Some(Panose::from_reader(&mut reader))
        } else {
            None
        };

        let default_font_name = if has_default_font {
            Some(reader.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        Self {
            name,
            default_font_name,
            panose,
            alternative_kind,
            alternative_font_name,
        }
    }
}

/// https://en.wikipedia.org/wiki/PANOSE
/// https://monotype.github.io/panose/pan1.htm
#[derive(Debug)]
pub struct Panose {
    /// 글꼴 계열
    pub kind: u8,
    /// 세리프 유형
    pub serif_style: u8,
    /// 굵기
    pub weight: u8,
    /// 비례
    pub proportion: u8,
    /// 대조
    pub contrast: u8,
    /// 스트로크 편차
    pub stroke_variation: u8,
    /// 자획 유형
    pub arm_style: u8,
    /// 글자형
    pub letterform: u8,
    /// 중간선
    pub midline: u8,
    /// X-높이
    pub x_height: u8,
}

impl Panose {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            kind: reader.read_u8().unwrap(),
            serif_style: reader.read_u8().unwrap(),
            weight: reader.read_u8().unwrap(),
            proportion: reader.read_u8().unwrap(),
            contrast: reader.read_u8().unwrap(),
            stroke_variation: reader.read_u8().unwrap(),
            arm_style: reader.read_u8().unwrap(),
            letterform: reader.read_u8().unwrap(),
            midline: reader.read_u8().unwrap(),
            x_height: reader.read_u8().unwrap(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum AlternativeKind {
    /// 원래 종류를 알 수 없을 때
    Unknown,
    /// 트루타입 글꼴
    TTF,
    /// 한/글 전용 글꼴
    HFT,
}
