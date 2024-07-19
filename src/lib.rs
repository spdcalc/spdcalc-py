use pyo3::prelude::*;
mod error;
use error::*;
mod spdc;
use spdc::SPDC;
mod spaces;
use spaces::*;

/// Get the joint spectral intensity of the SPDC within a given range
///
/// Parameters
/// ----------
/// `spdc`: SPDC
///   The SPDC object
/// `si_range`: SIRange
///   The range of signal-idler frequencies to consider
///
/// Returns
/// -------
/// `list`
///   The joint spectral intensities
#[pyfunction]
fn get_jsi(spdc: &SPDC, si_range: SIRange) -> Vec<f64> {
  spdc.0.joint_spectrum(None).jsi_normalized_range(si_range)
}

/// Module containing the python bindings for SPDCalc
#[pymodule(name = "spdcalc")]
fn spdcalc_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<SPDC>()?;

  m.add_function(wrap_pyfunction!(get_jsi, m)?)?;
  Ok(())
}
