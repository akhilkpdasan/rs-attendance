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
#[macro_use]
extern crate serde_json;
extern crate env_logger;

use actix::SyncArbiter;
use actix::{Addr, Syn};
use actix_web::http::*;
use actix_web::test::*;
use actix_web::*;
use db::*;
use diesel::prelude::*;
use dotenv::dotenv;
use futures::future::Future;
use models::Student;
use r2d2_diesel::ConnectionManager;
use std::env;

mod cors;
mod db;
mod models;
mod schema;

struct AppState {
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

fn main() {
    let sys = actix::System::new("example");
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(3, move || DbExecutor { pool: pool.clone() });

    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .middleware(middleware::Logger::default())
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
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}

#[cfg(test)]
mod test {
    use super::*;

    fn server() -> TestServerBuilder<AppState> {
        TestServer::build_with_state(|| {
            dotenv().ok();
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(db_url);
            let pool = r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool.");
            let addr = SyncArbiter::start(3, move || DbExecutor { pool: pool.clone() });
            AppState { db: addr.clone() }
        })
    }

    #[test]
    fn test_get_all() {
        let mut srv = server().start(|app| {
            app.resource("/students", |r| r.method(Method::GET).with(get_all));
        });

        let request = srv.client(Method::GET, "/students").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_get_one() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| r.method(Method::GET).with2(get_one));
        });

        let request = srv.client(Method::GET, "/students/s32").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_get_non_existent() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| r.method(Method::GET).with2(get_one));
        });

        let request = srv.client(Method::GET, "/students/s100").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_new() {
        let mut srv = server().start(|app| {
            app.resource("/students", |r| r.method(Method::POST).with2(new));
        });

        let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
        let request = srv.client(Method::POST, "/students").json(body).unwrap();

        let response = srv.execute(request.send()).unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_new_bad_input() {
        let mut srv = server().start(|app| {
            app.resource("/students", |r| r.method(Method::POST).with2(new));
        });

        let body = json!({"id": "s35", "name": "test", "roll_no": "int", "attendance": "float"});
        let request = srv.client(Method::POST, "/students").json(body).unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_update_bad_input() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| r.method(Method::PUT).with3(update));
        });

        let body = json!({"attendance": "float"});
        let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

        let response = srv.execute(request.send()).unwrap();
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_update() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| r.method(Method::PUT).with3(update));
        });

        let body = json!({"attendance": 33.33});
        let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

        let response = srv.execute(request.send()).unwrap();
        assert_eq!(response.status(), http::StatusCode::NO_CONTENT);
    }

    #[test]
    fn test_update_non_existent() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| r.method(Method::PUT).with3(update));
        });

        let body = json!({"attendance": 33.33});
        let request = srv.client(Method::GET, "/students/s100")
            .json(body)
            .unwrap();

        let response = srv.execute(request.send()).unwrap();
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_delete() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| {
                r.method(Method::DELETE).with2(delete)
            });
        });

        let request = srv.client(Method::DELETE, "/students/s36")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::NO_CONTENT);
    }

    #[test]
    fn test_delete_non_existent() {
        let mut srv = server().start(|app| {
            app.resource("/students/{sid}", |r| {
                r.method(Method::DELETE).with2(delete)
            });
        });

        let request = srv.client(Method::DELETE, "/students/s100")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }
}
