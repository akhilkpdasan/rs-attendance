# Student attendance management system

> REST API for students attendance management written in rust

[![Build Status](https://travis-ci.com/akhilkpdasan/rs-attendance.svg?branch=master)](https://travis-ci.com/akhilkpdasan/rs-attendance)
[![Build status](https://ci.appveyor.com/api/projects/status/lvs4gdvopqwh4rex?svg=true)](https://ci.appveyor.com/project/akhilkpdasan/rs-attendance)

## Starting Server

Install rustup [See](https://www.rust-lang.org/en-US/other-installers.html)

Install Postgres Database [See](https://www.postgresql.org/download/)

Clone this repo:
```
git clone git@github.com:akhilkpdasan/rs-attendance.git
```
or download [zip]( https://github.com/akhilkpdasan/rs-attendance/archive/master.zip
) file and extract

Install diesel_cli tool to create database and tables:
```
cargo install diesel_cli
```
Create Database:
```
diesel setup
```
Create Tables:
```
diesel migration run
```
Run Server:
Change directory to project folder and run
```
cargo run
```


## To run tests:

 Setup test database:

```
diesel setup --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db
```
Run migrations:
```
diesel migration --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db run
```
Run Integration test:
```
cargo test --release
```
Revert migrations:
```
diesel migration --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db revert
```
