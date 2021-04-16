/* 
DEPENDENCIES
they are defined in Cargo.toml
they are autodownloaded when compiling
*/

// external crate
extern crate bio;

/// libraries
use structopt::StructOpt; // easy to use args via the #[derive]
use anyhow::{Context, Result}; // for clean as fuck error reports
//use std::path::PathBuf;
//use std::io::{self, BufReader};
use std::fs::File;

/// rust-bio
use bio::io::fasta;
use bio::io::bed;
// use crate::bed::strand;
// use bio_types::strand;

use bio_types::strand::{Strand};

/* 
STRUCTs

We structure what our variable should be (strings or path)
the "#[derive(StructOpt)]" is a macro to generate code for "clap" - which parses command line arguments
we then only need to call the name of struct (Cli) to the ::from_args() function and done (see main)
*/


#[derive(StructOpt)]
#[structopt(name = "seqrs - sequence repair in rust", about = "Quickly extract primerpairs to amplify missing/masked regions of genomes.")]
struct Argparser {
    /// String input for the primer version
    #[structopt(short, long, default_value = "custom")]
    articversion: String,
    /// File input of the primer bed file
    #[structopt(parse(from_os_str), short, long, required_if("articversion", "custom"))]
    primerbed: std::path::PathBuf,
    /// File input for genomes
    #[structopt(parse(from_os_str), short, long)]
    genomes: std::path::PathBuf,

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

        // opening the file and read it with clean error codes
        let filecontent = std::fs::read_to_string(&args.genomes)
                .with_context(|| format!("could not read file `{}`", &args.genomes.to_str().unwrap()))?;


        // itterate through the file
        for line in filecontent.lines() {
                if line.contains(&args.articversion) {
                        println!("{}", line);
                }
        }

    
        /*************
          rust-bio
          help: https://docs.rs/bio/0.33.0/bio/


        Readers (bed::Reader) has the function .records() to iterate over records
        and the function .new() to create from new input
        ************/

        // open the fasta and bed file
        let mut fastarecords = fasta::Reader::new(File::open(&args.genomes)?).records();
        let mut bedfile = bed::Reader::new(File::open(&args.primerbed)?);

        // for statistics
        let mut nb_reads = 0;
        let mut nb_bases = 0;

        // iterate over fasta records
        while let Some(Ok(record)) = fastarecords.next() {
            nb_reads += 1;
            nb_bases += record.seq().len();

            // We search for every "N" (index 78) within each record.seq()
            for (_count, _v) in record.seq().iter().enumerate().filter(|&(_, c)| *c == 78) {
            
            //println!("{}: {}", record.id(), count);

   
                // I NEED HERE NOW TO FIND THE MINIMUM OF ALL FORWARDS STRANDS
                // makes sense to work here with a data frame like https://docs.rs/polars/0.12.1/polars/
                // or https://github.com/nevi-me/rust-dataframe
                // but check if i can somehow work / math with them in a good way
   
   
            }
        }
        
        // Terminal prints for the user regarding their input
        println!("Number of genomes: {}", nb_reads);
        println!("Total number of bases: {}", nb_bases);

        for record in bedfile.records() {
            
            // unwraping the record, all fields are here: https://docs.rs/bio/0.33.0/bio/io/bed/struct.Record.html
            // using .expect instead of .unwrap to include error code
            let recorddata = record.expect("Error reading record.");
            


            match recorddata.strand() {
                Some(Strand::Forward) => println!("{:?} is {:?} with end {}", recorddata.name(), recorddata.strand(), recorddata.end()),
                Some(Strand::Reverse) => println!("{:?} is {:?} with start {}", recorddata.name(), recorddata.strand(), recorddata.start()),
                Some(Strand::Unknown) => println!("{:?}", recorddata.name()),
                _ => println!("no value"),
                

            }
            //writer.write(&rec).expect("Error writing record.");
            };     
        
        

        Ok(())
}

