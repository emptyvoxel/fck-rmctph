use tiny_http::{Method, Response, Server};

fn set_volume (volume: u8) {
    println!("volume set to {}", volume);
}

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

            if let Some(level) = body.strip_prefix("level=") {
                if let Ok(volume) = level.parse::<u8>() {
                    if volume <= 100 {
                        set_volume(volume);

                        let _ = request.respond(
                            Response::from_string("OK")
                                .with_status_code(200)
                        );

                        continue;
                    }
                }
            }

            let _ = request.respond(
                Response::from_string("Bad request")
                    .with_status_code(400)
            );
        } else {
            let _ = request.respond(
                Response::from_string("Something Went Wrong")
                    .with_status_code(500)
            );
        }
    }
}
