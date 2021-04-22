# SEQuence Repair via ruSt
* if you are doing nanopore amplicon sequencing vor SARS-CoV-2 read on, else leave this git :)
* This small parser/tool aims to tell you which minimum set of primer would be need to repair a genome
* the main goal was to learn rust a bit so this git is super basic

## Installation
* when iam finished it will be a prober executable currently it can be run via rusts cargo
* install rust - done

```bash
# install a gcc compiler if its not available
sudo apt install build-essential
# install and go with 1) when prompted
curl https://sh.rustup.rs -sSf | sh
# refresh $PATH or restart terminal
. ~/.profile
# clone the git
git clone https://github.com/replikation/seqrs.git
```     

## run

````bash
# navigate into the git
cd seqrs/
# execute via cargo
cargo run -- --genomes data/multifasta_v1200.fasta --primerbed data/Primerfiles/V1200/nCoV-2019.bed --results results.txt
````

## results
you get a tsv file with `fastaheader primername`

## help

`cargo run -- --help`

````bash
seqrs - sequence repair in rust 0.2.0
Quickly extract primerpairs to amplify missing/masked regions of genomes.

USAGE:
    seqrs [OPTIONS] --genomes <genomes> --primerbed <primerbed>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --ampliconsize <ampliconsize>    amplicon size [default: 1200]
    -g, --genomes <genomes>              Fasta file input
    -p, --primerbed <primerbed>          bed file containing the primer infos
    -r, --results <results>              tab separated output [default: results.tsv]
````
