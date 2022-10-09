mod bin_data;
mod paragraph;
mod section;
mod version;

use std::fs;

use bin_data::PyFile;
use hwp::HWP;
use paragraph::control::common_properties::PyCommonProperties;
use paragraph::control::equation::PyEquation;
use paragraph::control::footnote_endnote::PyFootnoteEndnote;
use paragraph::control::header_footer::PyHeaderFooter;
use paragraph::control::table::PyTable;
use paragraph::PyParagraph;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use section::PySection;
use version::PyVersion;

#[pyclass]
struct HWPReader {
    #[pyo3(get)]
    pub version: PyVersion,
    #[pyo3(get)]
    pub sections: Vec<PySection>,
    #[pyo3(get)]
    pub bin_data: Vec<PyFile>,
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
            bin_data: hwp
                .bin_data
                .into_iter()
                .map(|b| PyFile::from_rust(&b))
                .collect(),
        })
    }

    #[args(kwargs = "**")]
    pub fn find_all(&self, tag: &str, kwargs: Option<&PyDict>) -> Vec<Py<PyAny>> {
        (&self.sections)
            .into_iter()
            .map(|s| s.find_all(tag, kwargs))
            .flatten()
            .collect()
    }
}

#[pymodule]
fn libhwp(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<HWPReader>()?;
    module.add_class::<PyParagraph>()?;
    module.add_class::<PyFile>()?;
    module.add_class::<PyTable>()?;
    module.add_class::<PyEquation>()?;
    module.add_class::<PyCommonProperties>()?;
    module.add_class::<PyHeaderFooter>()?;
    module.add_class::<PyFootnoteEndnote>()?;
    Ok(())
}
