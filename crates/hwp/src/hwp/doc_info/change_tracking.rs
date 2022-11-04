use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecord, Record},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTracking {}

impl FromRecord for ChangeTracking {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_TRACK_CHANGE as u32);

        Self {}
    }
}
