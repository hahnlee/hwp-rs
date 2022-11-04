use crate::hwp::{
    record::{tags::DocInfoRecord, FromRecord, Record},
    version::Version,
};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct MemoShape {}

impl FromRecord for MemoShape {
    fn from_record(record: &mut Record, _: &Version) -> Self {
        assert_eq!(record.tag_id, DocInfoRecord::HWPTAG_MEMO_SHAPE as u32);

        Self {}
    }
}
