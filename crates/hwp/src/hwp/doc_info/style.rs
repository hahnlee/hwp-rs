use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct Style {}

impl FromRecord for Style {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
