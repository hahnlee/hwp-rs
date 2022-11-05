pub mod bin_data;
pub mod border_fill;
pub mod bullet;
pub mod change_tracking;
pub mod change_tracking_author;
pub mod char_shape;
pub mod compatible_document;
pub mod font;
pub mod id_mappings;
pub mod memo_shape;
pub mod numbering;
pub mod paragraph_shape;
pub mod properties;
pub mod style;
pub mod tab_definition;
pub mod track_change;

use std::io::{Read, Seek};

use cfb::CompoundFile;
use flate2::read::DeflateDecoder;

use crate::hwp::{
    doc_info::{compatible_document::CompatibleDocument, track_change::TrackChange},
    record::RecordCursor,
};

use self::{id_mappings::IDMappings, properties::Properties};

use super::{header::Header, record::tags::DocInfoRecord, version::Version};

#[derive(Debug)]
pub struct DocInfo {
    pub properties: Properties,
    pub id_mappings: IDMappings,
    pub compatible_document: Option<CompatibleDocument>,
    /// 변경 추적 정보
    pub track_change: Option<TrackChange>,
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
        let mut cursor = RecordCursor::new(reader);

        let properties = Properties::from_record(&mut cursor.current());
        let id_mappings = IDMappings::from_record_cursor(&mut cursor, &version);

        if cursor.record_id(DocInfoRecord::HWPTAG_DOC_DATA as u32) {
            // TODO: (@hahnlee) 파싱하기
            cursor.current();
        }
        if cursor.record_id(DocInfoRecord::HWPTAG_FORBIDDEN_CHAR as u32) {
            // TODO: (@hahnlee) 파싱하기
            cursor.current();
        }

        let compatible_document =
            if cursor.record_id(DocInfoRecord::HWPTAG_COMPATIBLE_DOCUMENT as u32) {
                Some(CompatibleDocument::from_record_cursor(&mut cursor))
            } else {
                None
            };

        let track_change = if cursor.record_id(DocInfoRecord::HWPTAG_TRACKCHANGE as u32) {
            Some(TrackChange::from_record_cursor(&mut cursor))
        } else {
            None
        };

        assert!(!cursor.has_next());

        Self {
            properties,
            id_mappings,
            compatible_document,
            track_change,
        }
    }
}
