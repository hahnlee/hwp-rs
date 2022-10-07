pub mod char;
pub mod char_list;
pub mod char_shape;
pub mod control;
pub mod header;
pub mod line_segment;
pub mod range_tag;

use self::{
    char::{Char, CharControls},
    char_list::CharList,
    char_shape::CharShape,
    control::{parse_control, Control},
    header::ParagraphHeader,
    line_segment::LineSegment,
    range_tag::RangeTag,
};

use super::{
    record::{tags::BodyTextRecord, Record},
    version::Version,
};

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub header: ParagraphHeader,
    pub char_list: CharList,
    pub char_shapes: Vec<CharShape>,
    pub line_segments: Vec<LineSegment>,
    pub range_tags: Vec<RangeTag>,
    pub controls: Vec<Control>,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<Record>,
}

impl Paragraph {
    pub fn from_record(record: &mut Record, version: &Version) -> Paragraph {
        let header = ParagraphHeader::from_reader(&mut record.get_data_reader(), version);

        // NOTE: (@hahnlee) 문서와 달리 header.chars가 0이 아니어도 없을 수 있다.
        let char_list = if record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_TEXT as u32) {
            let data = record.next_child().data;
            CharList::from_data(data, header.chars as usize)
        } else {
            CharList::new()
        };

        let mut char_shapes = Vec::new();
        if header.char_shapes > 0 {
            assert!(
                record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_CHAR_SHAPE as u32),
                "잘못된 레코드 입니다"
            );
            let child = record.next_child();
            let mut record = child.get_data_reader();

            for _ in 0..header.char_shapes {
                let char_shape = CharShape::from_reader(&mut record);
                char_shapes.push(char_shape);
            }
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

        let mut range_tags = Vec::new();
        if header.ranges > 0 {
            assert!(
                record.is_next_child_id(BodyTextRecord::HWPTAG_PARA_RANGE_TAG as u32),
                "잘못된 레코드 입니다"
            );
            let child = record.next_child();
            let mut reader = child.get_data_reader();
            for _ in 0..header.ranges {
                let range_tag = RangeTag::from_reader(&mut reader);
                range_tags.push(range_tag);
            }
        }

        let control_count = char_list.extend_control_count();
        let mut controls: Vec<Control> = Vec::with_capacity(control_count);
        for _ in 0..control_count {
            let child = record.next_child();
            controls.push(parse_control(child, version));
        }

        let unknown = record.remain_children();

        Paragraph {
            header,
            char_list,
            char_shapes,
            line_segments,
            range_tags,
            controls,
            unknown,
        }
    }

    pub fn to_string(&self) -> String {
        // TODO: (@hahnlee) 테이블 어떻게 하는지 알아보기
        let mut out = format!("");

        let mut i = 0;
        for char in &self.char_list.chars {
            match char {
                Char::CharCode(code) => {
                    out.push(char::from_u32((*code).into()).unwrap());
                }

                Char::CharControl(CharControls::LineBreak) => {
                    out.push('\n');
                }
                Char::ExtendedControl(_, _) => {
                    match &self.controls[i] {
                        Control::AutoNumber(auto_number) => {
                            out.push_str(&auto_number.to_string());
                        },
                        _ => {}
                    };

                    i += 1;
                }
                _ => {}
            };
        }

        out
    }
}
