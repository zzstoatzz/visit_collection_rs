use pyo3::prelude::*;
use pyo3::types::{PyDict, PySet};

#[allow(clippy::too_many_arguments)]
pub fn visit_dict(
    py: Python<'_>,
    dict: &PyDict,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&PyDict>,
    remove_annotations: bool,
    seen: Option<&PySet>,
) -> PyResult<PyObject> {
    // At this point, visit_fn has already been called on the current dict by do_visit_collection
    // and max_depth/seen checks have been performed.
    // Our job is to visit each child with a fresh visit_collection call

    let mut modified = false;
    let new_dict = PyDict::new(py);

    for (key, value) in dict.iter() {
        // Each child gets a fresh visit_collection call with max_depth-1 and copied context
        let visited_key = super::do_visit_collection(
            py,
            key,
            visit_fn,
            return_data,
            max_depth - 1,
            super::copy_context(py, context)?,
            remove_annotations,
            seen,
        )?;

        let visited_value = super::do_visit_collection(
            py,
            value,
            visit_fn,
            return_data,
            max_depth - 1,
            super::copy_context(py, context)?,
            remove_annotations,
            seen,
        )?;

        if return_data && (!visited_key.as_ref(py).is(key) || !visited_value.as_ref(py).is(value)) {
            modified = true;
        }

        new_dict.set_item(visited_key, visited_value)?;
    }

    if return_data {
        if modified {
            Ok(new_dict.to_object(py))
        } else {
            Ok(dict.to_object(py))
        }
    } else {
        Ok(py.None())
    }
}
