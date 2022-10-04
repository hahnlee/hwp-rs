use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{record::{tags::BodyTextRecord, Record}, utils::bits::{get_value, get_value_range}};

/// 페이지 정의
#[derive(Debug, Clone)]
pub struct PageDefinition {
    /// 용지 가로 크기
    pub width: u32,
    /// 용지 세로 크기
    pub height: u32,
    /// 여백
    pub padding: Padding,
    // TODO: (@hahnlee) enum
    /// 용지방향
    pub direction: u32,
    /// 제책 방법
    pub binding_method: u32,
}

impl PageDefinition {
    pub fn from_record(record: Record) -> PageDefinition {
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_PAGE_DEF as u32,
            "잘못된 레코드 입니다"
        );

        let mut reader = record.get_data_reader();

        let width = reader.read_u32::<LittleEndian>().unwrap();
        let height = reader.read_u32::<LittleEndian>().unwrap();
        let padding = Padding {
            left: reader.read_u32::<LittleEndian>().unwrap(),
            right: reader.read_u32::<LittleEndian>().unwrap(),
            top: reader.read_u32::<LittleEndian>().unwrap(),
            bottom: reader.read_u32::<LittleEndian>().unwrap(),
            header: reader.read_u32::<LittleEndian>().unwrap(),
            footer: reader.read_u32::<LittleEndian>().unwrap(),
            binding: reader.read_u32::<LittleEndian>().unwrap(),
        };

        let properties = reader.read_u32::<LittleEndian>().unwrap();
        let direction = get_value(properties, 0);
        let binding_method = get_value_range(properties, 1, 2);

        PageDefinition {
            width,
            height,
            padding,
            direction,
            binding_method,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Padding {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
    /// 머리말 여백
    pub header: u32,
    /// 꼬리말 여백
    pub footer: u32,
    /// 제본 여백
    pub binding: u32,
}
