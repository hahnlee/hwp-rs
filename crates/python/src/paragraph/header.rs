use hwp::hwp::paragraph::header::ParagraphHeader;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "ParagraphHeader")]
pub struct PyParagraphHeader {
    /// 글자수
    #[pyo3(get)]
    pub chars: u32,
    /// 글자 모양 정보 수
    #[pyo3(get)]
    pub char_shapes: u16,
    /// 각 줄에 대한 align에 대한 정보 수
    #[pyo3(get)]
    pub aligns: u16,
    // range tag 정보 수
    #[pyo3(get)]
    pub ranges: u16,
}

pub fn to_py_paragraph_header(paragraph: &ParagraphHeader) -> PyParagraphHeader {
    PyParagraphHeader {
        chars: paragraph.chars,
        char_shapes: paragraph.char_shapes,
        aligns: paragraph.aligns,
        ranges: paragraph.ranges,
    }
}
