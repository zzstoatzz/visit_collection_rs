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
        let transformed_item: Bound<'py, PyAny> = if max_depth != 0 {
            if let Some(ctx) = context {
                visit_fn.call1(py, (item, ctx))?.extract(py)?
            } else {
                visit_fn.call1(py, (item,))?.extract(py)?
            }
        } else {
            item.clone()
        };

        let result = if !(0..=1).contains(&max_depth) && transformed_item.is_instance_of::<PyList>()
        {
            let nested_list = transformed_item.downcast::<PyList>()?;
            visit_list(
                py,
                nested_list,
                visit_fn,
                return_data,
                if max_depth > 0 {
                    max_depth - 1
                } else {
                    max_depth
                },
                context,
            )?
        } else {
            transformed_item.into()
        };

        if return_data {
            new_list.push(result);
        }
    }

    if return_data {
        let result_list = PyList::new_bound(py, new_list);
        if max_depth != 0 {
            if let Some(ctx) = context {
                visit_fn.call1(py, (result_list.as_ref(), ctx))
            } else {
                visit_fn.call1(py, (result_list.as_ref(),))
            }
        } else {
            Ok(result_list.into())
        }
    } else {
        Ok(py.None())
    }
}
