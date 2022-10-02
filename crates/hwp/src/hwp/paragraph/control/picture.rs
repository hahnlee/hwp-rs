use crate::hwp::record::Record;

use super::common_properties::CommonProperties;

/// 그림
#[derive(Debug)]
pub struct Picture {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl Picture {
    pub fn from_record(mut record: Record) -> Self {
        let common_properties = CommonProperties::from_reader(&mut record);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}
