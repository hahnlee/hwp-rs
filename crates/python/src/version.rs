use hwp::hwp::version::Version;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Version")]
pub struct PyVersion(pub Version);

#[pymethods]
impl PyVersion {
    #[getter]
    fn major(&self) -> PyResult<u8> {
        Ok(self.0.major)
    }

    #[getter]
    fn minor(&self) -> PyResult<u8> {
        Ok(self.0.minor)
    }

    #[getter]
    fn micro(&self) -> PyResult<u8> {
        Ok(self.0.micro)
    }

    #[getter]
    fn build_number(&self) -> PyResult<u8> {
        Ok(self.0.build_number)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}
