use byteorder::ReadBytesExt;

use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

use super::picture::Point;

/// 사각형
#[derive(Debug, Clone)]
pub struct ShapeRectangleControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
    /// 컨텐츠
    pub content: RectangleRecord,
}

impl ShapeRectangleControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        let content = RectangleRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            draw_text,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectangleRecord {
    /// 사각형 모서리 곡률(%) 직각은 0, 둥근 모양은 20, 반원은 50,
    /// 그 외는 적당한 값을 % 단위로 사용한다.
    pub ratio: u8,
    /// 좌표
    pub points: [Point; 4],
}

impl RectangleRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_RECTANGLE as u32
        );

        let mut reader = record.get_data_reader();

        let ratio = reader.read_u8().unwrap();
        let points = [
            Point::from_reader(&mut reader),
            Point::from_reader(&mut reader),
            Point::from_reader(&mut reader),
            Point::from_reader(&mut reader),
        ];

        assert_eq!(record.size as u64, reader.position());

        Self { ratio, points }
    }
}
