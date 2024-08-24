use super::*;
use spdcalc::dim::ucum::*;
use spdcalc::Complex;

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct JointSpectrum(pub(crate) ::spdcalc::JointSpectrum);

#[pymethods]
impl JointSpectrum {
  #[new]
  pub fn new(spdc: SPDC, integrator: Integrator) -> PyResult<Self> {
    Ok(Self(::spdcalc::JointSpectrum::new(spdc.0, integrator.0)))
  }

  pub fn jsa(&self, omega_s_hz: f64, omega_i_hz: f64) -> Complex<f64> {
    self.0.jsa(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  pub fn jsa_range(&self, si_range: SIRange) -> Vec<Complex<f64>> {
    self.0.jsa_range(si_range)
  }

  pub fn jsa_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> Complex<f64> {
    self
      .0
      .jsa_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  pub fn jsa_normalized_range(&self, si_range: SIRange) -> Vec<Complex<f64>> {
    self.0.jsa_normalized_range(si_range)
  }

  pub fn jsi(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    *(self.0.jsi(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ) / spdcalc::JSIUnits::new(1.))
  }

  pub fn jsi_range(&self, si_range: SIRange) -> Vec<f64> {
    self
      .0
      .jsi_range(si_range)
      .into_iter()
      .map(|jsi| *(jsi / spdcalc::JSIUnits::new(1.)))
      .collect()
  }

  pub fn jsi_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    self
      .0
      .jsi_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  pub fn jsi_normalized_range(&self, si_range: SIRange) -> Vec<f64> {
    self.0.jsi_normalized_range(si_range)
  }

  pub fn jsi_singles(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    *(self
      .0
      .jsi_singles(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
      / spdcalc::JSIUnits::new(1.))
  }

  pub fn jsi_singles_range(&self, si_range: SIRange) -> Vec<f64> {
    self
      .0
      .jsi_singles_range(si_range)
      .into_iter()
      .map(|jsi| *(jsi / spdcalc::JSIUnits::new(1.)))
      .collect()
  }

  pub fn jsi_singles_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    self
      .0
      .jsi_singles_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  pub fn jsi_singles_normalized_range(&self, si_range: SIRange) -> Vec<f64> {
    self.0.jsi_singles_normalized_range(si_range)
  }
}

impl From<::spdcalc::JointSpectrum> for JointSpectrum {
  fn from(js: ::spdcalc::JointSpectrum) -> Self {
    Self(js)
  }
}
