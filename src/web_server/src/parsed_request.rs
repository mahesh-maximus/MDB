use std::collections::HashMap;

use crate::request::{index::parse_get_index, signin::parse_signin, auth::parse_get_auth, not_found::parse_not_found};
use micro_http::{Body, Method, Request, Response, StatusCode, Version, Headers, MediaType};

pub(crate) struct ParsedRequest {}

impl ParsedRequest {
    pub(crate) fn try_from_request(request: &Request) -> Response  {
        println!("ParsedRequest.try_from_request fn");
        let request_uri = request.uri().get_abs_path().to_string();
        log_received_api_request(describe(
            request.method(),
            request_uri.as_str(),
            request.body.as_ref(),
        ));


        // Split request uri by '/' by doing:
        // 1. Trim starting '/' characters
        // 2. Splitting by '/'
        let path_tokens: Vec<&str> = request_uri
            .trim_start_matches('/')
            .split_terminator('/')
            .collect();
        let path = if path_tokens.is_empty() {
            ""
        } else {
            path_tokens[0]
        };

        print_headers(&request.headers);

        println!("Request ACCEPT header: {}", request.headers.accept().as_str());
        println!("Request Content_Type header: {}", request.headers.content_type().as_str());
        
        match (request.method(), path, request.headers.content_type(), request.body.as_ref(), is_authenticated(request.headers.custom_entries())) {
            (Method::Get, "", _, None, true) => parse_get_index(request),
            (Method::Post, "auth", MediaType::ApplicationJson, Some(body), false) => parse_get_auth(body),
            (Method::Get, "favicon.ico", _, None, _) => parse_get_favicon(),
            (_, _,_ , _, false) => parse_signin(request),
            (method, unknown_uri,MediaType::TextHtml, _, _) => {
                println!(
                    "ParsedRequest.try_from_request InvalidPathMethod URI: {}, METHOD {}",
                    unknown_uri.to_string(),
                    method.to_str()
                );

                parse_not_found(request)
            },
            _ => {
                println!("ParsedRequest.try_from_request URI cannot parse");
                let mut response = Response::new(Version::Http11, StatusCode::NotFound);
                response.set_content_type(MediaType::ApplicationJson);

                response
            }
        }
    }
}

fn is_authenticated(headers: &HashMap<String, String>) -> bool {
    println!("parsed_request.is_authenticated fn");
    if headers.contains_key("Cookie") {
        println!("parsed_request.is_authenticated auth key is available fn");
        return true
    }

    println!("parsed_request.is_authenticated auth key not found fn");
    false
}

fn parse_get_favicon() -> Response {
    println!("parsed_request.parse_get_favicon fn");
    let response = Response::new(Version::Http11, StatusCode::NotFound);
    return response;
}

fn print_headers(headers: &Headers) {
    for (key, val) in headers.custom_entries().iter() {
        println!("Header custom -> key: {key} val: {val}");
    }
}

/// Helper function for writing the received API requests to the log.
///
/// The `info` macro is used for logging.
#[inline]
fn log_received_api_request(api_description: String) {
    println!("The Web server received a {}.", api_description);
}

/// Helper function for metric-logging purposes on API requests.
///
/// # Arguments
///
/// * `method` - one of `GET`, `PATCH`, `PUT`
/// * `path` - path of the API request
/// * `body` - body of the API request
fn describe(method: Method, path: &str, body: Option<&Body>) -> String {
    match (path, body) {
        ("/mmds", Some(_)) | (_, None) => format!("{:?} request on {:?}", method, path),
        (_, Some(value)) => format!(
            "{:?} request on {:?} with body {:?}",
            method,
            path,
            std::str::from_utf8(value.body.as_slice())
                .unwrap_or("inconvertible to UTF-8")
                .to_string()
        ),
    }
}
