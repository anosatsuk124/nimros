use std::{path::PathBuf, fs};
use regex::Regex;
use std::fs::File;
use std::io::{Write, BufWriter};

use clap::Parser;

const BITMAP_PATTERN: &str = "[.*@]+";

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
    let mut ret: Vec<[u8; 8]> = Vec::new();

    for line in font.lines().into_iter() {
        if !re.is_match(line) {
            continue;
        }

        let bits = re.find_iter(line).map(|x| {
            if x.as_str() == "." {
                0
            } else {
                1
            }
        });

        let bits_int: usize = bits.into_iter().fold(0, |a, b| 2 * a + b);

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
