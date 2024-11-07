use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use vsag_sys as sys;

#[pyclass]
struct Index {
    core: sys::VsagIndex,
}

#[pymethods]
impl Index {
    #[new]
    fn try_new(index_type: &str, params: &str) -> PyResult<Self> {
        sys::VsagIndex::new(index_type, params)
            .map_err(|e| PyValueError::new_err(e.message))
            .map(|core| Self { core })
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn vsag(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Index>()?;
    Ok(())
}
