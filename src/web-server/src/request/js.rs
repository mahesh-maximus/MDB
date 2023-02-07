use micro_http::MediaType;
use crate::{request::{Body, StatusCode, Response, Version}};
use crate::common::get_content;

pub(crate) fn parse_js(path: String) -> Response  {
    println!("request.parse_js fn path: {path}");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(get_content(format!("{}{}", "/mdb/frontend/", path))));
    response
}