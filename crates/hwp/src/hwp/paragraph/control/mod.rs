pub mod footnote_shape;
pub mod page_definition;
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

pub fn parse_control<T: Read>(reader: &mut T) -> Control {
    let (tag_id, _, size) = reader.read_record_meta::<LittleEndian>().unwrap();
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
    // TODO: (@hahnlee) 상위버전의 파일을 읽을 수 있도록 구조를 잡는 방법을 찾아보기
    if result.is_none() {
        panic!("지원하지 않는 컨트롤 입니다. ctrl_id: {ctrl_id}");
    }

    result.unwrap()
}
