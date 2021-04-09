/* 
DEPENDENCIES
they are defined in Cargo.toml
they are autodownloaded when compiling
*/

use structopt::StructOpt; // easy to use args via the #[derive]
use anyhow::{Context, Result}; // for clean as fuck error reports

/* 
STRUCTURES

We structure what our variable should be (strings or path)
the "#[derive(StructOpt)]" is a macro to generate code for "clap" - which parses command line arguments
we then only need to call the name of struct (Cli) to the ::from_args() function and done (see main)
*/


#[derive(StructOpt)]
struct Argparser {
    // the search pattern
    pattern: String,
    // the file path
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
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
        let filecontent = std::fs::read_to_string(&args.path)
                .with_context(|| format!("could not read file `{}`", &args.path.to_str().unwrap()))?;
        // itterate through the file
        for line in filecontent.lines() {
                if line.contains(&args.pattern) {
                        println!("{}", line);
                }
        }
        Ok(())
}
