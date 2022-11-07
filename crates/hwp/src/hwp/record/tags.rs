const HWPTAG_BEGIN: u32 = 0x10;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
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
    #[allow(dead_code)]
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
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub enum BodyTextRecord {
    /// 문단 헤더
    HWPTAG_PARA_HEADER = HWPTAG_BEGIN + 50,
    /// 문단의 텍스트
    HWPTAG_PARA_TEXT,
    /// 문단의 글자 모양
    HWPTAG_PARA_CHAR_SHAPE,
    /// 문단의 레이아웃
    HWPTAG_PARA_LINE_SEG,
    /// 문단의 영역 태그
    HWPTAG_PARA_RANGE_TAG,
    /// 컨트롤 헤더
    HWPTAG_CTRL_HEADER,
    /// 문단 리스트 헤더
    HWPTAG_LIST_HEADER,
    /// 용지 설정
    HWPTAG_PAGE_DEF,
    /// 각주/미주 모양
    HWPTAG_FOOTNOTE_SHAPE,
    /// 쪽 테두리/배경
    HWPTAG_PAGE_BORDER_FILL,
    /// 개체
    HWPTAG_SHAPE_COMPONENT,
    /// 표 개체
    HWPTAG_TABLE,
    /// 직선 개체
    HWPTAG_SHAPE_COMPONENT_LINE,
    /// 사각형 개체
    HWPTAG_SHAPE_COMPONENT_RECTANGLE,
    /// 타원 개체
    HWPTAG_SHAPE_COMPONENT_ELLIPSE,
    /// 호 개체
    HWPTAG_SHAPE_COMPONENT_ARC,
    /// 다각형 개체
    HWPTAG_SHAPE_COMPONENT_POLYGON,
    /// 곡선 개체
    HWPTAG_SHAPE_COMPONENT_CURVE,
    /// OLE 개체
    HWPTAG_SHAPE_COMPONENT_OLE,
    /// 그림 개체
    HWPTAG_SHAPE_COMPONENT_PICTURE,
    /// 컨테이너 개체
    HWPTAG_SHAPE_COMPONENT_CONTAINER,
    /// 컨트롤 임의의 데이터
    HWPTAG_CTRL_DATA,
    /// 수식 개체
    HWPTAG_EQEDIT,
    /// 예약
    #[allow(dead_code)]
    RESERVED,
    /// 글맵시
    HWPTAG_SHAPE_COMPONENT_TEXTART,
    /// 양식 개체
    HWPTAG_FORM_OBJECT,
    /// 메모 모양
    HWPTAG_MEMO_SHAPE,
    /// 메모 리스트 헤더
    HWPTAG_MEMO_LIST,
    /// 차트 데이터
    HWPTAG_CHART_DATA,
    /// 비디오 데이터
    HWPTAG_VIDEO_DATA = HWPTAG_BEGIN + 82,
    /// Unknown
    HWPTAG_SHAPE_COMPONENT_UNKNOWN = HWPTAG_BEGIN + 99,
}
