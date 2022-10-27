use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::hwp::{
    paragraph::control::page_definition::PageDefinition,
    record::{tags::BodyTextRecord, Record},
    version::Version,
};

#[derive(Debug, Clone)]
pub struct SectionControl {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 동일한 페이지에서 서로 다른 단 사이의 간격
    pub column_space: i16,
    /// 세로 줄맞춤 간격
    pub vertical_alignment: i16,
    /// 가로 줄맛춤 간격
    pub horizontal_alignment: i16,
    /// 기본 탭 간격
    pub tab_space: u32,
    /// 번호 문단 모양 ID
    pub numbering_id: u16,
    /// 쪽 번호
    pub page_number: u16,
    /// 그림 번호
    pub picture_number: u16,
    /// 표 번호
    pub table_number: u16,
    /// 수식 번호
    pub equation_number: u16,
    /// 언어코드 (5.0.1.5 이상)
    /// https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/6c085406-a698-4e12-9d4d-c3b0ee3dbc4a
    pub lang_id: Option<u16>,
    /// 용지설정 정보
    pub page_definition: PageDefinition,
    /// 각주 모양 정보
    pub footnote_shape: FootnoteEndnoteShape,
    /// 미주 모양 정보
    pub endnote_shape: FootnoteEndnoteShape,
    // TODO: (@hahnlee) 재구성시 처리
    #[allow(dead_code)]
    unknown: Vec<u8>,
}

impl SectionControl {
    pub fn from_record(record: &mut Record, version: &Version) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        // TODO: (@hahnlee) 속성
        reader.read_u32::<LittleEndian>().unwrap();

        let column_space = reader.read_i16::<LittleEndian>().unwrap();
        let vertical_alignment = reader.read_i16::<LittleEndian>().unwrap();
        let horizontal_alignment = reader.read_i16::<LittleEndian>().unwrap();

        let tab_space = reader.read_u32::<LittleEndian>().unwrap();
        let numbering_id = reader.read_u16::<LittleEndian>().unwrap();

        let page_number = reader.read_u16::<LittleEndian>().unwrap();
        let picture_number = reader.read_u16::<LittleEndian>().unwrap();
        let table_number = reader.read_u16::<LittleEndian>().unwrap();
        let equation_number = reader.read_u16::<LittleEndian>().unwrap();

        let lang_id = if *version >= Version::from_str("5.0.1.5") {
            Some(reader.read_u16::<LittleEndian>().unwrap())
        } else {
            None
        };

        // NOTE: (@hahnlee) 표준 문서에 작성된 내용과 다르게 실제로는 더 많은 바이트가 있다.
        // TODO: (@hahnlee) 다른 속성의 바이트가 실제로 차이가 나는지 확인하기
        let mut unknown = Vec::new();
        reader.read_to_end(&mut unknown).unwrap();

        let page_definition = PageDefinition::from_record(&mut record.next_child());
        let footnote_shape = FootnoteEndnoteShape::from_record(&mut record.next_child());
        let endnote_shape = FootnoteEndnoteShape::from_record(&mut record.next_child());

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

        Self {
            ctrl_id,
            column_space,
            vertical_alignment,
            horizontal_alignment,
            tab_space,
            numbering_id,
            page_number,
            picture_number,
            table_number,
            equation_number,
            lang_id,
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
    pub fn from_record(record: &mut Record) -> Self {
        assert_eq!(record.tag_id, BodyTextRecord::HWPTAG_FOOTNOTE_SHAPE as u32);

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
