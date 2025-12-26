from pycaw.pycaw import AudioUtilities, IAudioEndpointVolume
from flask import Flask, request

app = Flask(__name__)
device = AudioUtilities.GetSpeakers()
volume = device.EndpointVolume.QueryInterface(IAudioEndpointVolume)

@app.route('/volume', methods=['POST'])
def set_volume():
    data = request.get_json()
    level = int(data.get('level'))

    volume.SetMasterVolumeLevelScalar(level / 100.0, None)

@app.route('/')
def index():
    return app.send_static_file('index.html')

if __name__ == '__main__':
    app.run('127.0.0.1', port=5000)