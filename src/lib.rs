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
extern crate time;

use actix::{Addr, Syn, SyncArbiter};
use actix_web::http::*;
use actix_web::middleware::{cors::Cors, Middleware, Started};
use actix_web::*;
use db_executor::*;
use diesel::prelude::*;
use dotenv::dotenv;
use error::MyError;
use futures::future::Future;
use jwt::{decode, Validation};
use models::{Claims, NewUser, Student};
use r2d2_diesel::ConnectionManager;
use std::env;

mod db;
mod db_executor;
mod error;
pub mod models;
pub mod schema;

pub struct AppState {
    db: Addr<Syn, DbExecutor>,
}

pub fn get_all(state: State<AppState>) -> FutureResponse<HttpResponse> {
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

pub fn get_one(state: State<AppState>, sid: Path<String>) -> FutureResponse<HttpResponse> {
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

pub fn new(state: State<AppState>, body: Json<Student>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(body.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(id) => Ok(HttpResponse::Ok().json(json!({ "URL": format!("/students/{}", id) }))),
            Err(MyError::Conflict) => Ok(HttpResponse::Conflict().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

#[derive(Deserialize, Debug)]
pub struct Attendance {
    attendance: f32,
}

pub fn update(
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

pub fn delete(state: State<AppState>, sid: Path<String>) -> FutureResponse<HttpResponse> {
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

pub fn login(state: State<AppState>, user: Json<UserLogin>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(token) => Ok(HttpResponse::Ok()
                .cookie(
                    http::Cookie::build("token", token)
                        .path("/")
                        .http_only(true)
                        .finish(),
                )
                .finish()),
            Err(err) => match err {
                MyError::BadPassword => Ok(HttpResponse::Unauthorized().body("Incorrect Password")),
                MyError::UserNotFound => {
                    Ok(HttpResponse::Unauthorized().body("User doesn't Exist"))
                }
                MyError::PasswordVerify => Ok(HttpResponse::InternalServerError().body("sad")),
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
        })
        .responder()
}

pub fn register(state: State<AppState>, user: Json<NewUser>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(MyError::Conflict) => Ok(HttpResponse::Conflict().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        })
        .responder()
}

pub fn logout(_req: HttpRequest<AppState>) -> HttpResponse {
    let cookie_str = "token=deleted; HttpOnly; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
    HttpResponse::Ok()
        .cookie(http::Cookie::parse(cookie_str).unwrap())
        .finish()
}

pub fn who_am_i(req: HttpRequest<AppState>) -> HttpResponse {
    //Only logged in user can reach this handle
    //so unwrap is fine
    let token = req.cookie("token").unwrap().value();

    let dec_token = decode::<Claims>(token, "secret".as_ref(), &Validation::default()).unwrap();

    HttpResponse::Ok().body(dec_token.claims.username)
}

pub struct Authorization {
    pub paths_to_ignore: Vec<String>,
}

impl<S> Middleware<S> for Authorization {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        if req.method() == &Method::OPTIONS {
            return Ok(Started::Done);
        }
        if self.paths_to_ignore.contains(&req.path().to_string()) {
            Ok(Started::Done)
        } else {
            let token = match req.cookie("token") {
                Some(cookie) => cookie.value(),
                None => {
                    return Ok(Started::Response(
                        HttpResponse::Unauthorized()
                            .header("Access-Control-Allow-Origin", "http://localhost:8080")
                            .header("Access-Control-Allow-Credentials", "true")
                            .body("Authorization Token missing"),
                    ))
                }
            };

            let dec_token = decode::<Claims>(token, "secret".as_ref(), &Validation::default());

            match dec_token {
                Ok(_) => Ok(Started::Done),
                Err(_) => Ok(Started::Response(
                    HttpResponse::Unauthorized().body("Token incorrect"),
                )),
            }
        }
    }
}

pub type DbPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

pub type DbAddr = actix::Addr<actix::Syn, db_executor::DbExecutor>;

pub fn build_app_state(pool: DbPool) -> AppState {
    let addr = SyncArbiter::start(3, move || DbExecutor { pool: pool.clone() });
    AppState { db: addr.clone() }
}

pub fn create_app() -> App<AppState> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let state = build_app_state(pool);

    App::with_state(state)
        .middleware(middleware::Logger::default())
        .middleware(Authorization {
            paths_to_ignore: vec!["/login".to_string(), "/register".to_string()],
        })
        .configure(|app| {
            Cors::for_app(app)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_origin("http://localhost:8080")
                .supports_credentials()
                .max_age(3600)
                .resource("/login", |r| r.method(Method::POST).with2(login))
                .resource("/logout", |r| r.method(Method::GET).h(logout))
                .resource("/whoami", |r| r.method(Method::GET).h(who_am_i))
                .resource("/register", |r| r.method(Method::POST).with2(register))
                .resource("/students", |r| {
                    r.method(Method::GET).with(get_all);
                    r.method(Method::POST).with2(new);
                })
                .resource("/students/{sid}", |r| {
                    r.method(Method::GET).with2(get_one);
                    r.method(Method::PUT).with3(update);
                    r.method(Method::DELETE).with2(delete);
                })
                .register()
        })
}
