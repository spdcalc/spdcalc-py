use ::spdcalc::dim::{f64prefixes::*, ucum::*};
use ::spdcalc::SPDCConfig;
use pyo3::types::PyDict;
use pyo3::{exceptions::PyValueError, prelude::*};
use spdcalc::utils::{from_celsius_to_kelvin, from_kelvin_to_celsius};
use spdcalc::{PeriodicPoling, Wavelength};

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

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct SPDC(pub(crate) ::spdcalc::SPDC);

#[pymethods]
impl SPDC {
  // allows for nice print statements in python
  fn __repr__(slf: PyRef<'_, Self>) -> pyo3::PyResult<String> {
    Ok(format!(
      "{}",
      serde_yaml::to_string(&::spdcalc::SPDCConfig::from(slf.0.clone())).unwrap()
    ))
  }

  #[staticmethod]
  pub fn default() -> Self {
    SPDC(spdcalc::SPDC::default())
  }

  #[staticmethod]
  pub fn from_yaml(yaml: &str) -> Result<Self, PySpdcError> {
    let config: SPDCConfig = serde_yaml::from_str(&yaml).unwrap();
    Ok(SPDC(config.try_into()?))
  }

  #[staticmethod]
  pub fn from_json(json: &str) -> Result<Self, PySpdcError> {
    let config: SPDCConfig = serde_json::from_str(&json).unwrap();
    Ok(SPDC(config.try_into()?))
  }

  // Getters and setters
  // crystal
  #[getter]
  pub fn crystal_kind(&self) -> String {
    self.0.crystal_setup.crystal.to_string()
  }

  #[setter]
  pub fn set_crystal_kind(&mut self, value: String) -> Result<(), PySpdcError> {
    self.0.crystal_setup.crystal = value.parse()?;
    Ok(())
  }

  #[getter]
  pub fn crystal_pm_type(&self) -> String {
    self.0.crystal_setup.pm_type.to_string()
  }

  #[setter]
  pub fn set_crystal_pm_type(&mut self, value: String) -> Result<(), PySpdcError> {
    self.0.crystal_setup.pm_type = value.parse()?;
    Ok(())
  }

  #[getter]
  pub fn crystal_phi_deg(&self) -> f64 {
    *(self.0.crystal_setup.phi / DEG)
  }

  #[setter]
  pub fn set_crystal_phi_deg(&mut self, value: f64) {
    self.0.crystal_setup.phi = value * DEG;
  }

  #[getter]
  pub fn crystal_theta_deg(&self) -> f64 {
    *(self.0.crystal_setup.theta / DEG)
  }

  #[setter]
  pub fn set_crystal_theta_deg(&mut self, value: f64) {
    self.0.crystal_setup.theta = value * DEG;
  }

  #[getter]
  pub fn crystal_length_um(&self) -> f64 {
    *(self.0.crystal_setup.length / M / MICRO)
  }

  #[setter]
  pub fn set_crystal_length_um(&mut self, value: f64) {
    self.0.crystal_setup.length = value * M * MICRO;
  }

  #[getter]
  pub fn crystal_temperature_c(&self) -> f64 {
    from_kelvin_to_celsius(self.0.crystal_setup.temperature)
  }

  #[setter]
  pub fn set_crystal_temperature_c(&mut self, value: f64) {
    self.0.crystal_setup.temperature = from_celsius_to_kelvin(value)
  }

  #[getter]
  pub fn counter_propagation(&self) -> bool {
    self.0.crystal_setup.counter_propagation
  }

  #[setter]
  pub fn set_counter_propagation(&mut self, value: bool) {
    self.0.crystal_setup.counter_propagation = value;
  }

  // pump
  #[getter]
  pub fn pump_wavelength_nm(&self) -> f64 {
    *(self.0.pump.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_pump_wavelength_nm(&mut self, value: f64) {
    self.0.pump.set_vacuum_wavelength(value * NANO * M);
  }

  #[getter]
  pub fn pump_waist_nm(&self) -> (f64, f64) {
    (
      *(self.0.pump.waist().x / NANO / M),
      *(self.0.pump.waist().y / NANO / M),
    )
  }

  #[setter]
  pub fn set_pump_waist_nm(&mut self, value: (f64, f64)) {
    self
      .0
      .pump
      .set_waist((value.0 * NANO * M, value.1 * NANO * M));
  }

  #[getter]
  pub fn pump_bandwidth_nm(&self) -> f64 {
    *(self.0.pump_bandwidth / NANO / M)
  }

  #[setter]
  pub fn set_pump_bandwidth_nm(&mut self, value: f64) {
    self.0.pump_bandwidth = value * NANO * M;
  }

  #[getter]
  pub fn pump_average_power_mw(&self) -> f64 {
    *(self.0.pump_average_power / MEGA / W)
  }

  #[setter]
  pub fn set_pump_average_power_mw(&mut self, value: f64) {
    self.0.pump_average_power = value * MEGA * W;
  }

  #[getter]
  pub fn pump_spectrum_threshold(&self) -> f64 {
    self.0.pump_spectrum_threshold
  }

  #[setter]
  pub fn set_pump_spectrum_threshold(&mut self, value: f64) {
    self.0.pump_spectrum_threshold = value;
  }

  // signal
  #[getter]
  pub fn signal_wavelength_nm(&self) -> f64 {
    *(self.0.signal.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_signal_wavelength_nm(&mut self, value: f64) {
    self.0.signal.set_vacuum_wavelength(value * NANO * M);
  }

  #[getter]
  pub fn signal_phi_deg(&self) -> f64 {
    *(self.0.signal.phi() / DEG)
  }

  #[setter]
  pub fn set_signal_phi_deg(&mut self, value: f64) {
    self.0.signal.set_phi(value * DEG);
  }

  #[getter]
  pub fn signal_theta_deg(&self) -> f64 {
    *(self.0.signal.theta_internal() / DEG)
  }

  #[setter]
  pub fn set_signal_theta_deg(&mut self, value: f64) {
    self.0.signal.set_theta_internal(value * DEG);
  }

  #[getter]
  pub fn signal_theta_external_deg(&self) -> f64 {
    *(self.0.signal.theta_external(&self.0.crystal_setup) / DEG)
  }

  #[setter]
  pub fn set_signal_theta_external_deg(&mut self, value: f64) {
    self
      .0
      .signal
      .set_theta_external(value * DEG, &self.0.crystal_setup);
  }

  #[getter]
  pub fn signal_waist_um(&self) -> (f64, f64) {
    (
      *(self.0.signal.waist().x / MICRO / M),
      *(self.0.signal.waist().y / MICRO / M),
    )
  }

  #[setter]
  pub fn set_signal_waist_um(&mut self, value: (f64, f64)) {
    self
      .0
      .signal
      .set_waist((value.0 * MICRO * M, value.1 * MICRO * M));
  }

  #[getter]
  pub fn signal_waist_position_um(&self) -> f64 {
    *(self.0.signal_waist_position / MICRO / M)
  }

  #[setter]
  pub fn set_signal_waist_position_um(&mut self, value: f64) {
    self.0.signal_waist_position = value * MICRO * M;
  }

  // idler
  #[getter]
  pub fn idler_wavelength_nm(&self) -> f64 {
    *(self.0.idler.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_idler_wavelength_nm(&mut self, value: f64) {
    self.0.idler.set_vacuum_wavelength(value * NANO * M);
  }

  #[getter]
  pub fn idler_phi_deg(&self) -> f64 {
    *(self.0.idler.phi() / DEG)
  }

  #[setter]
  pub fn set_idler_phi_deg(&mut self, value: f64) {
    self.0.idler.set_phi(value * DEG);
  }

  #[getter]
  pub fn idler_theta_deg(&self) -> f64 {
    *(self.0.idler.theta_internal() / DEG)
  }

  #[setter]
  pub fn set_idler_theta_deg(&mut self, value: f64) {
    self.0.idler.set_theta_internal(value * DEG);
  }

  #[getter]
  pub fn idler_theta_external_deg(&self) -> f64 {
    *(self.0.idler.theta_external(&self.0.crystal_setup) / DEG)
  }

  #[setter]
  pub fn set_idler_theta_external_deg(&mut self, value: f64) {
    self
      .0
      .idler
      .set_theta_external(value * DEG, &self.0.crystal_setup);
  }

  #[getter]
  pub fn idler_waist_um(&self) -> (f64, f64) {
    (
      *(self.0.idler.waist().x / MICRO / M),
      *(self.0.idler.waist().y / MICRO / M),
    )
  }

  #[setter]
  pub fn set_idler_waist_um(&mut self, value: (f64, f64)) {
    self
      .0
      .idler
      .set_waist((value.0 * MICRO * M, value.1 * MICRO * M));
  }

  #[getter]
  pub fn idler_waist_position_um(&self) -> f64 {
    *(self.0.idler_waist_position / MICRO / M)
  }

  #[setter]
  pub fn set_idler_waist_position_um(&mut self, value: f64) {
    self.0.idler_waist_position = value * MICRO * M;
  }

  // periodic poling
  #[getter]
  pub fn poling_period_um(&self) -> Option<f64> {
    match self.0.pp {
      PeriodicPoling::Off => None,
      PeriodicPoling::On { period, .. } => Some(*(period / MICRO / M)),
    }
  }

  #[setter]
  pub fn set_poling_period_um(&mut self, value: f64) {
    match &self.0.pp {
      PeriodicPoling::Off => {
        self.0.pp = PeriodicPoling::new(value * MICRO * M, spdcalc::Apodization::Off)
      }
      PeriodicPoling::On { .. } => {
        self.0.pp = self.0.pp.clone().with_period(value * MICRO * M);
      }
    };
  }

  #[getter]
  pub fn apodization(&self) {
    todo!("Apodization not done yet")
  }

  #[setter]
  pub fn set_apodization(&mut self, value: PyObject) {
    todo!("Apodization not done yet")
  }

  // deff
  #[getter]
  pub fn deff_pm_per_volt(&self) -> f64 {
    *(self.0.deff / (PICO * M / V))
  }

  #[setter]
  pub fn set_deff_pm_per_volt(&mut self, value: f64) {
    self.0.deff = value * PICO * M / V;
  }

  // General methods
  //
  pub fn try_as_optimum(mut slf: PyRefMut<'_, Self>) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().try_as_optimum()?.into();
    Ok(slf)
  }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_jsi(spdc: PyRef<'_, SPDC>) -> Vec<f64> {
  let range = spdc.0.optimum_range(100);
  spdc.0.joint_spectrum(None).jsi_normalized_range(range)
}

/// A Python module implemented in Rust.
#[pymodule]
fn spdcalc_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<SPDC>()?;

  m.add_function(wrap_pyfunction!(get_jsi, m)?)?;
  Ok(())
}
