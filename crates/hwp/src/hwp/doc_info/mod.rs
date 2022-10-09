pub mod bin_data;
pub mod font;
pub mod id_mappings;
pub mod properties;

use std::io::{Read, Seek};

use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use crate::hwp::record::reader::RecordReader;
use byteorder::LittleEndian;

use self::{bin_data::BinData, font::Font, id_mappings::IDMappings, properties::Properties};

use super::{header::Header, record::tags::DocInfoRecord, version::Version};

#[derive(Debug)]
pub struct DocInfo {
    pub properties: Properties,
    pub id_mappings: IDMappings,
    pub bin_data_list: Vec<BinData>,
    pub fonts: Vec<Font>,
}

impl DocInfo {
    pub fn from_cfb<T: Read + Seek>(cfb: &mut CompoundFile<T>, header: &Header) -> DocInfo {
        let mut stream = cfb.open_stream("/DocInfo").unwrap();
        if header.flags.compressed {
            let mut data = DeflateDecoder::new(&mut stream);
            return DocInfo::from_reader(&mut data, &header.version);
        } else {
            return DocInfo::from_reader(&mut stream, &header.version);
        }
    }

    pub fn from_reader<T: Read>(data: &mut T, version: &Version) -> Self {
        let properties = Properties::from_reader(data);
        let id_mappings = IDMappings::from_reader(data, &version);

        let mut bin_data_list: Vec<BinData> = Vec::with_capacity(id_mappings.binary_data as usize);
        for _ in 0..id_mappings.binary_data {
            bin_data_list.push(BinData::from_reader(data));
        }

        let mut fonts: Vec<Font> = Vec::with_capacity(id_mappings.fonts_count() as usize);

        for _ in 0..id_mappings.fonts_count() {
            let font = Font::from_reader(data);
            fonts.push(font);
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.border_fils {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_BORDER_FILL as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.char_shapes {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_CHAR_SHAPE as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.tab_definitions {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_TAB_DEF as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.numbering {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_NUMBERING as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.bullets {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_BULLET as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.paragraph_shapes {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_PARA_SHAPE as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.styles {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_STYLE as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.memo_shapes {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_MEMO_SHAPE as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.change_tracking_users {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_TRACK_CHANGE_AUTHOR as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: (@hahnlee)
        for _ in 0..id_mappings.change_trackings {
            let (tag_id, _, _, mut stream) = data.read_record::<LittleEndian>().unwrap();
            if tag_id != DocInfoRecord::HWPTAG_TRACK_CHANGE as u32 {
                // TODO: (@hahnlee) 옵셔널
                panic!("올바르지 않은 정보");
            }
            let mut buf: Vec<u8> = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
        }

        // TODO: @hahnlee 아래부터는 옵셔널로 보임
        // HWPTAG_DOC_DATA
        // HWPTAG_FORBIDDEN_CHAR
        // HWPTAG_COMPATIBLE_DOCUMENT
        // HWPTAG_LAYOUT_COMPATIBILITY
        // HWPTAG_DISTRIBUTE_DOC_DATA
        // HWPTAG_TRACKCHANGE

        DocInfo {
            properties,
            id_mappings,
            bin_data_list,
            fonts,
        }
    }
}
