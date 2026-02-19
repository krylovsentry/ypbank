//! Модели транзакций YPBank и служебные типы.

use crate::error::ParserError;
use std::fmt::Display;
use std::str::FromStr;

/// Тип банковской транзакции.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxType {
    /// Пополнение счёта.
    Deposit,
    /// Перевод между пользователями.
    Transfer,
    /// Списание средств.
    Withdrawal,
}

/// Статус выполнения транзакции.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxStatus {
    /// Транзакция успешно выполнена.
    Success,
    /// Транзакция завершилась ошибкой.
    Failure,
    /// Транзакция ещё находится в обработке.
    Pending,
}

/// Одна банковская транзакция в формате YPBank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции.
    pub tx_id: u64,
    /// Тип транзакции.
    pub tx_type: TxType,
    /// Идентификатор пользователя‑отправителя.
    pub from_user_id: u64,
    /// Идентификатор пользователя‑получателя.
    pub to_user_id: u64,
    /// Сумма транзакции в минимальных единицах валюты (может быть отрицательной).
    pub amount: i64,
    /// Момент времени совершения транзакции (обычно Unix‑timestamp).
    pub timestamp: u64,
    /// Статус обработки транзакции.
    pub status: TxStatus,
    /// Человекочитаемое описание операции.
    pub description: String,
}

impl FromStr for TxType {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            other => Err(ParserError::InvalidTransactionType(other.to_string())),
        }
    }
}

impl Display for TxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TxType::Deposit => "DEPOSIT".to_string(),
            TxType::Transfer => "TRANSFER".to_string(),
            TxType::Withdrawal => "WITHDRAWAL".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Display for TxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxStatus::Success => write!(f, "SUCCESS"),
            TxStatus::Failure => write!(f, "FAILURE"),
            TxStatus::Pending => write!(f, "PENDING"),
        }
    }
}

impl FromStr for TxStatus {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TxStatus::Success),
            "FAILURE" => Ok(TxStatus::Failure),
            "PENDING" => Ok(TxStatus::Pending),
            other => Err(ParserError::InvalidTransactionStatus(other.to_string())),
        }
    }
}

impl TxType {
    /// Восстанавливает тип транзакции по кодовому байту.
    ///
    /// Возвращает `None`, если передано некорректное значение.
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(TxType::Deposit),
            1 => Some(TxType::Transfer),
            2 => Some(TxType::Withdrawal),
            _ => None,
        }
    }

    /// Кодирует тип транзакции в однобайтовое представление.
    pub fn to_u8(&self) -> u8 {
        match self {
            TxType::Deposit => 0,
            TxType::Transfer => 1,
            TxType::Withdrawal => 2,
        }
    }
}

impl TxStatus {
    /// Восстанавливает статус транзакции по кодовому байту.
    ///
    /// Возвращает `None`, если передано некорректное значение.
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(TxStatus::Success),
            1 => Some(TxStatus::Failure),
            2 => Some(TxStatus::Pending),
            _ => None,
        }
    }

    /// Кодирует статус транзакции в однобайтовое представление.
    pub fn to_u8(&self) -> u8 {
        match self {
            TxStatus::Success => 0,
            TxStatus::Failure => 1,
            TxStatus::Pending => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn tx_type_from_str_parses_valid_values() {
        assert_eq!(TxType::from_str("DEPOSIT").unwrap(), TxType::Deposit);
        assert_eq!(TxType::from_str("TRANSFER").unwrap(), TxType::Transfer);
        assert_eq!(TxType::from_str("WITHDRAWAL").unwrap(), TxType::Withdrawal);
    }

    #[test]
    fn tx_type_from_str_returns_error_on_invalid_value() {
        let err = TxType::from_str("UNKNOWN").unwrap_err();
        match err {
            ParserError::InvalidTransactionType(s) => {
                assert_eq!(s, "UNKNOWN");
            }
            other => panic!("Unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn tx_status_from_str_parses_valid_values() {
        assert_eq!(TxStatus::from_str("SUCCESS").unwrap(), TxStatus::Success);
        assert_eq!(TxStatus::from_str("FAILURE").unwrap(), TxStatus::Failure);
        assert_eq!(TxStatus::from_str("PENDING").unwrap(), TxStatus::Pending);
    }

    #[test]
    fn tx_status_from_str_returns_error_on_invalid_value() {
        let err = TxStatus::from_str("BROKEN").unwrap_err();
        match err {
            ParserError::InvalidTransactionStatus(s) => {
                assert_eq!(s, "BROKEN");
            }
            other => panic!("Unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn tx_type_display_matches_string_representation() {
        assert_eq!(TxType::Deposit.to_string(), "DEPOSIT");
        assert_eq!(TxType::Transfer.to_string(), "TRANSFER");
        assert_eq!(TxType::Withdrawal.to_string(), "WITHDRAWAL");
    }

    #[test]
    fn tx_status_display_matches_string_representation() {
        assert_eq!(TxStatus::Success.to_string(), "SUCCESS");
        assert_eq!(TxStatus::Failure.to_string(), "FAILURE");
        assert_eq!(TxStatus::Pending.to_string(), "PENDING");
    }

    #[test]
    fn tx_type_u8_roundtrip_is_consistent() {
        for ty in [TxType::Deposit, TxType::Transfer, TxType::Withdrawal] {
            let byte = ty.to_u8();
            let parsed = TxType::from_u8(byte).expect("valid byte");
            assert_eq!(parsed, ty);
        }
    }

    #[test]
    fn tx_type_from_u8_invalid_values_return_none() {
        assert!(TxType::from_u8(3).is_none());
        assert!(TxType::from_u8(255).is_none());
    }

    #[test]
    fn tx_status_u8_roundtrip_is_consistent() {
        for status in [TxStatus::Success, TxStatus::Failure, TxStatus::Pending] {
            let byte = status.to_u8();
            let parsed = TxStatus::from_u8(byte).expect("valid byte");
            assert_eq!(parsed, status);
        }
    }

    #[test]
    fn tx_status_from_u8_invalid_values_return_none() {
        assert!(TxStatus::from_u8(3).is_none());
        assert!(TxStatus::from_u8(255).is_none());
    }
}
