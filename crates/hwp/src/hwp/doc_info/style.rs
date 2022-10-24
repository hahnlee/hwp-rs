use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecord, Record, reader::RecordReader},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct Style {
    /// 로컬 스타일 이름. 한글 윈도우에서는 한글 스타일 이름
    pub name: String,
    /// 영문 스타일 이름
    pub english_name: String,
}

impl FromRecord for Style {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_STYLE as u32);

        let mut reader = record.get_data_reader();

        let name = reader.read_string::<LittleEndian>().unwrap();
        let english_name = reader.read_string::<LittleEndian>().unwrap();

        // TODO: (@hahnlee)

        Self {
            name,
            english_name,
        }
    }
}
