use pyo3::prelude::*;
use pythonize::{Depythonizer, Pythonizer};
use serde_path_to_error::{Deserializer, Track};

use crate::serde::Parser;

/// Reads the object in `s` from its `CanonicalPath` form.
///
/// # Example
///
/// ```py
/// >>> import divatree
///
/// >>> file = open("./assets/pv_db.txt").read()
/// >>> pv_db = divatree.read(file)
///
/// >>> pv_db["pv_999"]["difficulty"]["normal"][0]
/// {'edition': 0,
/// 'level': 'PV_LV_01_0',
/// 'level_sort_index': 0,
/// 'script_file_name': 'rom/script/pv_999_normal.dsc',
/// 'script_format': '0x14012316',
/// 'version': 0}
/// ```
#[pyfunction]
fn read(py: Python, s: String) -> eyre::Result<PyObject> {
    let mut iter = s
        .lines()
        .filter(|x| !x.trim().is_empty())
        .filter(|x| !x.starts_with('#'));
    let mut lex = Parser::new(iter);
    let mut track = Track::new();
    let mut deser = Deserializer::new(&mut lex, &mut track);
    let topy = Pythonizer::new(py);
    let obj = serde_transcode::transcode(deser, topy);
    dbg!(track.path().to_string());
    Ok(obj?)
}

#[pyfunction]
fn write<'de>(obj: &'de PyAny) -> PyResult<String> {
    let mut frompy = Depythonizer::from_object(obj);
    todo!()
    // serde_transcode::transcode(&mut frompy, topy).map_err(Into::into)
}

/// Read and write files using SEGA's flavor of the `CanonicalProperties` format.
///
/// TODO: document this!
///
/// # Example
///
/// ```py
/// >>> import divatree
///
/// >>> file = open("./assets/pv_db.txt").read()
/// >>> pv_db = divatree.read(file)
///
/// >>> pv_db["pv_999"]["difficulty"]["normal"][0]
/// {'edition': 0,
/// 'level': 'PV_LV_01_0',
/// 'level_sort_index': 0,
/// 'script_file_name': 'rom/script/pv_999_normal.dsc',
/// 'script_format': '0x14012316',
/// 'version': 0}
/// ```
#[pymodule]
fn divatree(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(object_set))?;
    m.add_wrapped(wrap_pyfunction!(read))?;
    m.add_wrapped(wrap_pyfunction!(write))?;

    Ok(())
}
