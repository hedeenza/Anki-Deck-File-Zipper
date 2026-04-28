use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    target_language: String,
    base_language: String,
    output_file: std::path::PathBuf,
}

pub fn main() {
    let program_start = Instant::now(); // Begin Benchmarking Timer
    let args = Cli::parse();
    let content_target = std::fs::read_to_string(&args.target_language)
        .expect("Could not read Target Language File (1)");
    let content_base = std::fs::read_to_string(&args.base_language)
        .expect("Could not read Base Language File (2)");
    let output = std::env::args()
        .nth(3)
        .expect("No output file provided (3)");

    let mut content_vec = Vec::with_capacity(10_000);
    let mut base_vec = Vec::with_capacity(10_000);

    for line in content_target.lines() {
        content_vec.push(line);
    }

    for line in content_base.lines() {
        base_vec.push(line);
    }

    println!("{}", output);
    let mut outfile = File::create(output).expect("[ Error ] - Could not CREATE output file");

    let paired: Vec<_> = content_vec.into_iter().zip(base_vec).collect();
    for pairing in &paired {
        let out_lines = format!("{:?}\t{:?}\n", pairing.0, pairing.1);
        outfile
            .write_all(out_lines.as_bytes())
            .expect("[ Error ] - Could not WRITE output file");
    }

    let program_duration = program_start.elapsed(); // Finish Benchmarking Timer
    println!(
        "{} Lines Successfuly Zipped in {:.2?}",
        &paired.len(),
        program_duration
    );
}
