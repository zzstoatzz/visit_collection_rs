use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyTuple};
use std::collections::HashSet;

fn visit_list<'py>(
    py: Python<'py>,
    list: &Bound<'py, PyList>,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
    seen: &mut HashSet<isize>,
) -> PyResult<PyObject> {
    if !return_data {
        for item in list.iter() {
            visit_collection(
                py,
                &item,
                visit_fn.clone_ref(py),
                false,
                max_depth - 1,
                context,
                false,
                Some(seen.clone()),
            )?;
        }
        return Ok(py.None());
    }

    let new_list = PyList::empty_bound(py);
    for item in list.iter() {
        let new_item = visit_collection(
            py,
            &item,
            visit_fn.clone_ref(py),
            true,
            max_depth - 1,
            context,
            false,
            Some(seen.clone()),
        )?;
        new_list.append(new_item.into_bound(py))?;
    }
    Ok(new_list.into())
}

fn visit_dict<'py>(
    py: Python<'py>,
    dict: &Bound<'py, PyDict>,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
    seen: &mut HashSet<isize>,
) -> PyResult<PyObject> {
    if !return_data {
        for (key, value) in dict.iter() {
            visit_collection(
                py,
                &key,
                visit_fn.clone_ref(py),
                false,
                max_depth - 1,
                context,
                false,
                Some(seen.clone()),
            )?;
            visit_collection(
                py,
                &value,
                visit_fn.clone_ref(py),
                false,
                max_depth - 1,
                context,
                false,
                Some(seen.clone()),
            )?;
        }
        return Ok(py.None());
    }

    let new_dict = PyDict::new_bound(py);
    for (key, value) in dict.iter() {
        let new_key = visit_collection(
            py,
            &key,
            visit_fn.clone_ref(py),
            true,
            max_depth - 1,
            context,
            false,
            Some(seen.clone()),
        )?;
        let new_value = visit_collection(
            py,
            &value,
            visit_fn.clone_ref(py),
            true,
            max_depth - 1,
            context,
            false,
            Some(seen.clone()),
        )?;
        new_dict.set_item(new_key, new_value)?;
    }
    Ok(new_dict.into())
}

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
#[allow(clippy::too_many_arguments)]
fn visit_collection<'py>(
    py: Python<'py>,
    expr: &Bound<'py, PyAny>,
    visit_fn: PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
    remove_annotations: bool,
    _seen: Option<HashSet<isize>>,
) -> PyResult<PyObject> {
    let mut seen = _seen.unwrap_or_default();
    let expr_id = expr.as_ptr() as isize;

    if max_depth == 0 || seen.contains(&expr_id) {
        return if return_data {
            Ok(expr.to_object(py))
        } else {
            Ok(py.None())
        };
    }
    seen.insert(expr_id);

    let result = if let Some(ctx) = context {
        visit_fn.call1(py, (expr, ctx))?
    } else {
        visit_fn.call1(py, (expr,))?
    };

    if !return_data {
        return Ok(py.None());
    }

    let result = result.into_bound(py);

    if let Ok(list) = result.downcast::<PyList>() {
        visit_list(
            py,
            list,
            &visit_fn,
            return_data,
            max_depth,
            context,
            &mut seen,
        )
    } else if let Ok(dict) = result.downcast::<PyDict>() {
        visit_dict(
            py,
            dict,
            &visit_fn,
            return_data,
            max_depth,
            context,
            &mut seen,
        )
    } else if let Ok(tuple) = result.downcast::<PyTuple>() {
        let new_tuple = PyTuple::new_bound(
            py,
            tuple
                .iter()
                .map(|item| {
                    visit_collection(
                        py,
                        &item,
                        visit_fn.clone_ref(py),
                        true,
                        max_depth - 1,
                        context,
                        remove_annotations,
                        Some(seen.clone()),
                    )
                })
                .collect::<PyResult<Vec<_>>>()?,
        );
        Ok(new_tuple.into())
    } else {
        Ok(result.into())
    }
}

#[pymodule]
fn visit_collection_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visit_collection, m)?)?;
    Ok(())
}
