use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use tiny_http::{Request, Response};

// This function initializes the unsafe WASAPI stuff.
// This code only works in STA mode. This shouldn't be a problem, because 
//   tiny_http is single-threaded. 
pub fn init_volume() -> IAudioEndpointVolume {
    use windows::{
        Win32::{
            Media::Audio::{
                Endpoints::IAudioEndpointVolume, 
                IMMDeviceEnumerator, 
                MMDeviceEnumerator, 
                eConsole, 
                eRender
            },
            System::Com::{
                CLSCTX_ALL,
                COINIT_APARTMENTTHREADED,
                CoCreateInstance,
                CoInitializeEx
            }
        }
    };

    // Remove your condoms, cause things are getting unsafe!
    unsafe {
        // Initialize the COM Library for the current thread in STA mode.
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        // Create an enumerator that allows to query audio devices. 
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None, 
            CLSCTX_ALL
        ).unwrap();

        // Gets the current audio device.
        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .unwrap();

        // Activate the volume interface on the current device.
        let endpoint: IAudioEndpointVolume = device
            .Activate(CLSCTX_ALL, None)
            .unwrap();

        // Return for set_volume use.
        return endpoint; 
    }
}

// This function sets (set) the volume (volume): that's why it's called set_volume!
fn set_volume(volume: u8, endpoint: &IAudioEndpointVolume) {
    use std::ptr;
    unsafe {
        endpoint
            .SetMasterVolumeLevelScalar((volume as f32) / 100.0, ptr::null())
            .unwrap();
    }
}

// Volume API handler
pub fn handler(mut request: Request, endpoint: &IAudioEndpointVolume) {
    let mut body = String::new();

    request
        .as_reader()
        .read_to_string(&mut body)
        .unwrap();

    if let Some(level) = body.strip_prefix("level=") {
        if let Ok(volume) = level.parse::<u8>() {
            if volume <= 100 {
                set_volume(volume, &endpoint);

                let _ = request.respond(
                    Response::from_string("OK")
                        .with_status_code(200)
                );
            }
        }
    } else {
        let _ = request.respond(
            Response::from_string("Bad request")
                .with_status_code(400)
        );
    }
}
