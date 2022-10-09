use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    record::{reader::RecordReader, tags::DocInfoRecord},
    utils::bits::get_value_range, header::Header,
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
    pub fn from_reader<T: Read>(stream: &mut T) -> BinData {
        let (tag_id, _, _, mut data) = stream.read_record::<LittleEndian>().unwrap();
        if tag_id != DocInfoRecord::HWPTAG_BIN_DATA as u32 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        let properties = data.read_u16::<LittleEndian>().unwrap();
        let properties = BinDataProperties::from_bits(properties);

        let absolute_path = if properties.kind == BinDataKind::Link {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let relative_path = if properties.kind == BinDataKind::Link {
            Some(data.read_string::<LittleEndian>().unwrap())
        } else {
            None
        };

        let id = if properties.kind == BinDataKind::Embedding
            || properties.kind == BinDataKind::Storage
        {
            Some(data.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        let extension = if properties.kind == BinDataKind::Embedding {
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

    pub fn cfb_file_name(&self) -> Option<String> {
        if self.properties.kind != BinDataKind::Embedding {
            return None;
        }

        let mut extension = self.extension.clone().unwrap();
        extension.make_ascii_lowercase();

        let id = self.id.unwrap();

        Some(format!("BIN{:0>4X}.{extension}", id))
    }

    pub fn compressed(&self, header: &Header) -> bool {
        match self.properties.compress_mode {
            0x0000 => header.flags.compressed,
            0x0010 => true,
            _ => false,
        }
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum BinDataKind {
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
    pub kind: BinDataKind,
    /// 압축 모드
    pub compress_mode: u16,
    /// 상태
    pub status: u16,
}

impl BinDataProperties {
    pub fn from_bits(bits: u16) -> BinDataProperties {
        // TODO: (@hahnlee) 남는 비트정보 보존
        BinDataProperties {
            kind: BinDataKind::from_u16(get_value_range(bits, 0, 3)).unwrap(),
            compress_mode: get_value_range(bits, 4, 5),
            status: get_value_range(bits, 8, 9),
        }
    }
}
