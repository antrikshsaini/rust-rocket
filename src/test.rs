use rocket::{local::blocking::Client, http::{Status, Header,},};

#[test]
fn test_live() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.into_string(), Some("App is Live!!".into()));
}


#[test]
fn test_custom_greeting_without_salutation() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/greet?name=antrix").dispatch();
    assert_eq!(response.into_string(), Some("Welcome antrix".into()));
}

#[test]
fn test_custom_greeting_with_salutation() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/greet?name=antrix&salutation=Mr.").dispatch();
    assert_eq!(response.into_string(), Some("Welcome Mr. antrix".into()));
}

#[test]
fn test_auth_missing_key() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/auth").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_auth_invalid_key() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/auth")
        .header(Header::new("x-api-key", "yoooooo"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_auth_key() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/auth")
        .header(Header::new("x-api-key", "secretkey"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}