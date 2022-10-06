use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::page_definition::PageDefinition,
    record::{tags::BodyTextRecord, Record},
};

use super::footnote_shape::FootnoteShape;

#[derive(Debug, Clone)]
pub struct SectionControl {
    pub page_definition: PageDefinition,
    pub footnote_shape: FootnoteShape,
    pub endnote_shape: FootnoteShape,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<u8>,
}

impl SectionControl {
    pub fn from_record(mut record: Record) -> SectionControl {
        let mut reader = record.get_data_reader();
        // TODO: (@hahnlee) 관련파싱 추가하기
        reader.read_u32::<LittleEndian>().unwrap();

        reader.read_u16::<LittleEndian>().unwrap();
        reader.read_u16::<LittleEndian>().unwrap();
        reader.read_u16::<LittleEndian>().unwrap();

        reader.read_u32::<LittleEndian>().unwrap();

        reader.read_u16::<LittleEndian>().unwrap();
        reader.read_u16::<LittleEndian>().unwrap();

        reader.read_u16::<LittleEndian>().unwrap();
        reader.read_u16::<LittleEndian>().unwrap();
        reader.read_u16::<LittleEndian>().unwrap();

        // 버전 분기 추가
        reader.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 표준 문서에 작성된 내용과 다르게 실제로는 더 많은 바이트가 있다.
        // TODO: (@hahnlee) 다른 속성의 바이트가 실제로 차이가 나는지 확인하기
        let mut unknown = Vec::new();
        reader.read_to_end(&mut unknown).unwrap();

        let page_definition = PageDefinition::from_record(record.next_child());
        let footnote_shape = FootnoteShape::from_record(record.next_child());
        let endnote_shape = FootnoteShape::from_record(record.next_child());

        // NOTE: (@hahnlee) 양쪽, 홀수, 짝수 정보가 반복됨.
        // TODO: (@hahnlee) 항상 모든 모든 정보를 내려주는지 확인필요
        assert_eq!(
            record.next_child().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );
        assert_eq!(
            record.next_child().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );
        assert_eq!(
            record.next_child().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );

        // TODO: (@hahnlee) 바탕쪽 정보 관련된 파싱 추가하기

        SectionControl {
            unknown,
            page_definition,
            footnote_shape,
            endnote_shape,
        }
    }
}
