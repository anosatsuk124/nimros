use regex::Regex;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{fs, path::PathBuf};

use clap::Parser;

const BITMAP_PATTERN: &str = "([.*@]+)";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the font file
    #[arg(short, long)]
    font: PathBuf,
    /// Path to the output file
    #[arg(short, long)]
    output: PathBuf,
}

fn compile(font: String) -> Vec<u8> {
    let re = Regex::new(BITMAP_PATTERN).expect("This is a invalid pattern.");
    let font = font.trim_start();
    let mut ret: Vec<[u8; 1]> = Vec::new();

    for line in font.lines().into_iter() {
        if !re.is_match(line) {
            continue;
        }

        let bits: Vec<u8> = re
            .captures_iter(line)
            .map(|m| match m.get(1).unwrap().as_str() {
                "." => 0,
                _ => 1,
            })
            .map(|a| {
                println!("{}", a);
                a
            })
            .collect();

        let bits_int: u8 = bits.into_iter().reduce(|a, b| 2 * a + b).unwrap();

        ret.push(bits_int.to_le_bytes());
    }

    ret.concat()
}

fn main() {
    let args = Args::parse();
    let font = fs::read_to_string(args.font).expect("Cannot read the font text file.");
    let mut file = File::create(args.output).expect("Couldn't create the output file");
    {
        let mut writer = BufWriter::new(&mut file);
        let result = compile(font);
        for num in result {
            let byte = num.to_le_bytes();
            writer.write(&byte).expect("Couldn't write a byte");
        }
    }
    file.flush().expect("Couldn't flush");
}
