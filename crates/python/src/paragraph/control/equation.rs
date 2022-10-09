use hwp::hwp::paragraph::control::equation::Equation;
use pyo3::prelude::*;

use super::common_properties::PyCommonProperties;

#[derive(Clone)]
#[pyclass(name = "Equation")]
pub struct PyEquation {
    /// 개체 공통 속성
    #[pyo3(get)]
    pub common_properties: PyCommonProperties,
    /// 수식 내용
    #[pyo3(get)]
    pub script: String,
}

impl PyEquation {
    pub fn from_rust(equation: &Equation) -> Self {
        Self {
            common_properties: PyCommonProperties::from_rust(&equation.common_properties),
            script: equation.record.script.clone(),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}
