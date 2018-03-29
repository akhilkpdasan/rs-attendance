#[macro_use] extern crate diesel; 
extern crate bytes;
extern crate futures;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate actix;

use actix_web::*;
use futures::future::Future;
use diesel::prelude::*;
use diesel::insert_into;
//use diesel::debug_query;
use dotenv::dotenv;
//use std::error::Error;
//use std::time::SystemTime;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


mod schema {
    table! {
        students (id) {
            id -> Text,
            name -> Text,
            roll_no -> Int4,
            attendance -> Float4,
        }
    }
}

use schema::students;

#[derive(Deserialize, Insertable)]
#[table_name = "students"]
pub struct StudentForm<'a> {
    id: &'a str,
    name: &'a str,
    roll_no: i32,
    attendance: f32,
}

#[derive(Insertable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "students"]
struct Student {
    id: String,
    name: String,
    roll_no: i32,
    attendance: f32,
}

#[derive(Serialize)]
struct Students {
    students: Vec<Student>,
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



fn get_all(_req: HttpRequest) -> Students {
    use schema::students::dsl::*;
    let connection = establish_connection();

    let result = students
        .load::<Student>(&connection)
        .expect("Error retrieving student");
    Students {students: result}

}

fn get_one(req: HttpRequest) -> Students {
    use schema::students::dsl::*;
    let connection = establish_connection();

    let sid: String = req.match_info().query("sid").expect("id not found");

    let result = students
        .filter(id.eq(sid))
        .load::<Student>(&connection)
        .expect("Error getting student");
    Students {students: result}
}

#[derive(Deserialize, Debug)]
struct StudentUpdateForm {
    attendance: f32,

}

fn new(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    
    req.json()         // <- get UrlEncoded future
       .from_err()
       .and_then(|val: Student| {  // <- url encoded parameters
            use schema::students::dsl::*;
            let conn = establish_connection();
            insert_into(students).values(&val).execute(&conn).expect("Error");
            Ok(httpcodes::HttpOk.into())
       })
       .responder()
}

fn update(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let sid: String = req.match_info().query("sid").expect("id not found");
    req.json()
        .from_err()
        .and_then(|val: StudentUpdateForm| {
            use schema::students::dsl::*;
            let conn = establish_connection();
            diesel::update(students)
                .filter(id.eq(sid))
                .set(attendance.eq(val.attendance))
                .execute(&conn)
                .expect("Error executing query");
            Ok(httpcodes::HttpOk.into())
        })
    .responder()
}

fn delete(req: HttpRequest) -> HttpResponse {

    let sid: String = req.match_info().query("sid").expect("id not found");

    use schema::students::dsl::*;
    let conn = establish_connection();
    
    diesel::delete(students.filter(id.eq(sid)))
        .execute(&conn)
        .expect("Error Deleting");

    httpcodes::HttpOk.into()
}

fn main() {

    let sys = actix::System::new("example");

    HttpServer::new(
        || Application::new()
            .resource("/students", |r| {
                r.method(Method::GET).f(get_all);
                r.method(Method::POST).f(new);
            })
            .resource("/student/{sid}", |r| {
                r.method(Method::GET).f(get_one);
                r.method(Method::PUT).f(update);
                r.method(Method::DELETE).f(delete);
            }))
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
