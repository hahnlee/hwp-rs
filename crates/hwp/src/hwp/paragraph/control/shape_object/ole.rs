use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, element_properties::ElementProperties,
    },
    record::{Record, RecordCursor, tags::BodyTextRecord},
    version::Version,
};

/// OLE
#[derive(Debug, Clone)]
pub struct OleControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 컨텐츠
    pub content: OleRecord,
}

impl OleControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);
        let content = OleRecord::from_record_cursor(cursor);

        Self {
            common_properties,
            element_properties,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OleRecord {}

impl OleRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            BodyTextRecord::HWPTAG_SHAPE_COMPONENT_OLE as u32
        );

        // TODO: (@hahnlee)
        Self {}
    }
}
