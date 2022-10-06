use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::page_definition::PageDefinition,
    record::{tags::BodyTextRecord, Record},
};

#[derive(Debug, Clone)]
pub struct SectionControl {
    pub page_definition: PageDefinition,
    pub footnote_shape: FootnoteEndnoteShape,
    pub endnote_shape: FootnoteEndnoteShape,
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
        let footnote_shape = FootnoteEndnoteShape::from_record(record.next_child());
        let endnote_shape = FootnoteEndnoteShape::from_record(record.next_child());

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


/// 각주 / 미주 모양
#[derive(Debug, Clone)]
pub struct FootnoteEndnoteShape {
    /// 사용자 기호
    pub user_char: char,
    /// 앞 장식 문자
    pub prefix_char: char,
    /// 뒤 장식 문자
    pub suffix_char: char,
    /// 시작 번호
    pub start_number: u16,

    /// 구분선 길이
    ///
    /// NOTE: 공식 문서와 다르게 실제로는 4바이트다
    pub divide_line_length: u32,
}

impl FootnoteEndnoteShape {
    pub fn from_record(record: Record) -> Self {
        if record.tag_id != BodyTextRecord::HWPTAG_FOOTNOTE_SHAPE as u32 {
            // TODO: (@hahnlee) Result 타입으로 바꾸는것 검토
            panic!("다른 레코드 입니다 {}", record.tag_id);
        }

        let mut reader = record.get_data_reader();

        // TODO: (@hahnlee) 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let user_char = char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let prefix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let suffix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        let start_number = reader.read_u16::<LittleEndian>().unwrap();

        let divide_line_length = reader.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 위 여백
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 구분선 아래 여백
        reader.read_i16::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 주석 사이 여백
        reader.read_i16::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 구분선 종류
        reader.read_u8().unwrap();
        // TODO: (@hahnlee) 구분선 굵기
        reader.read_u8().unwrap();

        // TODO: (@hahnlee) 구분선 색상
        reader.read_u32::<LittleEndian>().unwrap();

        Self {
            user_char,
            prefix_char,
            suffix_char,
            start_number,
            divide_line_length,
        }
    }
}
