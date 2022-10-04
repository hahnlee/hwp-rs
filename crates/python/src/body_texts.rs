use hwp::hwp::body::Body;
use pyo3::prelude::*;

use crate::section::{to_py_section, PySection};

#[derive(Clone)]
#[pyclass(name = "HWP")]
pub struct PyBody {
    #[pyo3(get)]
    pub sections: Vec<PySection>,
}

#[pymethods]
impl PyBody {}

pub fn to_py_body(body: &Body) -> PyBody {
    let mut sections = Vec::new();

    for section in &body.sections {
        let py_section = to_py_section(section);
        sections.push(py_section);
    }

    PyBody { sections }
}
