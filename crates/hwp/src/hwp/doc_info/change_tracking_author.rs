use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecord, Record},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTrackingAuthor {}

impl FromRecord for ChangeTrackingAuthor {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(
            record.tag_id,
            DocInfoRecord::HWPTAG_TRACK_CHANGE_AUTHOR as u32
        );

        Self {}
    }
}
