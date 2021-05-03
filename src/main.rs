/*
DEPENDENCIES
they are defined in Cargo.toml
they are autodownloaded when compiling
*/

// external crate
extern crate bio;

use anyhow::{Context, Result}; // for clean as fuck error reports
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;
/// libraries
use structopt::StructOpt; // https://docs.rs/structopt/0.3.21/structopt/

/*************
    rust-bio
    help: https://docs.rs/bio/0.33.0/bio/
************/
use bio::io::bed;
use bio::io::fasta;
use bio_types::strand::Strand;

/// Argpaser
#[derive(StructOpt)]
#[structopt(
    name = "seqrs - sequence repair in rust",
    about = "Quickly extract primerpairs to amplify missing/masked regions of genomes."
)]
struct Argparser {
    /// bed file containing the primer infos
    #[structopt(parse(from_os_str), short, long, required_if("articversion", "custom"))]
    primerbed: std::path::PathBuf,

    /// Fasta file input
    #[structopt(parse(from_os_str), short, long)]
    genomes: std::path::PathBuf,

    /// tab separated output
    #[structopt(short, long, default_value = "results.tsv")]
    results: String,

    /// amplicon size
    #[structopt(short, long, default_value = "1200")]
    ampliconsize: usize,
}

// Errorcodes for string
#[derive(Debug)]
struct CustomError(String);

/*
The main function
*/

fn main() -> Result<()> {
    let now = Instant::now();
    // we call the arguments from our Cli struct program above
    let args = Argparser::from_args();

    // try opening the file and read it with clean error codes
    let _filecontent = std::fs::read_to_string(&args.genomes)
        .with_context(|| format!("could not read file `{}`", &args.genomes.to_str().unwrap()))?;

    // stringbuffer
    let mut stringbuffer = String::new();

    // storing data
    let mut fileoutput = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(true)
        .open(&args.results)
        .with_context(|| format!("Could not write to the output file `{}`", &args.results))?;

    // Terminal prints for the user regarding their input
    let mut nb_reads = 0;
    let mut nb_bases = 0;

    for result in fasta::Reader::new(File::open(&args.genomes)?).records() {
        let record = result.expect("Error during fasta record parsing");
        println!("Found Fasta: {}", record.id());

        nb_reads += 1;
        nb_bases += record.seq().len();
    }
    println!("Total number of genomes found: {}", nb_reads);
    println!("Total number of bases to process: {}", nb_bases);

    // iterate over fasta records to get primers
    for result in fasta::Reader::new(File::open(&args.genomes)?).records() {
        let record = result.expect("Error during fasta record parsing");

        // for each ambigous base positions in fasta
        for (count, _v) in record.seq().iter().enumerate().filter(|&(_, c)| *c == 78) {
            // iterate through the primer bed file and find suitable fwd primer
            for recordbed in bed::Reader::new(File::open(&args.primerbed)?).records() {
                let recorddata = recordbed.expect("Error reading record.");
                // for each forward primer in bed file
                match recorddata.strand() {
                    Some(Strand::Forward) => {
                        let primerend = recorddata.end() as usize;
                        if primerend < count && count - primerend < (args.ampliconsize + 100) {
                            // find suitable matching reverse primer
                            for recordrevbed in
                                bed::Reader::new(File::open(&args.primerbed)?).records()
                            {
                                let recordrevdata = recordrevbed.expect("Error reading record.");
                                match recordrevdata.strand() {
                                    Some(Strand::Reverse) => {
                                        let primerstart = recordrevdata.start() as usize;
                                        if primerend < count
                                            && primerstart > count
                                            && primerstart - primerend < (args.ampliconsize + 100)
                                            && primerstart - primerend > (args.ampliconsize / 2)
                                            && count > 80
                                        {
                                            // store the results into a string buffer
                                            stringbuffer.push_str(record.id());
                                            stringbuffer.push_str("\t");
                                            stringbuffer.push_str(recorddata.name().unwrap());
                                            stringbuffer.push_str("\t");
                                            stringbuffer.push_str(recordrevdata.name().unwrap());
                                            stringbuffer.push_str("\n");
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    // Storing all strings into a vector to get rid of duplicates
    let l = stringbuffer;
    let l = l.split("\n");
    let mut vec: Vec<&str> = l.collect();
    vec.sort();
    vec.dedup();

    // write vector to file
    for i in vec.iter() {
        if let Err(e) = writeln!(fileoutput, "{}", i) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    let elapsed = now.elapsed();
    println!("Finished in {:.2?}", elapsed);

    Ok(())
}
