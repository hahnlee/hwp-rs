use crate::hwp::{
    paragraph::control::{draw_text::DrawText, element_properties::ElementProperties},
    record::{tags::BodyTextRecord, Record, RecordCursor},
    version::Version,
};

use super::common_properties::CommonProperties;

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
        let children = cursor.collect_children(record.level);
        assert_ne!(children.len(), 0);

        Self {
            common_properties,
            element_properties,
            draw_text,
        }
    }
}

/// 선
#[derive(Debug, Clone)]
pub struct ShapeLineControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl ShapeLineControl {
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

/// 사각형
#[derive(Debug, Clone)]
pub struct ShapeRectangleControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl ShapeRectangleControl {
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

/// 타원
#[derive(Debug, Clone)]
pub struct ShapeEllipseControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
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

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self {
            common_properties,
            element_properties,
            draw_text,
        }
    }
}

/// 호
#[derive(Debug, Clone)]
pub struct ShapeArcControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
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

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self {
            common_properties,
            element_properties,
            draw_text,
        }
    }
}

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

/// 곡선
#[derive(Debug, Clone)]
pub struct ShapeCurveControl {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    /// 개체 요소 속성
    pub element_properties: ElementProperties,
    /// 글상자
    pub draw_text: Option<DrawText>,
}

impl ShapeCurveControl {
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
