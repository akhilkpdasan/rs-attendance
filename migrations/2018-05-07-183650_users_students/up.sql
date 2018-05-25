-- Your SQL goes here

CREATE TABLE students (id TEXT PRIMARY KEY, name TEXT NOT NULL, roll_no integer NOT NULL, attendance real NOT NULL);


CREATE TABLE users (id SERIAL PRIMARY KEY, username TEXT UNIQUE NOT NULL, password TEXT NOT NULL, email TEXT NOT NULL);

