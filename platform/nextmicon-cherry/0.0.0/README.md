# NextMicon Cherry

FPGA: `ICE40HX8K-BG121`. Clock: 48 MHz. GPIO: 64.

Requires the updated native NextMicon `micon` CLI (`micon fpga --help`).
`platform.nm.yaml` is registry task metadata. Copy `platform.yaml` to the project
root for NextMiconWeb/compiler platform information. Pin names are `clk`, `rstn`,
`gpio1` through `gpio64`, and the LED/control names in that file.
`rstn` is the active-low user button; synchronize its release in your RTL.

Build the supplied example from the installed project root:

```sh
micon fpga build --board cherry --source platform/hardware/blink.v --out build/blink
```

The `synth` task accepts a complete `hardware/top.v`. Pass additional `--source`
arguments when your design instantiates modules in other files. These tasks do
not supply a complete CPU/firmware runtime or USB bootloader. The browser compiler
buttons are still mock tasks; execute these commands using the native CLI.

Cherry uses Yosys, nextpnr-ice40 and icepack. Programming uses an external SPI programmer; the new JTAG command does not apply.
