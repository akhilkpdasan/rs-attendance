mod models;
mod schema;
mod db;

#[macro_use] extern crate diesel; 
#[macro_use] extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate http;

use actix_web::*;
use actix::*;
use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use futures::future::Future;
use models::Student;
use std::env;
use db::*;
use futures::future::result;

type RegisterResult = Either<HttpResponse, Box<Future<Item=HttpResponse, Error=Error>>>;

struct State {
    db: Addr<Syn, DbExecutor>,
}

#[derive(Serialize)]
struct Students {
    students: Vec<Student>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


impl Responder for Students {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)?)
    }
}


fn get_all(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {

    req.state().db.send(GetStudents{})
       .and_then(|res| {
            match res {
                Ok(student) => {
                    let body = serde_json::to_string(&student).unwrap();
                    Ok(httpcodes::HttpOk.build()
                       .content_type("application/json")
                       .body(body)?)
                },
                Err(_) => 
                    Ok(httpcodes::HttpInternalServerError.into()),
            }
       })
        .responder()
}

fn get_one(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {

    let id: String = req.match_info().query("id")
        .expect("Student not found");

    req.state().db.send(GetStudent{id})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(student) => Ok(Students{students: student}),
                Err(_) => Err(httpcodes::HttpInternalServerError)
            }
        })
    .responder()
}

#[derive(Deserialize, Debug)]
struct StudentUpdateForm {
    attendance: f32,

}

fn new(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    
    req.json()
       .from_err()
       .and_then(|val: Student| {
            use schema::students::dsl::*;
            let conn = establish_connection();
            insert_into(students).values(&val).execute(&conn).expect("Error");
            Ok(httpcodes::HttpOk.into())
       })
       .responder()
}

fn update(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let sid: String = req.match_info().query("sid")
        .expect("Student id not found");
    req.json()
        .from_err()
        .and_then(|val: StudentUpdateForm| {
            use schema::students::dsl::*;
            let conn = establish_connection();
            diesel::update(students)
                .filter(id.eq(sid))
                .set(attendance.eq(val.attendance))
                .execute(&conn)
                .expect("Error executing update query");
            Ok(httpcodes::HttpOk.into())
        })
    .responder()
}

fn delete(req: HttpRequest) -> HttpResponse {

    let sid: String = req.match_info().query("sid")
        .expect("Student id not found");

    use schema::students::dsl::*;
    let conn = establish_connection();
    
    diesel::delete(students.filter(id.eq(sid)))
        .execute(&conn)
        .expect("Error Deleting Students");

    httpcodes::HttpOk.into()
}

fn main() {

    let sys = actix::System::new("example");

    let connection = establish_connection();
    let addr = SyncArbiter::start(3, || {
        DbExecutor{conn: connection}
    });

    HttpServer::new(move || {
        Application::with_state(State{db: addr.clone()})
            .resource("/students", |r| {
                r.method(Method::GET).a(get_all);
                r.method(Method::POST).f(new);
            })
            .resource("/student/{sid}", |r| {
                r.method(Method::GET).f(get_one);
                r.method(Method::PUT).f(update);
                r.method(Method::DELETE).f(delete);
            })})
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
