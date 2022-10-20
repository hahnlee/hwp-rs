use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ChangeTrackingAuthor {}

impl FromRecord for ChangeTrackingAuthor {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
