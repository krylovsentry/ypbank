

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