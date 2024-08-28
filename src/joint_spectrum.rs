use super::*;
use spdcalc::dim::ucum::*;
use spdcalc::Complex;


/// Represents the joint spectrum of an SPDC process
///
/// This class has methods to calculate the joint spectral amplitude (JSA),
/// joint spectral intensity (JSI), and normalized JSA and JSI.
///
/// NOTE: The easiest way to create this is to use the `joint_spectrum` method
/// on a [`SPDC`] object.
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct JointSpectrum(pub(crate) ::spdcalc::JointSpectrum);

#[pymethods]
impl JointSpectrum {
  /// Create a new JointSpectrum instance
  ///
  /// Parameters
  /// ----------
  /// spdc : SPDC
  ///     SPDC configuration
  /// integrator : Integrator
  ///     Numerical integration method
  ///
  /// Returns
  /// -------
  /// JointSpectrum
  ///     The new JointSpectrum instance
  #[new]
  pub fn new(spdc: SPDC, integrator: Integrator) -> PyResult<Self> {
    Ok(Self(::spdcalc::JointSpectrum::new(spdc.0, integrator.0)))
  }

  /// Calculate the joint spectral amplitude (JSA) at a specific signal and idler frequency
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// complex
  ///     The JSA value
  pub fn jsa(&self, omega_s_hz: f64, omega_i_hz: f64) -> Complex<f64> {
    self.0.jsa(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  /// Calculate the JSA over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of complex
  ///     Vector of JSA values
  pub fn jsa_range(&self, si_range: SIRange) -> Vec<Complex<f64>> {
    self.0.jsa_range(si_range)
  }

  /// Calculate the normalized JSA at specific frequencies
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// complex
  ///     The normalized JSA value
  pub fn jsa_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> Complex<f64> {
    self
      .0
      .jsa_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  /// Calculate the normalized JSA over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of complex
  ///     Vector of normalized JSA values
  pub fn jsa_normalized_range(&self, si_range: SIRange) -> Vec<Complex<f64>> {
    self.0.jsa_normalized_range(si_range)
  }

  /// Calculate the joint spectral intensity (JSI) at specific frequencies
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// float
  ///     The JSI value
  pub fn jsi(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    *(self.0.jsi(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ) / spdcalc::JSIUnits::new(1.))
  }

  /// Calculate the JSI over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of float
  ///     Vector of JSI values
  pub fn jsi_range(&self, si_range: SIRange) -> Vec<f64> {
    self
      .0
      .jsi_range(si_range)
      .into_iter()
      .map(|jsi| *(jsi / spdcalc::JSIUnits::new(1.)))
      .collect()
  }

  /// Calculate the normalized JSI at specific frequencies
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// float
  ///     The normalized JSI value
  pub fn jsi_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    self
      .0
      .jsi_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  /// Calculate the normalized JSI over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of float
  ///     Vector of normalized JSI values
  pub fn jsi_normalized_range(&self, si_range: SIRange) -> Vec<f64> {
    self.0.jsi_normalized_range(si_range)
  }

  /// Calculate the singles JSI at specific frequencies
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// float
  ///     The singles JSI value
  pub fn jsi_singles(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    *(self
      .0
      .jsi_singles(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
      / spdcalc::JSIUnits::new(1.))
  }

  /// Calculate the singles JSI over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of float
  ///     Vector of singles JSI values
  pub fn jsi_singles_range(&self, si_range: SIRange) -> Vec<f64> {
    self
      .0
      .jsi_singles_range(si_range)
      .into_iter()
      .map(|jsi| *(jsi / spdcalc::JSIUnits::new(1.)))
      .collect()
  }

  /// Calculate the normalized singles JSI at specific frequencies
  ///
  /// Parameters
  /// ----------
  /// omega_s_hz : float
  ///     Signal frequency in Hz
  /// omega_i_hz : float
  ///     Idler frequency in Hz
  ///
  /// Returns
  /// -------
  /// float
  ///     The normalized singles JSI value
  pub fn jsi_singles_normalized(&self, omega_s_hz: f64, omega_i_hz: f64) -> f64 {
    self
      .0
      .jsi_singles_normalized(omega_s_hz * RAD * HZ, omega_i_hz * RAD * HZ)
  }

  /// Calculate the normalized singles JSI over a range of frequencies
  ///
  /// Parameters
  /// ----------
  /// si_range : SIRange
  ///     Range of signal and idler frequencies
  ///
  /// Returns
  /// -------
  /// list of float
  ///     Vector of normalized singles JSI values
  pub fn jsi_singles_normalized_range(&self, si_range: SIRange) -> Vec<f64> {
    self.0.jsi_singles_normalized_range(si_range)
  }
}

impl From<::spdcalc::JointSpectrum> for JointSpectrum {
  fn from(js: ::spdcalc::JointSpectrum) -> Self {
    Self(js)
  }
}
