# SoundSampler

## 使い方
```
$ cargo run --bin cli <json_path>
```

## jsonファイルフォーマット
examplesの例を参考にするとわかりやすいかもしれません。
### Mixdown
|名前|型|説明|
|:---|---:|---:|
|num_channel|u16|チャンネル数|
|sample_rate|u32|1秒間あたりのデータ数|
|tracks|Vec<InstrumentTrack>|Mixdownを構成するトラックのリスト|

### InstrumentTrack
|名前|型|デフォルト値|説明|
|:---|---:|---:|---:|
|source|SoundSource||音源|
|bpm|f32||BPM|
|source_notes|Vec<Note>||音符のリスト|
|volume|Option<f32>|1.0|このトラックの音量|
|channel|Option<u16>|0|チャンネル番号(0以上Mixdown.num_channel未満)|

### SoundSource
#### Sampler
wavファイルを音源として扱う
|名前|型|説明|
|:---|---:|---:|
||PathBuf|音源となるwavファイルのパス|

#### Sin
sin波のCを音源として扱う

#### Triangle
三角波のCを音源として扱う

### Note
|名前|型|デフォルト値|説明|
|:---|---:|---:|---:|
|semitone|Option<f32>|休符|音源からのトーンの変化|
|strart|Option<f32>|一番後ろに追加|start拍後にこの音符を追加|
|length|f32||length拍間伸ばす|
