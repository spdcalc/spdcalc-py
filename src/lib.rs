use pyo3::prelude::*;
mod error;
use error::*;
mod spdc;
use spdc::SPDC;
mod integrator;
use integrator::Integrator;
mod spaces;
use spaces::*;
mod joint_spectrum;
use joint_spectrum::*;
use spdcalc::{
  dim::{
    f64prefixes::NANO,
    ucum::{HZ, K, M, RAD},
  },
  Complex, CrystalMeta, CrystalType, JsiNorm, JsiSinglesNorm, PerMeter3, PerMeter4,
};

/// Get the joint spectral intensity of the SPDC within a given range
///
/// Parameters
/// ----------
/// `spdc`: SPDC
///     The SPDC object
/// `si_range`: SIRange
///     The range of signal-idler frequencies to consider
///
/// Returns
/// -------
/// `list`
///     The joint spectral intensities
#[pyfunction]
#[pyo3(signature = (spdc, si_range, integrator=None))]
fn get_jsi(spdc: &SPDC, si_range: SIRange, integrator: Option<Integrator>) -> Vec<f64> {
  spdc
    .0
    .joint_spectrum(integrator.unwrap_or_default().0)
    .jsi_normalized_range(si_range)
}

/// Get all crystal metadata
#[pyfunction]
fn get_all_crystal_meta() -> Vec<CrystalMeta> {
  CrystalType::get_all_meta()
}

/// Get the metadata for a specific crystal
///
/// Parameters
/// ----------
/// `crystal_kind`: str
///     The crystal type
///
/// Returns
/// -------
/// `dict`
#[pyfunction]
fn get_crystal_meta(crystal_kind: CrystalType) -> Result<CrystalMeta, PySpdcError> {
  Ok(crystal_kind.get_meta())
}

/// Get the refractive indices for a crystal
///
/// Parameters
/// ----------
/// `crystal_kind`: str
///     The crystal type
/// `wavelength_nm`: float
///     The wavelength in nanometers
/// `temp_kelvin`: float
///     The temperature in Kelvin
///
/// Returns
/// -------
/// `tuple`
///     The refractive indices
#[pyfunction]
fn get_crystal_indices(
  crystal_kind: CrystalType,
  wavelength_nm: f64,
  temp_kelvin: f64,
) -> (f64, f64, f64) {
  let i = crystal_kind.get_indices(wavelength_nm * NANO * M, temp_kelvin * K);
  (i.x, i.y, i.z)
}

/// Get the coincidences phasematching function amplitude for a given setup at a given frequencies
///
/// Parameters
/// ----------
/// `omega_s_rad_per_s`: float
///     The signal frequency in radians per second
/// `omega_i_rad_per_s`: float
///     The idler frequency in radians per second
/// `spdc`: SPDC
///     The SPDC object
/// `integrator`: Integrator, optional
///     The integrator object
///
/// Returns
/// -------
/// `complex`
///    The phasematching function amplitude
#[pyfunction]
#[pyo3(signature = (omega_s_rad_per_s, omega_i_rad_per_s, spdc, integrator=None))]
fn phasematch_fiber_coupling(
  omega_s_rad_per_s: f64,
  omega_i_rad_per_s: f64,
  spdc: &SPDC,
  integrator: Option<Integrator>,
) -> Complex<f64> {
  *(::spdcalc::phasematch_fiber_coupling(
    omega_s_rad_per_s * RAD * HZ,
    omega_i_rad_per_s * RAD * HZ,
    &spdc.0,
    integrator.unwrap_or_default().0,
  ) / PerMeter4::new(1.0))
}

/// Get the singles phasematching function *intensity* for a given setup at a given frequencies
///
/// Parameters
/// ----------
/// `omega_s_rad_per_s`: float
///     The signal frequency in radians per second
/// `omega_i_rad_per_s`: float
///     The idler frequency in radians per second
/// `spdc`: SPDC
///     The SPDC object
/// `integrator`: Integrator, optional
///     The integrator object
///
/// Returns
/// -------
/// `float`
///     The phasematching function intensity
#[pyfunction]
#[pyo3(signature = (omega_s_rad_per_s, omega_i_rad_per_s, spdc, integrator=None))]
fn phasematch_singles_fiber_coupling(
  omega_s_rad_per_s: f64,
  omega_i_rad_per_s: f64,
  spdc: &SPDC,
  integrator: Option<Integrator>,
) -> f64 {
  *(::spdcalc::phasematch_singles_fiber_coupling(
    omega_s_rad_per_s * RAD * HZ,
    omega_i_rad_per_s * RAD * HZ,
    &spdc.0,
    integrator.unwrap_or_default().0,
  ) / PerMeter3::new(1.0))
}

/// Get the normalization factor for the coincidences joint spectral intensity
///
/// Parameters
/// ----------
/// `omega_s_rad_per_s`: float
///     The signal frequency in radians per second
/// `omega_i_rad_per_s`: float
///     The idler frequency in radians per second
/// `spdc`: SPDC
///     The SPDC object
///
/// Returns
/// -------
/// `float`
///     The normalization factor
#[pyfunction]
fn jsi_normalization(omega_i_rad_per_s: f64, omega_s_rad_per_s: f64, spdc: &SPDC) -> f64 {
  *(::spdcalc::jsi_normalization(
    omega_i_rad_per_s * RAD * HZ,
    omega_s_rad_per_s * RAD * HZ,
    &spdc.0,
  ) / JsiNorm::new(1.))
}

/// Get the normalization factor for the singles joint spectral intensity
///
/// Parameters
/// ----------
/// `omega_s_rad_per_s`: float
///     The signal frequency in radians per second
/// `omega_i_rad_per_s`: float
///     The idler frequency in radians per second
/// `spdc`: SPDC
///     The SPDC object
///
/// Returns
/// -------
/// `float`
///     The normalization factor
#[pyfunction]
fn jsi_singles_normalization(omega_i_rad_per_s: f64, omega_s_rad_per_s: f64, spdc: &SPDC) -> f64 {
  *(::spdcalc::jsi_singles_normalization(
    omega_i_rad_per_s * RAD * HZ,
    omega_s_rad_per_s * RAD * HZ,
    &spdc.0,
  ) / JsiSinglesNorm::new(1.))
}

/// Get the pump spectral amplitude for a given setup at a given frequency
///
/// Parameters
/// ----------
/// `omega_rad_per_s`: float
///     The frequency in radians per second
/// `spdc`: SPDC
///     The SPDC object
///
/// Returns
/// -------
/// `float`
///     The pump spectral amplitude
#[pyfunction]
fn pump_spectral_amplitude(omega_rad_per_s: f64, spdc: SPDC) -> f64 {
  ::spdcalc::pump_spectral_amplitude(omega_rad_per_s * RAD * HZ, &spdc.0)
}

/// Module containing the python bindings for SPDCalc
#[pymodule(name = "spdcalc")]
fn spdcalc_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<SPDC>()?;
  m.add_class::<Integrator>()?;

  m.add_function(wrap_pyfunction!(get_jsi, m)?)?;
  m.add_function(wrap_pyfunction!(get_all_crystal_meta, m)?)?;
  m.add_function(wrap_pyfunction!(get_crystal_meta, m)?)?;
  m.add_function(wrap_pyfunction!(get_crystal_indices, m)?)?;
  m.add_function(wrap_pyfunction!(phasematch_fiber_coupling, m)?)?;
  m.add_function(wrap_pyfunction!(phasematch_singles_fiber_coupling, m)?)?;
  m.add_function(wrap_pyfunction!(jsi_normalization, m)?)?;
  m.add_function(wrap_pyfunction!(jsi_singles_normalization, m)?)?;
  m.add_function(wrap_pyfunction!(pump_spectral_amplitude, m)?)?;

  Ok(())
}
