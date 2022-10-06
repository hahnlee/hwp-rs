use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::record::{reader::RecordReader, Record};

/// 글자 겹침
#[derive(Debug, Clone)]
pub struct OverType {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 겹칠 글자
    pub text: String,
    /// 테두리 타입
    pub border_type: u8,
    /// 내부 글자 크기
    pub character_size: i8,
    /// 테두리 내부 글자 펼침
    pub character_fold: u8,
    /// 테두리 내부 글자의 char shape id 의 배열
    pub char_shape_ids: Vec<u32>,
}

impl OverType {
    pub fn from_record(record: Record) -> Self {
        let mut reader = record.get_data_reader();
        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let text = reader.read_string::<LittleEndian>().unwrap();
        let border_type = reader.read_u8().unwrap();
        let character_size = reader.read_i8().unwrap();
        let character_fold = reader.read_u8().unwrap();

        let count = reader.read_u8().unwrap();
        let mut char_shape_ids = Vec::with_capacity(count as usize);

        for _ in 0..count {
            char_shape_ids.push(reader.read_u32::<LittleEndian>().unwrap());
        }

        Self {
            ctrl_id,
            text,
            border_type,
            character_size,
            character_fold,
            char_shape_ids,
        }
    }
}
