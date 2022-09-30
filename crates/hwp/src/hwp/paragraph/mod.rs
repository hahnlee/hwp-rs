pub mod header;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use self::header::ParagraphHeader;

use super::{
    record::{reader::RecordReader, tags::BodyTextRecord},
    version::Version,
};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
}

impl Paragraph {
    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Paragraph {
        let header = ParagraphHeader::from_reader(reader, version);

        // TODO: (@hahnlee) header.chars가 0이면 무시
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_TEXT as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            panic!("잘못된 정보입니다");
        }
        let mut buf = Vec::new();
        for _ in 0..header.chars {
            buf.push(record.read_u16::<LittleEndian>().unwrap());
        }

        // TODO: (@hahnlee) header.char_shapes가 0일때 고려
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_CHAR_SHAPE as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            panic!("잘못된 정보입니다");
        }
        // TODO: (@hahnlee) header.char_shapes 수만큼 읽기
        let mut buf = Vec::new();
        record.read_to_end(&mut buf).unwrap();

        for _ in 0..header.aligns {
            let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
            if tag_id != BodyTextRecord::HWPTAG_PARA_LINE_SEG as u32 {
                // TODO: (@hahnlee) 옵셔널로 바꾸기
                panic!("잘못된 정보입니다");
            }
            let mut buf = Vec::new();
            record.read_to_end(&mut buf).unwrap();
        }

        for _ in 0..header.ranges {
            let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
            if tag_id != BodyTextRecord::HWPTAG_PARA_RANGE_TAG as u32 {
                // TODO: (@hahnlee) 옵셔널로 바꾸기
                panic!("잘못된 정보입니다");
            }
            let mut buf = Vec::new();
            record.read_to_end(&mut buf).unwrap();
        }

        Paragraph { header }
    }
}
