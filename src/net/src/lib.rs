mod http_request_processor;
mod ws_request_processor;

use http_request_processor::process_http_requests;
use ws_request_processor::process_ws_requests;

pub fn p_http_r() {
    process_http_requests();
}

pub fn p_ws_r() {
    process_ws_requests();
}

