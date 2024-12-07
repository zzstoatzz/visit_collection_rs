use crate::{do_visit_collection, maybe_copy_context};
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
    // Already visited the current dict and checked max_depth/seen in do_visit_collection

    // If no return_data, we never need to allocate new data unless children are changed
    // We'll track modification
    let mut modified = false;
    let new_dict = if return_data {
        Some(PyDict::new(py))
    } else {
        None
    };

    // Copy context once per dict, not for every item
    let sub_context = maybe_copy_context(py, context)?;

    for (key, value) in dict.iter() {
        let visited_key = do_visit_collection(
            py,
            key,
            visit_fn,
            return_data,
            max_depth - 1,
            sub_context,
            remove_annotations,
            seen,
        )?;

        let visited_value = do_visit_collection(
            py,
            value,
            visit_fn,
            return_data,
            max_depth - 1,
            sub_context,
            remove_annotations,
            seen,
        )?;

        if return_data {
            if !visited_key.as_ref(py).is(key) || !visited_value.as_ref(py).is(value) {
                modified = true;
            }

            // unwrap is safe because we only use new_dict if return_data = true
            new_dict
                .as_ref()
                .unwrap()
                .set_item(visited_key, visited_value)?;
        } else {
            // return_data = false, we do not store results
        }
    }

    if return_data {
        if modified {
            Ok(new_dict.unwrap().to_object(py))
        } else {
            Ok(dict.to_object(py))
        }
    } else {
        Ok(py.None())
    }
}
