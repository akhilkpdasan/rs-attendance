#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate futures;
extern crate jsonwebtoken as jwt;
extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate bcrypt;
extern crate env_logger;

use actix::SyncArbiter;
use actix::{Addr, Syn};
use actix_web::http::*;
use actix_web::*;
use actix_web::{middleware::Middleware, middleware::Response};
use db::*;
use diesel::prelude::*;
use dotenv::dotenv;
use futures::future::Future;
use jwt::{decode, Validation};
use models::Student;
use r2d2_diesel::ConnectionManager;
use std::env;

mod cors;
mod db;
mod models;
mod schema;

pub struct AppState {
    db: Addr<Syn, DbExecutor>,
}

fn get_all(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetStudents {})
        .from_err()
        .and_then(|res| match res {
            Ok(students) => Ok(HttpResponse::Ok().json(students)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn get_one(state: State<AppState>, sid: Path<String>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetStudent {
            id: sid.to_string(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(student) => Ok(HttpResponse::Ok().json(student)),
            Err(MyError::NotFound) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn new(state: State<AppState>, body: Json<Student>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(body.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(id) => Ok(HttpResponse::Ok().json(json!({ "URL": format!("/students/{}", id) }))),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

#[derive(Deserialize, Debug)]
struct Attendance {
    attendance: f32,
}

fn update(
    state: State<AppState>,
    body: Json<Attendance>,
    sid: Path<String>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(UpdateStudent {
            id: sid.to_string(),
            attendance: body.attendance,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::NoContent().into()),
            Err(MyError::NotFound) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn delete(state: State<AppState>, sid: Path<String>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DeleteStudent {
            id: sid.to_string(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::NoContent().into()),
            Err(MyError::NotFound) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn login(state: State<AppState>, user: Json<UserLogin>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(token) => Ok(HttpResponse::Ok().body(token)),
            Err(err) => match err {
                MyError::BadPassword => Ok(HttpResponse::Unauthorized().body("Incorrect Password")),
                MyError::UserNotFound => {
                    Ok(HttpResponse::Unauthorized().body("User doesn't Exist"))
                }
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
        })
        .responder()
}

fn register(
    state: State<AppState>,
    user: Json<models::UserRegister>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::BadRequest().into()),
        })
        .responder()
}

struct Authorization {}

impl<S> Middleware<S> for Authorization {
    fn response(&self, req: &mut HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        if req.path() == "/login" || req.path() == "/register" {
            Ok(Response::Done(resp))
        } else {
            let token = match req.headers_mut().get("Authorization") {
                Some(t) => match t.to_str() {
                    Ok(t) => t,
                    Err(_) => {
                        return Ok(Response::Done(
                            HttpResponse::BadRequest().body("Token contains non ascii characters"),
                        ))
                    }
                },
                None => {
                    return Ok(Response::Done(
                        HttpResponse::Unauthorized().body("Authorization token missing"),
                    ))
                }
            };

            let dec_token =
                decode::<Claims>(&token[7..], "secret".as_ref(), &Validation::default());

            match dec_token {
                Ok(_) => Ok(Response::Done(resp)),
                Err(_) => Ok(Response::Done(
                    HttpResponse::Unauthorized().body("Token incorrect"),
                )),
            }
        }
    }
}

pub fn create_app() -> App<AppState> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        //.connection_timeout(std::time::Duration::new(300, 0))
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || DbExecutor { pool: pool.clone() });
    App::with_state(AppState { db: addr.clone() })
        .middleware(middleware::Logger::default())
        .middleware(Authorization {})
        .resource("/login", |r| r.method(Method::POST).with2(login))
        .resource("/register", |r| r.method(Method::POST).with2(register))
        .resource("/students", |r| {
            cors::options().register(r);
            r.method(Method::GET).with(get_all);
            r.method(Method::POST).with2(new);
        })
        .resource("/students/{sid}", |r| {
            cors::options().register(r);
            r.method(Method::GET).with2(get_one);
            r.method(Method::PUT).with3(update);
            r.method(Method::DELETE).with2(delete);
        })
}
