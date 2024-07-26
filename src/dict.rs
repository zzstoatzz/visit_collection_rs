use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

pub fn visit_dict<'py>(
    py: Python<'py>,
    dict: &Bound<'py, PyDict>,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
) -> PyResult<PyObject> {
    if max_depth == 0 || !return_data {
        return Ok(if return_data {
            dict.to_object(py)
        } else {
            py.None()
        });
    }

    let new_dict = PyDict::new_bound(py);

    for (key, value) in dict.iter() {
        let transformed_key = visit_item(py, &key, visit_fn, max_depth - 1, context)?;
        let transformed_value = visit_item(py, &value, visit_fn, max_depth - 1, context)?;
        new_dict.set_item(transformed_key, transformed_value)?;
    }

    if let Some(ctx) = context {
        visit_fn.call1(py, (new_dict.as_ref(), ctx))
    } else {
        visit_fn.call1(py, (new_dict.as_ref(),))
    }
}

fn visit_item<'py>(
    py: Python<'py>,
    item: &Bound<'py, PyAny>,
    visit_fn: &PyObject,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, PyAny>> {
    // First, apply the visit function
    let transformed: Bound<'py, PyAny> = if let Some(ctx) = context {
        visit_fn.call1(py, (item, ctx))?.into_bound(py)
    } else {
        visit_fn.call1(py, (item,))?.into_bound(py)
    };

    // If we've reached max depth (and it's not negative), return the transformed item
    if max_depth == 0 {
        return Ok(transformed);
    }

    // Calculate the new max_depth for recursive calls
    let new_max_depth = if max_depth < 0 {
        max_depth
    } else {
        max_depth - 1
    };

    // Now, recursively visit the children of the transformed item
    if let Ok(dict) = transformed.downcast::<PyDict>() {
        let new_dict = PyDict::new_bound(py);
        for (key, value) in dict.iter() {
            let new_key = visit_item(py, &key, visit_fn, new_max_depth, context)?;
            let new_value = visit_item(py, &value, visit_fn, new_max_depth, context)?;
            new_dict.set_item(new_key, new_value)?;
        }
        Ok(new_dict.into_any())
    } else if let Ok(list) = transformed.downcast::<PyList>() {
        let new_list = PyList::empty_bound(py);
        for item in list.iter() {
            let new_item = visit_item(py, &item, visit_fn, new_max_depth, context)?;
            new_list.append(new_item)?;
        }
        Ok(new_list.into_any())
    } else {
        // If it's not a dict or list, return the transformed item
        Ok(transformed)
    }
}
