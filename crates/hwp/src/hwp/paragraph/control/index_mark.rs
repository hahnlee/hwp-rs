use byteorder::{ReadBytesExt, LittleEndian};

use crate::hwp::record::{Record, reader::RecordReader};

/// 찾아보기 표식
#[derive(Debug, Clone)]
pub struct IndexMark {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 첫번째 키워드
    pub first_keyword: String,
    /// 두번째 키워드
    pub second_keyword: String,
}

impl IndexMark {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();
        let first_keyword = reader.read_string::<LittleEndian>().unwrap();
        let second_keyword = reader.read_string::<LittleEndian>().unwrap();

        Self {
            ctrl_id,
            first_keyword,
            second_keyword,
        }
    }
}
