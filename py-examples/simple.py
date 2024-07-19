from spdcalc import *
import pathlib
basedir = pathlib.Path(__file__).parent.resolve()

def get_spdc():
    path = basedir / "settings.yaml"
    with open(path, 'r') as f:
        config = f.read()
    return SPDC.from_yaml(config)

spdc = get_spdc()

print(spdc)

spdc.try_as_optimum()

print(spdc.to_yaml())

print(get_jsi(spdc, spdc.optimum_range(100).to_wavelength_space()))
print(spdc.optimum_range(100))
print(get_jsi.__doc__)
print(spdc.delta_k(spdc.signal_frequency_hz, spdc.idler_frequency_hz))
