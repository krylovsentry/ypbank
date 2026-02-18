use std::error::Error;
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
        .map_err(|e| ParserError::FormatParseError(format!("Invalid UTF-8 in description: {}", e)))?;

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

fn read_u16<R: Read>(reader: &mut R) -> Result<u16, ParserError> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
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

fn write_u16<W: Write>(writer: &mut W, value: u16) -> Result<(), ParserError> {
    writer.write_all(&value.to_be_bytes())?;
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