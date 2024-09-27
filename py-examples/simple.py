from spdcalc import *
import pathlib
import time
basedir = pathlib.Path(__file__).parent.resolve()

def get_spdc():
    path = basedir / "settings.yaml"
    with open(path, 'r') as f:
        config = f.read()
    return SPDC.from_yaml(config)

# help(SPDC)
# print(get_all_crystal_meta())
spdc = get_spdc()
# spdc.crystal_kind = 'BBO_1'
# print(get_crystal_meta(spdc.crystal_kind))

spdc.apodization = {
  'kind': 'gaussian',
  'parameter': {
    'fwhm_um': 0.1
  }
}
print(spdc)
print(spdc.signal_waist_um)

spdc.crystal_kind = """
no = sqrt(2.7359+0.01878/(l^2-0.01822)-0.01354*l^2) - 9.3e-6 * T
ne = sqrt(2.3753+0.01224/(l^2-0.01667)-0.01516*l^2) - 16.6e-6 * T
"""

spdc.to_optimum()

print(spdc.to_yaml())

grid = spdc.optimum_range(100)
# tests
# print(grid)
# print(get_jsi(spdc, spdc.optimum_range(100).to_wavelength_space()))
# print(get_jsi.__doc__)
# print(spdc.delta_k(spdc.signal_frequency_hz, spdc.idler_frequency_hz))
spectrum = spdc.joint_spectrum()
print("shmidt", spectrum.schmidt_number(grid))
print("HOM 2 source visibilities")
print(spdc.hom_two_source_visibilities(grid.set_resolution(10), Integrator.default()))

print("HOM Series")
spdc = SPDC.default()
time_steps = [x * 1e-15 for x in range(-400, 800, 50)]
start_time = time.time()
rates = spdc.hom_rate_series(time_steps, grid.set_resolution(500))
print("took:", time.time() - start_time, "s")
print(rates)
