# SoundSampler
<img width="1243" height="785" alt="SoundSamplerScreenShot" src="https://github.com/user-attachments/assets/3d3aebcf-4c81-40db-a7c5-ea1a0a12e111" />

## 使い方
### GUI
ダブルクリックでも開けます。
```
$ gui.exe
```

### CLI
```
$ cli.exe <json_path>
```

## jsonファイルフォーマット
examplesの例を参考にするとわかりやすいかもしれません。
### Mixdown
Wavファイルに変換される情報
|名前|型|説明|
|:---|---:|---:|
|num_channel|u16|チャンネル数|
|sample_rate|u32|1秒間あたりのデータ数|
|tracks|Vec\<InstrumentTrack\>|Mixdownを構成するトラックのリスト|

### InstrumentTrack
Mixdownを構成するトラック情報
|名前|型|デフォルト値|説明|
|:---|---:|---:|---:|
|source|SoundSource||音源|
|bpm|f32||BPM|
|source_notes|Vec\<Note\>||音符のリスト|
|volume|Option\<f32\>|1.0|このトラックの音量|
|channel|Option\<u16\>|0|チャンネル番号(0以上Mixdown.num_channel未満)|
|envelope|Option\<Envelope\>|None|Envelopeが適用されていないNoteに適用するEnvelope|

### SoundSource
トラックの音源
#### Sampler
wavファイルを音源として扱う
|名前|型|説明|
|:---|---:|---:|
||PathBuf|音源となるwavファイルのパス|

#### Sin
sin波のCを音源として扱う

#### Triangle
三角波のCを音源として扱う

#### KarpusStrong
|名前|型|説明|
|:---|---:|---:|
||Option\<u64\>|KarpusStrongの音源を生成する乱数の値|

### Note
音符
|名前|型|デフォルト値|説明|
|:---|---:|---:|---:|
|semitone|Option\<f32\>|休符|音源からのトーンの変化|
|strart|Option\<f32\>|一番後ろに追加|start拍後にこの音符を追加|
|length|f32||length拍間伸ばす|
|Envelope|Option\<Envelope\>||このNoteに適用するEnvelope(InstrumentTrackに対するものよりも優先)|

### Envelope
エンベロープ
|名前|型|説明|
|:---|---:|---:|
|attack|f32|EnvelopeのAttack\(seconds\)|
|decay|f32|EnvelopeのDecay\(seconds\)
|sustain|f32|EnvelopeのSustain|
|release|f32|Envelopeのrelease\(Seconds\)|
