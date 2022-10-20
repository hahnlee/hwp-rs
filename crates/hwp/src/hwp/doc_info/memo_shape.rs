use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct MemoShape {}

impl FromRecord for MemoShape {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
