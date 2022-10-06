use std::io::Seek;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{reader::RecordReader, tags::BodyTextRecord, Record},
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

        // TODO: (@hahnlee) 더 많은 정보 얻기
        Self { paragraph_list }
    }
}
