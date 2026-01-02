use tiny_http::{Method, Response, Server};

fn main() {
    let ip_addr: &str = "0.0.0.0:5000";
    let server = Server::http(ip_addr).unwrap();

    for mut request in server.incoming_requests() {
        if request.method() == &Method::Post && request.url() == "/volume" {
            let mut body = String::new();

            request
                .as_reader()
                .read_to_string(&mut body)
                .unwrap();

            println!("{}", body);

            let _ = request.respond(
                Response::from_string(body)
                    .with_status_code(200)
            );
        } else {
            let _ = request.respond(
                Response::from_string("Something Went Wrong")
                    .with_status_code(400)
            );
        }
    }
}
