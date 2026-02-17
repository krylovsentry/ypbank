use std::process;
use clap::Parser;

// ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
// valid transactions
// Две транзакции считаются одинаковыми, если все соответствующие поля этих транзакций совпадают.
// Поэтому одного только ID транзакций не достаточно, чтобы их сравнить, нужно убедиться, что и остальные поля равны.
#[derive(Debug, Parser)]
struct ComparerArgs {
    file1: String,
    format1: String,
    file2: String,
    format2: String,
}

fn main() {
    println!("YPBank Comparer – not implemented yet");
    process::exit(0);
}