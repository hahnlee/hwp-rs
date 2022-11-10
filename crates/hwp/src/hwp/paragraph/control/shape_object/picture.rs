use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, element_properties::ElementProperties,
    },
    record::{Record, RecordCursor},
    version::Version,
};

/// 그림
#[derive(Debug, Clone)]
pub struct PictureControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
}

impl PictureControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);

        Self {
            common_properties,
            element_properties,
        }
    }
}
