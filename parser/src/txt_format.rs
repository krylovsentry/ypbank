use std::io::{Read, Write};
use crate::error::ParserError;
use crate::transaction::Transaction;

pub fn read_txt<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
    todo!()
}

pub fn write_txt<W: Write>(transactions: &[Transaction], writer: W) -> Result<(), ParserError> {
    todo!()
}