use std::io::{Read, Result};

use byteorder::{ReadBytesExt, ByteOrder};

use crate::hwp::record::reader::RecordReader;

pub trait ParameterSetReader: Read + ReadBytesExt {
    #[inline]
    fn read_pit_bstr<T: ByteOrder>(&mut self) -> Result<String> {
        let kind = self.read_u16::<T>()?;
        assert_eq!(kind, 1, "잘못된 파라미터 아이템 종류 입니다");
        let result = self.read_string::<T>()?;
        Ok(result)
    }

    #[inline]
    fn read_pit_ui<T: ByteOrder>(&mut self) -> Result<u16> {
        let kind = self.read_u16::<T>()?;
        assert_eq!(kind, 9, "잘못된 파라미터 아이템 종류 입니다");
        let result = self.read_u16::<T>()?;
        Ok(result)
    }
}

impl<R: Read + ?Sized> ParameterSetReader for R {}
