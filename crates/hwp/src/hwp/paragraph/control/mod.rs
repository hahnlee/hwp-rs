pub mod section;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;

use crate::hwp::record::{reader::RecordReader, tags::BodyTextRecord};

use self::section::SectionControl;

#[derive(Debug)]
pub enum Control {
    // 개체 이외 컨트롤
    Secd(SectionControl),
}

pub fn parse_control<T: Read>(reader: &mut T) -> Option<Control> {
    let (tag_id, _, size) = reader.read_record_meta::<LittleEndian>().unwrap();
    if tag_id != BodyTextRecord::HWPTAG_CTRL_HEADER as u32 {
        // TODO: (@hahnlee) Result로 바꾸기
        panic!("잘못된 레코드 입니다");
    }

    let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

    match ctrl_id {
        make_4chid!('s', 'e', 'c', 'd') => {
            Some(Control::Secd(SectionControl::from_reader(reader, size - 4)))
        }
        _ => None,
    }
}
