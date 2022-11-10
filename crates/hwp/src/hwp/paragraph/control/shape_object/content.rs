use hwp_macro::make_4chid;

use crate::hwp::{
    paragraph::control::element_properties::ElementProperties, record::RecordCursor,
    unknown::UnknownRecord,
};

use super::container::ContainerContent;

#[derive(Debug, Clone)]
pub enum GenShapeObjectContent {
    Container(ContainerContent),
    Unknown(UnknownRecord),
}

pub fn parse_content(
    properties: &ElementProperties,
    cursor: &mut RecordCursor,
) -> GenShapeObjectContent {
    match properties.ctrl_id {
        make_4chid!('$', 'c', 'o', 'n') => GenShapeObjectContent::Container(
            ContainerContent::from_record_cursor(properties, cursor),
        ),
        _ => GenShapeObjectContent::Unknown(UnknownRecord::from_record_cursor(cursor)),
    }
}
