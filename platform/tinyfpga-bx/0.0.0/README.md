# TinyFPGA BX

<https://tinyfpga.com/>

## Hardware

- FPGA: Lattice iCE40 UP5K - SG48
- Logic Cell: 5280
- Block RAM: 30 x 4 Kbit
- SPRAM: 4 x 32 Kbit
- Clock: 16 MHz
- SPI Flash: 4 Mbit

## Tools

- yosys: Synthesys
- nextpnr-ice40: Place & Route
- icepack (icestorm): Bitstream generation
- iceprog / tinyprog: Flash

### Linux (Ubuntu/Debian)

```bash
# YosysHQ oss-cad-suite をインストール (yosys, nextpnr-ice40, icestorm 含む)
# https://github.com/YosysHQ/oss-cad-suite-build/releases から最新版をダウンロード
tar -xzf oss-cad-suite-linux-x64-*.tgz
export PATH="$PWD/oss-cad-suite/bin:$PATH"

# TinyFPGA BX 用プログラマ
pip install tinyprog
```

### Windows

```powershell
# oss-cad-suite の Windows 版をダウンロード・展開
# https://github.com/YosysHQ/oss-cad-suite-build/releases
# 展開先を PATH に追加

pip install tinyprog
```

### macOS

```bash
brew install yosys nextpnr icestorm
pip install tinyprog
```

## USB ドライバ (Linux)

```bash
sudo tee /etc/udev/rules.d/99-tinyfpga.rules << 'EOF'
ATTRS{idVendor}=="1209", ATTRS{idProduct}=="2100", MODE="0666"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## 注意事項

- 書き込み時はボードのリセットボタンを押してブートローダモードに入る必要がある
- tinyprog で書き込む場合、ブートローダが起動している状態で実行

## 動作確認

```bash
# tinyprog でボード検出
tinyprog -l

# iceprog でも可
iceprog -t
```
