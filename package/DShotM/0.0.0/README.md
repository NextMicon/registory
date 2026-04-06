# DShot

DShotはドローンのESCとFCUの通信プロトコルです。

- FCU→ESC : モータ制御
- ESC→FCU : テレメトリ

の双方向通信ができます。

Bi-directional DShot（双方向通信）が可能なESCは限られています。

DShotのビットレートは3種類定義されています。
- DSHOT150 : 150,000 bps
- DSHOT300 : 300,000 bps
- DSHOT600 : 600,000 bps

公式の仕様書が見つからないため、以下オンライン記事ベースの情報です。

**参考**
- https://tom2rd.sakura.ne.jp/wp/2023/11/11/post-13404/
- https://brushlesswhoop.com/dshot-and-bidirectional-dshot/
- [betaflight/drivers/dshot.c]https://github.com/betaflight/betaflight/blob/915caae88d166c1fd8f931016f98558bab138490/src/main/drivers/dshot.c
- [ardupilot](https://ardupilot.org/copter/docs/common-esc-telemetry.html)

## FCS→ECS：制御信号

bitをH/Lの時間比で表現する。
- Hが長ければ1
- Lが長ければ0
- 具体的な時間比の制約は不明

1フレームは16bitで各ビットの意味は以下の通り。
- [0:10] モータ制御信号
  - 0~47: 制御信号
  - 48~2047: スロットル値（1/2000の分解能）
- [11] 1=テレメトリ要求
- [12:15] CRC
  - CRC=(DATA ^ (DATA >> 4) ^ (DATA >> 8)) & 0xf

## ECS→FCS：テレメトリ

制御信号でテレメトリ要求ビットが立っていた場合、制御信号の終了から30us後にテレメトリが返送される。

bitが逆転することに注意。
- Hが長ければ0
- Lが長ければ1

1フレームは16bitで各ビットの意味は以下の通り。
- [0:11] eRPMデータ
  - [0:2] 左シフト量
  - [3:11] 回転周期のベース
- [12:15] CRC
[3:11]<<[0:2]で回転周期[us]が求まる。

**EDT**
回転数以外にも、温度、電圧、電流などの情報を送ることができるプロトコル。

gcr = (value ^ (value >> 1));

