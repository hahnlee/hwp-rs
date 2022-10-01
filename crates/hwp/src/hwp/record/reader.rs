use std::io::{Read, Result, Take, Seek, SeekFrom};

use byteorder::{ByteOrder, ReadBytesExt, LittleEndian};
use num::ToPrimitive;

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

pub fn traverse_records<T: Read + Seek>(reader: &mut T, current_level: u32) -> Vec<u8> {
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
