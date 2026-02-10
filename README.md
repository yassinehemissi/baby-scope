# Baby Scope

<center>
<img src="icon.png" width="128">
</center>

Baby Scope is a lightweight command-line tool that scans a project directory and gives a quick, clear overview of what’s inside.

It’s designed to answer simple questions fast:
- How big is this project?
- What file types dominate it?
- Where are the largest files?

No configuration, no noise — just scope.

---

## Features

- Recursively scans a directory
- Counts:
  - Total files
  - Total lines of code
- Groups files and lines by extension
- Lists the largest files by line count
- Skips common directories like `.git/` and `target/`
- Fails gracefully on unreadable files

Example:

```
baby-scope .

```

---

## Why Baby Scope?

This project exists for two reasons:

1. Learn Rust properly  
   Baby Scope is built to practice core Rust concepts:
   - ownership and borrowing
   - error handling with `Result`
   - filesystem traversal
   - data aggregation with structs and enums

2. Build something real  
   Instead of toy examples, Baby Scope is a practical CLI tool that reflects how real Rust programs are structured.

The goal is correctness, clarity, and learning — not feature bloat.

---

## Note

This README was AI-assisted.
