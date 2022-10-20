use crate::hwp::{record::{Record, FromRecord}, version::Version};

// TODO: (@hahnlee)
#[derive(Debug)]
pub struct Bullet {}

impl FromRecord for Bullet {
    fn from_record(_: &mut Record, _: &Version) -> Self {
        Self {}
    }
}
