pub mod char;
pub mod header;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use self::{
    char::{read_char, Char},
    header::ParagraphHeader,
};

use super::{
    record::{reader::RecordReader, tags::BodyTextRecord},
    version::Version,
};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub chars: Vec<Char>,
}

impl Paragraph {
    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Paragraph {
        let header = ParagraphHeader::from_reader(reader, version);

        // TODO: (@hahnlee) header.chars가 0이면 무시?
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_TEXT as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            panic!("잘못된 정보입니다");
        }

        let mut chars = Vec::with_capacity(header.chars as usize);
        let mut i = 0;
        loop {
            if i >= header.chars {
                break;
            }
            let char = read_char(&mut record);
            match char {
                Char::CharCode(_) => {
                    i += 1;
                }
                _ => {
                    i += 8;
                }
            };
            chars.push(char);
        }

        let mut buf = Vec::new();
        record.read_to_end(&mut buf).unwrap();

        // TODO: (@hahnlee) header.char_shapes가 0일때 고려
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_CHAR_SHAPE as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            println!("{tag_id}");
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

        Paragraph { header, chars }
    }

    pub fn to_string(&self) -> String {
        // TODO: (@hahnlee) 테이블 어떻게 하는지 알아보기
        let mut buf: Vec<u16> = Vec::new();

        for char in &self.chars {
            // TODO: (@hahnlee) CharControl 확인하기
            if let Char::CharCode(char_code) = char {
                buf.push(*char_code);
            }
        }

        String::from_utf16(&buf).unwrap()
    }
}
