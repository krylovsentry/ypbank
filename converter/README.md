# converter crate (YPBank)

CLI‑утилита для конвертации файлов со списком транзакций YPBank
между форматами CSV, текстовый и бинарный.

Утилита использует библиотеку `parser` для разбора и записи данных.

## Установка и запуск

Запуск из исходников:

```bash
cargo run -p converter -- --help
```

Сборка бинарника:

```bash
cargo build -p converter --release
```

## Опции командной строки

```text
--input <PATH>           Путь к входному файлу.
--input-format <FORMAT>  Формат входного файла: csv, text/txt, binary/bin.
--output-format <FORMAT> Формат выходного файла: csv, text/txt, binary/bin.
```

## Примеры

### CSV → binary

```bash
cargo run -p converter -- \
  --input records_example.csv \
  --input-format csv \
  --output-format binary > records_example.bin
```

### text → CSV

```bash
cargo run -p converter -- \
  --input records_example.txt \
  --input-format text \
  --output-format csv > records_example.csv
```

Форматы файлов описаны в директории `../Спецификация_форматов`.

