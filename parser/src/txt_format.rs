use crate::error::ParserError;
use crate::transaction::{Transaction, TxStatus, TxType};
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::str::FromStr;

pub fn read_txt<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
    let mut lines = std::io::BufReader::new(reader).lines();

    let mut transaction_parts = HashMap::new();
    let mut transactions = Vec::new();
    let mut in_record = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('#') {
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
            if !in_record {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_transactions() -> Vec<Transaction> {
        vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 10,
                to_user_id: 20,
                amount: 100,
                timestamp: 1,
                status: TxStatus::Success,
                description: "desc1".into(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Withdrawal,
                from_user_id: 30,
                to_user_id: 40,
                amount: -200,
                timestamp: 2,
                status: TxStatus::Failure,
                description: "with spaces and \"quotes\"".into(),
            },
        ]
    }

    #[test]
    fn txt_roundtrip_preserves_transactions() {
        let original = sample_transactions();
        let mut buffer = Cursor::new(Vec::<u8>::new());

        write_txt(&original, &mut buffer).expect("write_txt");

        buffer.set_position(0);
        let parsed = read_txt(&mut buffer).expect("read_txt");

        assert_eq!(parsed, original);
    }

    #[test]
    fn txt_reader_ignores_comments_and_blank_lines() {
        let data = b"# comment 1\n\
\n\
TX_ID: 1\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 10\n\
TO_USER_ID: 20\n\
AMOUNT: 100\n\
TIMESTAMP: 1\n\
STATUS: SUCCESS\n\
DESCRIPTION: \"desc\"\n\
\n\
# another comment\n";

        let mut cursor = Cursor::new(&data[..]);
        let txs = read_txt(&mut cursor).expect("read_txt");
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].tx_id, 1);
    }

    #[test]
    fn duplicated_field_produces_error() {
        let data = b"TX_ID: 1\nTX_ID: 2\n";
        let mut cursor = Cursor::new(&data[..]);
        let res = read_txt(&mut cursor);
        assert!(matches!(
            res,
            Err(ParserError::DuplicatedFieldInRecord { key }) if key == "TX_ID"
        ));
    }

    #[test]
    fn line_without_colon_is_ignored_and_produces_no_transactions() {
        let data = b"TX_ID 1\n";
        let mut cursor = Cursor::new(&data[..]);
        let res = read_txt(&mut cursor).expect("read_txt");
        // Текущее поведение: строки без двоеточия игнорируются полностью.
        assert!(res.is_empty());
    }

    #[test]
    fn missing_required_field_produces_error() {
        let data = b"TX_ID: 1\nFROM_USER_ID: 1\n";
        let mut cursor = Cursor::new(&data[..]);
        let res = read_txt(&mut cursor);
        assert!(matches!(res, Err(ParserError::MissingRequiredFields(_))));
    }
}
