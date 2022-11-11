use byteorder::ReadBytesExt;
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

/// 호
#[derive(Debug, Clone)]
pub struct ShapeArcControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
    /// 컨텐츠
    pub content: ArcRecord,
}

impl ShapeArcControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        let content = ArcRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            draw_text,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArcRecord {
    /// 호(ARC)의 종류
    pub arc_kind: ArcKind,
    /// 중심 좌표
    pub center: Point,
    /// 제1축 좌표
    pub axis_1: Point,
    /// 제2축 좌표
    pub axis_2: Point,
}

impl ArcRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_ARC as u32
        );

        let mut reader = record.get_data_reader();

        let arc_kind = ArcKind::from_u8(reader.read_u8().unwrap()).unwrap();

        let center = Point::from_reader(&mut reader);
        let axis_1 = Point::from_reader(&mut reader);
        let axis_2 = Point::from_reader(&mut reader);

        assert_eq!(record.size as u64, reader.position());

        Self {
            arc_kind,
            center,
            axis_1,
            axis_2,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum ArcKind {
    /// 호
    Normal,
    /// 부채꼴
    Pie,
    /// 활
    Chord,
}
