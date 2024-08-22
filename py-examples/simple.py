from spdcalc import *
import pathlib
basedir = pathlib.Path(__file__).parent.resolve()

def get_spdc():
    path = basedir / "settings.yaml"
    with open(path, 'r') as f:
        config = f.read()
    return SPDC.from_yaml(config)

spdc = get_spdc()

spdc.apodization = {
  'kind': 'gaussian',
  'parameter': {
    'fwhm_um': 0.1
  }
}
print(spdc)

spdc.try_as_optimum()

print(spdc.to_yaml())

grid = spdc.optimum_range(100)
# tests
# print(grid)
# print(get_jsi(spdc, spdc.optimum_range(100).to_wavelength_space()))
# print(get_jsi.__doc__)
# print(spdc.delta_k(spdc.signal_frequency_hz, spdc.idler_frequency_hz))
print(spdc.hom_two_source_visibilities(grid.set_resolution(10), Integrator.default()))
time_steps = [x * 1e-15 for x in range(2000, 3000, 10)]
print(spdc.hom_rate_series(time_steps, grid.set_resolution(10)))
