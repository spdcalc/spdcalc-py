use crate::*;
use pyo3::exceptions::PyValueError;

pub(crate) struct PySpdcError(pub(crate) ::spdcalc::SPDCError);

impl From<::spdcalc::SPDCError> for PySpdcError {
  fn from(err: ::spdcalc::SPDCError) -> Self {
    PySpdcError(err)
  }
}

impl From<PySpdcError> for PyErr {
  fn from(err: PySpdcError) -> Self {
    PyValueError::new_err(err.0.to_string())
  }
}

impl From<serde_json::Error> for PySpdcError {
  fn from(err: serde_json::Error) -> Self {
    PySpdcError(::spdcalc::SPDCError(err.to_string()))
  }
}
