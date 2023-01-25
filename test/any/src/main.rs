use micro_http::{HttpServer, Response, StatusCode, Body, Version};

fn main() {
    println!("Hello, world!");

    let path_to_socket = "/tmp/example.sock";
    std::fs::remove_file(path_to_socket).unwrap_or_default();

    // Start the server.
    let mut server = HttpServer::new(path_to_socket).unwrap();
    server.start_server().unwrap();

    // Connect a client to the server so it doesn't block in our example.
    let mut socket = std::os::unix::net::UnixStream::connect(path_to_socket).unwrap();

    // Server loop processing requests.
    loop {
        println!("Loop:");
        for request in server.requests().unwrap() {
            println!("Request ____________________________");
           /* 
            let response = request.process(|request| {
                 // Your code here.
                Response::new(request.http_version(), StatusCode::OK)
            });
*/
            let response = request.process(|request| {
                let mut response = Response::new(Version::Http11, StatusCode::OK);
                let response_body = b"response body";
                response.set_body(Body::new(response_body.to_vec()));
                response
            });



            server.respond(response);
            
            println!("Responded >><<");
        }  
        // Break this example loop.
        // break;
    }
}
