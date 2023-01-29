
use std::fs;
use micro_http::MediaType;
use crate::request::{Body, StatusCode, Response, Version, Request};
use crate::common::get_content;

pub(crate) fn parse_get_index(_request: &Request) -> Response  {
    println!("request.parse_get_index fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    let content = get_content(format!("{}{}", "/mdb/frontend/", "index.html"));
    let response_body = content;

    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(response_body));
    response
}