use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTracking {}

impl FromRecordCursor for ChangeTracking {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_TRACK_CHANGE as u32);

        Self {}
    }
}
