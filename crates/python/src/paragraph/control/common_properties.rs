use hwp::hwp::paragraph::control::common_properties::{Caption, CommonProperties};
use pyo3::prelude::*;

use crate::paragraph::{to_py_paragraphs, PyParagraph};

#[derive(Clone)]
#[pyclass(name = "CommonProperties")]
pub struct PyCommonProperties {
    /// 컨트롤 ID
    #[pyo3(get)]
    pub ctrl_id: u32,
    /// width 오브젝트의 폭
    #[pyo3(get)]
    pub width: u32,
    /// height 오브젝트의 높이
    #[pyo3(get)]
    pub height: u32,
    /// 문서 내 각 개체에 대한 고유 아이디
    #[pyo3(get)]
    pub instance_id: u32,
    /// 개체 설명문
    #[pyo3(get)]
    pub description: String,
    /// 캡션
    #[pyo3(get)]
    pub caption: Option<PyCaption>,
}

impl PyCommonProperties {
    pub fn from_rust(properties: &CommonProperties) -> Self {
        Self {
            ctrl_id: properties.ctrl_id,
            width: properties.width,
            height: properties.height,
            instance_id: properties.instance_id,
            description: properties.description.clone(),
            caption: if properties.caption.is_some() {
                Some(PyCaption::from_rust(properties.caption.as_ref().unwrap()))
            } else {
                None
            },
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "Caption")]
pub struct PyCaption {
    /// 문단 리스트
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

impl PyCaption {
    pub fn from_rust(caption: &Caption) -> Self {
        Self {
            paragraphs: to_py_paragraphs(&caption.paragraph_list),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}
