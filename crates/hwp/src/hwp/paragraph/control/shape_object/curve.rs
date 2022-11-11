use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties, shape_object::picture::Point,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

/// 곡선
#[derive(Debug, Clone)]
pub struct ShapeCurveControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
    /// 컨텐츠
    pub content: CurveRecord,
}

impl ShapeCurveControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        let content = CurveRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            draw_text,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurveRecord {
    /// 좌표
    pub points: Vec<Point>,
    /// 세그먼트 타입
    pub segment_kinds: Vec<SegmentKind>,
}

impl CurveRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_CURVE as u32
        );

        let mut reader = record.get_data_reader();

        let count = reader.read_u32::<LittleEndian>().unwrap();
        let mut points = vec![];
        for _ in 0..count {
            points.push(Point::from_reader(&mut reader));
        }

        let mut segment_kinds = vec![];
        for _ in 0..count - 1 {
            segment_kinds.push(SegmentKind::from_u8(reader.read_u8().unwrap()).unwrap());
        }

        Self {
            points,
            segment_kinds,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum SegmentKind {
    Line,
    Curve,
}
