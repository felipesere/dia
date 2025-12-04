# dia

Calendar display tool.

## Usage

```bash
# Show current month
dia
   December 2025
Mon Tue Wed Thu Fri Sat Sun
  1   2   3   4   5   6   7
  8   9  10  11  12  13  14
 15  16  17  18  19  20  21
 22  23  24  25  26  27  28
 29  30  31

# Show specific month
dia --month jan
dia --month 3
dia --month march

# Show quarter
dia --quarter           # Current quarter
dia --quarter 1         # Q1 (Jan, Feb, Mar)
dia --quarter 2         # Q2 (Apr, May, Jun)
dia --quarter 3         # Q3 (Jul, Aug, Sep)
dia --quarter 4         # Q4 (Oct, Nov, Dec)

# Specify year
dia --year 2024
dia --month jun --year 2023
dia --quarter 2 --year 2025
```
