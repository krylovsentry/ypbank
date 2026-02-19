# parser crate (YPBank)

Библиотека для чтения и записи списка транзакций YPBank в нескольких файловых форматах:

- CSV;
- текстовый формат;
- бинарный формат.

## Основные типы

- `Transaction` — одна банковская транзакция (ID, тип, участники, сумма, статус, описание и т.д.).
- `TxType` — тип транзакции: `Deposit`, `Transfer`, `Withdrawal`.
- `TxStatus` — статус транзакции: `Success`, `Failure`, `Pending`.
- `ParserError` — единый тип ошибок парсинга и сериализации.
- `Format` — формат хранения: `Csv`, `Text`, `Binary`.

## Основные функции

- `parse_format(&str) -> Format` — преобразует строку (`"csv"`, `"text"`, `"txt"`, `"binary"`, `"bin"`) в перечисление `Format`.
- `read_transactions(format, reader) -> Result<Vec<Transaction>, ParserError>` — считывает транзакции из потока (`Read`) в заданном формате.
- `write_transactions(format, writer, &[Transaction]) -> Result<(), ParserError>` — записывает транзакции в поток (`Write`) в заданном формате.

## Пример использования (псевдокод)

```rust
use std::fs::File;
use std::io::BufReader;
use parser::{parse_format, read_transactions, write_transactions, Format};

fn example() -> Result<(), parser::error::ParserError> {
    let input_file = File::open("input.csv")?;
    let mut reader = BufReader::new(input_file);

    let input_format = parse_format("csv");
    let txs = read_transactions(input_format, &mut reader)?;

    // … здесь можно обработать список транзакций …

    let output_format = Format::Binary;
    let mut output = std::fs::File::create("output.bin")?;
    write_transactions(output_format, &mut output, &txs)?;

    Ok(())
}
```

Для детального описания форматов смотрите файлы в директории `../Спецификация_форматов`.

