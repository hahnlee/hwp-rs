const HWPTAG_BEGIN: u32 = 0x10;

#[repr(u32)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub enum DocInfoRecord {
    /// 문서 속성
    HWPTAG_DOCUMENT_PROPERTIES = HWPTAG_BEGIN,
    /// 아이디 매핑 헤더
    HWPTAG_ID_MAPPINGS,
    /// BinData
    HWPTAG_BIN_DATA,
    /// Typeface Name
    HWPTAG_FACE_NAME,
    /// 테두리/배경
    HWPTAG_BORDER_FILL,
    /// 글자모양
    HWPTAG_CHAR_SHAPE,
    /// 탭 정의
    HWPTAG_TAB_DEF,
    /// 번호 정의
    HWPTAG_NUMBERING,
    /// 불릿 정의
    HWPTAG_BULLET,
    /// 문단 모양
    HWPTAG_PARA_SHAPE,
    /// 스타일 (문단 스타일)
    HWPTAG_STYLE,
    /// 문서의 임의의 데이터
    HWPTAG_DOC_DATA,
    /// 배포용 문서 데이터
    HWPTAG_DISTRIBUTE_DOC_DATA,
    /// 예약
    RESERVED,
    /// 호환 문서
    HWPTAG_COMPATIBLE_DOCUMENT,
    /// 레이아웃 호환성
    HWPTAG_LAYOUT_COMPATIBILITY,
    /// 변경 추적 정보
    HWPTAG_TRACKCHANGE,
    /// 메모 모양
    HWPTAG_MEMO_SHAPE = HWPTAG_BEGIN + 76,
    /// 금칙처리 문자
    HWPTAG_FORBIDDEN_CHAR = HWPTAG_BEGIN + 78,
    /// 변경 추적 내용 및 모양
    HWPTAG_TRACK_CHANGE = HWPTAG_BEGIN + 80,
    /// 변경 추적 작성자
    HWPTAG_TRACK_CHANGE_AUTHOR = HWPTAG_BEGIN + 81,
}

#[repr(u32)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub enum BodyTextRecord {
    /// 문단 헤더
    HWPTAG_PARA_HEADER = HWPTAG_BEGIN + 50,
    HWPTAG_PARA_TEXT,
    HWPTAG_PARA_CHAR_SHAPE,
    HWPTAG_PARA_LINE_SEG,
    HWPTAG_PARA_RANGE_TAG,
    HWPTAG_CTRL_HEADER,
    HWPTAG_LIST_HEADER,
    HWPTAG_PAGE_DEF,
    HWPTAG_FOOTNOTE_SHAPE,
    HWPTAG_PAGE_BORDER_FILL,
    HWPTAG_SHAPE_COMPONENT,
    HWPTAG_TABLE,
    /// 수식
    HWPTAG_EQEDIT = HWPTAG_BEGIN + 72,
}
