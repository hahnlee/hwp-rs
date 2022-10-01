use std::io::{Cursor, Read, Result, Seek, SeekFrom, Take};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use num::ToPrimitive;

use super::Record;

pub trait RecordReader: Read + ReadBytesExt {
    #[inline]
    fn read_record<T: ByteOrder>(&mut self) -> Result<(u32, u32, u32, Take<&mut Self>)> {
        let value = self.read_u32::<T>()?;

        let tag_id = value & 0x3FF;
        let level = (value >> 10) & 0x3FF;
        let mut size = (value >> 20) & 0xFFF;

        if size == 0xFFF {
            size = self.read_u32::<T>()?;
        }

        let data = self.take(size.into());

        Ok((tag_id, level, size, data))
    }

    #[inline]
    fn read_record_meta<T: ByteOrder>(&mut self) -> Result<(u32, u32, u32)> {
        let value = self.read_u32::<T>()?;

        let tag_id = value & 0x3FF;
        let level = (value >> 10) & 0x3FF;
        let mut size = (value >> 20) & 0xFFF;

        if size == 0xFFF {
            size = self.read_u32::<T>()?;
        }

        Ok((tag_id, level, size))
    }

    #[inline]
    fn read_string<T: ByteOrder>(&mut self) -> Result<String> {
        let len = self.read_u16::<T>()? as usize;
        let mut buf = vec![0u16; len];
        for i in 0..len {
            buf[i] = self.read_u16::<T>()?;
        }

        Ok(String::from_utf16(&buf).unwrap())
    }

    #[inline]
    fn read_record_with_bytes<T: ByteOrder>(&mut self) -> Result<(u32, u32, u32, i64)> {
        let value = self.read_u32::<T>()?;
        let mut bytes = 4;

        let tag_id = value & 0x3FF;
        let level = (value >> 10) & 0x3FF;
        let mut size = (value >> 20) & 0xFFF;

        if size == 0xFFF {
            size = self.read_u32::<T>()?;
            bytes += 4;
        }

        Ok((tag_id, level, size, bytes))
    }
}

impl<R: Read + ?Sized> RecordReader for R {}

/// 레코드는 level 기반의 tree 구조로 이루어져 있다
///
/// 레코드 배열을 tree로 변환하는 함수
pub fn read_records(data: Vec<u8>) -> Vec<Record> {
    let len = data.len() - 1;

    let mut reader = Cursor::new(data);

    let mut records = Vec::new();

    while reader.position() < len as u64 {
        let (tag_id, level, size, mut data) = reader.read_record::<LittleEndian>().unwrap();
        let mut buf = Vec::new();
        data.read_to_end(&mut buf).unwrap();

        let mut current_record = Record::new(tag_id, level, size, buf);

        let next_record_info = reader.read_record_with_bytes::<LittleEndian>();
        if !next_record_info.is_err() {
            let (_, next_level, _, read_bytes) = next_record_info.unwrap();
            reader.seek(SeekFrom::Current(-read_bytes)).unwrap();

            if next_level > level {
                fill_tree(&mut current_record, level, &mut reader);
            }
        }

        records.push(current_record);
    }

    records
}

fn fill_tree(record: &mut Record, level: u32, reader: &mut Cursor<Vec<u8>>) {
    let size = reader.get_ref().len().to_u64().unwrap() - 1;
    while reader.position() < size {
        let (tag_id, next_level, record_size, read_bytes) =
            reader.read_record_with_bytes::<LittleEndian>().unwrap();

        if next_level <= level {
            reader.seek(SeekFrom::Current(-read_bytes)).unwrap();
            break;
        }

        let mut data = reader.take(record_size as u64);
        let mut buf = Vec::new();
        data.read_to_end(&mut buf).unwrap();

        let mut next_record = Record::new(tag_id, next_level, record_size, buf);

        let record_info = reader.read_record_with_bytes::<LittleEndian>();
        if !record_info.is_err() {
            let (_, next_next_level, _, read_bytes) = record_info.unwrap();
            reader.seek(SeekFrom::Current(-read_bytes)).unwrap();

            if next_next_level > next_level {
                fill_tree(&mut next_record, next_level, reader);
            }
        }

        record.add(next_record);
    }
}
