use actix::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel;
use diesel::prelude::*;
use jwt::{encode, Header};
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::io;

use models;

pub struct DbExecutor {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct GetStudent {
    pub id: String,
}

impl Message for GetStudent {
    type Result = Result<models::Student, MyError>;
}

impl Handler<GetStudent> for DbExecutor {
    type Result = Result<models::Student, MyError>;

    fn handle(&mut self, msg: GetStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        let items = students
            .filter(id.eq(msg.id))
            .first::<models::Student>(conn);

        match items {
            Ok(item) => Ok(item),
            Err(diesel::NotFound) => Err(MyError::NotFound),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

pub struct GetStudents;

impl Message for GetStudents {
    type Result = io::Result<Vec<models::Student>>;
}

impl Handler<GetStudents> for DbExecutor {
    type Result = io::Result<Vec<models::Student>>;

    fn handle(&mut self, _: GetStudents, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        match students.order(roll_no).load::<models::Student>(conn) {
            Ok(items) => Ok(items),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

pub struct UpdateStudent {
    pub id: String,
    pub attendance: f32,
}

impl Message for UpdateStudent {
    type Result = Result<(), MyError>;
}

impl Handler<UpdateStudent> for DbExecutor {
    type Result = Result<(), MyError>;

    fn handle(&mut self, msg: UpdateStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        let updated = diesel::update(students)
            .filter(id.eq(&msg.id))
            .set(attendance.eq(msg.attendance))
            .execute(conn);

        match updated {
            Ok(1) => Ok(()),
            Ok(_) => Err(MyError::NotFound),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

impl Message for models::Student {
    type Result = Result<String, MyError>;
}

impl Handler<models::Student> for DbExecutor {
    type Result = Result<String, MyError>;

    fn handle(&mut self, msg: models::Student, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        let new_student = models::Student {
            id: msg.id.clone(),
            name: msg.name,
            roll_no: msg.roll_no,
            attendance: msg.attendance,
        };
        let rows_inserted = diesel::insert_into(students)
            .values(&new_student)
            .execute(conn);

        match rows_inserted {
            Ok(_) => Ok(msg.id),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

pub enum MyError {
    NotFound,
    DatabaseError,
    AuthorizationError,
    BadPassword,
    TokenVerify,
    PasswordHash,
    UserNotFound,
    UserExists,
}

pub struct DeleteStudent {
    pub id: String,
}

impl Message for DeleteStudent {
    type Result = Result<(), MyError>;
}

impl Handler<DeleteStudent> for DbExecutor {
    type Result = Result<(), MyError>;

    fn handle(&mut self, msg: DeleteStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        match diesel::delete(students.filter(id.eq(msg.id))).execute(conn) {
            Ok(1) => Ok(()),
            Ok(_) => Err(MyError::NotFound),
            Err(_) => Err(MyError::DatabaseError),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl Message for UserLogin {
    type Result = Result<String, MyError>;
}

impl Handler<UserLogin> for DbExecutor {
    type Result = Result<String, MyError>;

    fn handle(&mut self, msg: UserLogin, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();

        match users
            .filter(username.eq(&msg.username))
            .first::<models::Users>(conn)
        {
            Ok(user) => match verify(&msg.password, &user.password) {
                Ok(valid) => {
                    if valid {
                        let token = encode(
                            &Header::default(),
                            &Claims {
                                username: msg.username,
                            },
                            "secret".as_ref(),
                        ).unwrap();
                        Ok(token)
                    } else {
                        Err(MyError::BadPassword)
                    }
                }
                Err(_) => Err(MyError::TokenVerify),
            },
            Err(_) => Err(MyError::UserNotFound),
        }
    }
}

impl Message for models::UserRegister {
    type Result = Result<String, MyError>;
}

impl Handler<models::UserRegister> for DbExecutor {
    type Result = Result<String, MyError>;

    fn handle(&mut self, mut msg: models::UserRegister, _: &mut Self::Context) -> Self::Result {
        use schema::users::dsl::*;

        let hash_pass = match hash(&msg.password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return Err(MyError::PasswordHash),
        };

        msg.password = hash_pass;

        let conn: &PgConnection = &self.pool.get().unwrap();

        let rows_inserted = diesel::insert_into(users).values(&msg).execute(conn);

        match rows_inserted {
            Ok(1) => Ok(msg.username),
            Ok(0) => Err(MyError::UserExists),
            _ => Err(MyError::DatabaseError),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
}
