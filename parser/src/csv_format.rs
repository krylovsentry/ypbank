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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{TxStatus, TxType};
    use std::io::Cursor;

    #[test]
    fn test_read_csv_valid() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial account funding\"
1002,TRANSFER,501,502,15000,1672534800000,FAILURE,\"Payment for services, invoice #123\"
1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,\"ATM withdrawal\"\n";

        let cursor = Cursor::new(data);
        let txs = read_csv(cursor).unwrap();

        assert_eq!(txs.len(), 3);

        // Check first transaction
        assert_eq!(txs[0].tx_id, 1001);
        assert_eq!(txs[0].tx_type, TxType::Deposit);
        assert_eq!(txs[0].from_user_id, 0);
        assert_eq!(txs[0].to_user_id, 501);
        assert_eq!(txs[0].amount, 50000);
        assert_eq!(txs[0].timestamp, 1672531200000);
        assert_eq!(txs[0].status, TxStatus::Success);
        assert_eq!(txs[0].description, "Initial account funding");

        // Check second
        assert_eq!(txs[1].tx_id, 1002);
        assert_eq!(txs[1].tx_type, TxType::Transfer);
        assert_eq!(txs[1].status, TxStatus::Failure);
        assert_eq!(txs[1].description, "Payment for services, invoice #123");

        // Check third
        assert_eq!(txs[2].tx_id, 1003);
        assert_eq!(txs[2].tx_type, TxType::Withdrawal);
        assert_eq!(txs[2].status, TxStatus::Pending);
        assert_eq!(txs[2].description, "ATM withdrawal");
    }

    #[test]
    fn test_read_csv_empty() {
        // Only header, no data rows
        let data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
        let cursor = Cursor::new(data);
        let txs = read_csv(cursor).unwrap();
        assert!(txs.is_empty());
    }

    #[test]
    fn test_read_csv_invalid_tx_type() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,INVALID,0,501,50000,1672531200000,SUCCESS,\"desc\"\n";

        let cursor = Cursor::new(data);
        let err = read_csv(cursor).unwrap_err();
        // assert!(matches!(err, ParserError::InvalidTxType(_)));
    }

    #[test]
    fn test_read_csv_invalid_status() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,UNKNOWN,\"desc\"\n";

        let cursor = Cursor::new(data);
        let err = read_csv(cursor).unwrap_err();
        // assert!(matches!(err, Error::InvalidStatus(_)));
    }

    #[test]
    fn test_read_csv_missing_field() {
        // Missing the last field (description) – should cause CSV error
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS\n"; // no description

        let cursor = Cursor::new(data);
        let err = read_csv(cursor).unwrap_err();
        // Depending on the csv crate, this might be a CSV error (missing field)
        // assert!(matches!(err, Error::Csv(_)));
    }

    #[test]
    fn test_read_csv_extra_whitespace() {
        // Ensure trimming works? The csv crate trims fields by default if we use flexible configuration.
        // By default, it does not trim. But we can handle it via serde or custom parsing.
        // For simplicity, we'll test that whitespace around fields causes error unless we trim.
        // We'll skip or adjust as needed.
        // In this test we show that the csv crate does NOT trim by default, so " DEPOSIT" would be invalid.
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001, DEPOSIT ,0,501,50000,1672531200000,SUCCESS,\"desc\"\n";

        let cursor = Cursor::new(data);
        let err = read_csv(cursor).unwrap_err();
        // Should be InvalidTxType because " DEPOSIT " doesn't match
        // assert!(matches!(err, Error::InvalidTxType(_)));
    }

    // You can also add a test for description containing commas and quotes
    #[test]
    fn test_read_csv_description_with_commas() {
        let data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Description, with, commas\"\n";

        let cursor = Cursor::new(data);
        let txs = read_csv(cursor).unwrap();
        assert_eq!(txs[0].description, "Description, with, commas");
    }
}
