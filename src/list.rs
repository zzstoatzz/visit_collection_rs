use crate::{do_visit_collection, maybe_copy_context};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet};

#[allow(clippy::too_many_arguments)]
pub fn visit_list(
    py: Python<'_>,
    list: &PyList,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&PyDict>,
    remove_annotations: bool,
    seen: Option<&PySet>,
) -> PyResult<PyObject> {
    // Already visited current list and checked max_depth/seen in do_visit_collection
    let mut modified = false;

    let sub_context = maybe_copy_context(py, context)?;

    let mut new_items = if return_data {
        Vec::with_capacity(list.len())
    } else {
        Vec::new() // might not use if no return_data
    };

    for item in list.iter() {
        let visited_item = do_visit_collection(
            py,
            item,
            visit_fn,
            return_data,
            max_depth - 1,
            sub_context,
            remove_annotations,
            seen,
        )?;

        if return_data {
            if !visited_item.as_ref(py).is(item) {
                modified = true;
            }
            new_items.push(visited_item);
        } else {
            // no return_data, just call visit_fn for side effects
        }
    }

    if return_data {
        if modified {
            Ok(PyList::new(py, new_items).to_object(py))
        } else {
            Ok(list.to_object(py))
        }
    } else {
        Ok(py.None())
    }
}
