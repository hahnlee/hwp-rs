use hwp::hwp::section::Section;
use pyo3::prelude::*;

use crate::paragraph::{to_py_paragraph, PyParagraph};

#[derive(Clone)]
#[pyclass(name = "Section")]
pub struct PySection {
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

pub fn to_py_section(section: &Section) -> PySection {
    PySection {
        paragraphs: (&section.paragraphs)
            .into_iter()
            .map(|p| to_py_paragraph(p))
            .collect::<Vec<PyParagraph>>(),
    }
}
