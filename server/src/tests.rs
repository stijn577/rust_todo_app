use super::*;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_json() {
    // set up the server for the test
    let rocket = super::rocket();
    let client = Client::tracked(rocket).unwrap();

    // make the request and receive the response
    let req = client.get("/hello/stijn/19");
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);

    let json_literal = RawJson(r#"{"name":"stijn","age":19}"#.to_string());
    let json_callback = RawJson(
        response
            .into_string()
            .unwrap(),
    );

    assert_eq!(json_callback, json_literal);
}
