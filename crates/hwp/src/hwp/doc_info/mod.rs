pub mod id_mappings;
pub mod properties;

use std::io::Cursor;

use byteorder::LittleEndian;
use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use self::{properties::Properties, id_mappings::IDMappings};

use super::{record::reader::RecordReader, version::Version};

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
