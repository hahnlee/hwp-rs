use crate::hwp::{
    paragraph::control::{
        common_properties::CommonProperties, element_properties::ElementProperties,
    },
    record::{Record, RecordCursor},
    version::Version,
};

use super::content::{parse_content, GenShapeObjectContent};

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
        let content = ContainerContent::from_record_cursor(&element_properties, cursor);

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
    pub fn from_record_cursor(properties: &ElementProperties, cursor: &mut RecordCursor) -> Self {
        let children = properties
            .children_ids
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|_| ContainerElement::from_record_cursor(cursor))
            .collect();

        Self { children }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerElement {
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 컨텐츠
    pub content: GenShapeObjectContent,
}

impl ContainerElement {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let element_properties = ElementProperties::from_record_cursor(cursor, false);
        let content = parse_content(&element_properties, cursor);

        Self {
            element_properties,
            content,
        }
    }
}
