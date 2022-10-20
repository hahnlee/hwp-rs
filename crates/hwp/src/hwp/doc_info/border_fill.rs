use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct BorderFill {}

impl FromRecord for BorderFill {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
