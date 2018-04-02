extern crate dotenv;

use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use std::env;
use std::io;

use models;

pub struct DbExecutor {
    pub conn: PgConnection,
}

impl DbExecutor {
    pub fn new() -> DbExecutor {
        dotenv::dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DbExecutor {
            conn: PgConnection::establish(&db_url)
                .expect(&format!("Error connecting to {}", db_url)),
        }
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct GetStudent {
    pub id: String,
}

impl Message for GetStudent {
    type Result = io::Result<models::Student>;
}

impl Handler<GetStudent> for DbExecutor {
    type Result = io::Result<models::Student>;

    fn handle(&mut self, msg: GetStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        match students
            .filter(id.eq(msg.id))
            .load::<models::Student>(&self.conn)
        {
            Ok(mut items) => Ok(items.pop().unwrap()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
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

        match students.load::<models::Student>(&self.conn) {
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
    type Result = io::Result<models::Student>;
}

impl Handler<UpdateStudent> for DbExecutor {
    type Result = io::Result<models::Student>;

    fn handle(&mut self, msg: UpdateStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let _ = diesel::update(students)
            .filter(id.eq(&msg.id))
            .set(attendance.eq(msg.attendance))
            .execute(&self.conn);

        match students
            .filter(id.eq(msg.id))
            .load::<models::Student>(&self.conn)
        {
            Ok(mut items) => Ok(items.pop().unwrap()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

pub struct PostStudent {
    pub id: String,
    pub name: String,
    pub roll_no: i32,
    pub attendance: f32,
}

impl Message for PostStudent {
    type Result = io::Result<models::Student>;
}

impl Handler<PostStudent> for DbExecutor {
    type Result = io::Result<models::Student>;

    fn handle(&mut self, msg: PostStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let new_student = models::Student {
            id: msg.id.clone(),
            name: msg.name,
            roll_no: msg.roll_no,
            attendance: msg.attendance,
        };
        let _ = diesel::insert_into(students)
            .values(&new_student)
            .execute(&self.conn);

        match students
            .filter(id.eq(msg.id))
            .load::<models::Student>(&self.conn)
        {
            Ok(mut items) => Ok(items.pop().unwrap()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

pub struct DeleteStudent {
    pub id: String,
}

impl Message for DeleteStudent {
    type Result = io::Result<()>;
}

impl Handler<DeleteStudent> for DbExecutor {
    type Result = io::Result<()>;

    fn handle(&mut self, msg: DeleteStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        match diesel::delete(students.filter(id.eq(msg.id))).execute(&self.conn) {
            Ok(_) => Ok(()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}
