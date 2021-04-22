/* 
DEPENDENCIES
they are defined in Cargo.toml
they are autodownloaded when compiling
*/

// external crate
extern crate bio;

/// libraries
use structopt::StructOpt; // https://docs.rs/structopt/0.3.21/structopt/
use anyhow::{Context, Result}; // for clean as fuck error reports

// std stuff
use std::fs::File;
//use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// rust-bio
use bio::io::fasta;
use bio::io::bed;

use bio_types::strand::{Strand};



#[derive(StructOpt)]
#[structopt(name = "seqrs - sequence repair in rust", about = "Quickly extract primerpairs to amplify missing/masked regions of genomes.")]
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

/*
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
*/
}

// Errorcodes for string
#[derive(Debug)]
struct CustomError(String);


/*
The main function 
*/

fn main() -> Result<()> {

    // we call the arguments from our Cli struct program above 
    let args = Argparser::from_args();

    // try opening the file and read it with clean error codes
    let _filecontent = std::fs::read_to_string(&args.genomes)
        .with_context(|| format!("could not read file `{}`", &args.genomes.to_str().unwrap()))?;

    /*************
        rust-bio
        help: https://docs.rs/bio/0.33.0/bio/


        Readers (bed::Reader) has the function .records() to iterate over records
        and the function .new() to create from new input
    ************/

    // open the fasta and bed file
    //let mut reader = fasta::Reader::new(File::open(&args.genomes)?);
    let mut bedfile = bed::Reader::new(File::open(&args.primerbed)?);

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
    println!("Total number of basepairs to process: {}", nb_bases);

    // iterate over fasta records

    for recordbed in bedfile.records() {
        let recorddata = recordbed.expect("Error reading record.");

        match recorddata.strand() {
            // Forward Strand Primer
            Some(Strand::Forward) => { 
                // we feed the full fasta file records into the loop 
                for result in fasta::Reader::new(File::open(&args.genomes)?).records() {
                    let record = result.expect("Error during fasta record parsing");

                    // development help
                    /* println!("Proccessing fasta: {} with length {}", record.id(), record.seq().len());*/

                    // we get every "N" position now and then we check for primers in the range of 1200bp
                    for (count, _v) in record.seq().iter().enumerate().filter(|&(_, c)| *c == 78) {
                        // converting the data to usize to allow comparision
                        let primerend = recorddata.end() as usize;
                        // we search now for appropriate forward primers
                        if primerend < count && count - primerend < args.ampliconsize && count > 100 {

                            // store the results into a strings buffer
                            stringbuffer.push_str(record.id());
                            stringbuffer.push_str("\t");
                            stringbuffer.push_str(recorddata.name().unwrap());
                            stringbuffer.push_str("\n");

                            // development help
                            /* println!("## Fasta:{} has 'N' at position {} is greater than {} of FPrimer {:?}", record.id(), count, recorddata.end(), recorddata.name()); */
                        }
                    }
                }
            },
                    // Reverse Strand Primer
            Some(Strand::Reverse) => { 
                // we feed the full fasta file records into the loop 
                for result in fasta::Reader::new(File::open(&args.genomes)?).records() {
                    let record = result.expect("Error during fasta record parsing");

                    // development help
                    /* println!("Proccessing fasta: {} with length {}", record.id(), record.seq().len());*/

                    // we get every "N" position now and then we check for primers in the range of 1200bp
                    for (count, _v) in record.seq().iter().enumerate().filter(|&(_, c)| *c == 78) {
                        // converting the data to usize to allow comparision
                        let primerstart = recorddata.start() as usize;
                        // we search now for appropriate forward primers
                        // missing for the if statment : count < (fastalength - 100)
                        if primerstart > count && primerstart - count < args.ampliconsize && count > 100 {

                            // store the results into a strings buffer
                            stringbuffer.push_str(record.id());
                            stringbuffer.push_str("\t");
                            stringbuffer.push_str(recorddata.name().unwrap());
                            stringbuffer.push_str("\n");

                            // development help
                            /* println!("## Fasta:{} has 'N' at position {} is greater than {} of FPrimer {:?}", record.id(), count, recorddata.start(), recorddata.name()); */
                        }
                    }
                }
            },

            Some(Strand::Unknown) => println!("{:?}", recorddata.name()),
            _ => continue,
            }
    }

    // Storing all strings into a vector to get rid of duplicates
    let l = stringbuffer;
    let l = l.split("\n");
    let mut vec: Vec<&str> = l.collect();
    vec.sort() ;
    vec.dedup() ;
    //vec.remove(0) ;
    //println!("{:?}", vec);

    // write vector to file
    for i in vec.iter() {
            if let Err(e) = writeln!(fileoutput, "{}", i) {
            eprintln!("Couldn't write to file: {}", e);
            }                                                                                                                           
        }  
        
    Ok(())
}

// rust loops:
// https://medium.com/qvault/loops-in-rust-breaking-from-nested-loops-26ab508fdce2