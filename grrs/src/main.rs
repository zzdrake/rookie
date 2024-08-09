use clap::Parser;
use anyhow::{Context, Result};

// 将要在文件中找的字段以行的形式展示
#[derive(Parser)]
struct Cli {
    // 要查找的字段
    pattern: String,
    // 文件路径
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    /* 使用std */

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    /* 使用clap */
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
