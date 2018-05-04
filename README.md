[![Build Status](https://travis-ci.com/akhilkpdasan/rs-attendance.svg?branch=master)](https://travis-ci.com/akhilkpdasan/rs-attendance)
# To run tests:

### Setup test database:

```
diesel setup --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db
```

### Run migrations:

```
diesel migration --migration-dir test_migrations --database-url postgres://postgres@localhost/test_db redo
```

### Run Integration test:

```
cargo test --release
```
