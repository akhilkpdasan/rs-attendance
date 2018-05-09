//extern crate actix;
extern crate actix_web;
extern crate attendance_rs;
#[macro_use]
extern crate serde_json;

use actix_web::http::{Method, StatusCode};
use actix_web::test::TestServer;
use actix_web::HttpMessage;
use attendance_rs::create_app;

#[test]
fn attendance_management_works() {
    let mut srv = TestServer::with_factory(create_app);

    //login with incorrect password
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"bad_pass"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());

    //login non-existing user
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"no_user", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());

    //login
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"test", "password":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    //get token
    let token = response.cookie("token").unwrap().value();

    //who_am_i returns username
    let request = srv.client(Method::GET, "/whoami")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();
    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    let username = srv.execute(response.body()).unwrap();
    assert_eq!(username, "test");

    //register
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test2", "password":"test2", "email":"test2"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    //register bad input
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"bad_input", "password":123, "email":"test2"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());

    //register username exists
    let request = srv.client(Method::POST, "/register")
        .json(json!({"username":"test", "password":"test", "email":"test"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    //get students without token
    let request = srv.client(Method::GET, "/students").finish().unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    //get students
    let request = srv.client(Method::GET, "/students")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //get Student
    let request = srv.client(Method::GET, "/students/s32")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    //get non-existing student
    let request = srv.client(Method::GET, "/students/s100")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    //create new student
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //create new student already existing
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    //create without token
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students").json(body).unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    //create student bad input
    let body = json!({"id": "s35", "name": "test", "roll_no": "int", "attendance": "float"});
    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //update student bad input
    let body = json!({"attendance": "float"});
    let request = srv.client(Method::PUT, "/students/s32")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //update student
    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::PUT, "/students/s32")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    //update non-existing student
    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::GET, "/students/s100")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    //update without token
    let body = json!({"attendance": 33.33});
    let request = srv.client(Method::PUT, "/students/s32").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());

    //delete student
    let request = srv.client(Method::DELETE, "/students/s36")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    //delete student without token
    let request = srv.client(Method::DELETE, "/students/s36")
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_client_error());

    //delete non-existing student
    let request = srv.client(Method::DELETE, "/students/s100")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    //logout
    let request = srv.client(Method::GET, "/logout")
        .header("Cookie", format!("token={}", token))
        .finish()
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    //token expires in year 1970
    let c = response.cookie("token").unwrap();

    assert_eq!(c.expires().map(|t| t.tm_year), Some(70));
}
