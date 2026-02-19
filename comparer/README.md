# comparer crate (YPBank)

Инструмент для сравнения двух файлов со списками транзакций YPBank.

Сравнение выполняется **по всем полям транзакции**, порядок записей **не важен**.
Если транзакции отличаются, утилита сообщает о первой найденной разнице.

## Запуск

Показать справку:

```bash
cargo run -p comparer -- --help
```

Сравнить два файла:

```bash
cargo run -p comparer -- \
  --file1 records_example.bin \
  --format1 binary \
  --file2 records_example.csv \
  --format2 csv
```

## Аргументы

- `--file1 <PATH>` — путь к первому файлу.
- `--format1 <FORMAT>` — формат первого файла.
- `--file2 <PATH>` — путь ко второму файлу.
- `--format2 <FORMAT>` — формат второго файла.

Поддерживаемые форматы: `csv`, `text`/`txt`, `binary`/`bin`.

## Вывод и коды завершения

Если файлы идентичны, печатается:

```text
The transaction records in 'file1' and 'file2' are identical.
```