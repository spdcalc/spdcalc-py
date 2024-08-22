use pyo3::{
  exceptions::{PyKeyError, PyValueError},
  types::PyDict,
};
use spdcalc::{
  dim::{f64prefixes::MICRO, ucum::M},
  Apodization,
};

use super::*;

pub(crate) fn apodization_from_py_dict<'py>(dict: &Bound<'py, PyDict>) -> PyResult<Apodization> {
  let kind = dict
    .get_item("kind")?
    .ok_or(PyKeyError::new_err(
      "Expecting 'kind' key in apodization config.",
    ))?
    .extract::<String>()?;

  let param = dict.get_item("parameter")?.ok_or(PyKeyError::new_err(
    "Expecting 'parameter' key in apodization config.",
  ))?;

  match kind.to_lowercase().as_str() {
    "off" => Ok(Apodization::Off),
    "gaussian" => {
      let fwhm = param
        .downcast::<PyDict>()?
        .get_item("fwhm_um")?
        .ok_or(PyKeyError::new_err(
          "Expecting dictionary with 'fwhm_um' key as gaussian apodization parameter value.",
        ))?
        .extract::<f64>()?;
      Ok(Apodization::Gaussian {
        fwhm: fwhm * MICRO * M,
      })
    }
    "interpolate" => {
      let points = param.extract::<Vec<f64>>()?;
      Ok(Apodization::Interpolate(points))
    }
    _ => serde_json::from_value(serde_json::json! {
      {
        "kind": kind,
        "parameter": param.extract::<f64>()?
      }
    })
    .map_err(|e| PyValueError::new_err(e.to_string())),
  }
}

pub(crate) fn apodization_to_py_dict<'py>(
  py: Python<'py>,
  apodization: &Apodization,
) -> PyResult<Bound<'py, PyDict>> {
  let dict = PyDict::new_bound(py);

  match apodization {
    &Apodization::Off => {
      dict.set_item("kind", "off")?;
    }
    &Apodization::Gaussian { fwhm } => {
      let fwhm_um = *(fwhm / MICRO / M);
      let param = PyDict::new_bound(py);
      param.set_item("fwhm_um", fwhm_um)?;
      dict.set_item("kind", "gaussian")?;
      dict.set_item("parameter", param)?;
    }
    Apodization::Interpolate(points) => {
      dict.set_item("kind", "interpolate")?;
      dict.set_item("parameter", points.clone())?;
    }
    &Apodization::Bartlett(p)
    | &Apodization::Blackman(p)
    | &Apodization::Connes(p)
    | &Apodization::Cosine(p)
    | &Apodization::Hamming(p)
    | &Apodization::Welch(p) => {
      dict.set_item("kind", apodization.kind())?;
      dict.set_item("parameter", p)?;
    }
  }

  Ok(dict)
}
