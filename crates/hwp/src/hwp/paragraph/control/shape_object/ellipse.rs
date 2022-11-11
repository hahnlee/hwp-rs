use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties,
        draw_text::DrawText,
        element_properties::ElementProperties,
        shape_object::{arc::ArcKind, picture::Point},
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

use num::FromPrimitive;

/// 타원
#[derive(Debug, Clone)]
pub struct ShapeEllipseControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
    /// 컨텐츠
    pub content: EllipseRecord,
}

impl ShapeEllipseControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        let content = EllipseRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            draw_text,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EllipseRecord {
    /// 호(ARC)로 바뀌었을 때, interval을 다시 계산해야 할 필요가 있는지 여부
    /// (interval - 원 위에 존재하는 두 점 사이의 거리)
    pub interval_dirty: bool,
    /// 호(ARC)로 바뀌었는지 여부
    pub has_arc_property: bool,
    /// 호(ARC)의 종류
    pub arc_kind: ArcKind,
    /// 중심 좌표
    pub center: Point,
    /// 제1축 좌표
    pub axis_1: Point,
    /// 제2축 좌표
    pub axis_2: Point,
    /// 시작지점 1 좌표
    pub start_1: Point,
    /// 끝지점 1 좌표
    pub end_1: Point,
    /// 시작지점 2 좌표
    pub start_2: Point,
    /// 끝지점 2 좌표
    pub end_2: Point,
}

impl EllipseRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_ELLIPSE as u32
        );

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let interval_dirty = get_flag(attribute, 0);
        let has_arc_property = get_flag(attribute, 1);
        let arc_kind = ArcKind::from_u32(get_value_range(attribute, 2, 9)).unwrap();

        let center = Point::from_reader(&mut reader);
        let axis_1 = Point::from_reader(&mut reader);
        let axis_2 = Point::from_reader(&mut reader);
        let start_1 = Point::from_reader(&mut reader);
        let end_1 = Point::from_reader(&mut reader);
        let start_2 = Point::from_reader(&mut reader);
        let end_2 = Point::from_reader(&mut reader);

        assert_eq!(record.size as u64, reader.position());

        Self {
            interval_dirty,
            has_arc_property,
            arc_kind,
            center,
            axis_1,
            axis_2,
            start_1,
            end_1,
            start_2,
            end_2,
        }
    }
}
