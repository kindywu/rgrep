use std::ops::Range;

use anyhow::Result;
use clap::{command, Parser};
use colored::*;
use glob::glob;
use regex::Regex;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pattern: String,

    /// Number of times to greet
    #[arg(short, long)]
    glob: String,
}

// cargo run -- --pattern he\\w+ --glob "assets/test*.txt"
// cargo run -- --pattern 好 --glob "assets/test*.txt"
#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行 rgrep ，读取正则表达式和文件
    let args = Args::parse();

    let regex = Regex::new(&args.pattern)?;

    let paths: Vec<_> = glob(&args.glob)?.filter_map(|g| g.ok()).collect();

    let mut file_no = 0;
    for path in paths {
        file_no += 1;
        println!(
            "{}: {}",
            file_no.to_string().blue(),
            path.display().to_string().blue()
        );
        let file = File::open(path).await?;

        let mut lines = BufReader::new(file).lines();

        let mut line_no = 0;
        while let Some(line) = lines.next_line().await? {
            line_no += 1;

            if let Some(m) = regex.find(&line) {
                let Range { start, end } = m.range();
                let prefix = &line[..start];
                println!(
                    "{0: >6}: {1}{2}{3}",
                    line_no.to_string().green(),
                    prefix,
                    &line[start..end].red(),
                    &line[end..]
                );
            }
        }
    }

    // 按行读取文件
    // 正则表达式匹配关键字
    // 返回匹配的行
    Ok(())
}
