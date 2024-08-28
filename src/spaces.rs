use crate::*;
use pyo3::exceptions::PyValueError;
use spdcalc::{
  dim::ucum::{HZ, M, RAD},
  utils::Steps2D,
  Frequency, IntoSignalIdlerIterator,
};

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum SIRange {
  #[pyo3(transparent)]
  FrequencySpace(FrequencySpace),
  #[pyo3(transparent)]
  FrequencyArray(FrequencyArray),
  #[pyo3(transparent)]
  Wavelength(WavelengthSpace),
  #[pyo3(transparent)]
  WavelengthArray(WavelengthArray),
  #[pyo3(transparent)]
  SumDiffFrequency(SumDiffFrequencySpace),
}

impl TryFrom<SIRange> for ::spdcalc::FrequencySpace {
  type Error = PyErr;
  fn try_from(si: SIRange) -> Result<Self, Self::Error> {
    match si {
      SIRange::FrequencySpace(fs) => Ok(fs.0),
      SIRange::FrequencyArray(_) => Err(PyValueError::new_err(
        "Cannot convert FrequencyArray to FrequencySpace",
      )),
      SIRange::Wavelength(ws) => Ok(ws.0.as_frequency_space()),
      SIRange::WavelengthArray(_) => Err(PyValueError::new_err(
        "Cannot convert WavelengthArray to FrequencySpace",
      )),
      SIRange::SumDiffFrequency(sdfs) => Ok(sdfs.0.as_frequency_space()),
    }
  }
}

impl From<FrequencySpace> for SIRange {
  fn from(fs: FrequencySpace) -> Self {
    SIRange::FrequencySpace(fs)
  }
}

impl From<::spdcalc::FrequencySpace> for SIRange {
  fn from(fs: ::spdcalc::FrequencySpace) -> Self {
    SIRange::FrequencySpace(fs.into())
  }
}

impl From<::spdcalc::SignalIdlerFrequencyArray> for SIRange {
  fn from(fa: ::spdcalc::SignalIdlerFrequencyArray) -> Self {
    SIRange::FrequencyArray(fa.into())
  }
}

impl From<::spdcalc::WavelengthSpace> for SIRange {
  fn from(ws: ::spdcalc::WavelengthSpace) -> Self {
    SIRange::Wavelength(ws.into())
  }
}

impl From<::spdcalc::SignalIdlerWavelengthArray> for SIRange {
  fn from(wa: ::spdcalc::SignalIdlerWavelengthArray) -> Self {
    SIRange::WavelengthArray(wa.into())
  }
}

impl From<::spdcalc::SumDiffFrequencySpace> for SIRange {
  fn from(sdfs: ::spdcalc::SumDiffFrequencySpace) -> Self {
    SIRange::SumDiffFrequency(sdfs.into())
  }
}

impl IntoSignalIdlerIterator for SIRange {
  fn into_signal_idler_iterator(self) -> impl Iterator<Item = (Frequency, Frequency)> {
    match self {
      SIRange::FrequencySpace(fs) => SIIterator(Box::new(fs.0.into_signal_idler_iterator())),
      SIRange::FrequencyArray(fa) => SIIterator(Box::new(fa.0.into_signal_idler_iterator())),
      SIRange::Wavelength(ws) => SIIterator(Box::new(ws.0.into_signal_idler_iterator())),
      SIRange::WavelengthArray(wa) => SIIterator(Box::new(wa.0.into_signal_idler_iterator())),
      SIRange::SumDiffFrequency(sdfs) => SIIterator(Box::new(sdfs.0.into_signal_idler_iterator())),
    }
  }
}

pub(crate) struct SIIterator(pub(crate) Box<dyn Iterator<Item = (Frequency, Frequency)>>);

impl Iterator for SIIterator {
  type Item = (Frequency, Frequency);

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

/// Represents a range of signal-idler frequencies
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct FrequencySpace(pub(crate) ::spdcalc::FrequencySpace);

#[pymethods]
impl FrequencySpace {
  /// Create a new FrequencySpace
  ///
  /// Parameters
  /// ----------
  /// xsteps : tuple (start, end, num_steps)
  ///     X-axis frequency range and number of steps in Hz
  /// ysteps : tuple (start, end, num_steps)
  ///     Y-axis frequency range and number of steps in Hz
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     New FrequencySpace object
  #[new]
  pub fn new(xsteps: (f64, f64, usize), ysteps: (f64, f64, usize)) -> Self {
    let (xs, xf, xn) = xsteps;
    let (ys, yf, yn) = ysteps;
    let xsteps = (xs * RAD * HZ, xf * RAD * HZ, xn);
    let ysteps = (ys * RAD * HZ, yf * RAD * HZ, yn);
    Self(Steps2D(xsteps, ysteps).into())
  }

  /// Convert from WavelengthSpace to FrequencySpace
  ///
  /// Parameters
  /// ----------
  /// ws : WavelengthSpace
  ///     The WavelengthSpace to convert
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     Converted FrequencySpace object
  #[staticmethod]
  pub fn from_wavelength_space(ws: &WavelengthSpace) -> Self {
    Self(ws.0.into())
  }

  /// Convert to WavelengthSpace
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     Converted WavelengthSpace object
  pub fn to_wavelength_space(&self) -> WavelengthSpace {
    WavelengthSpace(self.0.into())
  }

  /// Convert from SumDiffFrequencySpace to FrequencySpace
  ///
  /// Parameters
  /// ----------
  /// sdfs : SumDiffFrequencySpace
  ///     The SumDiffFrequencySpace to convert
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     Converted FrequencySpace object
  #[staticmethod]
  pub fn from_sum_diff_frequency_space(sdfs: &SumDiffFrequencySpace) -> Self {
    Self(sdfs.0.into())
  }

  /// Convert to SumDiffFrequencySpace
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     Converted SumDiffFrequencySpace object
  pub fn to_sum_diff_frequency_space(&self) -> SumDiffFrequencySpace {
    SumDiffFrequencySpace(self.0.into())
  }

  /// Set the resolution (number of steps) for both axes
  ///
  /// Parameters
  /// ----------
  /// steps : int
  ///     The number of steps to set for both axes
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     Updated FrequencySpace object
  pub fn set_resolution(mut slf: PyRefMut<'_, Self>, steps: usize) -> PyRefMut<'_, Self> {
    slf.0.set_resolution(steps);
    slf
  }

  /// String representation of FrequencySpace
  ///
  /// Returns
  /// -------
  /// str
  ///     String representation of the FrequencySpace
  pub fn __repr__(&self) -> String {
    let steps = self.0.steps();
    format!(
      "FrequencySpace(({}, {}, {}), ({}, {}, {}))",
      steps.0 .0 / RAD / HZ,
      steps.0 .1 / RAD / HZ,
      steps.0 .2,
      steps.1 .0 / RAD / HZ,
      steps.1 .1 / RAD / HZ,
      steps.1 .2
    )
  }
}

impl From<::spdcalc::FrequencySpace> for FrequencySpace {
  fn from(fs: ::spdcalc::FrequencySpace) -> Self {
    Self(fs)
  }
}

/// Represents a range of signal-idler wavelengths
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct WavelengthSpace(pub(crate) ::spdcalc::WavelengthSpace);

#[pymethods]
impl WavelengthSpace {
  /// Create a new WavelengthSpace
  ///
  /// Parameters
  /// ----------
  /// xsteps : tuple (start, end, num_steps)
  ///     X-axis wavelength range and number of steps in meters
  /// ysteps : tuple (start, end, num_steps)
  ///     Y-axis wavelength range and number of steps in meters
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     New WavelengthSpace object
  #[new]
  pub fn new(xsteps: (f64, f64, usize), ysteps: (f64, f64, usize)) -> Self {
    let (xs, xf, xn) = xsteps;
    let (ys, yf, yn) = ysteps;
    let xsteps = (xs * M, xf * M, xn);
    let ysteps = (ys * M, yf * M, yn);
    Self(Steps2D(xsteps, ysteps).into())
  }

  /// Convert from FrequencySpace to WavelengthSpace
  ///
  /// Parameters
  /// ----------
  /// fs : FrequencySpace
  ///     The FrequencySpace to convert
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     Converted WavelengthSpace object
  #[staticmethod]
  pub fn from_frequency_space(fs: &FrequencySpace) -> Self {
    Self(fs.0.into())
  }

  /// Convert to FrequencySpace
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     Converted FrequencySpace object
  pub fn to_frequency_space(&self) -> FrequencySpace {
    FrequencySpace(self.0.into())
  }

  /// Convert from SumDiffFrequencySpace to WavelengthSpace
  ///
  /// Parameters
  /// ----------
  /// sdfs : SumDiffFrequencySpace
  ///     The SumDiffFrequencySpace to convert
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     Converted WavelengthSpace object
  #[staticmethod]
  pub fn from_sum_diff_frequency_space(sdfs: &SumDiffFrequencySpace) -> Self {
    Self(sdfs.0.into())
  }

  /// Convert to SumDiffFrequencySpace
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     Converted SumDiffFrequencySpace object
  pub fn to_sum_diff_frequency_space(&self) -> SumDiffFrequencySpace {
    SumDiffFrequencySpace(self.0.into())
  }

  /// Set the resolution (number of steps) for both axes
  ///
  /// Parameters
  /// ----------
  /// steps : int
  ///     The number of steps to set for both axes
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     Updated WavelengthSpace object
  pub fn set_resolution(mut slf: PyRefMut<'_, Self>, steps: usize) -> PyRefMut<'_, Self> {
    slf.0.set_resolution(steps);
    slf
  }

  /// String representation of WavelengthSpace
  ///
  /// Returns
  /// -------
  /// str
  ///     String representation of the WavelengthSpace
  pub fn __repr__(&self) -> String {
    let steps = self.0.steps();
    format!(
      "WavelengthSpace(({}, {}, {}), ({}, {}, {}))",
      steps.0 .0 / M,
      steps.0 .1 / M,
      steps.0 .2,
      steps.1 .0 / M,
      steps.1 .1 / M,
      steps.1 .2
    )
  }
}

impl From<::spdcalc::WavelengthSpace> for WavelengthSpace {
  fn from(ws: ::spdcalc::WavelengthSpace) -> Self {
    Self(ws)
  }
}

/// Represents a range of signal-idler frequencies such that
/// one axis is the sum of two frequencies (divided by 2) and the other axis
/// is the difference of two frequencies (divided by 2)
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct SumDiffFrequencySpace(pub(crate) ::spdcalc::SumDiffFrequencySpace);

#[pymethods]
impl SumDiffFrequencySpace {
  /// Create a new SumDiffFrequencySpace
  ///
  /// Parameters
  /// ----------
  /// xsteps : tuple (start, end, num_steps)
  ///     X-axis frequency range and number of steps in Hz
  /// ysteps : tuple (start, end, num_steps)
  ///     Y-axis frequency range and number of steps in Hz
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     New SumDiffFrequencySpace object
  #[new]
  pub fn new(xsteps: (f64, f64, usize), ysteps: (f64, f64, usize)) -> Self {
    let (xs, xf, xn) = xsteps;
    let (ys, yf, yn) = ysteps;
    let xsteps = (xs * RAD * HZ, xf * RAD * HZ, xn);
    let ysteps = (ys * RAD * HZ, yf * RAD * HZ, yn);
    Self(Steps2D(xsteps, ysteps).into())
  }

  /// Convert from FrequencySpace to SumDiffFrequencySpace
  ///
  /// Parameters
  /// ----------
  /// fs : FrequencySpace
  ///     The FrequencySpace to convert
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     Converted SumDiffFrequencySpace object
  #[staticmethod]
  pub fn from_frequency_space(fs: &FrequencySpace) -> Self {
    Self(fs.0.into())
  }

  /// Convert to FrequencySpace
  ///
  /// Returns
  /// -------
  /// FrequencySpace
  ///     Converted FrequencySpace object
  pub fn to_frequency_space(&self) -> FrequencySpace {
    FrequencySpace(self.0.into())
  }

  /// Convert from WavelengthSpace to SumDiffFrequencySpace
  ///
  /// Parameters
  /// ----------
  /// ws : WavelengthSpace
  ///     The WavelengthSpace to convert
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     Converted SumDiffFrequencySpace object
  #[staticmethod]
  pub fn from_wavelength_space(ws: &WavelengthSpace) -> Self {
    Self(ws.0.into())
  }

  /// Convert to WavelengthSpace
  ///
  /// Returns
  /// -------
  /// WavelengthSpace
  ///     Converted WavelengthSpace object
  pub fn to_wavelength_space(&self) -> WavelengthSpace {
    WavelengthSpace(self.0.into())
  }

  /// Set the resolution (number of steps) for both axes
  ///
  /// Parameters
  /// ----------
  /// steps : int
  ///     The number of steps to set for both axes
  ///
  /// Returns
  /// -------
  /// SumDiffFrequencySpace
  ///     Updated SumDiffFrequencySpace object
  pub fn set_resolution(mut slf: PyRefMut<'_, Self>, steps: usize) -> PyRefMut<'_, Self> {
    slf.0.set_resolution(steps);
    slf
  }

  /// String representation of SumDiffFrequencySpace
  ///
  /// Returns
  /// -------
  /// str
  ///     String representation of the SumDiffFrequencySpace
  pub fn __repr__(&self) -> String {
    let steps = self.0.steps();
    format!(
      "SumDiffFrequencySpace(({}, {}, {}), ({}, {}, {}))",
      steps.0 .0 / RAD / HZ,
      steps.0 .1 / RAD / HZ,
      steps.0 .2,
      steps.1 .0 / RAD / HZ,
      steps.1 .1 / RAD / HZ,
      steps.1 .2
    )
  }
}

impl From<::spdcalc::SumDiffFrequencySpace> for SumDiffFrequencySpace {
  fn from(sdfs: ::spdcalc::SumDiffFrequencySpace) -> Self {
    Self(sdfs)
  }
}

/// Represents an array of signal-idler frequencies
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct FrequencyArray(pub(crate) ::spdcalc::SignalIdlerFrequencyArray);

#[pymethods]
impl FrequencyArray {
  /// Create a new FrequencyArray
  ///
  /// Parameters
  /// ----------
  /// frequencies : list of float
  ///     List of frequencies in Hz
  ///
  /// Returns
  /// -------
  /// FrequencyArray
  ///     New FrequencyArray object
  #[new]
  pub fn new(frequencies: Vec<f64>) -> Self {
    Self(::spdcalc::SignalIdlerFrequencyArray(
      frequencies.into_iter().map(|f| f * RAD * HZ).collect(),
    ))
  }

  /// String representation of FrequencyArray
  ///
  /// Returns
  /// -------
  /// str
  ///     String representation of the FrequencyArray
  pub fn __repr__(&self) -> String {
    format!("{:?}", self.0)
  }
}

impl From<::spdcalc::SignalIdlerFrequencyArray> for FrequencyArray {
  fn from(fa: ::spdcalc::SignalIdlerFrequencyArray) -> Self {
    Self(fa)
  }
}

/// Represents an array of signal-idler wavelengths
#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct WavelengthArray(pub(crate) ::spdcalc::SignalIdlerWavelengthArray);

#[pymethods]
impl WavelengthArray {
  /// Create a new WavelengthArray
  ///
  /// Parameters
  /// ----------
  /// wavelengths : list of float
  ///     List of wavelengths in meters
  ///
  /// Returns
  /// -------
  /// WavelengthArray
  ///     New WavelengthArray object
  #[new]
  pub fn new(wavelengths: Vec<f64>) -> Self {
    Self(::spdcalc::SignalIdlerWavelengthArray(
      wavelengths.into_iter().map(|w| w * M).collect(),
    ))
  }

  pub fn __repr__(&self) -> String {
    format!("{:?}", self.0)
  }
}

impl From<::spdcalc::SignalIdlerWavelengthArray> for WavelengthArray {
  fn from(wa: ::spdcalc::SignalIdlerWavelengthArray) -> Self {
    Self(wa)
  }
}
