use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct Numbering {}

impl FromRecord for Numbering {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
