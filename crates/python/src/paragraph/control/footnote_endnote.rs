use hwp::hwp::paragraph::control::footnote_endnote::FootnoteEndnote;
use pyo3::prelude::*;

use crate::paragraph::{PyParagraph, to_py_paragraphs};

#[derive(Clone)]
#[pyclass(name = "FootnoteEndnote")]
pub struct PyFootnoteEndnote {
    /// 문단 리스트
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

impl PyFootnoteEndnote {
    pub fn from_rust(control: &FootnoteEndnote) -> Self {
        Self {
            paragraphs: to_py_paragraphs(&control.paragraph_list),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}
