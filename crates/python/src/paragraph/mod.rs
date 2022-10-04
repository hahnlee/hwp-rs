pub mod char;
pub mod control;
pub mod header;

use hwp::hwp::paragraph::{char_list::CharList, Paragraph};
use pyo3::prelude::*;

use self::{
    char::to_py_char,
    header::{to_py_paragraph_header, PyParagraphHeader},
};

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

    fn chars(&self) -> Vec<Py<PyAny>> {
        let mut chars = Vec::new();
        for char in &self.char_list.chars {
            chars.push(Python::with_gil(|py| {
                Py::new(py, to_py_char(char)).unwrap().into_py(py)
            }));
        }
        chars
    }
}

pub fn to_py_paragraph(paragraph: &Paragraph) -> PyParagraph {
    PyParagraph {
        header: to_py_paragraph_header(&paragraph.header),
        char_list: paragraph.char_list.clone(),
    }
}

#[derive(Clone)]
#[pyclass(name = "Test")]
pub struct PyTest {}
