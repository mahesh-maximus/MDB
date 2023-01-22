use std::collections::HashMap;

use crate::request::index::parse_get_index;
use micro_http::{Body, Method, Request, Response, StatusCode, Version, Headers};

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

        match (request.method(), path, request.body.as_ref(), is_authenticated(request.headers.custom_entries())) {
            (Method::Get, "", None, true) => parse_get_index(request),

            (method, unknown_uri, _, _) => {
                println!(
                    "InvalidPathMethod URI: {}, METHOD {}",
                    unknown_uri.to_string(),
                    method.to_str()
                );

                let mut response = Response::new(Version::Http11, StatusCode::OK);
                let response_body = b"InvalidPathMethod response body";
                response.set_body(Body::new(response_body.to_vec()));
                return response;

            }
        }
    }
}

fn is_authenticated(headers: &HashMap<String, String>) -> bool {

    //headers.

    true
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
