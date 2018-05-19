extern crate actix_web;
extern crate attendance_rs;
#[macro_use]
extern crate serde_json;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use actix_web::http::*;
use attendance_rs::schema::*;
use diesel::prelude::*;

mod test_server;

fn get_token(srv: &mut actix_web::test::TestServer) -> String {
    let request = srv.client(Method::POST, "/login")
        .timeout(std::time::Duration::new(120, 0))
        .json(json!({"username":"auth_test", "password":"auth_test_pass"}))
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    let token = response.cookie("token").unwrap().value();
    token.to_string()
}

#[test]
fn create_test() {
    let urls = vec!["/login".to_string(), "/register".to_string()];
    let (mut srv, pool) = test_server::create_server(urls);

    {
        let conn: &PgConnection = &pool.get().expect("Cannot get connection");
        conn.begin_test_transaction()
            .expect("Failed to begin test transaction");

        diesel::insert_into(users::table)
            .values((
                users::username.eq("auth_test"),
                users::password.eq("$2y$12$JRd5xkxekLvUeoTbYa8q3.lyHpHzX/MFS8arbTZN7dOi7RnOoZ7oq"),
                users::email.eq("auth_test_email"),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_into(students::table)
            .values((
                students::id.eq("s32"),
                students::name.eq("bedki"),
                students::roll_no.eq(32),
                students::attendance.eq(12.0),
            ))
            .execute(conn)
            .unwrap();

        diesel::insert_into(students::table)
            .values((
                students::id.eq("s36"),
                students::name.eq("yogesh"),
                students::roll_no.eq(36),
                students::attendance.eq(16.0),
            ))
            .execute(conn)
            .unwrap();
    }

    let token = get_token(&mut srv);

    //create new student
    let body = json!({"id": "s35", "name": "akhil", "roll_no": 35, "attendance": 55.0});

    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    //create new student already existing
    let body = json!({"id": "s35", "name": "already_exist", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    //create student bad input
    let body = json!({"id": "s35", "name": "test", "roll_no": "int", "attendance": "float"});
    let request = srv.client(Method::POST, "/students")
        .header("Cookie", format!("token={}", token))
        .json(body)
        .unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    //create without token fails
    let body = json!({"id": "s888", "name": "akhil", "roll_no": 35, "attendance": 55.0});
    let request = srv.client(Method::POST, "/students").json(body).unwrap();

    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
