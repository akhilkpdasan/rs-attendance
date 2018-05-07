-- Your SQL goes here

CREATE TABLE students (id TEXT PRIMARY KEY, name TEXT, roll_no integer, attendance real);


CREATE TABLE users (id SERIAL PRIMARY KEY, username TEXT UNIQUE, password TEXT, email TEXT);

