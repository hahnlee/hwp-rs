use super::record::reader::RecordReader;

use std::io::Cursor;

use byteorder::LittleEndian;
use cfb::Stream;
use flate2::read::DeflateDecoder;

#[derive(Debug)]
pub struct Section {}

impl Section {
    pub fn from_stream(stream: &mut Stream<Cursor<Vec<u8>>>) -> Section {
        let mut data = DeflateDecoder::new(stream);

        // TODO: (@hahnlee) record 구현
        data.read_record::<LittleEndian>().unwrap();

        Section {}
    }
}
