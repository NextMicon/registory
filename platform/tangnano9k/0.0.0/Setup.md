# Tang Nano 9K セットアップガイド

## ボード情報

| 項目               | 値                        |
| ------------------ | ------------------------- |
| メーカー           | Sipeed                    |
| FPGA               | GOWIN GW1NR-LV9QN88PC6/I5 |
| ロジックエレメント | 8,640 LUT4                |
| オンボードクロック | 27 MHz                    |
| HDMI 出力          | あり                      |
| SPI Flash          | 32 Mbit                   |
| PSRAM              | 64 Mbit (QSPI)            |
| パッケージ         | QFN88P                    |

## 必要なツール

### OSS ツールチェイン (推奨)

| ツール               | 用途                 |
| -------------------- | -------------------- |
| yosys                | 論理合成             |
| nextpnr-gowin        | 配置配線             |
| gowin_pack (apicula) | ビットストリーム生成 |
| openFPGALoader       | FPGA への書き込み    |

### Linux (Ubuntu/Debian)

```bash
# YosysHQ oss-cad-suite をインストール (yosys, nextpnr-gowin, apicula 含む)
# https://github.com/YosysHQ/oss-cad-suite-build/releases から最新版をダウンロード
tar -xzf oss-cad-suite-linux-x64-*.tgz
export PATH="$PWD/oss-cad-suite/bin:$PATH"

# openFPGALoader
sudo apt install openfpgaloader
# または oss-cad-suite に同梱
```

### Windows

```powershell
# oss-cad-suite の Windows 版をダウンロード・展開
# https://github.com/YosysHQ/oss-cad-suite-build/releases
# 展開先を PATH に追加
```

### macOS

```bash
brew install yosys nextpnr apicula openfpgaloader
```

## USB ドライバ (Linux)

```bash
sudo tee /etc/udev/rules.d/99-tangnano.rules << 'EOF'
ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6010", MODE="0666"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## 動作確認

```bash
openFPGALoader --detect
```
