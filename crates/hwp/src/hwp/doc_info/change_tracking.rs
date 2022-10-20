use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTracking {}

impl FromRecord for ChangeTracking {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
