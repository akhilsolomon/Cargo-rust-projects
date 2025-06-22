# Student Report Card (Rust CLI + PDF)

This is a simple Rust console application that collects a student's details, calculates their average marks and grade, and generates a formatted report card as a PDF.

---

## ✅ What the project does

- Takes user input: student name, total marks obtained, maximum marks, and number of subjects.
- Calculates:
  - Average marks
  - Percentage
  - Grade (A/B/C/D based on percentage)
- Generates a clean report card PDF with:
  - A titled heading
  - Underline below the title
  - A box containing the report content
  - Text neatly centered inside the box

---

## 🛠 What it’s built with

- Rust
- Libraries used:
  - `printpdf` — for creating the PDF
  - `rustyline` — for terminal input

---

## ▶️ How to run

1. Make sure Rust is installed (`cargo`, `rustc`)
2. Clone the repo and enter the folder:

```bash
git clone https://github.com/akhilsolomon/Cargo-rust-projects.git
cd Cargo-rust-projects/student_report_card
``` 

```bash
cargo run
```
