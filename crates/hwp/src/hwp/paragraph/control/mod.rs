pub mod bookmark;
pub mod column;
pub mod common_properties;
pub mod container;
pub mod equation;
pub mod footnote_endnote;
pub mod header_footer;
pub mod hidden_comment;
pub mod index_mark;
pub mod number;
pub mod ole;
pub mod over_type;
pub mod page_definition;
pub mod page_hiding;
pub mod page_number_control;
pub mod page_number_position;
pub mod paragraph_list;
pub mod picture;
pub mod section;
pub mod shape_object;
pub mod sub_text;
pub mod table;
pub mod unknown;

use byteorder::{LittleEndian, ReadBytesExt};
use hwp_macro::make_4chid;

use crate::hwp::{
    record::{tags::BodyTextRecord, Record},
    version::Version,
};

use self::{
    bookmark::Bookmark,
    column::ColumnControl,
    container::Container,
    equation::Equation,
    footnote_endnote::FootnoteEndnote,
    header_footer::HeaderFooter,
    hidden_comment::HiddenComment,
    index_mark::IndexMark,
    number::AutoNumber,
    number::NewNumber,
    ole::Ole,
    over_type::OverType,
    page_hiding::PageHiding,
    page_number_control::PageNumberControl,
    page_number_position::PageNumberPosition,
    picture::Picture,
    section::SectionControl,
    shape_object::{
        GenShapeObject, ShapeArc, ShapeCurve, ShapeEllipse, ShapeLine, ShapePolygon, ShapeRectangle,
    },
    sub_text::SubText,
    table::TableControl,
    unknown::UnknownControl,
};

#[derive(Debug, Clone)]
pub enum Control {
    // 개체 공통 속성 컨트롤
    Table(TableControl),
    GenShapeObject(GenShapeObject),
    ShapeLine(ShapeLine),
    ShapeRectangle(ShapeRectangle),
    ShapeEllipse(ShapeEllipse),
    ShapeArc(ShapeArc),
    ShapePolygon(ShapePolygon),
    ShapeCurve(ShapeCurve),
    Equation(Equation),
    Picture(Picture),
    Ole(Ole),
    Container(Container),

    // 개체 이외 컨트롤
    AutoNumber(AutoNumber),
    NewNumber(NewNumber),
    PageHiding(PageHiding),
    PageNumberControl(PageNumberControl),
    PageNumberPosition(PageNumberPosition),
    IndexMark(IndexMark),
    Bookmark(Bookmark),
    OverType(OverType),
    SubText(SubText),

    // 개체 이외 컨트롤 + 문단리스트
    SectionDefinition(SectionControl),
    Header(HeaderFooter),
    Footer(HeaderFooter),
    Footnote(FootnoteEndnote),
    Endnote(FootnoteEndnote),
    HiddenComment(HiddenComment),
    Column(ColumnControl),

    // 지원 안하는 레코드
    Unknown(UnknownControl),
}

pub fn parse_control(record: Record, version: &Version) -> Control {
    assert_eq!(
        record.tag_id,
        BodyTextRecord::HWPTAG_CTRL_HEADER as u32,
        "잘못된 레코드 입니다 {}",
        record.tag_id
    );

    let mut reader = record.get_data_reader();
    let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

    // NOTE: (@hahnlee) 한글 표준 문서에는 누락된 컨트롤이 있다
    // https://www.hancom.com/board/devmanualList.do
    match ctrl_id {
        // 개체 공통 속성 컨트롤
        make_4chid!('t', 'b', 'l', ' ') => {
            Control::Table(TableControl::from_record(record, version))
        }
        make_4chid!('g', 's', 'o', ' ') => {
            Control::GenShapeObject(GenShapeObject::from_record(record, version))
        }
        make_4chid!('$', 'l', 'i', 'n') => {
            Control::ShapeLine(ShapeLine::from_record(record, version))
        }
        make_4chid!('$', 'r', 'e', 'c') => {
            Control::ShapeRectangle(ShapeRectangle::from_record(record, version))
        }
        make_4chid!('$', 'e', 'l', 'l') => {
            Control::ShapeEllipse(ShapeEllipse::from_record(record, version))
        }
        make_4chid!('$', 'a', 'r', 'c') => {
            Control::ShapeArc(ShapeArc::from_record(record, version))
        }
        make_4chid!('$', 'p', 'o', 'l') => {
            Control::ShapePolygon(ShapePolygon::from_record(record, version))
        }
        make_4chid!('$', 'c', 'u', 'r') => {
            Control::ShapeCurve(ShapeCurve::from_record(record, version))
        }
        make_4chid!('e', 'q', 'e', 'd') => {
            Control::Equation(Equation::from_record(record, version))
        }
        make_4chid!('$', 'p', 'i', 'c') => Control::Picture(Picture::from_record(record, version)),
        make_4chid!('$', 'o', 'l', 'e') => Control::Ole(Ole::from_record(record, version)),
        make_4chid!('$', 'c', 'o', 'n') => {
            Control::Container(Container::from_record(record, version))
        }

        // TODO: (@hahnlee) 파싱하기
        // 개체 이외 컨트롤
        make_4chid!('c', 'o', 'l', 'd') => Control::Column(ColumnControl::from_record(record)),
        make_4chid!('a', 't', 'n', 'o') => Control::AutoNumber(AutoNumber::from_record(record)),
        make_4chid!('n', 'w', 'n', 'o') => Control::NewNumber(NewNumber::from_record(record)),
        make_4chid!('p', 'g', 'h', 'd') => Control::PageHiding(PageHiding::from_record(record)),
        make_4chid!('p', 'g', 'c', 't') => {
            Control::PageNumberControl(PageNumberControl::from_record(record))
        }
        make_4chid!('p', 'g', 'n', 'p') => {
            Control::PageNumberPosition(PageNumberPosition::from_record(record))
        }
        make_4chid!('i', 'd', 'x', 'm') => Control::IndexMark(IndexMark::from_record(record)),
        make_4chid!('b', 'o', 'k', 'm') => Control::Bookmark(Bookmark::from_record(record)),
        make_4chid!('t', 'c', 'p', 's') => Control::OverType(OverType::from_record(record)),
        make_4chid!('t', 'd', 'u', 't') => Control::SubText(SubText::from_record(record)),

        // 개체 이외 컨트롤 + 문단리스트
        make_4chid!('s', 'e', 'c', 'd') => {
            Control::SectionDefinition(SectionControl::from_record(record))
        }
        make_4chid!('h', 'e', 'a', 'd') => {
            Control::Header(HeaderFooter::from_record(record, version))
        }
        make_4chid!('f', 'o', 'o', 't') => {
            Control::Footer(HeaderFooter::from_record(record, version))
        }
        make_4chid!('f', 'n', ' ', ' ') => {
            Control::Footnote(FootnoteEndnote::from_record(record, version))
        }
        make_4chid!('e', 'n', ' ', ' ') => {
            Control::Endnote(FootnoteEndnote::from_record(record, version))
        }
        make_4chid!('t', 'c', 'm', 't') => {
            Control::HiddenComment(HiddenComment::from_record(record, version))
        }

        // 필드 컨트롤
        make_4chid!('%', 'u', 'n', 'k')
        | make_4chid!('%', 'd', 't', 'e')
        | make_4chid!('%', 'd', 'd', 't')
        | make_4chid!('%', 'p', 'a', 't')
        | make_4chid!('%', 'b', 'm', 'k')
        | make_4chid!('%', 'm', 'm', 'g')
        | make_4chid!('%', 'x', 'r', 'f')
        | make_4chid!('%', 'f', 'm', 'u')
        | make_4chid!('%', 'c', 'l', 'k')
        | make_4chid!('%', 's', 'm', 'r')
        | make_4chid!('%', 'u', 's', 'r')
        | make_4chid!('%', 'h', 'l', 'k')
        | make_4chid!('%', 's', 'i', 'g')
        | make_4chid!('%', '%', '*', 'd')
        | make_4chid!('%', '%', '*', 'a')
        | make_4chid!('%', '%', '*', 'C')
        | make_4chid!('%', '%', '*', 'S')
        | make_4chid!('%', '%', '*', 'T')
        | make_4chid!('%', '%', '*', 'P')
        | make_4chid!('%', '%', '*', 'L')
        | make_4chid!('%', '%', '*', 'c')
        | make_4chid!('%', '%', '*', 'h')
        | make_4chid!('%', '%', '*', 'A')
        | make_4chid!('%', '%', '*', 'i')
        | make_4chid!('%', '%', '*', 't')
        | make_4chid!('%', '%', '*', 'r')
        | make_4chid!('%', '%', '*', 'l')
        | make_4chid!('%', '%', '*', 'n')
        | make_4chid!('%', '%', '*', 'e')
        | make_4chid!('%', 's', 'p', 'l')
        | make_4chid!('%', '%', 'm', 'r')
        | make_4chid!('%', '%', 'm', 'e')
        | make_4chid!('%', 'c', 'p', 'r')
        | make_4chid!('%', 't', 'o', 'c')
        | _ => Control::Unknown(UnknownControl::from_record(record)),
    }
}
