# comparer crate (YPBank)

Инструмент для сравнения двух файлов со списками транзакций YPBank.


## Планируемое использование

Пример предполагаемого вызова:

```shell
ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
```

Две транзакции считаются одинаковыми, если все их поля совпадают (не только ID).