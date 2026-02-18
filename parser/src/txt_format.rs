use crate::error::ParserError;
use crate::transaction::{Transaction, TxStatus, TxType};
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::str::FromStr;

//# Record 1 (DEPOSIT)
// TX_TYPE: DEPOSIT
// TO_USER_ID: 9223372036854775807
// FROM_USER_ID: 0
// TIMESTAMP: 1633036860000
// DESCRIPTION: "Record number 1"
// TX_ID: 1000000000000000
// AMOUNT: 100
// STATUS: FAILURE
pub fn read_txt<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
    let mut lines = std::io::BufReader::new(reader).lines();

    let mut transaction_parts = HashMap::new();
    let mut transactions = Vec::new();
    let mut in_record = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed_line = line.trim();

        if (trimmed_line.starts_with('#')) {
            continue;
        }

        if trimmed_line.is_empty() {
            if in_record {
                in_record = false;
                transactions.push(construct_transaction(&transaction_parts)?);
                transaction_parts.clear();
            }

            continue;
        }

        if trimmed_line.contains(':') {
            if (!in_record) {
                in_record = true;
            }

            if let Some((key, value)) = trimmed_line.split_once(':') {
                if transaction_parts.contains_key(key) {
                    return Err(ParserError::DuplicatedFieldInRecord { key: key.into() });
                } else {
                    transaction_parts.insert(key.trim().into(), value.trim().into());
                }
            } else {
                return Err(ParserError::InvalidTransactionLineFormat {
                    line: trimmed_line.into(),
                });
            }
        }
    }

    if in_record {
        transactions.push(construct_transaction(&transaction_parts)?);
        transaction_parts.clear();
    }
    Ok(transactions)
}

fn construct_transaction(
    transaction_parts: &HashMap<String, String>,
) -> Result<Transaction, ParserError> {
    macro_rules! parse_tx_part {
        ($key:expr, $type:ty) => {{
            let s = transaction_parts
                .get($key)
                .ok_or_else(|| ParserError::MissingRequiredFields($key.to_string()))?;
            s.parse::<$type>()?
        }};
    }

    let tx_id = parse_tx_part!("TX_ID", u64);
    let from_user_id = parse_tx_part!("FROM_USER_ID", u64);
    let to_user_id = parse_tx_part!("TO_USER_ID", u64);
    let amount = parse_tx_part!("AMOUNT", i64);
    let timestamp = parse_tx_part!("TIMESTAMP", u64);

    let tx_type_str = transaction_parts
        .get("TX_TYPE")
        .ok_or_else(|| ParserError::MissingRequiredFields("TX_TYPE".into()))?;
    let tx_type = TxType::from_str(&tx_type_str)?;

    let status_str = transaction_parts
        .get("STATUS")
        .ok_or_else(|| ParserError::MissingRequiredFields("STATUS".into()))?;
    let status = TxStatus::from_str(status_str)?;

    let description_raw = transaction_parts
        .get("DESCRIPTION")
        .ok_or_else(|| ParserError::MissingRequiredFields("DESCRIPTION".to_string()))?;

    let description = if description_raw.starts_with('"') && description_raw.ends_with('"') {
        &description_raw[1..description_raw.len() - 1]
    } else {
        description_raw.as_str()
    };

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description: description.into(),
    })
}

pub fn write_txt<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<(), ParserError> {
    for (i, tx) in transactions.iter().enumerate() {
        if i > 0 {
            writeln!(writer)?;
        }
        writeln!(writer, "TX_ID: {}", tx.tx_id)?;
        writeln!(writer, "TX_TYPE: {}", tx.tx_type)?;
        writeln!(writer, "FROM_USER_ID: {}", tx.from_user_id)?;
        writeln!(writer, "TO_USER_ID: {}", tx.to_user_id)?;
        writeln!(writer, "AMOUNT: {}", tx.amount)?;
        writeln!(writer, "TIMESTAMP: {}", tx.timestamp)?;
        writeln!(writer, "STATUS: {}", tx.status)?;
        writeln!(writer, "DESCRIPTION: \"{}\"", tx.description)?;
    }
    Ok(())
}
