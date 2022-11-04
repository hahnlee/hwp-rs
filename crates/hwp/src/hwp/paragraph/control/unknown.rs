use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{Record, RecordCursor};

#[derive(Debug, Clone)]
pub struct UnknownControl {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 데이터
    pub data: Vec<u8>,
    /// 레코드
    pub children: Vec<Record>,
}

impl UnknownControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();

        let children = cursor.collect_children(record.level);

        Self {
            ctrl_id,
            data,
            children,
        }
    }
}
