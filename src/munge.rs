mod constants;

use std::{fs::File, path::PathBuf};
use polars::prelude::*;

pub fn read_input(input_file: PathBuf) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .map_parse_options(|parse_options| parse_options.with_separator(b'\t'))
        .try_into_reader_with_file_path(Some(input_file))
        .unwrap()
        .finish()
}

pub fn munge(input_path: &PathBuf, sample_size: u32, merge_alleles: Option<PathBuf>, output_path: &PathBuf) -> PolarsResult<()>{
    let mut data_frame = read_input(input_path.to_path_buf()).unwrap();
    println!("{}", data_frame.head(Some(3)));
    let data_frame = data_frame.rename("snpid", "SNP".into()).unwrap();
    let mut output_file = File::create(output_path).unwrap();
    CsvWriter::new(&mut output_file)
        .include_header(true)
        .with_separator(b'\t')
        .finish(data_frame)
}
