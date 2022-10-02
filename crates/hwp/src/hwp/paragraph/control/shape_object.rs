use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;

use crate::hwp::record::Record;

use super::common_properties::CommonProperties;

/// 그리기 객체
#[derive(Debug)]
pub struct GenShapeObject {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl GenShapeObject {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();
        assert_eq!(
            ctrl_id,
            make_4chid!('g', 's', 'o', ' '),
            "GenShapeObject일 경우 ctrl_id가 두 번 기록되어야 합니다"
        );

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) children 파싱하기
        GenShapeObject { common_properties }
    }
}

/// 선
#[derive(Debug)]
pub struct ShapeLine {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeLine {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 사각형
#[derive(Debug)]
pub struct ShapeRectangle {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeRectangle {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 타원
#[derive(Debug)]
pub struct ShapeEllipse {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeEllipse {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 호
#[derive(Debug)]
pub struct ShapeArc {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeArc {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 다각형
#[derive(Debug)]
pub struct ShapePolygon {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapePolygon {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}

/// 곡선
#[derive(Debug)]
pub struct ShapeCurve {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl ShapeCurve {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}
