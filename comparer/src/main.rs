use std::fs::File;
use std::io::BufReader;
use std::process;

use clap::Parser;
use parser::error::ParserError;
use parser::{parse_format, read_transactions};

/// ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
///
/// Две транзакции считаются одинаковыми, если все соответствующие поля этих транзакций совпадают.
/// Поэтому одного только ID транзакций недостаточно, нужно сравнивать все поля структуры.
#[derive(Debug, Parser)]
#[command(author, version, about = "Compare two YPBank transaction files", long_about = None)]
struct ComparerArgs {
    /// Путь к первому файлу с транзакциями.
    #[arg(long = "file1")]
    file1: String,

    /// Формат первого файла: csv, text/txt, binary/bin.
    #[arg(long = "format1")]
    format1: String,

    /// Путь ко второму файлу с транзакциями.
    #[arg(long = "file2")]
    file2: String,

    /// Формат второго файла: csv, text/txt, binary/bin.
    #[arg(long = "format2")]
    format2: String,
}

#[derive(Debug)]
enum ComparerError {
    Io(std::io::Error),
    Parser(String),
    Difference(String),
}

impl std::fmt::Display for ComparerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparerError::Io(e) => write!(f, "IO error: {}", e),
            ComparerError::Parser(msg) => write!(f, "Parser error: {}", msg),
            ComparerError::Difference(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ComparerError {}

impl From<std::io::Error> for ComparerError {
    fn from(err: std::io::Error) -> Self {
        ComparerError::Io(err)
    }
}

impl From<ParserError> for ComparerError {
    fn from(err: ParserError) -> Self {
        ComparerError::Parser(err.to_string())
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), ComparerError> {
    let args = ComparerArgs::parse();

    let format1 = parse_format(&args.format1);
    let format2 = parse_format(&args.format2);

    let file1 = File::open(&args.file1)?;
    let file2 = File::open(&args.file2)?;

    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    let txs1 = read_transactions(format1, &mut reader1)?;
    let txs2 = read_transactions(format2, &mut reader2)?;

    if let Some(diff_msg) = compare_transaction_lists(&txs1, &txs2, &args.file1, &args.file2) {
        return Err(ComparerError::Difference(diff_msg));
    }

    println!(
        "The transaction records in '{}' and '{}' are identical.",
        args.file1, args.file2
    );
    Ok(())
}

fn compare_transaction_lists<T>(
    txs1: &[T],
    txs2: &[T],
    file1_name: &str,
    file2_name: &str,
) -> Option<String>
where
    T: Eq + std::fmt::Debug + Clone,
{
    if txs1.len() != txs2.len() {
        return Some(format!(
            "The number of transactions differs: {} has {}, {} has {}.",
            file1_name,
            txs1.len(),
            file2_name,
            txs2.len()
        ));
    }

    // Сравниваем множества транзакций без учёта порядка.
    let mut remaining: Vec<T> = txs2.to_vec();

    for tx in txs1 {
        if let Some(pos) = remaining.iter().position(|other| other == tx) {
            remaining.remove(pos);
        } else {
            return Some(format!(
                "Transaction present in '{}' but not in '{}': {:?}",
                file1_name, file2_name, tx
            ));
        }
    }

    if let Some(extra) = remaining.first() {
        return Some(format!(
            "Transaction present in '{}' but not in '{}': {:?}",
            file2_name, file1_name, extra
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct DummyTx(u64);

    #[test]
    fn equal_lists_in_different_order_are_identical() {
        let t1 = DummyTx(1);
        let t2 = DummyTx(2);

        let a = vec![t1.clone(), t2.clone()];
        let b = vec![t2, t1];

        let diff = compare_transaction_lists(&a, &b, "a", "b");
        assert!(diff.is_none());
    }

    #[test]
    fn different_lengths_are_reported() {
        let t1 = DummyTx(1);
        let a = vec![t1.clone()];
        let b: Vec<DummyTx> = Vec::new();

        let diff = compare_transaction_lists(&a, &b, "a", "b")
            .expect("should report difference");
        assert!(diff.contains("number of transactions differs"));
    }

    #[test]
    fn missing_transaction_is_reported() {
        let a = vec![DummyTx(1)];
        let b = vec![DummyTx(2)];

        let diff = compare_transaction_lists(&a, &b, "file_a", "file_b")
            .expect("should report difference");
        assert!(diff.contains("present in 'file_a' but not in 'file_b'"));
    }
}