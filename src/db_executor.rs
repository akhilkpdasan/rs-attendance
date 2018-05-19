use actix::prelude::*;
use db::*;
use diesel::prelude::*;
use error::MyError;
use models::{NewUser, Student};
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

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
    type Result = Result<Student, MyError>;
}

impl Handler<GetStudent> for DbExecutor {
    type Result = Result<Student, MyError>;

    fn handle(&mut self, msg: GetStudent, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.pool.get().unwrap();

        get_student(conn, &msg.id)
    }
}

pub struct GetStudents;

impl Message for GetStudents {
    type Result = Result<Vec<Student>, MyError>;
}

impl Handler<GetStudents> for DbExecutor {
    type Result = Result<Vec<Student>, MyError>;

    fn handle(&mut self, _: GetStudents, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.pool.get().unwrap();

        get_students(conn)
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
        let conn: &PgConnection = &self.pool.get().unwrap();

        update_student(conn, &msg.id, msg.attendance)
    }
}

impl Message for Student {
    type Result = Result<String, MyError>;
}

impl Handler<Student> for DbExecutor {
    type Result = Result<String, MyError>;

    fn handle(&mut self, msg: Student, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.pool.get().unwrap();

        new_student(conn, &msg)
    }
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
        let conn: &PgConnection = &self.pool.get().unwrap();

        delete_student(conn, &msg.id)
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
        let conn: &PgConnection = &self.pool.get().expect("Could not get connection");

        login_user(conn, &msg.username, &msg.password)
    }
}

impl Message for NewUser {
    type Result = Result<(), MyError>;
}

impl Handler<NewUser> for DbExecutor {
    type Result = Result<(), MyError>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.pool.get().unwrap();

        register_user(conn, msg)
    }
}
