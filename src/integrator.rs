use super::*;

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct Integrator(pub(crate) ::spdcalc::math::Integrator);

#[pymethods]
impl Integrator {
  fn __repr__(&self) -> String {
    format!("{}", serde_yaml::to_string(&self.0).unwrap())
  }

  #[staticmethod]
  pub fn default() -> Self {
    <Self as Default>::default()
  }

  #[staticmethod]
  #[pyo3(signature = (divs=None))]
  pub fn simpson(divs: Option<usize>) -> Self {
    let divs = divs.unwrap_or(50);
    Self(::spdcalc::math::Integrator::Simpson { divs })
  }

  #[staticmethod]
  #[pyo3(signature = (tolerance=None, max_depth=None))]
  pub fn adaptive_simpson(tolerance: Option<f64>, max_depth: Option<usize>) -> Self {
    let tolerance = tolerance.unwrap_or(1e5);
    let max_depth = max_depth.unwrap_or(1_000_000);
    Self(::spdcalc::math::Integrator::AdaptiveSimpson {
      tolerance,
      max_depth,
    })
  }

  #[staticmethod]
  #[pyo3(signature = (tolerance=None, max_depth=None))]
  pub fn gauss_konrod(tolerance: Option<f64>, max_depth: Option<usize>) -> Self {
    let tolerance = tolerance.unwrap_or(1e5);
    let max_depth = max_depth.unwrap_or(1_000_000);
    Self(::spdcalc::math::Integrator::GaussKonrod {
      tolerance,
      max_depth,
    })
  }

  #[staticmethod]
  #[pyo3(signature = (degree=None))]
  pub fn gauss_legendre(degree: Option<usize>) -> Self {
    let degree = degree.unwrap_or(40);
    Self(::spdcalc::math::Integrator::GaussLegendre { degree })
  }

  #[staticmethod]
  #[pyo3(signature = (tolerance=None))]
  pub fn clenshaw_curtis(tolerance: Option<f64>) -> Self {
    let tolerance = tolerance.unwrap_or(1e5);
    Self(::spdcalc::math::Integrator::ClenshawCurtis { tolerance })
  }
}

impl Default for Integrator {
  fn default() -> Self {
    Self(::spdcalc::math::Integrator::default())
  }
}
