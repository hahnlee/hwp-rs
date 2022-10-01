use std::io::{Read, Seek, SeekFrom};

use byteorder::LittleEndian;
use num::ToPrimitive;

use crate::hwp::record::reader::RecordReader;

#[derive(Debug)]
pub struct Unknown {
    pub ctrl_id: u32,
    pub data: Vec<u8>,
    pub children: Vec<u8>,
}

impl Unknown {
    pub fn from_reader<T: Read + Seek>(
        ctrl_id: u32,
        reader: &mut T,
        level: u32,
        size: u32,
    ) -> Unknown {
        let mut self_data = reader.take(size.into());
        let mut data = Vec::new();
        self_data.read_to_end(&mut data).unwrap();

        let children = traverse_records(reader, level);

        Unknown { ctrl_id, data, children }
    }
}

fn traverse_records<T: Read + Seek>(reader: &mut T, current_level: u32) -> Vec<u8> {
    let mut records = Vec::new();

    loop {
        let record = reader.read_record_with_bytes::<LittleEndian>();
        if record.is_err() {
            break;
        }

        let (_, level, size, read_bytes) = record.unwrap();
        reader.seek(SeekFrom::Current(-read_bytes)).unwrap();

        if current_level >= level {
            break;
        }

        let record_size: u64 = size.to_u64().unwrap() + read_bytes.to_u64().unwrap();
        let mut take = reader.take(record_size);
        let mut buf = Vec::new();
        take.read_to_end(&mut buf).unwrap();

        records.append(&mut buf);
    }

    return records;
}
