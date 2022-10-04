use crate::hwp::{record::Record, version::Version};

use super::common_properties::CommonProperties;

/// 그리기 객체
#[derive(Debug, Clone)]
pub struct GenShapeObject {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl GenShapeObject {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) children 파싱하기
        GenShapeObject { common_properties }
    }
}

/// 선
#[derive(Debug, Clone)]
pub struct ShapeLine {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeLine {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 사각형
#[derive(Debug, Clone)]
pub struct ShapeRectangle {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeRectangle {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 타원
#[derive(Debug, Clone)]
pub struct ShapeEllipse {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeEllipse {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 호
#[derive(Debug, Clone)]
pub struct ShapeArc {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeArc {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 다각형
#[derive(Debug, Clone)]
pub struct ShapePolygon {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapePolygon {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 곡선
#[derive(Debug, Clone)]
pub struct ShapeCurve {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeCurve {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}
