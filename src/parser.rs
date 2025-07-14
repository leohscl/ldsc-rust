use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input file
    #[arg(short, long)]
    pub input_file: PathBuf,

    /// Prefix for output
    #[arg(short, long)]
    pub output: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Munge dataset, creating a new file with munged sumstats
    Munge {
        /// Sample size, rounded to nearest integer
        #[arg(short, long)]
        sample_size: u32,

        /// List of subset of snps
        #[arg(short, long)]
        merge_alleles: Option<PathBuf>,
    },
}
