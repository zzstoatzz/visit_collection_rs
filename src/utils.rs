use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub fn visit_list<'py>(
    py: Python<'py>,
    list: &Bound<'py, PyList>,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
) -> PyResult<PyObject> {
    let mut new_list = Vec::new();

    for item in list.iter() {
        let result = if max_depth != 0 && item.is_instance_of::<PyList>() && context.is_none() {
            let nested_list = item.downcast::<PyList>()?;
            visit_list(
                py,
                nested_list,
                visit_fn,
                return_data,
                max_depth - 1,
                context,
            )?
        } else {
            visit_fn.call1(py, (item,))?
        };

        if return_data {
            new_list.push(result);
        }
    }

    if return_data {
        Ok(PyList::new_bound(py, new_list).into())
    } else {
        Ok(py.None().into())
    }
}
