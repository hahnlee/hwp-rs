pub mod footnote_shape;
pub mod page_definition;
pub mod section;
pub mod unknown;

use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;

use crate::hwp::record::{reader::RecordReader, tags::BodyTextRecord};

use self::{section::SectionControl, unknown::Unknown};

#[derive(Debug)]
pub enum Control {
    // 개체 이외 컨트롤
    Secd(SectionControl),

    // 지원 안하는 레코드
    Unknown(Unknown),
}

pub fn parse_control<T: Read + Seek>(reader: &mut T) -> Control {
    let (tag_id, level, size) = reader.read_record_meta::<LittleEndian>().unwrap();
    if tag_id != BodyTextRecord::HWPTAG_CTRL_HEADER as u32 {
        // TODO: (@hahnlee) Result로 바꾸기
        panic!("잘못된 레코드 입니다 {tag_id}");
    }

    let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

    let result = match ctrl_id {
        make_4chid!('s', 'e', 'c', 'd') => {
            Some(Control::Secd(SectionControl::from_reader(reader, size - 4)))
        }
        _ => None,
    };

    // NOTE: (@hahnlee) 모르는 컨트롤을 만날 경우 하위에 레코드가 있을 수 있어 잘못 파싱할 수 있다.
    // TODO: (@hahnlee) 에러를 발생하는게 맞는지 검토
    if result.is_none() {
        // TODO: (@hahnlee) 로거 변경
        println!("해석할 수 없는 컨트롤입니다. {ctrl_id}");
        return Control::Unknown(Unknown::from_reader(ctrl_id, reader, level, size - 4));
    }

    result.unwrap()
}
