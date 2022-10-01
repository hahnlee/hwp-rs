pub mod footnote_shape;
pub mod page_definition;
pub mod section;

use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;

use crate::hwp::record::{tags::BodyTextRecord, Record};

use self::section::SectionControl;

#[derive(Debug)]
pub enum Control {
    // 개체 이외 컨트롤
    Secd(SectionControl),

    // 지원 안하는 레코드
    Unknown(u32, Vec<Record>),
}

pub fn parse_control(record: Record) -> Control {
    if record.tag_id != BodyTextRecord::HWPTAG_CTRL_HEADER as u32 {
        // TODO: (@hahnlee) Result로 바꾸기
        panic!("잘못된 레코드 입니다 {}", record.tag_id);
    }

    let mut reader = record.get_data_reader();
    let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

    // NOTE: (@hahnlee) 모르는 컨트롤을 만날 경우 하위에 레코드가 있을 수 있어 잘못 파싱할 수 있다.
    // TODO: (@hahnlee) 에러를 발생하는게 맞는지 검토
    match ctrl_id {
        make_4chid!('s', 'e', 'c', 'd') => Control::Secd(SectionControl::from_record(record)),
        _ => Control::Unknown(ctrl_id, record.remain_children()),
    }
}
