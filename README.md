# YPBank tools

Набор утилит и библиотек для работы со списками транзакций YPBank.

## Структура workspace

- `parser` — библиотека для чтения и записи транзакций в форматах CSV, текстовый и бинарный.
- `converter` — CLI‑утилита для конвертации файлов с транзакциями между поддерживаемыми форматами.
- `comparer` — CLI‑утилита для сравнения двух файлов с транзакциями.

Подробные спецификации форматов лежат в директории `Спецификация_форматов`:

- `YPBankCsvFormat_ru.md`
- `YPBankTextFromat_ru.md`
- `YPBankBinFormat_ru.md`

## Быстрый старт

Сборка и запуск тестов библиотеки парсера:

```bash
cargo test -p parser
```

Запуск конвертера (чтение из файла и вывод в stdout):

```bash
cargo run -p converter -- --input records_example.csv \
  --input-format csv \
  --output-format binary > records_example.bin
```

## Кратко о крейтах

### parser

Библиотека предоставляет:

- типы `Transaction`, `TxType`, `TxStatus`;
- функции:
  - `parse_format(&str) -> Format`,
  - `read_transactions(format, reader) -> Result<Vec<Transaction>, ParserError>`,
  - `write_transactions(format, writer, &[Transaction]) -> Result<(), ParserError>`.


### converter

Консольная утилита, которая:

1. читает входной файл в заданном формате;
2. парсит его с помощью библиотеки `parser`;
3. выводит транзакции в stdout в другом формате.

Пример:

```bash
cargo run -p converter -- \
  --input records_example.txt \
  --input-format text \
  --output-format csv > records_example.csv
```

### comparer



