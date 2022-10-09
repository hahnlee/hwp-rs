use hwp::hwp::bin_data::File;
use pyo3::{prelude::*, types::PyBytes};

#[derive(Clone)]
#[pyclass(name = "File")]
pub struct PyFile {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub data: PyObject,
}

impl PyFile {
    pub fn from_rust(file: &File) -> Self {
        let data: PyObject = Python::with_gil(|py| PyBytes::new(py, &file.data).into());

        Self {
            name: file.name.clone(),
            data,
        }
    }
}
