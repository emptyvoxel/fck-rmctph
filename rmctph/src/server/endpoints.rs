use tiny_http::{Request, Response, Header};

// Shutdown API handler
pub fn shutdown(request: Request) {
    let _ = request.respond(
        Response::from_string("Shutting down")
            .with_status_code(200)
    );
}

// Index page renderer
pub fn index(request: Request) {
    let html = include_str!("index.html");

    let response = Response::from_string(html)
        .with_header("Content-Type: text/html".parse::<Header>().unwrap());

    let _ = request.respond(response);
}