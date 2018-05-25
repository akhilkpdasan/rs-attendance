use bcrypt::{hash, verify, DEFAULT_COST};
use diesel;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use error::MyError;
use jwt::{encode, Header};
use models::{Claims, NewUser, Student, User};

pub fn get_student(conn: &PgConnection, student_id: &str) -> Result<Student, MyError> {
    use schema::students::dsl::*;

    let result = students.filter(id.eq(student_id)).first::<Student>(conn);

    match result {
        Ok(student) => Ok(student),
        Err(diesel::NotFound) => Err(MyError::NotFound),
        Err(_) => Err(MyError::InternalError),
    }
}

pub fn get_students(conn: &PgConnection) -> Result<Vec<Student>, MyError> {
    use schema::students::dsl::*;

    let result = students.order(roll_no).load::<Student>(conn);

    match result {
        Ok(items) => Ok(items),
        Err(_) => Err(MyError::InternalError),
    }
}

pub fn update_student(conn: &PgConnection, s_id: &str, s_attendance: f32) -> Result<(), MyError> {
    use schema::students::dsl::*;

    let updated = diesel::update(students)
        .filter(id.eq(s_id))
        .set(attendance.eq(s_attendance))
        .execute(conn);

    match updated {
        Ok(1) => Ok(()),
        Ok(_) => Err(MyError::NotFound),
        Err(_) => Err(MyError::InternalError),
    }
}

pub fn new_student(conn: &PgConnection, student: &Student) -> Result<String, MyError> {
    use schema::students::dsl::*;

    let rows_inserted = diesel::insert_into(students).values(student).execute(conn);

    match rows_inserted {
        Ok(_) => Ok(student.id.clone()),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(MyError::Conflict),
        Err(_) => Err(MyError::InternalError),
    }
}

pub fn delete_student(conn: &PgConnection, s_id: &str) -> Result<(), MyError> {
    use schema::students::dsl::*;

    match diesel::delete(students.filter(id.eq(s_id))).execute(conn) {
        Ok(1) => Ok(()),
        Ok(_) => Err(MyError::NotFound),
        Err(_) => Err(MyError::InternalError),
    }
}

pub fn login_user(
    conn: &PgConnection,
    u_username: &str,
    u_password: &str,
) -> Result<String, MyError> {
    use schema::users::dsl::*;

    match users.filter(username.eq(u_username)).first::<User>(conn) {
        Ok(user) => match verify(u_password, &user.password) {
            Ok(valid) => {
                if valid {
                    let token = encode(
                        &Header::default(),
                        &Claims {
                            username: u_username.to_string(),
                        },
                        "secret".as_ref(),
                    ).unwrap();
                    Ok(token)
                } else {
                    Err(MyError::BadPassword)
                }
            }
            Err(_) => Err(MyError::PasswordVerify)
        },
        Err(_) => Err(MyError::UserNotFound),
    }
}

pub fn register_user(conn: &PgConnection, mut new_user: NewUser) -> Result<(), MyError> {
    use schema::users::dsl::*;

    let hash_pass = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(MyError::PasswordHash),
    };

    new_user.password = hash_pass;

    let rows_inserted = diesel::insert_into(users).values(&new_user).execute(conn);

    match rows_inserted {
        Ok(1) => Ok(()),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(MyError::Conflict),
        _ => Err(MyError::InternalError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schema::students::dsl::*;

    fn connection() -> PgConnection {
        let conn =
            PgConnection::establish("postgres://postgres@localhost/attendance_management").unwrap();

        let _ = conn.begin_test_transaction();

        diesel::insert_into(students)
            .values((
                id.eq("s99"),
                name.eq("bedki"),
                roll_no.eq(32),
                attendance.eq(12.0),
            ))
            .execute(&conn)
            .unwrap();

        diesel::insert_into(students)
            .values((
                id.eq("s89"),
                name.eq("yogesh"),
                roll_no.eq(36),
                attendance.eq(16.0),
            ))
            .execute(&conn)
            .unwrap();

        diesel::insert_into(students)
            .values((
                id.eq("s02"),
                name.eq("yogesh"),
                roll_no.eq(36),
                attendance.eq(16.0),
            ))
            .execute(&conn)
            .unwrap();

        conn
    }

    #[test]
    fn get_all_students_works() {
        let conn = connection();

        assert!(get_students(&conn).is_ok());
    }

    #[test]
    fn get_student_works() {
        let conn = connection();

        assert!(get_student(&conn, "s99").is_ok());
    }

    #[test]
    fn new_student_works() {
        let conn = connection();

        let stud = Student {
            id: "s14".to_string(),
            name: "omkar".to_string(),
            roll_no: 14,
            attendance: 27.0,
        };
        assert!(new_student(&conn, &stud).is_ok());
    }

    #[test]
    fn update_student_works() {
        let conn = connection();

        assert!(update_student(&conn, "s89", 29.0).is_ok());
    }

    #[test]
    fn update_non_existent() {
        let conn = connection();

        assert!(update_student(&conn, "s100", 100.0).is_err());
    }

    #[test]
    fn delete_student_works() {
        let conn = connection();

        assert!(delete_student(&conn, "s02").is_ok());
    }

    #[test]
    fn delete_non_existent() {
        let conn = connection();

        assert!(delete_student(&conn, "s100").is_err());
    }
}
