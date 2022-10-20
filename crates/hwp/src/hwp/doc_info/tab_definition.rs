use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct TabDefinition {}

impl FromRecord for TabDefinition {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
