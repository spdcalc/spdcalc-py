use std::collections::HashMap;

use crate::*;
use ::spdcalc::dim::{f64prefixes::*, ucum::*};
use ::spdcalc::SPDCConfig;
use pyo3::exceptions::PyValueError;
use spdcalc::utils::{from_celsius_to_kelvin, from_kelvin_to_celsius};
use spdcalc::{Apodization, PMType, PeriodicPoling, Time};

pub(crate) type Visibility = HashMap<String, f64>;

/// SPDC configuration object
///
/// This is the primary object that is used to hold the configuration of the SPDC process.
/// There are two main ways to create an instance of this object:
///
/// 1. By using the default constructor `SPDC.default()` which creates an
/// instance with default values. These can then be modified as needed.
///
/// 2. By using the `SPDC.from_yaml(yaml: str)` or `SPDC.from_json(json: str)`
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct SPDC(pub(crate) ::spdcalc::SPDC);

#[pymethods]
impl SPDC {
  // allows for nice print statements in python
  fn __repr__(&self) -> pyo3::PyResult<String> {
    Ok(format!("{}", self.to_yaml()?))
  }

  /// Create a new SPDC object with default values
  #[staticmethod]
  pub fn default() -> Self {
    SPDC(spdcalc::SPDC::default())
  }

  /// Create a new SPDC object from a YAML string
  #[staticmethod]
  pub fn from_yaml(yaml: &str) -> Result<Self, PyErr> {
    let spdc: ::spdcalc::SPDC =
      serde_yaml::from_str(&yaml).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(Self(spdc))
  }

  /// Create a new SPDC object from a JSON string
  #[staticmethod]
  pub fn from_json(json: &str) -> Result<Self, PyErr> {
    let spdc: ::spdcalc::SPDC =
      serde_json::from_str(&json).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(Self(spdc))
  }

  /// Convert the SPDC object to a YAML string
  pub fn to_yaml(&self) -> Result<String, PySpdcError> {
    Ok(serde_yaml::to_string(&SPDCConfig::from(self.0.clone())).unwrap())
  }

  /// Convert the SPDC object to a JSON string
  pub fn to_json(&self) -> Result<String, PySpdcError> {
    Ok(serde_json::to_string(&SPDCConfig::from(self.0.clone())).unwrap())
  }

  // Getters and setters

  /// The type of crystal used in the SPDC process
  ///
  /// The values can be seen by looking at the `id` feilds from `get_all_crystal_meta()`
  #[getter]
  pub fn crystal_kind(&self) -> CrystalType {
    self.0.crystal_setup.crystal
  }

  #[setter]
  pub fn set_crystal_kind(&mut self, value: CrystalType) {
    self.0.crystal_setup.crystal = value;
  }

  /// The phasematching type used in the SPDC process
  ///
  /// The format for setting this is flexible. The following are all valid:
  /// - "ooo"
  /// - "o-oo"
  /// - "Type2 e eo"
  /// - "type 2 e->eo"
  #[getter]
  pub fn crystal_pm_type(&self) -> PMType {
    self.0.crystal_setup.pm_type
  }

  #[setter]
  pub fn set_crystal_pm_type(&mut self, value: PMType) {
    self.0.crystal_setup.pm_type = value;
  }

  /// The crystal polar angle in degrees
  #[getter]
  pub fn crystal_phi_deg(&self) -> f64 {
    *(self.0.crystal_setup.phi / DEG)
  }

  #[setter]
  pub fn set_crystal_phi_deg(&mut self, value: f64) {
    self.0.crystal_setup.phi = value * DEG;
  }

  /// The crystal azimuthal angle in degrees
  #[getter]
  pub fn crystal_theta_deg(&self) -> f64 {
    *(self.0.crystal_setup.theta / DEG)
  }

  #[setter]
  pub fn set_crystal_theta_deg(&mut self, value: f64) {
    self.0.crystal_setup.theta = value * DEG;
  }

  /// The crystal length in micrometers
  #[getter]
  pub fn crystal_length_um(&self) -> f64 {
    *(self.0.crystal_setup.length / M / MICRO)
  }

  #[setter]
  pub fn set_crystal_length_um(&mut self, value: f64) {
    self.0.crystal_setup.length = value * M * MICRO;
  }

  /// The crystal temperature in degrees Celsius
  #[getter]
  pub fn crystal_temperature_c(&self) -> f64 {
    from_kelvin_to_celsius(self.0.crystal_setup.temperature)
  }

  #[setter]
  pub fn set_crystal_temperature_c(&mut self, value: f64) {
    self.0.crystal_setup.temperature = from_celsius_to_kelvin(value)
  }

  /// Whether or not counter-propagation is used
  #[getter]
  pub fn counter_propagation(&self) -> bool {
    self.0.crystal_setup.counter_propagation
  }

  #[setter]
  pub fn set_counter_propagation(&mut self, value: bool) {
    self.0.crystal_setup.counter_propagation = value;
  }

  // pump

  /// The pump wavelength in nanometers
  #[getter]
  pub fn pump_wavelength_nm(&self) -> f64 {
    *(self.0.pump.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_pump_wavelength_nm(&mut self, value: f64) {
    self.0.pump.set_vacuum_wavelength(value * NANO * M);
  }

  /// The pump frequency in radians per second
  #[getter]
  pub fn pump_frequency_rad_per_s(&self) -> f64 {
    *(self.0.pump.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_pump_frequency_rad_per_s(&mut self, value: f64) {
    self.0.pump.set_frequency(value * RAD / S);
  }

  /// The pump waist in nanometers (x, y)
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

  /// The pump spectral bandwidth in nanometers
  #[getter]
  pub fn pump_bandwidth_nm(&self) -> f64 {
    *(self.0.pump_bandwidth / NANO / M)
  }

  #[setter]
  pub fn set_pump_bandwidth_nm(&mut self, value: f64) {
    self.0.pump_bandwidth = value * NANO * M;
  }

  /// The pump average power in milliwatts
  #[getter]
  pub fn pump_average_power_mw(&self) -> f64 {
    *(self.0.pump_average_power / MEGA / W)
  }

  #[setter]
  pub fn set_pump_average_power_mw(&mut self, value: f64) {
    self.0.pump_average_power = value * MEGA * W;
  }

  /// The pump spectrum threshold
  ///
  /// Values below this threshold are considered to be zero
  #[getter]
  pub fn pump_spectrum_threshold(&self) -> f64 {
    self.0.pump_spectrum_threshold
  }

  #[setter]
  pub fn set_pump_spectrum_threshold(&mut self, value: f64) {
    self.0.pump_spectrum_threshold = value;
  }

  // signal

  /// The signal wavelength in nanometers
  #[getter]
  pub fn signal_wavelength_nm(&self) -> f64 {
    *(self.0.signal.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_signal_wavelength_nm(&mut self, value: f64) {
    self.0.signal.set_vacuum_wavelength(value * NANO * M);
  }

  /// The signal frequency in radians per second
  #[getter]
  pub fn signal_frequency_rad_per_s(&self) -> f64 {
    *(self.0.signal.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_signal_frequency_rad_per_s(&mut self, value: f64) {
    self.0.signal.set_frequency(value * RAD / S);
  }

  /// The signal polar angle in degrees
  #[getter]
  pub fn signal_phi_deg(&self) -> f64 {
    *(self.0.signal.phi() / DEG)
  }

  #[setter]
  pub fn set_signal_phi_deg(&mut self, value: f64) {
    self.0.signal.set_phi(value * DEG);
  }

  /// The signal (internal) azimuthal angle in degrees
  #[getter]
  pub fn signal_theta_deg(&self) -> f64 {
    *(self.0.signal.theta_internal() / DEG)
  }

  #[setter]
  pub fn set_signal_theta_deg(&mut self, value: f64) {
    self.0.signal.set_theta_internal(value * DEG);
  }

  /// The signal external azimuthal angle in degrees
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

  /// The signal waist in nanometers (x, y)
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

  /// The signal waist position in micrometers
  #[getter]
  pub fn signal_waist_position_um(&self) -> f64 {
    *(self.0.signal_waist_position / MICRO / M)
  }

  #[setter]
  pub fn set_signal_waist_position_um(&mut self, value: f64) {
    self.0.signal_waist_position = value * MICRO * M;
  }

  // idler

  /// The idler wavelength in nanometers
  #[getter]
  pub fn idler_wavelength_nm(&self) -> f64 {
    *(self.0.idler.vacuum_wavelength() / NANO / M)
  }

  #[setter]
  pub fn set_idler_wavelength_nm(&mut self, value: f64) {
    self.0.idler.set_vacuum_wavelength(value * NANO * M);
  }

  /// The idler frequency in radians per second
  #[getter]
  pub fn idler_frequency_rad_per_s(&self) -> f64 {
    *(self.0.idler.frequency() * S / RAD)
  }

  #[setter]
  pub fn set_idler_frequency_rad_per_s(&mut self, value: f64) {
    self.0.idler.set_frequency(value * RAD / S);
  }

  /// The idler polar angle in degrees
  #[getter]
  pub fn idler_phi_deg(&self) -> f64 {
    *(self.0.idler.phi() / DEG)
  }

  #[setter]
  pub fn set_idler_phi_deg(&mut self, value: f64) {
    self.0.idler.set_phi(value * DEG);
  }

  /// The idler (internal) azimuthal angle in degrees
  #[getter]
  pub fn idler_theta_deg(&self) -> f64 {
    *(self.0.idler.theta_internal() / DEG)
  }

  #[setter]
  pub fn set_idler_theta_deg(&mut self, value: f64) {
    self.0.idler.set_theta_internal(value * DEG);
  }

  /// The idler external azimuthal angle in degrees
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

  /// The idler waist in nanometers (x, y)
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

  /// The idler waist position in micrometers
  #[getter]
  pub fn idler_waist_position_um(&self) -> f64 {
    *(self.0.idler_waist_position / MICRO / M)
  }

  #[setter]
  pub fn set_idler_waist_position_um(&mut self, value: f64) {
    self.0.idler_waist_position = value * MICRO * M;
  }

  // periodic poling

  /// The poling period in micrometers
  #[getter]
  pub fn poling_period_um(&self) -> Option<f64> {
    match self.0.pp {
      PeriodicPoling::Off => None,
      PeriodicPoling::On { period, .. } => Some(*(period / MICRO / M)),
    }
  }

  #[setter]
  pub fn set_poling_period_um(&mut self, value: Option<f64>) {
    if let None = value {
      self.0.pp = PeriodicPoling::Off;
      return;
    }
    let value = value.unwrap();
    match &self.0.pp {
      PeriodicPoling::Off => {
        self.0.pp = PeriodicPoling::new(value * MICRO * M, spdcalc::Apodization::Off)
      }
      PeriodicPoling::On { .. } => {
        self.0.pp = self.0.pp.clone().with_period(value * MICRO * M);
      }
    };
  }

  /// The apodization
  ///
  /// This is a dictionary with the following keys:
  /// - `kind`: the kind of apodization
  /// - `parameter`: the parameter depending on the kind
  ///
  /// The kind can be one of the following:
  /// - `off`: no apodization
  /// - `gaussian`: Gaussian function (parameter: `{ fwhm_nm: float }`)
  /// - `bartlett`: Bartlett function (parameter: float)
  /// - `blackman`: Blackman function (parameter: float)
  /// - `connes`: Connes function (parameter: float)
  /// - `cosine`: Cosine function (parameter: float)
  /// - `hamming`: Hamming function (parameter: float)
  /// - `welch`: Welch function (parameter: float)
  /// - `interpolate`: Interpolated evenly spaced points (parameter: list of floats)
  #[getter]
  pub fn apodization<'py>(&self) -> Apodization {
    self.0.pp.apodization().clone()
  }

  #[setter]
  pub fn set_apodization(&mut self, value: Option<Apodization>) {
    self.0.pp.set_apodization(value.unwrap_or(Apodization::Off));
  }

  // deff

  /// The effective nonlinear coefficient in pm/V
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

  /// Get the poling domains
  ///
  /// They are a list of fractions of poling period
  pub fn poling_domains(&self) -> Vec<(f64, f64)> {
    self.0.pp.poling_domains(self.0.crystal_setup.length)
  }

  /// Get the poling domains as lengths in meters
  pub fn poling_domain_lengths_m(&self) -> Vec<(f64, f64)> {
    self
      .0
      .pp
      .poling_domain_lengths(self.0.crystal_setup.length)
      .into_iter()
      .map(|(start, end)| (*(start / M), *(end / M)))
      .collect()
  }

  /// Convert this setup to an optimum setup
  pub fn to_optimum(mut slf: PyRefMut<'_, Self>) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().try_as_optimum()?.into();
    Ok(slf)
  }

  /// Convert this setup to one with an optimum idler
  pub fn with_optimum_idler(
    mut slf: PyRefMut<'_, Self>,
  ) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().with_optimum_idler()?.into();
    Ok(slf)
  }

  /// Convert this setup to one with an optimum crystal theta
  pub fn with_optimum_crystal_theta(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.0 = slf.0.clone().with_optimum_crystal_theta().into();
    slf
  }

  /// Convert this setup to one with an optimum periodic poling
  pub fn with_optimum_periodic_poling(
    mut slf: PyRefMut<'_, Self>,
  ) -> Result<PyRefMut<'_, Self>, PySpdcError> {
    slf.0 = slf.0.clone().with_optimum_periodic_poling()?.into();
    Ok(slf)
  }

  /// Swap the signal and idler
  pub fn with_swapped_signal_idler(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.0 = slf.0.clone().with_swapped_signal_idler().into();
    slf
  }

  /// Get the optimum crystal theta
  pub fn optimum_crystal_theta(slf: PyRef<'_, Self>) -> f64 {
    *(slf.0.optimum_crystal_theta() / DEG)
  }

  /// Get the optimum range in frequency space
  pub fn optimum_range(slf: PyRef<'_, Self>, n: usize) -> FrequencySpace {
    let range = slf.0.optimum_range(n);
    range.into()
  }

  /// Compute delta_k vector
  ///
  /// Parameters
  /// ----------
  /// signal_frequency_rad_per_s : float
  ///     The signal frequency in rad/s
  /// idler_frequency_rad_per_s : float
  ///     The idler frequency in rad/s
  ///
  /// Returns
  /// -------
  /// tuple of floats
  pub fn delta_k(
    &self,
    signal_frequency_rad_per_s: f64,
    idler_frequency_rad_per_s: f64,
  ) -> (f64, f64, f64) {
    let dk = *(self.0.delta_k(
      signal_frequency_rad_per_s * RAD / S,
      idler_frequency_rad_per_s * RAD / S,
    ) * M
      / RAD);
    (dk.x, dk.y, dk.z)
  }

  /// Calculate the coincidence counts
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// float
  ///     The coincidence counts
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

  /// Calculate the singles rate for the signal
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// float
  ///     The singles rate for the signal
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

  /// Calculate the singles rate for the idler
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// float
  ///     The singles rate for the idler
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

  /// Calculate the efficiencies (symmetric, signal, idler)
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// dict
  ///     The efficiencies (symmetric, signal, idler), and rates (coincidences, singles signal, singles idler)
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

  /// Calculate the Hong-Ou-Mandel visibility
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// dict
  ///     The Hong-Ou-Mandel visibility
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

  /// Calculate the Hong-Ou-Mandel rate for different time delays
  ///
  /// Parameters
  /// ----------
  /// time_delays : list of floats
  ///     The time delays in seconds
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// list of floats
  ///     The Hong-Ou-Mandel rate for different time delays
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

  /// Calculate the two-source Hong-Ou-Mandel visibilities
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// dict
  ///     The two-source Hong-Ou-Mandel visibilities
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

  /// Calculate the two-source Hong-Ou-Mandel rate series
  ///
  /// Parameters
  /// ----------
  /// time_delays : list of floats
  ///     The time delays in seconds
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// dict
  ///     The two-source Hong-Ou-Mandel rate series
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

  /// Calculate the joint spectrum
  ///
  /// Parameters
  /// ----------
  /// integrator : Integrator, optional
  ///     The integrator to use, which defaults to a simple Simpson's rule
  ///
  /// Returns
  /// -------
  /// JointSpectrum
  ///     The joint spectrum
  #[pyo3(signature = (integrator = None))]
  pub fn joint_spectrum(&self, integrator: Option<Integrator>) -> JointSpectrum {
    self
      .0
      .joint_spectrum(integrator.unwrap_or_default().0)
      .into()
  }
}
