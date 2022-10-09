use hwp::hwp::section::Section;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::paragraph::PyParagraph;

#[derive(Clone)]
#[pyclass(name = "Section")]
pub struct PySection {
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

#[pymethods]
impl PySection {
    #[args(kwargs = "**")]
    pub fn find_all(&self, tag: &str, kwargs: Option<&PyDict>) -> Vec<Py<PyAny>> {
        let recursive = if kwargs.is_some() {
            let option = kwargs.unwrap().get_item("recursive");
            if option.is_some() {
                option.unwrap().is_true().unwrap_or_else(|_| true)
            } else {
                true
            }
        } else {
            true
        };

        (&self.paragraphs)
            .into_iter()
            .map(|p| {
                // NOTE: (@hahnlee) find_all('paragraph')은 따로 처리해야함
                if tag == "paragraph" {
                    let mut result = vec![];
                    result.push(p.to_py_any());
                    if recursive {
                        result = [result,  p.find_all(tag, kwargs)].concat();
                    }

                    result
                } else {
                    p.find_all(tag, kwargs)
                }
            })
            .flatten()
            .collect()
    }
}

impl PySection {
    pub fn from_section(section: &Section) -> Self {
        Self {
            paragraphs: (&section.paragraphs)
                .into_iter()
                .map(|p| PyParagraph::from_rust(p))
                .collect::<Vec<PyParagraph>>(),
        }
    }
}
