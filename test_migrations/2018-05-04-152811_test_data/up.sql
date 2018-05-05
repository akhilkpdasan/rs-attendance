-- Your SQL goes here

CREATE TABLE students (id TEXT PRIMARY KEY, name TEXT, roll_no integer, attendance real);

INSERT INTO students values ('s32', 'bedki', 32, 12.0);
INSERT INTO students values ('s36', 'yogesh', 36, 16.0);
INSERT INTO students values ('s02', 'ashish', 2, 19.0);
INSERT INTO students values ('s99', 'last' , 99, 44.2);

CREATE TABLE users (id SERIAL PRIMARY KEY, username TEXT UNIQUE, password TEXT, email TEXT);

INSERT INTO users (username, password, email) values ('test', '$2y$12$mBpFsVrXcCegx9en5cTDjeSfBXT4jbk4WRuijk/O2D1KgcKOiTE7O', 'test');
