use crate::error::ParserError;
use crate::transaction::{Transaction, TxStatus, TxType};
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

// Assuming that order of fields are always the same
// That's why we're trying to parse in such way
// ```rust
//  let tx_id = fields[0].parse::<u64>()?;
// ```
pub fn read_csv<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();
    let mut transactions: Vec<Transaction> = Vec::new();

    if let Some(header_line) = lines.next() {
        let header = header_line?;
        let expected = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

        if header.trim() != expected {
            return Err(ParserError::FormatParseError(format!(
                "Header does not match: expected - {}, got - {}",
                expected, header
            )));
        }
    } else {
        return Err(ParserError::FormatParseError("Empty csv file".to_string()));
    }

    for line in lines {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(trimmed_line)?;

        if fields.len() < 7 {
            return Err(ParserError::MissingRequiredFields(format!(
                "Expected at least 7 fields, got: {}",
                fields.len()
            )));
        }

        let transaction = fields_to_transaction(fields)?;
        transactions.push(transaction);
    }
    Ok(transactions)
}

fn parse_csv_line(line: &str) -> Result<Vec<String>, ParserError> {
    let mut fields: Vec<String> = Vec::new();
    let mut chars = line.chars().peekable();
    let mut current = String::new();
    let mut in_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                fields.push(current.clone());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    fields.push(current);
    Ok(fields)
}

fn fields_to_transaction(fields: Vec<String>) -> Result<Transaction, ParserError> {
    Ok(Transaction {
        tx_id: fields[0].parse::<u64>()?,
        tx_type: TxType::from_str(&fields[1])?,
        from_user_id: fields[2].parse::<u64>()?,
        to_user_id: fields[3].parse::<u64>()?,
        amount: fields[4].parse::<i64>()?,
        timestamp: fields[5].parse::<u64>()?,
        status: TxStatus::from_str(&fields[6])?,
        description: fields[7].to_string(),
    })
}

pub fn write_csv<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<(), ParserError> {
    writeln!(
        writer,
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
    )?;

    for tx in transactions {
        let tx_type_str = tx.tx_type.to_string();
        let tx_status_str = tx.status.to_string();
        let description = format!("\"{}\"", tx.description.replace('"', "\"\""));

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            tx.tx_id,
            tx_type_str,
            tx.from_user_id,
            tx.to_user_id,
            tx.amount,
            tx.timestamp,
            tx_status_str,
            description
        )?;
    }
    writer.flush()?;
    Ok(())
}