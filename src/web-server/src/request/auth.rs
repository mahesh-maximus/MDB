use crate::request::{Body, StatusCode, Response, Version, Cookie};

pub(crate) fn parse_get_auth(body: &Body) -> Response {
    println!("request.parse_get_auth fn");
    let mut is_authenticated = true;
    if body.is_empty() {
        println!("request.parse_get_auth body empty fn");
        is_authenticated = false;
    }

    println!("request.parse_get_auth body not empty fn");

    let mut response = Response::new(Version::Http11, StatusCode::OK);

    let cookie = Cookie::new("auth".to_string(), is_authenticated.to_string());
    response.set_cookie(cookie);

    let response_body = b"{}";
    response.set_body(Body::new(response_body.to_vec()));
    
    response
}