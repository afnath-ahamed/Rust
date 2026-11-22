# Expense tracker

Build an expense tracker using a validated date, category, currency, and exact amount in minor units. Persist entries and group summaries by month and currency.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
add amount=1250 currency=USD date=2026-01-05 category=books
summary month=2026-01 -> USD books 12.50
export -> stable CSV schema with dates and currencies
```

## 2. Design check

Avoid summing currencies together without a defined conversion policy. Use checked arithmetic and preserve the original values used to calculate a summary.

## Acceptance criteria

- Reopening the storage preserves records and totals.
- Budget summaries reject invalid dates and handle large values without overflow.
- CSV output round-trips a category containing punctuation.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: File organizer](<../01 - File organizer/README.md>) | [Back to Intermediate](<../README.md>) | [Next: Multithreaded file processor →](<../03 - Multithreaded file processor/README.md>)
