use std::io::Read;

use crate::hwp::record::{Record, RecordCursor};

#[derive(Debug, Clone)]
pub struct UnknownRecord {
    /// 태그 ID
    pub tag_id: u32,
    /// 데이터
    pub data: Vec<u8>,
    /// 레코드
    pub children: Vec<Record>,
}

impl UnknownRecord {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        let mut reader = record.get_data_reader();

        let tag_id = record.tag_id;
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();

        let children = cursor.collect_children(record.level);

        Self {
            tag_id,
            data,
            children,
        }
    }
}
