# NextMicon Grape

FPGA: `xc7s50csga324-1`. Clock: 100 MHz. GPIO: 128.

Requires the updated native NextMicon `micon` CLI (`micon fpga --help`).
`platform.nm.yaml` is registry task metadata. Copy `platform.yaml` to the project
root for NextMiconWeb/compiler platform information. Pin names are `clk`, `rstn`,
`gpio1` through `gpio128`, and the LED/control names in that file.
`rstn` is the active-low user button; synchronize its release in your RTL.

Build the supplied example from the installed project root:

```sh
micon fpga build --board grape --source platform/hardware/blink.v --out build/blink
```

The `synth` task accepts a complete `hardware/top.v`. Pass additional `--source`
arguments when your design instantiates modules in other files. These tasks do
not supply a complete CPU/firmware runtime. The browser compiler
buttons are still mock tasks; execute these commands using the native CLI.

Requires Yosys, openXC7 nextpnr-xilinx, fasm2frames and xc7frames2bit.
Set `PRJXRAY_DB_DIR` to the directory containing the `spartan7` / `artix7`
databases. `JTAG_CABLE` selects your actual external probe for the program task.
The `program` task loads volatile FPGA SRAM. `bootloader` builds the USB recovery
image, `install_bootloader` installs it once over JTAG, `usb` writes and verifies
the persistent user slot via USB-C, and `flash` writes that slot over JTAG.
Set `USB_PORT` to the actual CDC port for the USB task. The OSS Xilinx backend
is experimental; physical USB, flash and power-cycle boot are unverified.

```sh
micon fpga program --board grape --cable "$JTAG_CABLE" --bitstream build/blink/hardware.bit --dry-run
```

## USB-C and persistent boot

```sh
micon fpga bootloader --board grape --out build/bootloader
micon fpga install-bootloader --board grape --cable "$JTAG_CABLE" --bitstream build/bootloader/hardware.bit
micon fpga usb --board grape --port "$USB_PORT" --bitstream build/blink/hardware.bit
```

Use the last command only after entering recovery: open JP1 (DEFAULT_USER) and
press SW1 (PROGRAM), or hold SW2 (USER) while pressing SW1. Close JP1 for automatic
validated user boot after power-on. Updating normally disconnects the recovery
USB port and boots the user circuit; use `--no-boot` to remain in recovery.
Add `--dry-run` to programming commands to inspect without accessing hardware.
The default USB ID 1209:0001 is private-testing only; specify assigned IDs with
`--usb-vid` / `--usb-pid` before distribution. A data-capable Type-C host supplying
the board's required 3 A advertisement is needed for power and communication.

The recovery region occupies 0..4 MiB; the user raw bitstream starts at
`0x400000`, with its board/length/CRC manifest in the last sector at `0x7ff000`.
USB cannot alter recovery or the flash above 8 MiB. The old Cherry COBS uploader
and 256 KiB-slot layout are incompatible. Keep `bootloader.json` beside the
recovery `.bit` file. Full instructions are in NextMiconWeb
`compiler/bootloader/README.md`.
