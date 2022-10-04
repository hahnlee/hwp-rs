use hwp::hwp::header::{Flags, Header};
use pyo3::prelude::*;

use crate::version::PyVersion;

#[derive(Clone)]
#[pyclass(name = "Header")]
pub struct PyHeader {
    #[pyo3(get)]
    version: PyVersion,

    #[pyo3(get)]
    flags: PyFlags,
}

#[pymethods]
impl PyHeader {}

pub fn to_py_header(header: &Header) -> PyHeader {
    PyHeader {
        version: PyVersion(header.version.clone()),
        flags: to_py_flag(&header.flags),
    }
}

#[derive(Clone)]
#[pyclass(name = "Flags")]
pub struct PyFlags {
    #[pyo3(get)]
    pub distributed: bool,
}

pub fn to_py_flag(flags: &Flags) -> PyFlags {
    PyFlags {
        distributed: flags.distributed,
    }
}
