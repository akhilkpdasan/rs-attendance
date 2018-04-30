extern crate attendance_rs;
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_json;

use actix_web::test::TestServer;
use actix_web::http::{Method, StatusCode};
use attendance_rs::create_app;

#[test]
fn test_get_all() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::GET, "/students").finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_get_one() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::GET, "/students/s32").finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_get_non_existent() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::GET, "/students/s100").finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_new() {
    let mut srv = TestServer::with_factory(create_app);
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_new_bad_input() {
    let mut srv = TestServer::with_factory(create_app);

    let body = json!({"id": "s35", "name": "test", "roll_no": "int", "attendance": "float"});
    let request = srv.client(Method::POST, "/students").json(body).unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_update_bad_input() {
    let mut srv = TestServer::with_factory(create_app);

    let body = json!({"attendance": "float"});
    let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_update() {
    let mut srv = TestServer::with_factory(create_app);

    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_update_non_existent() {
    let mut srv = TestServer::with_factory(create_app);

    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::GET, "/students/s100")
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_delete() {
    let mut srv = TestServer::with_factory(create_app);

    let request = srv.client(Method::DELETE, "/students/s36")
        .finish()
        .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_delete_non_existent() {
    let mut srv = TestServer::with_factory(create_app);

    let request = srv.client(Method::DELETE, "/students/s100")
        .finish()
        .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
