pub mod bin_data;
pub mod border_fill;
pub mod bullet;
pub mod change_tracking;
pub mod change_tracking_author;
pub mod char_shape;
pub mod font;
pub mod id_mappings;
pub mod memo_shape;
pub mod numbering;
pub mod paragraph_shape;
pub mod properties;
pub mod style;
pub mod tab_definition;

use std::io::{Read, Seek};

use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use crate::hwp::record::reader::read_records;

use self::{id_mappings::IDMappings, properties::Properties};

use super::{header::Header, version::Version};

#[derive(Debug)]
pub struct DocInfo {
    pub properties: Properties,
    pub id_mappings: IDMappings,
}

impl DocInfo {
    pub fn from_cfb<T: Read + Seek>(cfb: &mut CompoundFile<T>, header: &Header) -> Self {
        let mut stream = cfb.open_stream("/DocInfo").unwrap();
        if header.flags.compressed {
            let mut data = DeflateDecoder::new(&mut stream);
            return DocInfo::from_reader(&mut data, &header.version);
        } else {
            return DocInfo::from_reader(&mut stream, &header.version);
        }
    }

    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Self {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();
        let mut records = read_records(&mut data);
        records.reverse();

        let properties = Properties::from_record(&mut records.pop().unwrap());
        let id_mappings = IDMappings::from_record(&mut records.pop().unwrap(), &version);

        // TODO: @hahnlee 아래부터는 옵셔널로 보임
        // HWPTAG_DOC_DATA
        // HWPTAG_FORBIDDEN_CHAR
        // HWPTAG_COMPATIBLE_DOCUMENT
        // HWPTAG_LAYOUT_COMPATIBILITY
        // HWPTAG_DISTRIBUTE_DOC_DATA
        // HWPTAG_TRACKCHANGE

        Self {
            properties,
            id_mappings,
        }
    }
}
