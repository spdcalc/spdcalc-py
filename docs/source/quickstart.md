# Quickstart

## Installation

```bash
pip install spdcalc-py
```

## Configuration

Configuration of an SPDC setup can be done by a JSON or YAML file. See the
[Full Config Reference](./full-config-reference.md) for a complete list of options.

Here is an example configuration file:

```yaml
---
crystal:
  kind: KTP
  pm_type: e->eo
  phi_deg: 0
  theta_deg: 90
  length_um: 2000
  temperature_c: 20
pump:
  wavelength_nm: 775
  waist_um: 100
  bandwidth_nm: 5.35
  average_power_mw: 1.0
signal:
  wavelength_nm: 1550
  phi_deg: 0
  theta_external_deg: 1
  waist_um: 100
  waist_position_um: auto
idler: auto
periodic_poling:
  poling_period_um: auto
deff_pm_per_volt: 1
```

## Usage

For ease of use one can import all from the spdcalc module and then load the config file directly
into an SPDC object:

```py
from spdcalc import *

f = open("my_config.yaml", "r")
config = f.read()

spdc = SPDC.from_yaml(config)
```

The `spdc` object can then be used in calculations. For example, to calculate the JSI:

```py
js = spdc.joint_spectrum()
range = spdc.optimum_range(100).to_wavelength_space()
jsi = js.jsi_range(range)

jsi = np.reshape(jsi, (100, 100))
x_values = np.array(range.x_values()) * 1e9
y_values = np.array(range.y_values()) * 1e9

fig = go.Figure(data=go.Heatmap(
  z=jsi,
  x=x_values,
  y=y_values
))

fig.update_layout(
  title='Joint Spectrum Intensity',
  xaxis_title='Signal Frequency (nm)',
  yaxis_title='Idler Frequency (nm)',
)

fig.show()
```
