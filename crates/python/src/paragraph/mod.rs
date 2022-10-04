pub mod header;

use hwp::hwp::paragraph::{char_list::CharList, Paragraph};
use pyo3::prelude::*;

use self::header::{to_py_paragraph_header, PyParagraphHeader};

#[derive(Clone)]
#[pyclass(name = "Paragraph")]
pub struct PyParagraph {
    #[pyo3(get)]
    pub header: PyParagraphHeader,
    char_list: CharList,
}

#[pymethods]
impl PyParagraph {
    fn __str__(&self) -> String {
        self.char_list.to_string()
    }
}

pub fn to_py_paragraph(paragraph: &Paragraph) -> PyParagraph {
    PyParagraph {
        header: to_py_paragraph_header(&paragraph.header),
        char_list: paragraph.char_list.clone(),
    }
}
