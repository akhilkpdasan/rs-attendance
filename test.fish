psql -c 'create database test_db' -U postgres

psql -c 'create table students (id varchar(5) PRIMARY KEY, name varchar(15), roll_no integer, attendance real);' -U postgres -d test_db
psql -c "insert into students values ('s32', 'bedki', 32, 12.0);" -U postgres -d test_db
psql -c "insert into students values ('s36', 'yogesh', 37, 16.0);" -U postgres -d test_db

psql -c 'create table users (id SERIAL PRIMARY KEY, username TEXT UNIQUE,  password TEXT, email TEXT);' -U postgres -d test_db

psql -c "insert into users (username, password, email) values ('test', '\$2y\$12\$mBpFsVrXcCegx9en5cTDjeSfBXT4jbk4WRuijk/O2D1KgcKOiTE7O', 'test');" -U postgres -d test_db

cargo test --test integration_tests
cargo test --test integration_tests -- --test-threads=1

psql -c 'drop database test_db;' -U postgres
