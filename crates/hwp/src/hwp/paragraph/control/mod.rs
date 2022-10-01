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

        // 개체 공통 속성 컨트롤
        make_4chid!('t', 'b', 'l', ' ') |
        // NOTE: (@hahnlee) 표준문서에는 없고, 웹한글 기안기 API 문서에 설명 되어있다.
        make_4chid!('g', 's', 'o', ' ') |
        make_4chid!('$', 'l', 'i', 'n') |
        make_4chid!('$', 'r', 'e', 'c') |
        make_4chid!('$', 'e', 'l', 'l') |
        make_4chid!('$', 'a', 'r', 'c') |
        make_4chid!('$', 'p', 'o', 'l') |
        make_4chid!('$', 'c', 'u', 'r') |
        make_4chid!('e', 'q', 'e', 'd') |
        make_4chid!('$', 'p', 'i', 'c') |
        make_4chid!('$', 'o', 'l', 'e') |
        make_4chid!('$', 'c', 'o', 'n') |

        // // 개체 이외 컨트롤
        make_4chid!('a', 't', 'n', 'o') |
        make_4chid!('n', 'w', 'n', 'o') |
        make_4chid!('p', 'g', 'h', 'd') |
        make_4chid!('p', 'g', 'c', 't') |
        make_4chid!('p', 'g', 'n', 'p') |
        make_4chid!('i', 'd', 'x', 'm') |
        make_4chid!('b', 'o', 'k', 'm') |
        make_4chid!('t', 'c', 'p', 's') |
        make_4chid!('t', 'd', 'u', 't') |

        // 개체 이외 컨트롤 + 문단리스트
        make_4chid!('h', 'e', 'a', 'd') |
        make_4chid!('f', 'o', 'o', 't') |
        make_4chid!('f', 'n', ' ', ' ')  |
        make_4chid!('e', 'n', ' ', ' ')  |
        make_4chid!('t', 'c', 'm', 't')  |
        make_4chid!('c', 'o', 'l', 'd') |

        // 필드 컨트롤
        make_4chid!('%', 'u', 'n', 'k') |
        make_4chid!('%', 'd', 't', 'e') |
        make_4chid!('%', 'd', 'd', 't') |
        make_4chid!('%', 'p', 'a', 't') |
        make_4chid!('%', 'b', 'm', 'k') |
        make_4chid!('%', 'm', 'm', 'g') |
        make_4chid!('%', 'x', 'r', 'f') |
        make_4chid!('%', 'f', 'm', 'u') |
        make_4chid!('%', 'c', 'l', 'k') |
        make_4chid!('%', 's', 'm', 'r') |
        make_4chid!('%', 'u', 's', 'r') |
        make_4chid!('%', 'h', 'l', 'k') |
        make_4chid!('%', 's', 'i', 'g') |
        make_4chid!('%', '%', '*', 'd') |
        make_4chid!('%', '%', '*', 'a') |
        make_4chid!('%', '%', '*', 'C') |
        make_4chid!('%', '%', '*', 'S') |
        make_4chid!('%', '%', '*', 'T') |
        make_4chid!('%', '%', '*', 'P') |
        make_4chid!('%', '%', '*', 'L') |
        make_4chid!('%', '%', '*', 'c') |
        make_4chid!('%', '%', '*', 'h') |
        make_4chid!('%', '%', '*', 'A') |
        make_4chid!('%', '%', '*', 'i') |
        make_4chid!('%', '%', '*', 't') |
        make_4chid!('%', '%', '*', 'r') |
        make_4chid!('%', '%', '*', 'l') |
        make_4chid!('%', '%', '*', 'n') |
        make_4chid!('%', '%', '*', 'e') |
        make_4chid!('%', 's', 'p', 'l') |
        make_4chid!('%', '%', 'm', 'r') |
        make_4chid!('%', '%', 'm', 'e') |
        make_4chid!('%', 'c', 'p', 'r') |
        make_4chid!('%', 't', 'o', 'c') |
        _ => Control::Unknown(ctrl_id, record.remain_children()),
    }
}
