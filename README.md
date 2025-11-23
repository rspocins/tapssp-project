# tapssp-project
A small Rust project that lets you type the name of a book from **The Elder Scrolls V: Skyrim** and automatically opens that book (stored as an HTML file) in your default web browser.


Under the hood, it uses a **MySQL** database to store:


- `name` – book title (string)

- `path` – file system path to the HTML file (string)

- `genres` – boolean flags representing genres ( History, fiction, educational)


The program reads a book name from **standard input**, looks it up in the database, finds the HTML file path, and launches it in your browser.


---


## Features


- Catalog of Skyrim books backed by MySQL  

- Simple schema: title, file path, and genre flags  

- Lookup by exact book name from stdin  

- Opens the corresponding HTML file in your default browser  

- Implemented in Rust  
---
## Project Structure (example)

```text
.

├── src
│   ├── main.rs
│   └── db.rs          # database connection / queries
├── books              # directory containing HTML book files
│   ├── Ancestor-and-the-dunmer.html
│   └── ...
├── Cargo.toml
└── README.md