use kitn;
use pyo3::prelude::*;

#[pymodule]
fn kitn_python(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
