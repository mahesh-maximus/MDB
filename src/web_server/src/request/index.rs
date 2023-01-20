
use crate::request::{Body, Method, StatusCode, Response, Version};

pub(crate) fn parse_get_index() -> Response  {
    println!("request.parse_get_index fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    let response_body = b"request.parse_get_index fn response body";
    response.set_body(Body::new(response_body.to_vec()));
    response
}