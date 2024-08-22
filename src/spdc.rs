use std::collections::HashMap;

use crate::*;
use ::spdcalc::dim::{f64prefixes::*, ucum::*};
use ::spdcalc::SPDCConfig;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyDict;
use spdcalc::utils::{from_celsius_to_kelvin, from_kelvin_to_celsius};
use spdcalc::{Frequency, PeriodicPoling, Time};

pub(crate) type Visibility = HashMap<String, f64>;

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct SPDC(pub(crate) ::spdcalc::SPDC);

#[pymethods]
impl SPDC {
  // allows for nice print statements in python
  fn __repr__(&self) -> pyo3::PyResult<String> {
    Ok(format!("{}", self.to_yaml()?))
  }

  #[staticmethod]
  pub fn default() -> Self {
    SPDC(spdcalc::SPDC::default())
  }

  #[staticmethod]
  pub fn from_yaml(yaml: &str) -> Result<Self, PyErr> {
    let spdc: ::spdcalc::SPDC =
      serde_yaml::from_str(&yaml).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(Self(spdc))
  }

  #[staticmethod]
  pub fn from_json(json: &str) -> Result<Self, PyErr> {
    let spdc: ::spdcalc::SPDC =
      serde_json::from_str(&json).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(Self(spdc))
  }

  pub fn to_yaml(&self) -> Result<String, PySpdcError> {
    Ok(serde_yaml::to_string(&SPDCConfig::from(self.0.clone())).unwrap())
  }

  pub fn to_json(&self) -> Result<String, PySpdcError> {
    Ok(serde_json::to_string(&SPDCConfig::from(self.0.clone())).unwrap())
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
  pub fn pump_frequency_hz(&self) -> f64 {
    *(self.0.pump.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_pump_frequency_hz(&mut self, value: f64) {
    self.0.pump.set_frequency(value * RAD / S);
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
  pub fn signal_frequency_hz(&self) -> f64 {
    *(self.0.signal.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_signal_frequency_hz(&mut self, value: f64) {
    self.0.signal.set_frequency(value * RAD / S);
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
  pub fn idler_frequency_hz(&self) -> f64 {
    *(self.0.idler.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_idler_frequency_hz(&mut self, value: f64) {
    self.0.idler.set_frequency(value * RAD / S);
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
  pub fn apodization<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
    apodization_to_py_dict(py, &self.0.pp.apodization())
  }

  #[setter]
  pub fn set_apodization(&mut self, value: Option<&Bound<'_, PyDict>>) -> PyResult<()> {
    let apo = if let Some(dict) = value {
      apodization_from_py_dict(dict)?
    } else {
      spdcalc::Apodization::default()
    };

    self.0.pp.set_apodization(apo);

    Ok(())
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

  pub fn with_optimum_idler(
    mut slf: PyRefMut<'_, Self>,
  ) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().with_optimum_idler()?.into();
    Ok(slf)
  }

  pub fn with_optimum_crystal_theta(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.0 = slf.0.clone().with_optimum_crystal_theta().into();
    slf
  }

  pub fn with_optimum_periodic_poling(
    mut slf: PyRefMut<'_, Self>,
  ) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().with_optimum_periodic_poling()?.into();
    Ok(slf)
  }

  pub fn with_swapped_signal_idler(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.0 = slf.0.clone().with_swapped_signal_idler().into();
    slf
  }

  pub fn optimum_crystal_theta(slf: PyRef<'_, Self>) -> f64 {
    *(slf.0.optimum_crystal_theta() / DEG)
  }

  pub fn optimum_range(slf: PyRef<'_, Self>, n: usize) -> FrequencySpace {
    let range = slf.0.optimum_range(n);
    range.into()
  }

  pub fn delta_k(&self, signal_frequency_hz: f64, idler_frequency_hz: f64) -> (f64, f64, f64) {
    let dk = *(self
      .0
      .delta_k(signal_frequency_hz * RAD / S, idler_frequency_hz * RAD / S)
      * M
      / DEG);
    (dk.x, dk.y, dk.z)
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn counts_coincidences(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<f64> {
    let counts = self.0.counts_coincidences(
      ::spdcalc::FrequencySpace::try_from(si_range)?,
      integrator.unwrap_or_default().0,
    );
    Ok(*(counts * S))
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn counts_singles_signal(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<f64> {
    let counts = self.0.counts_singles_signal(
      ::spdcalc::FrequencySpace::try_from(si_range)?,
      integrator.unwrap_or_default().0,
    );
    Ok(*(counts * S))
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn counts_singles_idler(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<f64> {
    let counts = self.0.counts_singles_idler(
      ::spdcalc::FrequencySpace::try_from(si_range)?,
      integrator.unwrap_or_default().0,
    );
    Ok(*(counts * S))
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn efficiencies(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<HashMap<String, f64>> {
    let effs: HashMap<String, f64> = self
      .0
      .efficiencies(
        ::spdcalc::FrequencySpace::try_from(si_range)?,
        integrator.unwrap_or_default().0,
      )
      .into();

    Ok(effs)
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn hom_visibility(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<Visibility> {
    let (time, v): (Time, f64) = self.0.hom_visibility(
      ::spdcalc::FrequencySpace::try_from(si_range)?,
      integrator.unwrap_or_default().0,
    );

    let mut vis = HashMap::new();
    vis.insert("time".to_string(), *(time / S));
    vis.insert("visibility".to_string(), v);
    Ok(vis)
  }

  #[pyo3(signature = (time_delays, si_range, integrator = None))]
  pub fn hom_rate_series(
    &self,
    time_delays: Vec<f64>,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<Vec<f64>> {
    let time_delays: Vec<_> = time_delays.into_iter().map(|t| t * S).collect();
    let rates = self.0.hom_rate_series(
      time_delays,
      ::spdcalc::FrequencySpace::try_from(si_range)?,
      integrator.unwrap_or_default().0,
    );
    Ok(rates)
  }

  #[pyo3(signature = (si_range, integrator = None))]
  pub fn hom_two_source_visibilities(
    &self,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<HashMap<String, Visibility>> {
    let vis: HashMap<String, (Time, f64)> = self
      .0
      .hom_two_source_visibilities(
        ::spdcalc::FrequencySpace::try_from(si_range)?,
        integrator.unwrap_or_default().0,
      )
      .into();
    let vis: HashMap<String, Visibility> = vis
      .into_iter()
      .map(|(k, (time, v))| {
        let mut vis = HashMap::new();
        vis.insert("time".to_string(), *(time / S));
        vis.insert("visibility".to_string(), v);
        (k, vis)
      })
      .collect();
    Ok(vis)
  }

  #[pyo3(signature = (time_delays, si_range, integrator = None))]
  pub fn hom_two_source_rate_series(
    &self,
    time_delays: Vec<f64>,
    si_range: SIRange,
    integrator: Option<crate::Integrator>,
  ) -> PyResult<HashMap<String, Vec<f64>>> {
    let time_delays: Vec<_> = time_delays.into_iter().map(|t| t * S).collect();
    let rates: HashMap<String, Vec<f64>> = self
      .0
      .hom_two_source_rate_series(
        time_delays,
        ::spdcalc::FrequencySpace::try_from(si_range)?,
        integrator.unwrap_or_default().0,
      )
      .into();
    Ok(rates)
  }
}
