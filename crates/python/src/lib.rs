mod body_texts;
mod header;
mod paragraph;
mod section;
mod version;

use std::fs;

use body_texts::{to_py_body, PyBody};
use header::{to_py_header, PyHeader};
use hwp::HWP;
use pyo3::prelude::*;

#[pyclass(name = "HWP")]
struct PyHWP {
    #[pyo3(get)]
    pub header: PyHeader,
    #[pyo3(get)]
    pub body_texts: PyBody,
    #[pyo3(get)]
    pub view_texts: Option<PyBody>,
}

#[pymethods]
impl PyHWP {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        // TODO: (@hahnlee) 메모리에 있는 파일 읽기 등 더 좋은 방법 필요
        let file = fs::read(path)?;
        let hwp = HWP::from_bytes(file);

        let view_texts = if let Some(view_texts) = hwp.view_texts {
            Some(to_py_body(&view_texts))
        } else {
            None
        };

        Ok(Self {
            header: to_py_header(&hwp.header),
            body_texts: to_py_body(&hwp.body_texts),
            view_texts,
        })
    }
}

#[pymodule]
fn hwppy(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyHWP>()?;
    Ok(())
}
