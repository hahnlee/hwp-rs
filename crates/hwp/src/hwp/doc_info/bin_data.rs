use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use cfb::Stream;
use flate2::read::DeflateDecoder;

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord},
    utils::bits::get_value,
};

#[derive(Debug)]
pub struct BinData {
    pub properties: BinDataProperties,
    pub absolute_path: Option<String>,
    pub relative_path: Option<String>,
    pub id: Option<u16>,
    pub extension: Option<String>,
}

impl BinData {
    pub fn from_reader(stream: &mut DeflateDecoder<&mut Stream<Cursor<Vec<u8>>>>) -> BinData {
        let (tag_id, _, _, mut data) = stream.read_record::<LittleEndian>().unwrap();
        if tag_id != DocInfoRecord::HWPTAG_BIN_DATA as u32 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        let properties = data.read_u16::<LittleEndian>().unwrap();
        let properties = BinDataProperties::from_bits(properties);

        let absolute_path = if properties.data_type == BinDataType::Link as u16 {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let relative_path = if properties.data_type == BinDataType::Link as u16 {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let id = if properties.data_type == BinDataType::Embedding as u16
            || properties.data_type == BinDataType::Storage as u16
        {
            Some(data.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        let extension = if properties.data_type == BinDataType::Embedding as u16 {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        BinData {
            properties,
            absolute_path,
            relative_path,
            id,
            extension,
        }
    }
}

#[repr(u16)]
#[derive(PartialEq, Eq)]
pub enum BinDataType {
    /// 그림 외부 파일 참조
    Link,
    /// 그림 파일 포함
    Embedding,
    /// OLE 포함
    Storage,
}

#[derive(Debug)]
pub struct BinDataProperties {
    // TODO: (@hahnlee) enum
    /// 타입
    pub data_type: u16,
    /// 압축 모드
    pub compress_mode: u16,
    /// 상태
    pub status: u16,
}

impl BinDataProperties {
    pub fn from_bits(bits: u16) -> BinDataProperties {
        // TODO: (@hahnlee) 남는 비트정보 보존
        BinDataProperties {
            data_type: get_value(bits, 0, 3),
            compress_mode: get_value(bits, 4, 5),
            status: get_value(bits, 8, 9),
        }
    }
}
