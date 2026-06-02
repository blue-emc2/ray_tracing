use std::io::{self, Write};

fn main() {
    // 画像サイズ
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // PPMヘッダ
    writeln!(out, "P3").unwrap();
    writeln!(out, "{} {}", image_width, image_height).unwrap();
    writeln!(out, "255").unwrap();

    // ピクセルデータ
    for j in 0..image_height {
        // 進捗をstderrに出す（stdoutはPPMデータ専用にする）
        eprint!("\rScanlines remaining: {:>4}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            // 0.0〜1.0に正規化した色
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0_f64;

            // 0〜255に変換
            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(out, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    eprintln!("\nDone.");
}
