use actix::prelude::*;
use diesel;
use diesel::prelude::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::io;

use models;

#[derive(Serialize, Deserialize)]
pub struct StudentResponseList {
    status: i32,
    message: String,
    data: Vec<models::Student>,
}

#[derive(Serialize, Deserialize)]
pub struct StudentResponse {
    status: i32,
    message: String,
    data: models::Student,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteResponse {
    status: i32,
    message: String,
}

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
    type Result = io::Result<StudentResponse>;
}

impl Handler<GetStudent> for DbExecutor {
    type Result = io::Result<StudentResponse>;

    fn handle(&mut self, msg: GetStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();

        let student_result = students.filter(id.eq(msg.id)).load::<models::Student>(conn);

        let student = match &student_result {
            Ok(some_student) => match some_student.first() {
                Some(student) => Some(student),
                None => None,
            },
            Err(_) => None,
        };

        match student {
            Some(s) => Ok(StudentResponse {
                status: 200,
                message: "Student Attendance Info".to_string(),
                data: models::Student {
                    id: s.id.clone(),
                    name: s.name.clone(),
                    roll_no: s.roll_no,
                    attendance: s.attendance,
                },
            }),
            None => Ok(StudentResponse {
                status: 400,
                message: "Student no found".to_string(),
                data: models::Student {
                    id: "".to_string(),
                    name: "".to_string(),
                    roll_no: 0,
                    attendance: 0.0,
                },
            }),
        }
    }
}

pub struct StudentsList;

impl Message for StudentsList {
    type Result = io::Result<StudentResponseList>;
}

impl Handler<StudentsList> for DbExecutor {
    type Result = io::Result<StudentResponseList>;

    fn handle(&mut self, _: StudentsList, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        match students.load::<models::Student>(conn) {
            Ok(items) => Ok(StudentResponseList {
                status: 200,
                message: "Students Attendance List".to_string(),
                data: items,
            }),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

pub struct UpdateStudent {
    pub id: String,
    pub attendance: f32,
}

impl Message for UpdateStudent {
    type Result = io::Result<StudentResponse>;
}

impl Handler<UpdateStudent> for DbExecutor {
    type Result = io::Result<StudentResponse>;

    fn handle(&mut self, msg: UpdateStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        let _ = diesel::update(students)
            .filter(id.eq(&msg.id))
            .set(attendance.eq(msg.attendance))
            .execute(conn);

        match students.filter(id.eq(msg.id)).load::<models::Student>(conn) {
            Ok(mut items) => match items.pop() {
                Some(s) => Ok(StudentResponse {
                    status: 200,
                    message: "Student Record Updated".to_string(),
                    data: models::Student {
                        id: s.id,
                        name: s.name,
                        roll_no: s.roll_no,
                        attendance: s.attendance,
                    },
                }),
                None => Ok(StudentResponse {
                    status: 400,
                    message: "Student not found".to_string(),
                    data: models::Student {
                        id: "".to_string(),
                        name: "".to_string(),
                        roll_no: 0,
                        attendance: 0.0,
                    },
                }),
            },
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
    type Result = io::Result<StudentResponse>;
}

impl Handler<PostStudent> for DbExecutor {
    type Result = io::Result<StudentResponse>;

    fn handle(&mut self, msg: PostStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        let new_student = models::Student {
            id: msg.id.clone(),
            name: msg.name,
            roll_no: msg.roll_no,
            attendance: msg.attendance,
        };
        let _ = diesel::insert_into(students)
            .values(&new_student)
            .execute(conn);

        match students.filter(id.eq(msg.id)).load::<models::Student>(conn) {
            Ok(mut items) => {
                let s = items.pop().unwrap();
                Ok(StudentResponse {
                    status: 200,
                    message: "Student Record Inserted".to_string(),
                    data: models::Student {
                        id: s.id,
                        name: s.name,
                        roll_no: s.roll_no,
                        attendance: s.attendance,
                    },
                })
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

pub struct DeleteStudent {
    pub id: String,
}

impl Message for DeleteStudent {
    type Result = io::Result<DeleteResponse>;
}

impl Handler<DeleteStudent> for DbExecutor {
    type Result = io::Result<DeleteResponse>;

    fn handle(&mut self, msg: DeleteStudent, _: &mut Self::Context) -> Self::Result {
        use schema::students::dsl::*;

        let conn: &PgConnection = &self.pool.get().unwrap();
        match diesel::delete(students.filter(id.eq(msg.id))).execute(conn) {
            Ok(_) => Ok(DeleteResponse {
                status: 200,
                message: "Student data deleted".to_string(),
            }),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}
