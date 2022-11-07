use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::hwp::{
    doc_info::border_fill::Border,
    paragraph::control::page_definition::PageDefinition,
    record::{tags::BodyTextRecord, Record, RecordCursor},
    utils::bits::{get_flag, get_value_range},
    version::Version,
};

#[derive(Debug, Clone)]
pub struct SectionControl {
    /// 컨트롤 ID
    pub ctrl_id: u32,
    /// 머리말을 감출지 여부
    pub hide_header: bool,
    /// 꼬리말을 감출지 여부
    pub hide_footer: bool,
    /// 바탕쪽 숨김 여부
    pub hide_master_page: bool,
    /// 테두리 숨김 여부
    pub hide_border: bool,
    /// 배경 숨김 여부
    pub hide_fill: bool,
    /// 페이지 번호 숨김 여부
    pub hide_page_number: bool,
    /// 구역의 첫 쪽에만 테두리 표시 여부
    pub border_on_first_page: bool,
    /// 구역의 첫 쪽에만 배경 표시 여부
    pub fill_on_first_page: bool,
    /// 텍스트 방향
    pub text_direction: TextDirection,
    /// 빈 줄 감춤 여부
    pub hide_empty_line: bool,
    /// 구역 나눔으로 새 페이지가 생길 때의 페이지 번호 적용할지 여부
    pub new_page_number: bool,
    /// 원고지 정서법 적용 여부
    pub manuscript_paper_orthography: bool,
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
    pub unknown: Vec<u8>,
}

impl SectionControl {
    pub fn from_record(record: &mut Record, cursor: &mut RecordCursor, version: &Version) -> Self {
        let mut reader = record.get_data_reader();

        let ctrl_id = reader.read_u32::<LittleEndian>().unwrap();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        let hide_header = get_flag(attribute, 0);
        let hide_footer = get_flag(attribute, 1);
        let hide_master_page = get_flag(attribute, 2);
        let hide_border = get_flag(attribute, 3);
        let hide_fill = get_flag(attribute, 4);
        let hide_page_number = get_flag(attribute, 5);
        let border_on_first_page = get_flag(attribute, 8);
        let fill_on_first_page = get_flag(attribute, 9);
        let text_direction = TextDirection::from_u32(get_value_range(attribute, 16, 18)).unwrap();
        let hide_empty_line = get_flag(attribute, 19);
        let new_page_number = if get_value_range(attribute, 20, 21) == 0 {
            false
        } else {
            true
        };
        let manuscript_paper_orthography = get_flag(attribute, 22);

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

        let page_definition = PageDefinition::from_record(&mut cursor.current());
        let footnote_shape = FootnoteEndnoteShape::from_record(&mut cursor.current());
        let endnote_shape = FootnoteEndnoteShape::from_record(&mut cursor.current());

        // NOTE: (@hahnlee) 양쪽, 홀수, 짝수 정보가 반복됨.
        // TODO: (@hahnlee) 항상 모든 모든 정보를 내려주는지 확인필요
        assert_eq!(
            cursor.current().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );
        assert_eq!(
            cursor.current().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );
        assert_eq!(
            cursor.current().tag_id,
            BodyTextRecord::HWPTAG_PAGE_BORDER_FILL as u32,
            "쪽/배경 설정이 아닙니다"
        );

        // TODO: (@hahnlee) 바탕쪽 정보 관련된 파싱 추가하기

        Self {
            ctrl_id,
            hide_header,
            hide_footer,
            hide_master_page,
            hide_border,
            hide_fill,
            hide_page_number,
            border_on_first_page,
            fill_on_first_page,
            text_direction,
            hide_empty_line,
            new_page_number,
            manuscript_paper_orthography,
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

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum TextDirection {
    Horizontal,
    Vertical,
}

/// 각주 / 미주 모양
#[derive(Debug, Clone)]
pub struct FootnoteEndnoteShape {
    /// 번호 모양
    pub number_shape: NumberShape,
    /// 사용자 기호
    pub user_char: char,
    /// 앞 장식 문자
    pub prefix_char: char,
    /// 뒤 장식 문자
    pub suffix_char: char,
    /// 시작 번호
    pub start_number: u16,
    /// 구분선 위 여백
    pub margin_top: i16,
    /// 구분선 아래 여백
    pub margin_bottom: i16,
    /// 주석 사이 여백
    pub comment_margin: i16,
    /// 구분선 길이
    ///
    /// NOTE: 공식 문서와 다르게 실제로는 4바이트다
    pub divide_line_length: u32,
    /// 구분선
    pub border: Border,
}

impl FootnoteEndnoteShape {
    pub fn from_record(record: &mut Record) -> Self {
        assert_eq!(record.tag_id, BodyTextRecord::HWPTAG_FOOTNOTE_SHAPE as u32);

        let mut reader = record.get_data_reader();

        let attribute = reader.read_u32::<LittleEndian>().unwrap();
        // TODO: (@hahnlee) 속성 파싱
        let number_shape = NumberShape::from_u32(get_value_range(attribute, 0, 7)).unwrap();

        let user_char = char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let prefix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();
        let suffix_char =
            char::from_u32(reader.read_u16::<LittleEndian>().unwrap().into()).unwrap();

        let start_number = reader.read_u16::<LittleEndian>().unwrap();

        let divide_line_length = reader.read_u32::<LittleEndian>().unwrap();

        let margin_top = reader.read_i16::<LittleEndian>().unwrap();
        let margin_bottom = reader.read_i16::<LittleEndian>().unwrap();

        let comment_margin = reader.read_i16::<LittleEndian>().unwrap();

        let border = Border::from_reader(&mut reader);

        Self {
            number_shape,
            user_char,
            prefix_char,
            suffix_char,
            start_number,
            divide_line_length,
            margin_top,
            margin_bottom,
            comment_margin,
            border,
        }
    }
}

/// 번호종류, hwpx 표준문서 참고
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, FromPrimitive)]
pub enum NumberShape {
    /// 1, 2, 3
    Digit,
    /// 동그라미 쳐진 1, 2, 3
    CircledDigit,
    /// I, II, III
    RomanCapital,
    /// i, ii, iii
    RomanSmall,
    /// A, B, C .. Z
    LatinCapital,
    /// a, b, c, ... z
    LatinSmall,
    /// 동그라미 쳐진 A, B, C
    CircledLatinCapital,
    /// 동그라미 쳐진 a, b, c
    CircledLatinSmall,
    /// 가, 나, 다
    HangulSyllable,
    /// 동그라미 쳐진 가,나,다
    CircledHangulSyllable,
    /// ᄀ, ᄂ, ᄃ
    HangulJamo,
    /// 동그라미 쳐진 ᄀ,ᄂ,ᄃ
    CircledHangulJamo,
    /// 일, 이, 삼
    HangulPhonetic,
    /// 一, 二, 三
    Ideograph,
    /// 동그라미 쳐진 一,二,三
    CircledIdeograph,
    /// 갑, 을, 병, 정, 무, 기, 경, 신, 임, 계
    DecagonCircle,
    /// 甲, 乙, 丙, 丁, 戊, 己, 庚, 辛, 壬, 癸
    DecagonCircleHanja,
    /// 4가지 문자가 차례로 반복
    Symbol = 0x80,
    /// 사용자 지정 문자 반복
    UserChar = 0x81,
}

const HANGUL_PHONETIC: [char; 10] = ['일', '이', '삼', '사', '오', '육', '칠', '팔', '구', '십'];
// NOTE: (@hahnlee) 초성기호로, 맥 입력기의 'ㄱ'과 코드가 다르다
const HANGUL_CHOSONG: [char; 14] = [
    'ᄀ', 'ᄂ', 'ᄃ', 'ᄅ', 'ᄆ', 'ᄇ', 'ᄉ', 'ᄋ', 'ᄌ', 'ᄎ', 'ᄏ', 'ᄐ', 'ᄑ', 'ᄒ',
];
// NOTE: (@hahnlee) 종성기호로, 맥 입력기의 'ㅏ'와 코드가 다르다
const HANGUL_JONGSONG: [char; 6] = ['ᅡ', 'ᅥ', 'ᅩ', 'ᅮ', 'ᅳ', 'ᅵ'];

const IDEOGRAPH: [char; 10] = ['一', '二', '三', '四', '五', '六', '七', '八', '九', '十'];
const DECAGON_CIRCLE: [char; 10] = ['갑', '을', '병', '정', '무', '기', '경', '신', '임', '계'];
const DECAGON_CIRCLE_CN: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];

pub fn format_number_shape(number_shape: &NumberShape, number: u16) -> String {
    match number_shape {
        NumberShape::Digit => format!("{}", number),
        NumberShape::CircledDigit => String::from_utf16(&[0x2460 + ((number - 1) % 20)]).unwrap(),
        NumberShape::LatinCapital => String::from_utf16(&[0x0041 + ((number - 1) % 26)]).unwrap(),
        NumberShape::LatinSmall => String::from_utf16(&[0x0061 + ((number - 1) % 26)]).unwrap(),
        NumberShape::CircledLatinCapital => {
            String::from_utf16(&[0x24B6 + ((number - 1) % 26)]).unwrap()
        }
        NumberShape::CircledLatinSmall => {
            String::from_utf16(&[0x24D0 + ((number - 1) % 26)]).unwrap()
        }
        NumberShape::HangulSyllable => {
            let choseong = HANGUL_CHOSONG[((number - 1) % 14) as usize];
            let jongseong = HANGUL_JONGSONG[(((number - 1) / 14) % 6) as usize];
            let choseong_index = (choseong as u16) - 0x1100;
            let jongseong_index = (jongseong as u16) - 0x1161;
            String::from_utf16(&[((choseong_index * 21) + jongseong_index) * 28 + 0xAC00]).unwrap()
        }
        NumberShape::CircledHangulSyllable => {
            String::from_utf16(&[0x326E + ((number - 1) % 14)]).unwrap()
        }
        NumberShape::HangulJamo => format!("{}", HANGUL_CHOSONG[((number - 1) % 14) as usize]),
        NumberShape::CircledHangulJamo => {
            String::from_utf16(&[0x3260 + ((number - 1) % 14)]).unwrap()
        }
        NumberShape::HangulPhonetic => format!("{}", HANGUL_PHONETIC[((number - 1) % 10) as usize]),
        NumberShape::Ideograph => format!("{}", IDEOGRAPH[((number - 1) % 10) as usize]),
        NumberShape::CircledIdeograph => {
            String::from_utf16(&[0x3280 + ((number - 1) % 10)]).unwrap()
        }
        NumberShape::DecagonCircle => {
            format!("{}", DECAGON_CIRCLE[((number - 1) % 10) as usize])
        }
        NumberShape::DecagonCircleHanja => {
            format!("{}", DECAGON_CIRCLE_CN[((number - 1) % 10) as usize])
        }
        _ => format!(""),
    }
}
