use std::io::{Read, Write};
use crate::error::ParserError;
use crate::transaction::Transaction;

pub mod error;
mod csv_format;
mod txt_format;
mod bin_format;
mod transaction;

pub enum Format {
    Csv,
    Text,
    Binary,
}

pub fn parse_format(s: &str) -> Format {
    match s {
        "csv" => Format::Csv,
        "text" | "txt" => Format::Text,
        "binary" | "bin" => Format::Binary,
        other => panic!("Unsupported format: {}", other),
    }
}

pub fn read_transactions<R: Read>(format: Format, reader: &mut R) -> Result<Vec<Transaction>, ParserError> {
    match format {
        Format::Csv => csv_format::read_csv(reader),
        Format::Text => txt_format::read_txt(reader),
        Format::Binary => bin_format::read_bin(reader),
    }
}

pub fn write_transactions<W: Write>(format: Format, writer: &mut W, transactions: &[Transaction]) -> Result<(), ParserError> {
    match format {
        Format::Csv => csv_format::write_csv(transactions, writer),
        Format::Text => txt_format::write_txt(transactions, writer),
        Format::Binary => bin_format::write_bin(transactions, writer),
    }
}
