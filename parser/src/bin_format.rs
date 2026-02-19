use crate::error::ParserError;
use crate::transaction::{Transaction, TxStatus, TxType};
use std::io::{Read, Write};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

pub fn read_bin<R: Read>(mut reader: R) -> Result<Vec<Transaction>, ParserError> {
    let mut transactions = Vec::new();

    loop {
        let mut magic = [0u8; 4];
        if let Err(e) = reader.read_exact(&mut magic) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            } else {
                return Err(ParserError::Io(e));
            }
        }

        if magic != MAGIC {
            return Err(ParserError::FormatParseError("Invalid magic".to_string()));
        }

        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf)?;
        let record_size = u32::from_be_bytes(size_buf) as usize;

        let mut body = vec![0u8; record_size];
        reader.read_exact(&mut body)?;

        let tx = parse_body(&body)?;
        transactions.push(tx);
    }

    Ok(transactions)
}

fn parse_body(body: &[u8]) -> Result<Transaction, ParserError> {
    let mut cursor = std::io::Cursor::new(body);

    let tx_id = read_u64(&mut cursor)?;
    let tx_type_byte = read_u8(&mut cursor)?;
    let tx_type = TxType::from_u8(tx_type_byte)
        .ok_or_else(|| ParserError::InvalidTransactionType(format!("Invalid byte: {}", tx_type_byte)))?;
    let from_user_id = read_u64(&mut cursor)?;
    let to_user_id = read_u64(&mut cursor)?;
    let amount = read_i64(&mut cursor)?;
    let timestamp = read_u64(&mut cursor)?;
    let status_byte = read_u8(&mut cursor)?;
    let status = TxStatus::from_u8(status_byte)
        .ok_or_else(|| ParserError::InvalidTransactionStatus(format!("Invalid byte: {}", status_byte)))?;
    let desc_len = read_u32(&mut cursor)? as usize;
    let mut desc_buf = vec![0u8; desc_len];
    cursor.read_exact(&mut desc_buf)?;
    let description = String::from_utf8(desc_buf)
        .map_err(|e| ParserError::FormatParseError(format!("Invalid UTF-8 in description: {}", e)))?
        .trim_matches('"')
        .to_string();

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

fn read_u8<R: Read>(reader: &mut R) -> Result<u8, ParserError> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}


fn read_u32<R: Read>(reader: &mut R) -> Result<u32, ParserError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_u64<R: Read>(reader: &mut R) -> Result<u64, ParserError> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_be_bytes(buf))
}

fn read_i64<R: Read>(reader: &mut R) -> Result<i64, ParserError> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}


pub fn write_bin<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<(), ParserError> {
    for tx in transactions {
        let mut body = Vec::new();

        write_u64(&mut body, tx.tx_id)?;
        write_u8(&mut body, tx.tx_type.to_u8())?;
        write_u64(&mut body, tx.from_user_id)?;
        write_u64(&mut body, tx.to_user_id)?;
        write_i64(&mut body, tx.amount)?;
        write_u64(&mut body, tx.timestamp)?;
        write_u8(&mut body, tx.status.to_u8())?;

        let desc_bytes = tx.description.as_bytes();
        write_u32(&mut body, desc_bytes.len() as u32)?;
        body.write_all(desc_bytes)?;

        writer.write_all(&MAGIC)?;
        write_u32(&mut writer, body.len() as u32)?;
        writer.write_all(&body)?;
    }
    writer.flush()?;
    Ok(())
}

fn write_u8<W: Write>(writer: &mut W, value: u8) -> Result<(), ParserError> {
    writer.write_all(&[value])?;
    Ok(())
}

fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<(), ParserError> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

fn write_u64<W: Write>(writer: &mut W, value: u64) -> Result<(), ParserError> {
    writer.write_all(&value.to_be_bytes())?;
    Ok(())
}

fn write_i64<W: Write>(writer: &mut W, value: i64) -> Result<(), ParserError> {
    writer.write_all(&value.to_be_bytes())?;
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
                description: "bin one".into(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Withdrawal,
                from_user_id: 30,
                to_user_id: 40,
                amount: -200,
                timestamp: 2,
                status: TxStatus::Failure,
                description: "second".into(),
            },
        ]
    }

    #[test]
    fn bin_roundtrip_preserves_transactions() {
        let original = sample_transactions();
        let mut buffer = Cursor::new(Vec::<u8>::new());

        write_bin(&original, &mut buffer).expect("write_bin");

        buffer.set_position(0);
        let parsed = read_bin(&mut buffer).expect("read_bin");

        assert_eq!(parsed, original);
    }

    #[test]
    fn invalid_magic_produces_format_error() {
        // Build a valid body but corrupt the magic.
        let mut buf = Vec::new();
        buf.extend_from_slice(&[0, 0, 0, 0]); // wrong magic
        buf.extend_from_slice(&1u32.to_be_bytes()); // record size (dummy)
        buf.extend_from_slice(&[0u8; 1]); // incomplete body

        let res = read_bin(&buf[..]);
        assert!(matches!(
            res,
            Err(ParserError::FormatParseError(msg)) if msg.contains("Invalid magic")
        ));
    }

    #[test]
    fn invalid_tx_type_byte_produces_error() {
        let mut body = Vec::new();
        write_u64(&mut body, 1).unwrap();
        write_u8(&mut body, 9).unwrap(); // invalid type byte
        write_u64(&mut body, 10).unwrap();
        write_u64(&mut body, 20).unwrap();
        write_i64(&mut body, 100).unwrap();
        write_u64(&mut body, 1).unwrap();
        write_u8(&mut body, TxStatus::Success.to_u8()).unwrap();
        write_u32(&mut body, 0).unwrap();

        let mut buf = Vec::new();
        buf.extend_from_slice(&MAGIC);
        write_u32(&mut buf, body.len() as u32).unwrap();
        buf.extend_from_slice(&body);

        let res = read_bin(&buf[..]);
        assert!(matches!(
            res,
            Err(ParserError::InvalidTransactionType(msg)) if msg.contains("Invalid byte")
        ));
    }

    #[test]
    fn invalid_tx_status_byte_produces_error() {
        let mut body = Vec::new();
        write_u64(&mut body, 1).unwrap();
        write_u8(&mut body, TxType::Deposit.to_u8()).unwrap();
        write_u64(&mut body, 10).unwrap();
        write_u64(&mut body, 20).unwrap();
        write_i64(&mut body, 100).unwrap();
        write_u64(&mut body, 1).unwrap();
        write_u8(&mut body, 9).unwrap(); // invalid status byte
        write_u32(&mut body, 0).unwrap();

        let mut buf = Vec::new();
        buf.extend_from_slice(&MAGIC);
        write_u32(&mut buf, body.len() as u32).unwrap();
        buf.extend_from_slice(&body);

        let res = read_bin(&buf[..]);
        assert!(matches!(
            res,
            Err(ParserError::InvalidTransactionStatus(msg)) if msg.contains("Invalid byte")
        ));
    }
}