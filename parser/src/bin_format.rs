use std::io::{Read, Write};
use crate::error::ParserError;
use crate::transaction::Transaction;

pub fn read_bin<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
    todo!()
}

pub fn write_bin<W: Write>(transactions: &[Transaction], writer: W) -> Result<(), ParserError> {
    todo!()
}