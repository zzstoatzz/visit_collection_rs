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
    // At this point, visit_fn has already been called on the current list by do_visit_collection
    // and max_depth/seen checks have been performed.
    // Our job is to visit each child with a fresh visit_collection call

    let mut modified = false;
    let mut new_items = Vec::with_capacity(list.len());

    for item in list.iter() {
        // Each child gets a fresh visit_collection call with max_depth-1 and copied context
        let visited_item = super::do_visit_collection(
            py,
            item,
            visit_fn,
            return_data,
            max_depth - 1,
            super::copy_context(py, context)?,
            remove_annotations,
            seen,
        )?;

        if return_data && !visited_item.as_ref(py).is(item) {
            modified = true;
        }

        new_items.push(visited_item);
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
