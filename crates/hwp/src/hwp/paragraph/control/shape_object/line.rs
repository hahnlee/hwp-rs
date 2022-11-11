use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

/// 선
#[derive(Debug, Clone)]
pub struct ShapeLineControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
    /// 컨텐츠
    pub content: LineRecord,
}

impl ShapeLineControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        let content = LineRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            draw_text,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineRecord {
    /// 시작점 X 좌표
    pub start_x: i32,
    /// 시작점 Y 좌표
    pub start_y: i32,
    /// 끝점 X 좌표
    pub end_x: i32,
    /// 끝점 Y 좌표
    pub end_y: i32,
    /// 처음 생성 시 수직 또는 수평선일 때, 선의 방향이 언제나
    /// 오른쪽(위쪽)으로 잡힘으로 인한 현상 때문에, 방향을 바로
    /// 잡아주기 위한 플래그.
    pub is_reverse_hs: Option<bool>,
    pub unknown: Vec<u8>,
}

impl LineRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_LINE as u32
        );

        let mut reader = record.get_data_reader();

        let start_x = reader.read_i32::<LittleEndian>().unwrap();
        let start_y = reader.read_i32::<LittleEndian>().unwrap();
        let end_x = reader.read_i32::<LittleEndian>().unwrap();
        let end_y = reader.read_i32::<LittleEndian>().unwrap();

        let is_reverse_hs = if record.size as u64 > reader.position() {
            Some(reader.read_u16::<LittleEndian>().unwrap() > 0)
        } else {
            None
        };

        // TODO: (@hahnlee) unknown 내부에는 HWPX의 controlPoints 요소를 가지고 있음
        let mut unknown = vec![];
        reader.read_to_end(&mut unknown).unwrap();

        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            is_reverse_hs,
            unknown,
        }
    }
}
