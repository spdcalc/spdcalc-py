import spdcalc_py
import pathlib
basedir = pathlib.Path(__file__).parent.resolve()

def get_spdc():
    path = basedir / "settings.yaml"
    with open(path, 'r') as f:
        config = f.read()
    return spdcalc_py.SPDC.from_yaml(config)

spdc = get_spdc()

print(spdc)

spdc.try_as_optimum()

print(spdc)

print(spdcalc_py.get_jsi(spdc))
