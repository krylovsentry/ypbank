//! Библиотека для чтения и записи списка транзакций YPBank
//! в различных файловых форматах (CSV, текстовый, бинарный).
//!
//! Основной сценарий использования:
//! 1. Определить формат входного файла с помощью [`parse_format`].
//! 2. Считать транзакции с помощью [`read_transactions`].
//! 3. При необходимости сохранить их в другом формате через [`write_transactions`].

use std::io::{Read, Write};
use crate::error::ParserError;
use crate::transaction::Transaction;

pub mod error;
mod csv_format;
mod txt_format;
mod bin_format;
mod transaction;

pub enum Format {
    /// CSV-файл с заголовком и строками записей.
    Csv,
    /// Человекочитаемый текстовый формат согласно спецификации YPBankTextFormat.
    Text,
    /// Компактный бинарный формат согласно спецификации YPBankBinFormat.
    Binary,
}

/// Преобразует строковое представление формата в [`Format`].
///
/// Допустимые значения:
/// - `"csv"`
/// - `"text"` или `"txt"`
/// - `"binary"` или `"bin"`
///
/// # Panics
///
/// Паникует, если передана строка с неподдерживаемым форматом.
pub fn parse_format(s: &str) -> Format {
    match s {
        "csv" => Format::Csv,
        "text" | "txt" => Format::Text,
        "binary" | "bin" => Format::Binary,
        other => panic!("Unsupported format: {}", other),
    }
}

/// Считывает список транзакций из потока в указанном формате.
///
/// В случае ошибок парсинга или ввода-вывода возвращает [`ParserError`].
pub fn read_transactions<R: Read>(format: Format, reader: &mut R) -> Result<Vec<Transaction>, ParserError> {
    match format {
        Format::Csv => csv_format::read_csv(reader),
        Format::Text => txt_format::read_txt(reader),
        Format::Binary => bin_format::read_bin(reader),
    }
}

/// Записывает список транзакций в поток в указанном формате.
///
/// В случае ошибок сериализации или вывода возвращает [`ParserError`].
pub fn write_transactions<W: Write>(format: Format, writer: &mut W, transactions: &[Transaction]) -> Result<(), ParserError> {
    match format {
        Format::Csv => csv_format::write_csv(transactions, writer),
        Format::Text => txt_format::write_txt(transactions, writer),
        Format::Binary => bin_format::write_bin(transactions, writer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample_transactions() -> Vec<Transaction> {
        vec![
            Transaction {
                tx_id: 1,
                tx_type: crate::transaction::TxType::Deposit,
                from_user_id: 100,
                to_user_id: 200,
                amount: 5000,
                timestamp: 1_700_000_000,
                status: crate::transaction::TxStatus::Success,
                description: "Test deposit".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: crate::transaction::TxType::Transfer,
                from_user_id: 200,
                to_user_id: 300,
                amount: -2500,
                timestamp: 1_700_000_100,
                status: crate::transaction::TxStatus::Pending,
                description: "Transfer, \"quoted\" description".to_string(),
            },
        ]
    }

    #[test]
    fn parse_format_supports_all_known_variants() {
        assert!(matches!(parse_format("csv"), Format::Csv));
        assert!(matches!(parse_format("text"), Format::Text));
        assert!(matches!(parse_format("txt"), Format::Text));
        assert!(matches!(parse_format("binary"), Format::Binary));
        assert!(matches!(parse_format("bin"), Format::Binary));
    }

    #[test]
    #[should_panic(expected = "Unsupported format")]
    fn parse_format_panics_on_unsupported_value() {
        let _ = parse_format("xml");
    }

    #[test]
    fn csv_roundtrip_preserves_transactions() {
        let original = sample_transactions();
        let mut buffer = Cursor::new(Vec::<u8>::new());

        write_transactions(Format::Csv, &mut buffer, &original).expect("write csv");

        buffer.set_position(0);
        let parsed =
            read_transactions(Format::Csv, &mut buffer).expect("read csv");

        assert_eq!(parsed, original);
    }

    #[test]
    fn text_roundtrip_preserves_transactions() {
        let original = sample_transactions();
        let mut buffer = Cursor::new(Vec::<u8>::new());

        write_transactions(Format::Text, &mut buffer, &original).expect("write txt");

        buffer.set_position(0);
        let parsed =
            read_transactions(Format::Text, &mut buffer).expect("read txt");

        assert_eq!(parsed, original);
    }

    #[test]
    fn binary_roundtrip_preserves_transactions() {
        let original = sample_transactions();
        let mut buffer = Cursor::new(Vec::<u8>::new());

        write_transactions(Format::Binary, &mut buffer, &original).expect("write bin");

        buffer.set_position(0);
        let parsed =
            read_transactions(Format::Binary, &mut buffer).expect("read bin");

        assert_eq!(parsed, original);
    }
}
