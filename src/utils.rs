use pyo3::prelude::*;
use pyo3::types::any::PyAnyMethods;
use pyo3::types::PyList;

pub fn visit_list<'py>(py: Python<'py>, list: &PyList, visit_fn: &PyObject) -> PyResult<PyObject> {
    let mut new_list = Vec::new();
    let bound_list: Bound<'py, PyList> = list.as_borrowed(py).to_owned();
    for item in bound_list.iter()? {
        let result = visit_fn.call1(py, (item?,))?;
        new_list.push(result);
    }
    Ok(PyList::new_bound(py, new_list).into())
}
