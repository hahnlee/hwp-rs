use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, draw_text::DrawText,
        element_properties::ElementProperties,
    },
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

use super::content::{parse_content, ShapeObjectContent};

/// 묶음 개체
#[derive(Debug, Clone)]
pub struct ContainerControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 컨텐츠
    pub content: ContainerContent,
}

impl ContainerControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);
        let element_properties = ElementProperties::from_record_cursor(cursor, false);
        let content = ContainerContent::from_record_cursor(&element_properties, cursor, version);

        Self {
            common_properties,
            element_properties,
            content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerContent {
    pub children: Vec<ContainerElement>,
}

impl ContainerContent {
    pub fn from_record_cursor(
        properties: &ElementProperties,
        cursor: &mut RecordCursor,
        version: &Version,
    ) -> Self {
        let children = properties
            .children_ids
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|_| ContainerElement::from_record_cursor(cursor, version))
            .collect();

        Self { children }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerElement {
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 컨텐츠
    pub content: ShapeObjectContent,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl ContainerElement {
    pub fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self {
        let element_properties = ElementProperties::from_record_cursor(cursor, false);
        let draw_text = if cursor.record_id(BodyTextRecord::HWPTAG_LIST_HEADER as u32) {
            Some(DrawText::from_record_cursor(cursor, version))
        } else {
            None
        };
        let content = parse_content(&element_properties, cursor, version);

        Self {
            element_properties,
            content,
            draw_text,
        }
    }
}
