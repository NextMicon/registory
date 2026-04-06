# Tang Nano 9K

<https://wiki.sipeed.com/hardware/en/tang/Tang-Nano-9K/Nano-9K.html>

## Hardware

- FPGA: GOWIN GW1NR-9C - QFN88P
- Logic Cell: 8640
- Block RAM: 26 x 18 Kbit
- PSRAM: 64 Mbit (HyperRAM)
- Clock: 27 MHz
- Flash: 32 Mbit (SPI)

## Tools

- yosys: Synthesis
- nextpnr-gowin: Place & Route
- gowin_pack: Bitstream generation
- openFPGALoader: Flash

### Linux (Ubuntu/Debian)

```bash
# YosysHQ oss-cad-suite をインストール (yosys, nextpnr-gowin 含む)
# https://github.com/YosysHQ/oss-cad-suite-build/releases から最新版をダウンロード
tar -xzf oss-cad-suite-linux-x64-*.tgz
export PATH="$PWD/oss-cad-suite/bin:$PATH"

# openFPGALoader
sudo apt install openfpgaloader
```

### Windows

```powershell
# oss-cad-suite の Windows 版をダウンロード・展開
# https://github.com/YosysHQ/oss-cad-suite-build/releases
# 展開先を PATH に追加
```

### macOS

```bash
brew install yosys nextpnr openfpgaloader
```

## USB ドライバ (Linux)

```bash
sudo tee /etc/udev/rules.d/99-tangnano.rules << 'EOF'
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6010", MODE="0666"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## Sys Ports

| Name | Type |
|------|------|
| io_0 ~ io_23 | inout tri logic |
| led_0 ~ led_5 | output logic |
| btn_0, btn_1 | input logic |

## 動作確認

```bash
openFPGALoader --detect
```
