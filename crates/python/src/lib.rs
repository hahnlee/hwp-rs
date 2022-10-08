mod paragraph;
mod section;
mod version;

use std::fs;

use hwp::HWP;
use pyo3::prelude::*;
use section::PySection;
use version::PyVersion;

#[pyclass]
struct HWPReader {
    #[pyo3(get)]
    pub version: PyVersion,
    #[pyo3(get)]
    pub sections: Vec<PySection>,
}

#[pymethods]
impl HWPReader {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        // TODO: (@hahnlee) 메모리에 있는 파일 읽기 등 더 좋은 방법 필요
        let file = fs::read(path)?;
        let hwp = HWP::from_bytes(&file);

        let body = if hwp.header.flags.distributed {
            hwp.view_texts.as_ref().unwrap()
        } else {
            &hwp.body_texts
        };

        let sections = (&body.sections)
            .into_iter()
            .map(|s| PySection::from_section(s))
            .collect::<Vec<PySection>>();

        Ok(Self {
            version: PyVersion(hwp.header.version.clone()),
            sections,
        })
    }

    pub fn find_all(&self, tag: &str) -> Vec<Py<PyAny>> {
        (&self.sections)
            .into_iter()
            .map(|s| s.find_all(tag))
            .flatten()
            .collect()
    }
}

#[pymodule]
fn hwppy(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<HWPReader>()?;
    Ok(())
}
