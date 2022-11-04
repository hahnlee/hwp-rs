use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecordCursor, RecordCursor},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTrackingAuthor {}

impl FromRecordCursor for ChangeTrackingAuthor {
    fn from_record_cursor(cursor: &mut RecordCursor, _: &Version) -> Self {
        let record = cursor.current();
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_TRACK_CHANGE_AUTHOR as u32
        );

        Self {}
    }
}
