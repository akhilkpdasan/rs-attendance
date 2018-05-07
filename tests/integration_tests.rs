extern crate actix;
extern crate actix_web;
extern crate attendance_rs;
#[macro_use]
extern crate serde_json;
extern crate futures;

use actix_web::http::{Method, StatusCode};
use actix_web::test::TestServer;
use actix_web::HttpMessage;
use attendance_rs::create_app;
use futures::Future;

struct TestApp {
    server: TestServer,
    token: String,
}

impl TestApp {
    fn new() -> TestApp {
        let mut test_server = TestServer::with_factory(create_app);
        let token = TestApp::get_token(&mut test_server);

        TestApp {
            server: test_server,
            token: token,
        }
    }

    fn get_token(srv: &mut TestServer) -> String {
        let request = srv.client(Method::POST, "/login")
            .timeout(std::time::Duration::new(120, 0))
            .json(json!({"username":"test", "password": "test"}))
            .unwrap();

        let response_bytes = srv.execute(
            request
                .send()
                .map_err(|_| ())
                .and_then(|res| res.body().map_err(|_| ()).and_then(|bytes| Ok(bytes))),
        ).unwrap();

        String::from_utf8(response_bytes.to_vec()).unwrap()
    }
}

#[test]
fn login() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn login_bad_pass() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"bad_pass"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn login_no_user() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"no_user", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn register() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test2", "password":"test2", "email":"test2"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn register_bad_input() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"bad_input", "password":123, "email":"test2"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn register_user_exists() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test", "password":"test", "email":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn get_without_auth() {
    let mut srv = TestServer::with_factory(create_app);
    let request = srv.client(Method::GET, "/students").finish().unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn get_all_students() {
    let mut app = TestApp::new();
    let request = app.server
        .client(Method::GET, "/students")
        .header("Authorization", format!("Bearer {}", app.token))
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn get_student() {
    let mut app = TestApp::new();
    let request = app.server
        .client(Method::GET, "/students/s32")
        .header("Authorization", format!("Bearer {}", app.token))
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn get_non_existent() {
    let mut app = TestApp::new();
    let request = app.server
        .client(Method::GET, "/students/s100")
        .header("Authorization", format!("Bearer {}", app.token))
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn new_student() {
    let mut app = TestApp::new();
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = app.server
        .client(Method::POST, "/students")
        .header("Authorization", format!("Bearer {}", app.token))
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn new_without_token() {
    let mut app = TestApp::new();
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = app.server
        .client(Method::POST, "/students")
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn new_bad_input() {
    let mut app = TestApp::new();

    let body = json!({"id": "s35", "name": "test", "roll_no": "int", "attendance": "float"});
    let request = app.server
        .client(Method::POST, "/students")
        .header("Authorization", format!("Bearer {}", app.token))
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn update_bad_input() {
    let mut app = TestApp::new();

    let body = json!({"attendance": "float"});
    let request = app.server
        .client(Method::PUT, "/students/s32")
        .header("Authorization", format!("Bearer {}", app.token))
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn update_student() {
    let mut app = TestApp::new();

    let body = json!({"attendance": 33.33});
    let request = app.server
        .client(Method::PUT, "/students/s32")
        .header("Authorization", format!("Bearer {}", app.token))
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_success());
}

#[test]
fn update_non_existent() {
    let mut app = TestApp::new();

    let body = json!({"attendance": 33.33});
    let request = app.server
        .client(Method::GET, "/students/s100")
        .header("Authorization", format!("Bearer {}", app.token))
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn update_without_token() {
    let mut app = TestApp::new();

    let body = json!({"attendance": 33.33});
    let request = app.server
        .client(Method::PUT, "/students/s32")
        .json(body)
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn delete_student() {
    let mut app = TestApp::new();

    let request = app.server
        .client(Method::DELETE, "/students/s36")
        .header("Authorization", format!("Bearer {}", app.token))
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn delete_without_token() {
    let mut app = TestApp::new();

    let request = app.server
        .client(Method::DELETE, "/students/s36")
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());
}

#[test]
fn delete_non_existent() {
    let mut app = TestApp::new();

    let request = app.server
        .client(Method::DELETE, "/students/s100")
        .header("Authorization", format!("Bearer {}", app.token))
        .finish()
        .unwrap();

    let response = app.server.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
