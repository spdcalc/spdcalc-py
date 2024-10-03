# SPDC Config Reference

An SPDC setup can be described by a configuration file. This file can be in JSON or YAML format.
The following is a reference for the configuration file.

*note*: For `crystal.kind` see [All built-in crystal ids](./crystals.txt) for a list of supported crystals.

```yaml
---
crystal:
  kind: KTP
  pm_type: Type2_e_eo # many formats accepted: "Type2 e eo", "type 2 e-eo", "e eo", "e-eo", ...
  phi_deg: 0.0
  theta_deg: 90 # or "auto" to automatically calculate the optimal angle if no periodic poling is used
  length_um: 2000.0
  temperature_c: 20.0
  counter_propagation: false
pump:
  wavelength_nm: 775.0
  waist_um: 100.0
  bandwidth_nm: 5.53
  average_power_mw: 1.0
  spectrum_threshold: 0.01
signal:
  wavelength_nm: 1550.0
  phi_deg: 0.0
  theta_deg: 0.0
  theta_external_deg: null
  waist_um: 100.0
  waist_position_um: -576.6732 # or "auto" to automatically calculate the optimal position
idler: # config or "auto"
  wavelength_nm: 1550.0
  phi_deg: 180.0
  theta_deg: 0.0
  theta_external_deg: null
  waist_um: 100.0
  waist_position_um: -560.9707069211875
periodic_poling:
  poling_period_um: auto
  apodization:
    kind: Gaussian
    parameter:
      fwhm_um: 0.1
deff_pm_per_volt: 1.0
```

## Practical Example

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
