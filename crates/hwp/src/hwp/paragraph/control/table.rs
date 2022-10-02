use crate::hwp::record::Record;

use super::common_properties::CommonProperties;

/// 표
#[derive(Debug)]
pub struct Table {
    /// 개체 공통 속성
    pub common_properties: CommonProperties,
}

impl Table {
    pub fn from_record(mut record: Record) -> Table {
        let common_properties = CommonProperties::from_reader(&mut record);

        Table { common_properties }
    }
}
