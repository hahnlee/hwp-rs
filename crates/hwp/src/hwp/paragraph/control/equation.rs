use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{reader::RecordReader, tags::BodyTextRecord, Record},
    version::Version,
};

use super::common_properties::CommonProperties;

/// 한글 97 수식
#[derive(Debug, Clone)]
pub struct Equation {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
    pub record: EquationRecord,
}

impl Equation {
    pub fn from_record(mut record: Record, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(&mut record, version);

        assert!(
            record.is_next_child_id(BodyTextRecord::HWPTAG_EQEDIT as u32),
            "수식객체가 아닙니다"
        );

        let equation_record = EquationRecord::from_record(&mut record.next_child());

        Self {
            common_properties,
            record: equation_record,
        }
    }
}

// TODO: (@hahnlee) 합치는게 낫지 않을까?
#[derive(Debug, Clone)]
pub struct EquationRecord {
    pub script: String,
    pub font: String,
}

impl EquationRecord {
    pub fn from_record(record: &mut Record) -> Self {
        let mut reader = record.get_data_reader();

        // 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let script = reader.read_string::<LittleEndian>().unwrap();

        // 수식 글자 크기
        reader.read_u32::<LittleEndian>().unwrap();
        // 글자 색상
        reader.read_u32::<LittleEndian>().unwrap();

        // base line
        reader.read_i16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 한글 표준 문서와 다른 부분, 용도 파악 필요.
        reader.read_u16::<LittleEndian>().unwrap();

        reader.read_string::<LittleEndian>().unwrap();

        let font = reader.read_string::<LittleEndian>().unwrap();

        Self { script, font }
    }
}
