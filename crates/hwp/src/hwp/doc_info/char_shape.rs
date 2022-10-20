use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct CharShape {}

impl FromRecord for CharShape {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
