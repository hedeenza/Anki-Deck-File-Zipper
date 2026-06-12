use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file containing Target Language sentences
    #[arg(short, long)]
    target_language: String,

    /// The file containing Native Language sentences
    #[arg(short, long)]
    native_language: String,

    /// The name of the Output File
    #[arg(short, long)]
    output_file: String,
}

pub fn main() {
    // Begin Benchmarking Timer
    let program_start = Instant::now();

    // Parse command line arguments
    let args = Cli::parse();

    // Create vectors to hold the file contents
    let target_vec = Vec::with_capacity(20_000);
    let native_vec = Vec::with_capacity(20_000);

    // Read in the Language files, push their contents to their vectors
    let target_sentences = read_to_vec(args.target_language, target_vec);
    let native_sentences = read_to_vec(args.native_language, native_vec);

    // Create the Output file
    let mut outfile =
        File::create(args.output_file).expect("[ Error ] - Could not CREATE output file");

    // Zip the Target Language and Native Language sentences together
    let paired: Vec<_> = target_sentences.into_iter().zip(native_sentences).collect();

    // Write the zipped sentences to the output file
    for pairing in &paired {
        let out_line = format!("{:?}\t{:?}\n", pairing.0, pairing.1);
        let _write = writeln!(outfile, "{}", out_line);
    }

    // Finish Benchmarking Timer
    let program_duration = program_start.elapsed();
    println!(
        "{} Lines Successfuly Zipped in {:.2?}",
        &paired.len(),
        program_duration
    );
}

fn read_to_vec(file_name: String, mut language_vector: Vec<String>) -> Vec<String> {
    // Read in the Native Language file
    let file = File::open(file_name).expect("Could not read Language File");
    let reader = BufReader::new(file);
    // Push Native Language file contents to vector
    for line in reader.lines() {
        match line {
            Ok(line) => {
                language_vector.push(line);
            }
            Err(err) => {
                println!("[ ERROR ] : {}", err);
            }
        }
    }
    language_vector
}
