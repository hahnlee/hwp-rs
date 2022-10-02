use crate::hwp::record::Record;

use super::common_properties::CommonProperties;

/// OLE
#[derive(Debug)]
pub struct Ole {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl Ole {
    pub fn from_record(record: Record) -> Self {
        let size = record.data.len();
        let mut reader = record.get_data_reader();

        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        // TODO: (@hahnlee) 남은 데이터 파싱하기
        Self { common_properties }
    }
}
