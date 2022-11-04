use std::io::{Cursor, Read};

use byteorder::LittleEndian;

use self::reader::RecordReader;

use super::version::Version;

pub mod reader;
pub mod tags;

#[derive(Debug, Clone)]
pub struct Record {
    pub tag_id: u32,
    pub level: u32,
    pub size: u32,
    pub data: Vec<u8>,
}

impl Record {
    pub fn new(tag_id: u32, level: u32, size: u32, data: Vec<u8>) -> Self {
        Self {
            tag_id,
            level,
            size,
            data,
        }
    }

    pub fn get_data_reader(&self) -> Cursor<&Vec<u8>> {
        Cursor::new(&self.data)
    }
}

pub struct RecordCursor {
    records: Vec<Record>,
}

impl RecordCursor {
    pub fn new<T: Read>(reader: &mut T) -> Self {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();

        let mut reader = Cursor::new(&data);
        let mut records = vec![];

        while reader.position() < (data.len() as u64) {
            let (tag_id, level, size, mut data) = reader.read_record::<LittleEndian>().unwrap();
            let mut buf = Vec::new();
            data.read_to_end(&mut buf).unwrap();

            records.insert(0, Record::new(tag_id, level, size, buf));
        }

        Self { records }
    }

    pub fn current(&mut self) -> Record {
        self.records.pop().unwrap()
    }

    pub fn record_id(&self, tag_id: u32) -> bool {
        if self.records.len() == 0 {
            return false;
        }

        return self.records.last().unwrap().tag_id == tag_id;
    }

    pub fn next_level(&self) -> u32 {
        return self.records.last().unwrap().level;
    }

    pub fn has_next(&self) -> bool {
        self.records.len() > 0
    }

    pub fn collect_children(&mut self, level: u32) -> Vec<Record> {
        let mut children = vec![];
        while self.has_next() && self.next_level() > level {
            children.push(self.current());
        }
        children
    }
}

pub trait FromRecordCursor {
    fn from_record_cursor(cursor: &mut RecordCursor, version: &Version) -> Self;
}

pub fn read_items<T: FromRecordCursor>(
    cursor: &mut RecordCursor,
    version: &Version,
    size: usize,
) -> Vec<T> {
    let mut read_items: Vec<T> = Vec::with_capacity(size);
    for _ in 0..size {
        read_items.push(T::from_record_cursor(cursor, version));
    }

    read_items
}
