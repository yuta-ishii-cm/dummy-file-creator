# fgen

![CI](https://github.com/yuta-ishii-cm/dummy-file-creator/workflows/CI/badge.svg)

指定サイズ（KB/MB/GB）のダミーファイルを簡単に生成できるRust製CLIツールです。
ディスク容量のテストや、ファイル転送速度の検証などに便利です。

## 特徴

- KB/MB/GBの単位指定に対応
- 高速なファイル生成
- シンプルで使いやすいコマンドラインインターフェース
- クロスプラットフォーム対応（Linux/macOS/Windows）

## インストール

### バイナリをダウンロード

[Releases](https://github.com/yuta-ishii-cm/dummy-file-creator/releases)から、お使いのプラットフォーム向けのバイナリをダウンロードしてください。

### ソースからビルド

```bash
cd fgen
cargo build --release
```

ビルドされたバイナリは `fgen/target/release/fgen` に生成されます。

## 使い方

### 基本的な使い方

```bash
# 100MBのダミーファイルを生成
fgen new output.dat 100MB

# 1GBのダミーファイルを生成
fgen new large.dat 1GB

# 512KBのダミーファイルを生成
fgen new small.dat 512KB

# バイト単位で指定（1024バイト）
fgen new tiny.dat 1024
```

### サイズ指定

以下の単位をサポートしています：

- `KB`: キロバイト（1024バイト）
- `MB`: メガバイト（1024KB）
- `GB`: ギガバイト（1024MB）
- 単位なし: バイト

## 開発

### 必要な環境

- Rust 1.70.0以降

### ビルド

```bash
cd fgen
cargo build
```

### テスト

```bash
cd fgen
cargo test
```

### フォーマット

```bash
cd fgen
cargo fmt
```

### Lint

```bash
cd fgen
cargo clippy
```

## CI/CD

GitHub Actionsを使用して自動的にCI/CDを実行しています：

- **CI**: push/PR時にテスト、フォーマットチェック、Clippy、ビルドを実行
- **Release**: `v*`形式のタグをpushすると、複数プラットフォーム向けのバイナリを自動ビルドしてリリース

### リリース方法

```bash
git tag v0.1.0
git push origin v0.1.0
```

タグをpushすると、自動的にGitHub Releasesにバイナリが公開されます。

## ライセンス

MIT License
