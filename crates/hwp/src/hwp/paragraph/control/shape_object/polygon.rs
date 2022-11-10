use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

/// 다각형
#[derive(Debug, Clone)]
pub struct ShapePolygonControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl ShapePolygonControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self {
            common_properties,
            element_properties,
            draw_text,
        }
    }
}
