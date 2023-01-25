
use std::fs;
use micro_http::MediaType;
use crate::request::{Body, StatusCode, Response, Version, Request};

pub(crate) fn parse_signin(_request: &Request) -> Response  {
    println!("request.parse_signin fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    let file_name = format!("{}{}", "/mdb/frontend/", "signin.html");
    println!("Response filename : {}", file_name);
    let content = fs::read_to_string(file_name).unwrap();
    let response_body = content;

    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(response_body));
    response
}