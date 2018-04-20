echo "DATABASE_URL=postgres://postgres@localhost/test_db" > .env
psql -c 'create database test_db' -U postgres
psql -c 'create table students (id varchar(5) PRIMARY KEY, name varchar(15), roll_no integer, attendance real);' -U postgres -d test_db
psql -c "insert into students values ('s32', 'bedki', 32, 12.0);" -U postgres -d test_db
psql -c "insert into students values ('s36', 'yogesh', 37, 16.0);" -U postgres -d test_db
cargo test
psql -c 'drop database test_db;' -U postgres
echo "DATABASE_URL=postgres://postgres@localhost/attendance_management" > .env
