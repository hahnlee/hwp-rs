pub mod char;
pub mod chars;
pub mod control;
pub mod header;

use std::io::{Read, Seek};

use byteorder::LittleEndian;

use self::{
    chars::Chars,
    control::{parse_control, Control},
    header::ParagraphHeader,
};

use super::{
    record::{reader::{RecordReader, traverse_records}, tags::BodyTextRecord},
    version::Version,
};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub controls: Vec<Control>,
    chars: Chars,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<u8>,
}

impl Paragraph {
    pub fn from_reader<T: Read + Seek>(reader: &mut T, version: &Version) -> Paragraph {
        let header = ParagraphHeader::from_reader(reader, version);

        // TODO: (@hahnlee) header.chars가 0이면 무시?
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PARA_TEXT as u32 {
            // TODO: (@hahnlee) 옵셔널로 바꾸기
            panic!("잘못된 정보입니다");
        }

        let chars = Chars::from_reader(&mut record, header.chars as usize);

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

        let control_count = chars.extend_control_count();
        let mut controls: Vec<Control> = Vec::with_capacity(control_count);
        for _ in 0..control_count {
            controls.push(parse_control(reader));
        }

        // TODO: (@hahnlee) 상태에 따른 파싱 추가
        let unknown = traverse_records(reader, 0);

        Paragraph {
            header,
            chars,
            controls,
            unknown,
        }
    }

    pub fn to_string(&self) -> String {
        self.chars.to_string()
    }
}
