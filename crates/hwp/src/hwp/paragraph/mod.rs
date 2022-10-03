pub mod char;
pub mod chars;
pub mod control;
pub mod header;
pub mod line_segment;

use std::io::Read;

use byteorder::LittleEndian;

use self::{
    chars::Chars,
    control::{parse_control, Control},
    header::ParagraphHeader,
    line_segment::LineSegment,
};

use super::{
    record::{reader::RecordReader, tags::BodyTextRecord, Record},
    version::Version,
};

#[derive(Debug)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub line_segments: Vec<LineSegment>,
    pub controls: Vec<Control>,
    chars: Chars,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<Record>,
}

impl Paragraph {
    pub fn from_record(record: &mut Record, version: &Version) -> Paragraph {
        let header = ParagraphHeader::from_reader(&mut record.get_data_reader(), version);

        // NOTE: (@hahnlee) 문서와 달리 header.chars가 0이 아니어도 없을 수 있다.
        let chars = if record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_TEXT as u32) {
            let data = record.next_child().data;
            Chars::from_data(data, header.chars as usize)
        } else {
            Chars::new()
        };

        if record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_CHAR_SHAPE as u32) {
            let child = record.next_child();
            let mut record = child.get_data_reader();
            // TODO: (@hahnlee) header.char_shapes 수만큼 읽기
            let mut buf = Vec::new();
            record.read_to_end(&mut buf).unwrap();
        }

        let mut line_segments = Vec::new();
        if header.aligns > 0 {
            assert!(
                record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_LINE_SEG as u32),
                "잘못된 레코드 입니다"
            );
            let child = record.next_child();
            let mut reader = child.get_data_reader();
            for _ in 0..header.aligns {
                let line_segment = LineSegment::from_reader(&mut reader);
                line_segments.push(line_segment);
            }
        }

        for _ in 0..header.ranges {
            let child = record.next_child();
            let mut reader = child.get_data_reader();
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
            let child = record.next_child();
            controls.push(parse_control(child, version));
        }

        let unknown = record.remain_children();

        Paragraph {
            header,
            line_segments,
            chars,
            controls,
            unknown,
        }
    }

    pub fn to_string(&self) -> String {
        self.chars.to_string()
    }
}
