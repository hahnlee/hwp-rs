pub mod char;

use hwp::hwp::paragraph::Paragraph;
use pyo3::prelude::*;

use self::char::PyChar;

#[derive(Clone)]
#[pyclass(name = "Paragraph")]
pub struct PyParagraph {
    paragraph: Paragraph,
}

#[pymethods]
impl PyParagraph {
    #[getter]
    fn chars(&self) -> Vec<Py<PyAny>> {
        let mut chars = Vec::new();
        for char in &self.paragraph.char_list.chars {
            chars.push(Python::with_gil(|py| {
                Py::new(py, PyChar::from_char(char)).unwrap().into_py(py)
            }));
        }
        chars
    }

    fn __str__(&self) -> String {
        self.paragraph.to_string()
    }

    pub fn find_all(&self, _tag: &str) -> Vec<Py<PyAny>> {
        // TODO: (@hahnlee) 구현필요
        vec![]
    }
}

impl PyParagraph {
    pub fn from_paragraph(paragraph: &Paragraph) -> Self {
        Self {
            paragraph: paragraph.clone(),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| {
            self.clone().into_py(py)
        })
    }
}
