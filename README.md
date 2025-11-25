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
## Features

|     Column       |     Column       |      Column       |
 History                                      | Fiction                           | Spells                          |
|----------------------------------------------|------------------------------------|----------------------------------|
| ahzirr-trajijazaeri                          | amulet-of-kings                    | arcana-restored                 |
| argonian-account3                             | arcturian-heresy                   | biography-of-the-wolf-queen     |
| battle-of-sancre-tor                          | biography-of-barenziah1            | black-arts-on-trial             |
| brief-history-of-the-empire3                  | black-star                         | buying-game                     |
| cabin-in-the-woods                            | buying-game                        | chances-folly                   |
| charwich-koniinge-letters1                    | charwich-koniinge-letters1         | charwich-koniinge-letters1      |
| chronicles-of-holy-brothers-of-marukh4        | charwich-koniinge-letters3         | charwich-koniinge-letters4      |
| commentaries-on-the-mysterium-xarxes2         | city-of-stone                      | chaurus-pie                     |
| complete-catalogue-of-enchantments-for-armor  | dance-in-the-fire2                 | dance-in-the-fire5              |
| de-rerum-dirennis                              | dance-in-the-fire7                 | feyfolken1                      |
| dwarves-lost-race-of-tamriel2                 | dream-of-sovngarde                 | legend-of-red-eagle             |
| fall-from-glory                                | dwemer-inquiries3                  | light-armor-forging             |
| fall-of-the-snow-prince                        | explorers-guide-to-skyrim          | x292006                         |
| fire-and-darkness                              | feyfolken3                         |                                  |
| firmament                                       | fragment                           |                                  |
| firsthold-revolt                               | herbanes-beastiary1                |                                  |
| forge-hammer-and-anvil                         | hope-of-the-redoran                |                                  |
| game-at-dinner                                 | interviews-with-tapestrists18      |                                  |
| gold-ribbon-of-merit                           | journal-of-gallus-desidenius       |                                  |
| hallgerds-tale                                 | keepers-of-the-razor               |                                  |
| heavy-armor-forging                            | killing                            |                                  |
| herbanes-beastiary3                            | kolb-and-the-dragon                |                                  |
| holds-of-skyrim                                | last-king-of-ayleids               |                                  |
| how-orsinium-passed-to-the-orcs                | lord-jornibrets-last-dance         |                                  |
| kiss-sweet-mother                              | lusty-argonian-maid1               |                                  |
| knights-of-the-nine                            | madmen-of-the-reach                |                                  |
| legendary-city-of-sancre-tor                   | markarth-deco-guide                |                                  |
| lycanthropic-legends-of-skyrim                 | solitude-deco-guide                |                                  |
| x16-accords-of-madness6                        | warrior                            |                                  |
| x292010                                        | x16-accords-of-madness12           |                                  |
|                                                | x292003                            |     

| value 4  | value 5  | value 6  |
---
## Project Structure (example)

```text
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
