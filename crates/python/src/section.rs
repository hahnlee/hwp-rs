use hwp::hwp::section::Section;
use pyo3::prelude::*;

use crate::paragraph::PyParagraph;

#[derive(Clone)]
#[pyclass(name = "Section")]
pub struct PySection {
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

#[pymethods]
impl PySection {
    pub fn find_all(&self) -> Vec<Py<PyAny>> {
        // TODO: (@hahnlee) find_all('paragraph') 같은건 따로 처리해야함
        (&self.paragraphs)
            .into_iter()
            .map(|p| p.find_all())
            .flatten()
            .collect()
    }
}

impl PySection {
    pub fn from_section(section: &Section) -> Self {
        Self {
            paragraphs: (&section.paragraphs)
                .into_iter()
                .map(|p| PyParagraph::from_paragraph(p))
                .collect::<Vec<PyParagraph>>(),
        }
    }
}
