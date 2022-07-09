-- Your SQL goes here

create table words (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    mother_tongue TEXT NOT NULL,
    foreign_language TEXT NOT NULL
)