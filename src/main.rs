mod db;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate num_cpus;

use actix::{Addr, Syn};
use actix_web::*;
use db::*;
use diesel::prelude::*;
use dotenv::dotenv;
//use futures::Stream;
use actix::SyncArbiter;
use futures::future::Future;
use models::Student;
use std::env;

//type RegisterResult = Either<HttpResponse, Box<Future<Item=HttpResponse, Error=Error>>>;

struct State {
    db: Addr<Syn, DbExecutor>,
}

#[derive(Serialize)]
struct Students {
    students: Vec<Student>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

//impl Responder for Students {
//    type Item = HttpResponse;
//    type Error = Error;
//
//    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse> {
//        let body = serde_json::to_string(&self)?;
//
//        Ok(HttpResponse::Ok()
//            .content_type("application/json")
//            .body(body)?)
//    }
//}

fn get_all(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.state()
        .db
        .send(GetStudents {})
        .from_err()
        .and_then(|res| match res {
            Ok(student) => Ok(HttpResponse::Ok().json(student)?),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

//fn get_one(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
//    let id: String = req.match_info().query("id").expect("Student not found");
//
//    req.state()
//        .db
//        .send(GetStudent { id })
//        .from_err()
//        .and_then(|res| match res {
//            Ok(student) => Ok(HttpResponse::Ok().json(student)?),
//            Err(_) => Ok(HttpResponse::InternalServerError().into()),
//        })
//        .responder()
//}
//
//#[derive(Deserialize, Debug)]
//struct StudentUpdateForm {
//    attendance: f32,
//}
//
//fn new(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
//    let executor = req.state().db.clone();
//    req.json()
//        .from_err()
//        .and_then(move |val: Student| {
//            executor.send(PostStudent{
//                id: val.id,
//                name: val.name,
//                roll_no: val.roll_no,
//                attendance: val.attendance,
//            })
//            .from_err()
//            .and_then(|res| {
//                match res {
//                    Ok(msg) => Ok(HttpResponse::Ok().into()),
//                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
//                }
//            })
//        })
//    .responder()
//}
//
////pub fn webhook(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
////    let db = req.state().db.clone();
////
////    req.concat2()
////        .from_err()
////        .and_then(move |body| {
////            // body is loaded, now we can deserialize json-rust
////            let payload = json::parse(::std::str::from_utf8(&body).unwrap()).expect("json err"); // FIXME: Handle error
////
////            // TODO: Validate signature
////
////            // Push into database (will eventually use the payload above)
////            let name = "Test";
////            db.send(CreateBuild{name: name.to_owned()}).map_err(Error::from)
////        })
////        .and_then(|res| {
////            match res {
////                Ok(build) => Ok(httpcodes::HTTPOk.build().json(build)?),
////                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
////            }
////        })
////        .responder()
////}
//
//fn update(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
//    let sid: String = req.match_info().query("sid").expect("Student id not found");
//    req.concat2()
//        .from_err()
//        .and_then(|_body| Ok(HttpResponse::Ok().into()))
//        .responder()
//}
//
//fn delete(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
//    let sid: String = req.match_info().query("sid").expect("Student id not found");
//
//    req.concat2()
//        .from_err()
//        .and_then(|body| Ok(HttpResponse::Ok().into()))
//        .responder()
//}

fn main() {
    let sys = actix::System::new("example");

    let connection = establish_connection();
    let addr = SyncArbiter::start(3, || DbExecutor { conn: connection });

    HttpServer::new(move || {
        Application::with_state(State { db: addr.clone() })
            .resource("/students", |r| {
                r.method(Method::GET).a(get_all);
                //r.method(Method::POST).a(new);
            })
            //.resource("/student/{sid}", |r| {
            //    r.method(Method::GET).a(get_one);
            //    r.method(Method::PUT).a(update);
            //    r.method(Method::DELETE).a(delete);
            //})
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
