#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;

use actix::SyncArbiter;
use actix::{Addr, Syn};
use actix_web::*;
use db::*;
use diesel::prelude::*;
use dotenv::dotenv;
use futures::future::Future;
use models::Student;
use r2d2_diesel::ConnectionManager;
use std::env;

mod db;
mod models;
mod schema;

struct State {
    db: Addr<Syn, DbExecutor>,
}

fn get_all(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.state()
        .db
        .send(StudentsList {})
        .from_err()
        .and_then(|res| match res {
            Ok(students) => Ok(HttpResponse::Ok().json(students)?),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn get_one(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let id: String = req.match_info().query("sid").expect("Student not found");

    req.state()
        .db
        .send(GetStudent { id: id })
        .from_err()
        .and_then(|res| match res {
            Ok(student) => Ok(HttpResponse::Ok().json(student)?),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn new(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let executor = req.state().db.clone();
    req.json()
        .from_err()
        .and_then(move |val: Student| {
            executor
                .send(PostStudent {
                    id: val.id,
                    name: val.name,
                    roll_no: val.roll_no,
                    attendance: val.attendance,
                })
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(HttpResponse::Ok().json(msg)?),
                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
        })
        .responder()
}

#[derive(Deserialize, Debug)]
struct StudentUpdateForm {
    attendance: f32,
}

fn update(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let sid: String = req.match_info().query("sid").expect("Student id not found");
    let executor = req.state().db.clone();
    req.json()
        .from_err()
        .and_then(move |val: StudentUpdateForm| {
            executor
                .send(UpdateStudent {
                    id: sid,
                    attendance: val.attendance,
                })
                .from_err()
                .and_then(|res| match res {
                    Ok(msg) => Ok(HttpResponse::Ok().json(msg)?),
                    Err(_) => Ok(HttpResponse::InternalServerError().into()),
                })
        })
        .responder()
}

fn delete(req: HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let sid: String = req.match_info().query("sid").expect("Student id not found");

    req.state()
        .db
        .send(DeleteStudent { id: sid })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)?),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    let sys = actix::System::new("example");
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || DbExecutor { pool: pool.clone() });

    HttpServer::new(move || {
        Application::with_state(State { db: addr.clone() })
            .middleware(middleware::Logger::default())
            .resource("/students", |r| {
                r.method(Method::GET).a(get_all);
                r.method(Method::POST).a(new);
            })
            .resource("/student/{sid}", |r| {
                r.method(Method::GET).a(get_one);
                r.method(Method::PUT).a(update);
                r.method(Method::DELETE).a(delete);
            })
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
