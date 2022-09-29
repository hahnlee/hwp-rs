pub mod bin_data;
pub mod font;
pub mod id_mappings;
pub mod properties;

use std::io::Cursor;

use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use self::{bin_data::BinData, font::Font, id_mappings::IDMappings, properties::Properties};

use super::version::Version;

#[derive(Debug)]
pub struct DocInfo {
    pub properties: Properties,
    pub id_mappings: IDMappings,
    pub bin_data_list: Vec<BinData>,
    pub fonts: Vec<Font>,
}

impl DocInfo {
    pub fn from_cfb(cfb: &mut CompoundFile<Cursor<Vec<u8>>>, version: &Version) -> DocInfo {
        let mut stream = cfb.open_stream("/DocInfo").unwrap();
        let mut data = DeflateDecoder::new(&mut stream);

        let properties = Properties::from_reader(&mut data);
        let id_mappings = IDMappings::from_reader(&mut data, version);

        let mut bin_data_list: Vec<BinData> = Vec::with_capacity(id_mappings.binary_data as usize);
        for _ in 0..id_mappings.binary_data {
            bin_data_list.push(BinData::from_reader(&mut data));
        }

        let mut fonts: Vec<Font> = Vec::with_capacity(id_mappings.fonts_count() as usize);

        for _ in 0..id_mappings.fonts_count() {
            let font = Font::from_reader(&mut data);
            fonts.push(font);
        }

        DocInfo {
            properties,
            id_mappings,
            bin_data_list,
            fonts,
        }
    }
}
