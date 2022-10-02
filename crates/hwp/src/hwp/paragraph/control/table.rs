use crate::hwp::record::Record;

use super::common_properties::CommonProperties;

#[derive(Debug)]
pub struct Table {
    pub common_properties: CommonProperties,
}

impl Table {
    pub fn from_record(record: Record) -> Table {
        let size = record.data.len();
        let mut reader = record.get_data_reader();
        let common_properties = CommonProperties::from_reader(&mut reader, size as u64);

        Table { common_properties }
    }
}
