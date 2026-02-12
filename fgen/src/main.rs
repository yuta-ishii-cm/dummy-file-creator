use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数を取得
    let args: Vec<String> = std::env::args().collect();

    // 第1引数（サブコマンド）で分岐
    match args.get(1).map(|s| s.as_str()) {
        Some("new") => {
            // 引数の数をチェック
            if args.len() < 4 {
                eprintln!("Usage: fgen new <output> <size>");
                std::process::exit(1);
            }

            // 出力ファイル名とサイズを取得
            let output = &args[2];
            let size = parse_size(&args[3])?;

            // ファイル生成処理
            println!("Generating {} file...", format_size(size));
            generate_file(output, size)?;
            println!("Done!");
        }
        _ => {
            // 未知のサブコマンドまたは引数なし
            eprintln!("Usage:");
            eprintln!("  fgen new <output> <size>");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn parse_size(s: &str) -> Result<u64, String> {
    // "GB" で終わる場合
    if let Some(n) = s.strip_suffix("GB") {
        n.parse::<u64>()
            .map(|v| v * 1024 * 1024 * 1024)
            .map_err(|e| format!("Invalid size: {}", e))
    // "MB" で終わる場合
    } else if let Some(n) = s.strip_suffix("MB") {
        n.parse::<u64>()
            .map(|v| v * 1024 * 1024)
            .map_err(|e| format!("Invalid size: {}", e))
    // "KB" で終わる場合
    } else if let Some(n) = s.strip_suffix("KB") {
        n.parse::<u64>()
            .map(|v| v * 1024)
            .map_err(|e| format!("Invalid size: {}", e))
    // すべて None だった場合（単位なしの数値として扱う）
    } else {
        s.parse::<u64>().map_err(|e| format!("Invalid size: {}", e))
    }
}

fn format_size(bytes: u64) -> String {
    // 各単位のバイト数を定数で定義
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    // バイト数に応じて適切な単位で表示
    if bytes >= GB {
        format!("{:.2}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn generate_file(path: &str, size: u64) -> io::Result<()> {
    // ファイルを作成
    let mut file = File::create(path)?;
    // 8KBのゼロ埋めチャンクを用意
    let chunk = vec![0u8; 8192];
    // 残りの書き込みバイト数
    let mut remaining = size;

    // 残りがなくなるまでループ
    while remaining > 0 {
        // 書き込むサイズを決定
        let write_size = std::cmp::min(remaining, chunk.len() as u64) as usize;
        // ファイルに書き込み
        file.write_all(&chunk[..write_size])?;
        // 残りを減らす
        remaining -= write_size as u64;
    }

    // 成功を返す
    Ok(())
}
