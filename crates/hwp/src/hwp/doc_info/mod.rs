pub mod id_mappings;
pub mod properties;

use std::io::Cursor;

use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use self::{id_mappings::IDMappings, properties::Properties};

use super::version::Version;

#[derive(Debug)]
pub struct DocInfo {
    pub properties: Properties,
    pub id_mappings: IDMappings,
}

impl DocInfo {
    pub fn from_cfb(cfb: &mut CompoundFile<Cursor<Vec<u8>>>, version: &Version) -> DocInfo {
        let mut stream = cfb.open_stream("/DocInfo").unwrap();
        let mut data = DeflateDecoder::new(&mut stream);

        let properties = Properties::from_reader(&mut data);
        let id_mappings = IDMappings::from_reader(&mut data, version);

        DocInfo {
            properties,
            id_mappings,
        }
    }
}
