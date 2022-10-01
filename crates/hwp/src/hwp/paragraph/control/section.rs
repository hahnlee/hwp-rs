use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::page_definition::PageDefinition,
    record::{reader::RecordReader, tags::BodyTextRecord},
};

use super::footnote_shape::FootnoteShape;

#[derive(Debug)]
pub struct SectionControl {
    pub page_definition: PageDefinition,
    pub footnote_shape: FootnoteShape,
    pub endnote_shape: FootnoteShape,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<u8>,
}

impl SectionControl {
    pub fn from_reader<T: Read>(reader: &mut T, size: u32) -> SectionControl {
        let mut record = reader.take(size.into());

        // TODO: (@hahnlee) 관련파싱 추가하기
        record.read_u32::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        record.read_u32::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();
        record.read_u16::<LittleEndian>().unwrap();

        // 버전 분기 추가
        record.read_u16::<LittleEndian>().unwrap();

        // NOTE: (@hahnlee) 표준 문서에 작성된 내용과 다르게 실제로는 더 많은 바이트가 있다.
        // TODO: (@hahnlee) 다른 속성의 바이트가 실제로 차이가 나는지 확인하기
        let mut unknown = Vec::new();
        record.read_to_end(&mut unknown).unwrap();

        let page_definition = PageDefinition::from_reader(reader);
        let footnote_shape = FootnoteShape::from_reader(reader);
        let endnote_shape = FootnoteShape::from_reader(reader);

        // NOTE: (@hahnlee) 양쪽, 홀수, 짝수 정보가 반복됨.
        // TODO: (@hahnlee) 항상 모든 모든 정보를 내려주는지 확인필요
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32 {
            panic!("쪽/배경 설정이 아닙니다");
        }
        let mut buf = Vec::new();
        record.read_to_end(&mut buf).unwrap();

        // NOTE: (@hahnlee) 양쪽, 홀수, 짝수 정보가 반복됨.
        // TODO: (@hahnlee) 항상 모든 모든 정보를 내려주는지 확인필요
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32 {
            panic!("쪽/배경 설정이 아닙니다");
        }
        let mut buf = Vec::new();
        record.read_to_end(&mut buf).unwrap();

        // NOTE: (@hahnlee) 양쪽, 홀수, 짝수 정보가 반복됨.
        // TODO: (@hahnlee) 항상 모든 모든 정보를 내려주는지 확인필요
        let (tag_id, _, _, mut record) = reader.read_record::<LittleEndian>().unwrap();
        if tag_id != BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32 {
            panic!("쪽/배경 설정이 아닙니다");
        }
        let mut buf = Vec::new();
        record.read_to_end(&mut buf).unwrap();

        // TODO: (@hahnlee) 바탕쪽 정보 관련된 파싱 추가하기

        SectionControl {
            unknown,
            page_definition,
            footnote_shape,
            endnote_shape,
        }
    }
}
