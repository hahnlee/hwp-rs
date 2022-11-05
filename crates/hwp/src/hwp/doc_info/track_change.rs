use std::io::Read;

use crate::hwp::record::{tags::DocInfoRecord, RecordCursor};

/// 변경 추적 정보
#[derive(Debug)]
pub struct TrackChange {
    pub unknown: Vec<u8>,
}

impl TrackChange {
    pub fn from_record_cursor(cursor: &mut RecordCursor) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_TRACKCHANGE as u32);

        let mut reader = record.get_data_reader();

        // NOTE: (@hahnlee) 문서와 되어있지 않음, 정확한 정보는 HWPX와 대조해서 유추해야함
        let mut unknown = vec![];
        reader.read_to_end(&mut unknown).unwrap();

        Self { unknown }
    }
}
