use std::fmt::Display;
use std::fs::write;
use std::str::FromStr;
use crate::error::ParserError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TxStatus,
    pub description: String,
}

impl FromStr for TxType {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            other => Err(ParserError::InvalidTransactionType(other.to_string()))
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
            other => Err(ParserError::InvalidTransactionStatus(other.to_string()))
        }
    }
}