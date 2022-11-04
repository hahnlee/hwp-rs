use crate::hwp::{
    record::{Record, RecordCursor},
    version::Version,
};

use super::common_properties::CommonProperties;

/// 묶음 개체
#[derive(Debug, Clone)]
pub struct Container {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl Container {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let common_properties = CommonProperties::from_record(record, cursor, version);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}
