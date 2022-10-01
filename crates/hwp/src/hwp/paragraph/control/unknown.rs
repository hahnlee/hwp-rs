use std::io::{Read, Seek};

use crate::hwp::record::reader::traverse_records;

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

        Unknown {
            ctrl_id,
            data,
            children,
        }
    }
}
