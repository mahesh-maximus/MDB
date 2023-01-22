
use std::fs;

use micro_http::MediaType;

use crate::request::{Body, Method, StatusCode, Response, Version, Request, Headers, Cookie};

pub(crate) fn parse_get_index(request: &Request) -> Response  {
    println!("request.parse_get_index fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    let file_name = format!("{}{}", "/mdb/frontend/", "index.html");
    println!("Response filename : {}", file_name);
    let content = fs::read_to_string(file_name).unwrap();
    let response_body = content;

    let cookie = Cookie::new("test-key".to_string(), "test-value".to_string());
    response.set_cookie(cookie);

    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(response_body));
    response
}