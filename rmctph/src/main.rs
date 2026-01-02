use tiny_http::{Method, Response, Server};

fn set_volume (volume: u8) {
    use std::ptr;
    use windows::{
        Win32::{
        Media::Audio::{
            Endpoints::IAudioEndpointVolume, IMMDeviceEnumerator, MMDeviceEnumerator, eConsole, eRender
        },
        System::Com::{
            CLSCTX_ALL,
            COINIT_APARTMENTTHREADED,
            CoCreateInstance,
            CoInitializeEx
        }
    }};

    println!("Removing condons and starting integration hell...");
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None, 
            CLSCTX_ALL
        ).unwrap();

        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .unwrap();

        let endpoint: IAudioEndpointVolume = device
            .Activate(CLSCTX_ALL, None)
            .unwrap();

        endpoint
            .SetMasterVolumeLevelScalar((volume as f32) / 100.0, ptr::null())
            .unwrap();
    }

    println!("It worked (theoretically): {}", volume);
}

fn run_server(addr: &str) {
    let server = Server::http(addr).unwrap();
    println!("Starting server at {}...", addr);

    for mut request in server.incoming_requests() {
        if request.method() == &Method::Post && request.url() == "/volume" {
            let mut body = String::new();

            request
                .as_reader()
                .read_to_string(&mut body)
                .unwrap();

            println!("Request from client: {}...", body);

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

fn main() {
    run_server("0.0.0.0:5000");
}
