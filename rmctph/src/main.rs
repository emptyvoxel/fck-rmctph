mod server;

fn main() {
    use std::env;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:5000".to_string());

    let allowed = env::args()
        .nth(2)
        .unwrap_or("".to_string());

    server::run(addr, allowed);
}
