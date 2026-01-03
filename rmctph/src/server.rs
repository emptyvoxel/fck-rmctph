use tiny_http::{Method, Response, Server, Request};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;

mod endpoints;
mod volume;

fn handler(request: Request, endpoint: &IAudioEndpointVolume) {
    match (request.method(), request.url()) {
        (&Method::Post, "/volume") => {
            volume::handler(request, &endpoint);
        }
        (&Method::Post, "/shutdown") => {
            endpoints::shutdown(request);
        }
        (&Method::Get, "/") => {
            endpoints::index(request);
        }
        (&Method::Post, _) => {
            // Invalid endpoint clause
            let _ = request.respond(
                Response::from_string("Bad request")
                    .with_status_code(400)
            );
        }
        _ => {
            // Not POST clause
            let _ = request.respond(
                Response::from_string("Method not allowed")
                    .with_status_code(405)
            );
        }
    }
}

// Closed Server: Handler is restricted to 
fn closed_server(server: Server, endpoint: &IAudioEndpointVolume, allowed: String) {
    println!("[+] Generating whitelist...");
    let whitelist: Vec<String> = allowed
        .split(',')
        .map(|s| s.to_string())
        .collect();

    println!("[+] All operational! Waiting requests...");
    for request in server.incoming_requests() {
        if let Some(remote_addr) = request.remote_addr() {
            let ip = remote_addr.ip().to_string();
            
            if whitelist.contains(&ip) {
                handler(request, &endpoint);
            } else {
                let _ = request.respond(
                    Response::from_string("Forbidden")
                        .with_status_code(403)
                );
            }
        }
    }
}

fn open_server(server:Server, endpoint: &IAudioEndpointVolume) {
    println!("[+] All operational! Waiting requests...");
    for request in server.incoming_requests() {
        handler(request, &endpoint);
    }
}

// The brains of the thing
pub fn run(addr: String, allowed: String) {
    println!("[+] Starting WASAPI stuff...");
    let endpoint: IAudioEndpointVolume = volume::init_volume();

    println!("[+] Starting server at {}...", addr);
    let server = Server::http(&addr).unwrap();
    
    if allowed != "" {
        return closed_server(server, &endpoint, allowed);
    }

    return open_server(server, &endpoint);
}