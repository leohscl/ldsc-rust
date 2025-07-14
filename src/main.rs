use clap::Parser;
use ldsc_rust::parser::Cli;
use ldsc_rust::parser::Commands;
use ldsc_rust::munge::munge;

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let input = cli.input_file;
    let output = cli.output;
    dbg!(&input);
    dbg!(&output);
    match cli.command {
        Commands::Munge { sample_size, merge_alleles } => {
            munge(&input, sample_size, merge_alleles, &output).unwrap()
        },
    }
}
