use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    header::Header,
    record::{reader::RecordReader, tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    utils::bits::get_value_range,
    version::Version,
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
            CompressMode::Default => header.flags.compressed,
            CompressMode::Compress => true,
            _ => false,
        }
    }
}

impl FromRecordCursor for BinData {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();

        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_BIN_DATA as u32,
            "올바르지 않은 정보"
        );

        let mut data = record.get_data_reader();
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

        Self {
            properties,
            absolute_path,
            relative_path,
            id,
            extension,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum BinDataKind {
    /// 그림 외부 파일 참조
    Link,
    /// 그림 파일 포함
    Embedding,
    /// OLE 포함
    Storage,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum CompressMode {
    /// 스토리지의 디폴트 모드 따라감
    Default,
    /// 무조건 압축
    Compress,
    /// 무조건 압축하지 않음
    None,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum BinDataStatus {
    /// 아직 access 된 적이 없는 상태
    Initial,
    /// access에 성공하여 파일을 찾은 상태
    Success,
    /// access가 실패한 에러 상태
    Failed,
    /// 링크 access가 실패했으나 무시된 상태
    Ignored,
}

#[derive(Debug)]
pub struct BinDataProperties {
    /// 타입
    pub kind: BinDataKind,
    /// 압축 모드
    pub compress_mode: CompressMode,
    /// 상태
    pub status: BinDataStatus,
}

impl BinDataProperties {
    pub fn from_bits(bits: u16) -> Self {
        Self {
            kind: BinDataKind::from_u16(get_value_range(bits, 0, 3)).unwrap(),
            compress_mode: CompressMode::from_u16(get_value_range(bits, 4, 5)).unwrap(),
            status: BinDataStatus::from_u16(get_value_range(bits, 8, 9)).unwrap(),
        }
    }
}
