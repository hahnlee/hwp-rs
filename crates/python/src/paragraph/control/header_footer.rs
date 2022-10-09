use hwp::hwp::paragraph::control::header_footer::HeaderFooter;
use pyo3::prelude::*;

use crate::paragraph::{to_py_paragraphs, PyParagraph};

#[derive(Clone)]
#[pyclass(name = "HeaderFooter")]
pub struct PyHeaderFooter {
    /// 문단 리스트
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

impl PyHeaderFooter {
    pub fn from_rust(control: &HeaderFooter) -> Self {
        Self {
            paragraphs: to_py_paragraphs(&control.paragraph_list),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}
