use clap::Parser;
use parser::error::ParserError;
use parser::{parse_format, read_transactions, write_transactions};
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ConverterArgs {
    #[arg(long = "input")]
    input: String,

    /// Input format: csv, txt, or bin
    #[arg(long = "input-format")]
    input_format: String,

    /// Output format: csv, txt, or bin
    #[arg(long = "output-format")]
    output_format: String,
}

fn main() -> Result<(), ParserError> {
    let args = ConverterArgs::parse();
    let file = File::open(&args.input)?;
    let output_format = parse_format(&args.output_format);
    let input_format = parse_format(&args.input_format);

    let mut input = BufReader::new(file);

    let transactions = read_transactions(input_format, &mut input)?;
    println!("Transactions: {:?}", transactions);
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    write_transactions(output_format, &mut writer, &transactions)?;
    writer.flush().map_err(ParserError::Io)?;

    Ok(())
}
