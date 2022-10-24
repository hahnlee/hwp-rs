use std::io::Seek;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::BodyTextRecord, Record},
    utils::bits::{get_value_range, get_flag},
    version::Version,
};

use super::paragraph_list::ParagraphList;

/// 개체 공통 속성
#[derive(Debug, Clone)]
pub struct CommonProperties {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    // 오프셋
    pub offset: Offset,
    /// width 오브젝트의 폭
    pub width: u32,
    /// height 오브젝트의 높이
    pub height: u32,
    /// z-index
    pub z_order: i32,
    /// 문서 내 각 개체에 대한 고유 아이디
    pub instance_id: u32,
    /// 개체 설명문
    pub description: String,
    /// 캡션
    pub caption: Option<Caption>,
}

impl CommonProperties {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        let size = record.data.len() as u64;
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        // 속성
        reader.read_u32::<LittleEndian>().unwrap();

        // 세로 오프셋 값
        let offset = Offset {
            vertical: reader.read_u32::<LittleEndian>().unwrap(),
            horizontal: reader.read_u32::<LittleEndian>().unwrap(),
        };

        let width = reader.read_u32::<LittleEndian>().unwrap();
        let height = reader.read_u32::<LittleEndian>().unwrap();
        let z_order = reader.read_i32::<LittleEndian>().unwrap();

        // 2x4 오브젝트의 바깥 4방향 여백 방향확인 필요
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();
        reader.read_i16::<LittleEndian>().unwrap();

        let instance_id = reader.read_u32::<LittleEndian>().unwrap();

        // 쪽나눔 방지 on(1) / off(0)
        reader.read_i32::<LittleEndian>().unwrap();

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

        let caption = if record.is_next_child_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(Caption::from_record(record, version))
        } else {
            None
        };

        Self {
            ctrl_id,
            offset,
            width,
            height,
            z_order,
            instance_id,
            description,
            caption,
        }
    }
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
    pub include_margin: bool,
}

impl Caption {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        let meta = record.next_child();

        assert_eq!(
            meta.tag_id,
            BodyTextRecord::HWPTAG_LIST_HEADER as u32,
            "다른 레코드 입니다"
        );

        let mut reader = meta.get_data_reader();

        let paragraph_list = ParagraphList::from_record(&mut reader, record, version);

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let align = CaptionAlign::from_u32(get_value_range(attribute, 0, 1)).unwrap();
        let include_margin = get_flag(attribute, 2);

        // TODO: (@hahnlee) 남은데이터 파싱하기
        // NOTE: (@hahnlee) 바이트가 문서와 다름...
        Self {
            paragraph_list,
            align,
            include_margin,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, FromPrimitive)]
pub enum CaptionAlign {
    Left,
    Right,
    Top,
    Bottom,
}
