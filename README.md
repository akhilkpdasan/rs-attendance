# Student attendance management system

REST API for students attendance management written in rust

[![Build Status](https://travis-ci.com/akhilkpdasan/rs-attendance.svg?branch=master)](https://travis-ci.com/akhilkpdasan/rs-attendance)

## To run tests:

#### Setup test database:

```
diesel setup --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db
```

#### Run migrations:

```
diesel migration --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db run
```

#### Run Integration test:

```
cargo test --release
```

#### Revert migrations:

```
diesel migration --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db revert
```
