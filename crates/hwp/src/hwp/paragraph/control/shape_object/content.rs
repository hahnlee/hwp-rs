use hwp_macro::make_4chid;

use crate::hwp::{
    paragraph::control::element_properties::ElementProperties, record::RecordCursor,
    unknown::UnknownRecord, version::Version,
};

use super::{
    arc::ArcRecord, container::ContainerContent, curve::CurveRecord, ellipse::EllipseRecord,
    line::LineRecord, ole::OleRecord, picture::PictureRecord, polygon::PolygonRecord,
    rectangle::RectangleRecord,
};

#[derive(Debug, Clone)]
pub enum ShapeObjectContent {
    Arc(ArcRecord),
    Container(ContainerContent),
    ConnectLine(LineRecord),
    Curve(CurveRecord),
    Ellipse(EllipseRecord),
    Line(LineRecord),
    Ole(OleRecord),
    Picture(PictureRecord),
    Polygon(PolygonRecord),
    Rectangle(RectangleRecord),
    Unknown(UnknownRecord),
}

pub fn parse_content(
    properties: &ElementProperties,
    cursor: &mut RecordCursor,
    version: &Version,
) -> ShapeObjectContent {
    match properties.ctrl_id {
        make_4chid!('$', 'a', 'r', 'c') => {
            ShapeObjectContent::Arc(ArcRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'c', 'o', 'n') => ShapeObjectContent::Container(
            ContainerContent::from_record_cursor(properties, cursor, version),
        ),
        make_4chid!('$', 'c', 'u', 'r') => {
            ShapeObjectContent::Curve(CurveRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'e', 'l', 'l') => {
            ShapeObjectContent::Ellipse(EllipseRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'l', 'i', 'n') => {
            ShapeObjectContent::Line(LineRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'o', 'l', 'e') => {
            ShapeObjectContent::Ole(OleRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'p', 'i', 'c') => {
            ShapeObjectContent::Picture(PictureRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'p', 'o', 'l') => {
            ShapeObjectContent::Polygon(PolygonRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'r', 'e', 'c') => {
            ShapeObjectContent::Rectangle(RectangleRecord::from_record_cursor(cursor))
        }
        make_4chid!('$', 'c', 'o', 'l') => {
            ShapeObjectContent::ConnectLine(LineRecord::from_record_cursor(cursor))
        }
        _ => ShapeObjectContent::Unknown(UnknownRecord::from_record_cursor(cursor)),
    }
}
