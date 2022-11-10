use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    doc_info::numbering::ParagraphHead,
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    version::Version,
};

#[derive(Debug)]
pub struct Bullet {
    /// 문단 머리의 정보
    pub paragraph_head: ParagraphHead,
    /// 글머리표 문자
    pub bullet_char: char,
    /// 이미지 글머리표 여부
    pub use_image: bool,
    /// 이미지 정보
    pub image: Image,
    /// 체크 글머리표 문자
    pub checked_char: char,
}

impl FromRecordCursor for Bullet {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_BULLET as u32);

        let mut reader = record.get_data_reader();
        let paragraph_head = ParagraphHead::from_reader(&mut reader, false);
        let bullet_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let use_image = reader.read_u32::<LittleEndian>().unwrap() > 0;
        let image = Image::from_reader(&mut reader);
        let checked_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        Self {
            paragraph_head,
            bullet_char,
            use_image,
            image,
            checked_char,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    /// 밝기
    pub bright: u8,
    /// 명암
    pub contrast: u8,
    /// 그림 효과
    pub effect: ImageEffect,
    /// BinItem의 아이디 참조값
    pub bin_item_id: u16,
}

impl Image {
    pub fn from_reader<T: Read>(reader: &mut T) -> Self {
        Self {
            bright: reader.read_u8().unwrap(),
            contrast: reader.read_u8().unwrap(),
            effect: ImageEffect::from_u8(reader.read_u8().unwrap()).unwrap(),
            bin_item_id: reader.read_u16::<LittleEndian>().unwrap(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ImageEffect {
    /// 원래 그림에서
    RealPic,
    /// 그레이스케일로
    GrayScale,
    /// 흑백으로
    BlackWhite,
    /// 패턴 8x8
    Pattern8x8,
}
