use std::io::{Read, Result};

use byteorder::{ByteOrder, ReadBytesExt};

pub trait RecordReader: Read + ReadBytesExt {
    fn read_record<T: ByteOrder>(&mut self) -> Result<(u32, u32, u32)> {
        let value = self.read_u32::<T>()?;

        let tag_id = value & 0x3FF;
        let level = (value >> 10) & 0x3FF;
        let mut size = (value >> 20) & 0xFFF;

        if size == 0xFFF {
            size = self.read_u32::<T>()?;
        }

        Ok((tag_id, level, size))
    }
}

impl<R: Read + ?Sized> RecordReader for R {}
