use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct ParagraphShape {}

impl FromRecord for ParagraphShape {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
