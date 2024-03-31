# saori_qrcode.dll

[GitHub repository](https://github.com/nikolat/saori-qrcode)

## これは何?

デスクトップマスコット、「伺か」で使用できるSAORIの一種です。

入力したテキストをQR Codeとして出力します。

## 使い方

Argument0に、使用する機能名を指定して使用します。
指定できる機能は`image`と`text`です。

### `image`

+ Argument1: QR Codeに変換したいテキスト
+ Argument2: 画像として出力するファイルのパス

+ Result: 無し

入力されたテキストのQR Codeを画像ファイル(PNG)として指定されたファイルのパスに出力します。

### `text`

+ Argument1: QR Codeに変換したいテキスト

+ Result: テキスト形式で整形されたQR Code

入力されたテキストのQR Codeを`\n`を改行コードと見立てて` `および`#`で表現したテキストを返します。

## 使用ライブラリ/参考にしたプロジェクト

+ [saori-resized-png](https://github.com/tukinami/saori-resized-png) / 月波 清火 (tukinami seika)
+ [qrcode-rust](https://github.com/kennytm/qrcode-rust) / kennytm
+ [image](https://github.com/image-rs/image) / The image-rs Developers
+ (テスト実行時) [encoding\_rs](https://github.com/hsivonen/encoding_rs) / Henri Sivonen
+ (テスト実行時) [tempfile](https://github.com/Stebalien/tempfile) / Steven Allen, The Rust Project Developers, Ashley Mannix, Jason White

## ライセンス

LICENSEファイルを見てください。

## 作成者

Don

[GitHub](https://github.com/nikolat)
