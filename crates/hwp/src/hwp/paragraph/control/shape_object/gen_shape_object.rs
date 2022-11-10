use hwp_macro::make_4chid;

use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    unknown::UnknownRecord,
    version::Version,
};

/// 그리기 객체
#[derive(Debug, Clone)]
pub struct GenShapeObjectControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl GenShapeObjectControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, true);

        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };

        // TODO: (@hahnlee) children 파싱하기
        if element_properties.ctrl_id == make_4chid!('$', 'c', 'o', 'n') {
            for _ in element_properties.children_ids.as_ref().unwrap() {
                ElementProperties::from_record_cursor(cursor, false);
                UnknownRecord::from_record_cursor(cursor);
            }
        } else {
            UnknownRecord::from_record_cursor(cursor);
        }

        Self {
            common_properties,
            element_properties,
            draw_text,
        }
    }
}
