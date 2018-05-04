-- Your SQL goes here
CREATE TABLE users (
	id SERIAL NOT NULL PRIMARY KEY,
	email TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL
);
