use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet};

mod utils;

#[pyfunction]
#[pyo3(signature = (
    expr,
    visit_fn,
    return_data = false,
    max_depth = -1,
    context = None,
    remove_annotations = false,
    _seen = None
))]
fn visit_collection(
    py: Python,
    expr: PyObject,
    visit_fn: PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<Py<PyDict>>,
    remove_annotations: bool,
    _seen: Option<Py<PySet>>,
) -> PyResult<PyObject> {
    // Use all parameters to avoid compiler warnings
    let _ = (return_data, max_depth, context, remove_annotations, _seen);

    if let Ok(list) = expr.downcast_bound::<PyList>(py) {
        utils::visit_list(py, list, &visit_fn)
    // } else if let Ok(tuple) = expr.downcast_bound::<PyTuple>(py) {
    //     utils::visit_tuple(py, tuple, &visit_fn)
    // } else if let Ok(dict) = expr.downcast_bound::<PyDict>(py) {
    //     utils::visit_dict(py, dict, &visit_fn)
    // } else if let Ok(set) = expr.downcast_bound::<PySet>(py) {
    //     utils::visit_set(py, set, &visit_fn)
    } else {
        // If it's not a collection type we recognize, just apply visit_fn directly
        visit_fn.call1(py, (expr,))
    }
}

#[pymodule]
fn visit_collection_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visit_collection, m)?)?;
    Ok(())
}
