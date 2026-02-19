//! Типы ошибок, которые может возвращать крейт `parser`.
//!
//! Все функции парсера и сериализации используют [`ParserError`] в качестве
//! единого типа ошибки.

use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::num::ParseIntError;

/// Ошибка парсинга или сериализации транзакций.
#[derive(Debug)]
pub enum ParserError {
    /// Ошибка ввода‑вывода при чтении или записи данных.
    Io(std::io::Error),
    /// Ошибка формата файла (например, неверный заголовок или MAGIC).
    FormatParseError(String),
    /// Некорректный статус транзакции.
    InvalidTransactionStatus(String),
    /// Некорректный тип транзакции.
    InvalidTransactionType(String),
    /// Отсутствуют обязательные поля в записи транзакции.
    MissingRequiredFields(String),
    /// Ошибка при разборе целочисленного значения.
    ParseIntError(ParseIntError),
    /// Некорректная строка при парсинге текстового формата транзакций.
    InvalidTransactionLineFormat {
        /// Исходное содержимое строки.
        line: String,
    },
    /// Дублирующееся поле в описании транзакции.
    DuplicatedFieldInRecord {
        /// Имя поля, которое встретилось повторно.
        key: String,
    },
    /// Ошибка декодирования строки в UTF‑8.
    Utf8Error(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Io(err) => write!(f, "IO error: {}", err),
            ParserError::FormatParseError(err) => write!(f, "Format parsing error: {}", err),
            ParserError::InvalidTransactionStatus(err) => {
                write!(f, "Invalid transaction state: {}", err)
            }
            ParserError::InvalidTransactionType(err) => {
                write!(f, "Invalid transaction type: {}", err)
            }
            ParserError::MissingRequiredFields(err) => {
                write!(f, "Missing required transaction fields: {}", err)
            }
            ParserError::ParseIntError(err) => {
                write!(f, "Error parsing integer: {}", err)
            }
            ParserError::InvalidTransactionLineFormat { line } => {
                write!(f, "Invalid transaction line format: {}", line)
            }
            ParserError::DuplicatedFieldInRecord { key } => {
                write!(f, "Duplicate field '{}' in record", key)
            }
            ParserError::Utf8Error(err) => {
                write!(f, "Error parsing UTF-8: {}", err)
            }
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<std::string::FromUtf8Error> for ParserError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ParserError::Utf8Error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io_error_contains_message() {
        let io_err = std::io::Error::other("boom");
        let err = ParserError::Io(io_err);
        let msg = err.to_string();
        assert!(msg.contains("IO error:"));
        assert!(msg.contains("boom"));
    }

    #[test]
    fn display_format_parse_error_contains_message() {
        let err = ParserError::FormatParseError("bad header".into());
        let msg = err.to_string();
        assert!(msg.contains("Format parsing error:"));
        assert!(msg.contains("bad header"));
    }

    #[test]
    fn display_invalid_transaction_status_and_type_contain_values() {
        let status_err = ParserError::InvalidTransactionStatus("X".into());
        let type_err = ParserError::InvalidTransactionType("Y".into());

        assert!(
            status_err
                .to_string()
                .contains("Invalid transaction state: X")
        );
        assert!(type_err.to_string().contains("Invalid transaction type: Y"));
    }

    #[test]
    fn display_missing_required_fields_contains_field_name() {
        let err = ParserError::MissingRequiredFields("TX_ID".into());
        let msg = err.to_string();
        assert!(msg.contains("Missing required transaction fields: TX_ID"));
    }

    #[test]
    fn display_invalid_transaction_line_format_contains_line() {
        let err = ParserError::InvalidTransactionLineFormat {
            line: "oops".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Invalid transaction line format: oops"));
    }

    #[test]
    fn display_duplicated_field_in_record_contains_key() {
        let err = ParserError::DuplicatedFieldInRecord {
            key: "STATUS".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Duplicate field 'STATUS' in record"));
    }

    #[test]
    fn display_utf8_error_contains_message() {
        let err = ParserError::Utf8Error("broken utf8".into());
        let msg = err.to_string();
        assert!(msg.contains("Error parsing UTF-8: broken utf8"));
    }

    #[test]
    fn from_io_error_creates_io_variant() {
        let io_err = std::io::Error::other("io");
        let err: ParserError = io_err.into();
        match err {
            ParserError::Io(e) => assert_eq!(e.to_string(), "io"),
            other => panic!("Unexpected variant: {:?}", other),
        }
    }

    #[test]
    fn from_parse_int_error_creates_parse_int_variant() {
        let parse_err = "abc".parse::<u32>().unwrap_err();
        let err: ParserError = parse_err.into();
        match err {
            ParserError::ParseIntError(_) => {}
            other => panic!("Unexpected variant: {:?}", other),
        }
    }

    #[test]
    fn from_utf8_error_creates_utf8_variant() {
        let bytes = vec![0xff, 0xfe];
        let utf8_err = String::from_utf8(bytes).unwrap_err();
        let err: ParserError = utf8_err.into();
        match err {
            ParserError::Utf8Error(msg) => {
                assert!(msg.contains("invalid utf-8 sequence"));
            }
            other => panic!("Unexpected variant: {:?}", other),
        }
    }
}
