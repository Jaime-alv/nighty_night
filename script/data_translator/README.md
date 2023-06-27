# Data Translator

A raw data to sql queries python script.

## How to use it

Open corresponding file inside `batch` folder.

Paste into the appropriate file all data it's going to be processed. Leave an empty line between dates.

Run python script with `data_translator.py` and select a numeric option:

1) Eat file
2) Sleep file
3) Both files

If everything went ok, queries should appear inside `sql` folder, already formatted.

## Variables

file `service/const.py`

```python
YEAR: int = 2023
PATH: str = "./script/data_translator/"
BABY_ID: int = 1
INPUT: str = "batch/"
OUTPUT: str = "sql/"
LEVEL: int = logging.INFO
```

| Python value | Meaning                    |
| ------------ | -------------------------- |
| YEAR         | Default year               |
| PATH         | Path to data_translator.py |
| BABY_ID      | Id in postgreSQL           |
| INPUT        | Input folder name          |
| OUTPUT       | Output folder name         |
| LEVEL        | Logging level              |

## Demo

### Eat demo

#### Input file: eat

default path: `./batch/eat`

```txt
day-month
time quantity
```

```eat
06-06
3:20 180
09:30 120

07-06
09:20 150
13:00 100
```

#### Output file: eat.sql

default path: `./sql/eat.sql`

```sql
-- 06-06-2023
INSERT INTO meals (baby_id, date, quantity) VALUES (1, '2023-06-06 03:20:00', 180);
INSERT INTO meals (baby_id, date, quantity) VALUES (1, '2023-06-06 09:30:00', 120);
-- 07-06-2023
INSERT INTO meals (baby_id, date, quantity) VALUES (1, '2023-06-07 09:20:00', 150);
INSERT INTO meals (baby_id, date, quantity) VALUES (1, '2023-06-07 13:00:00', 100);
```

### Sleep demo

#### Input file: sleep

default path: `./batch/sleep`

```txt
day-month
from_time to_time
```

```sleep
16-06
05:45 7:40
9:22 10:10
12:22 13:02
16:30 17:02
21:32 04:00

17-06
4:30 8:30
10:00 10:29
```

#### Output file: sleep.sql

default path: `./sql/sleep.sql`

```sql
-- 16-06-2023
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-16 05:45:00', '2023-06-16 07:40:00');
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-16 09:22:00', '2023-06-16 10:10:00');
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-16 12:22:00', '2023-06-16 13:02:00');
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-16 16:30:00', '2023-06-16 17:02:00');
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-16 21:32:00', '2023-06-17 04:00:00');
-- 17-06-2023
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-17 04:30:00', '2023-06-17 08:30:00');
INSERT INTO dreams (baby_id, from_date, to_date) VALUES (1, '2023-06-17 10:00:00', '2023-06-17 10:29:00');
```
