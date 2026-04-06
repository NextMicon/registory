# AMD Kria KV260 Vision AI Starter Kit

<https://www.amd.com/en/products/system-on-modules/kria/k26/kv260-vision-starter-kit.html>

## Hardware

- SOM: AMD Kria K26 (XCK26 - SFVC784-2LV-c)
- Logic Cell: 256,200
- Block RAM: 11.6 Mbit
- UltraRAM: 6.75 Mbit
- DSP Slices: 1,248
- Clock: 100 MHz
- DDR4: 4 GB
- Flash: 64 Mbit QSPI
- MIPI CSI-2 カメラ入力
- DisplayPort 出力
- USB 3.0 x4, Gigabit Ethernet x1

## Tools

- Vivado: Synthesis, Implementation, Bitstream generation
- openFPGALoader: Flash

### Linux (Ubuntu/Debian)

```bash
# Xilinx Vivado をインストール
# https://www.xilinx.com/support/download.html
# Vivado ML Edition (Kria / Zynq UltraScale+ 対応)

# openFPGALoader
sudo apt install openfpgaloader
```

### Windows

```powershell
# Xilinx Vivado をインストール
# https://www.xilinx.com/support/download.html
```

### macOS

Vivado は macOS 非対応。Linux VM を使用してください。

## USB ドライバ (Linux)

```bash
cd /tools/Xilinx/Vivado/2024.1/data/xicom/cable_drivers/lin64/install_script/install_drivers
sudo ./install_drivers
```

## Sys Ports

| Name | Type |
|------|------|
| pmod_1 ~ pmod_8 | inout tri logic |
| fan_en | output logic |

## 注意事項

- Kria K26 SOM + KV260 キャリアボードの構成
- PL (Programmable Logic) 部分のみ NextMicon で使用
- PMOD ヘッダ 1 基 (8 ピン) を汎用 I/O として使用可能
- MIPI / DisplayPort は専用 IP が必要なため NextMicon の直接サポート外

## 動作確認

```bash
openFPGALoader --detect
```
