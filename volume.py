from pycaw.pycaw import AudioUtilities, IAudioEndpointVolume
from flask import Flask, jsonify, request

app = Flask(__name__)
device = AudioUtilities.GetSpeakers()
volume = device.EndpointVolume.QueryInterface(IAudioEndpointVolume)

@app.route('/volume', methods=['POST'])
def set_volume():
    data = request.get_json()
    level = data.get('level')

    if level is None:
        return jsonify({'error': 'missing level'}), 400 # Bad request
    
    volume.SetMasterVolumeLevelScalar(int(level) / 100.0, None)
    return jsonify({'volume': level}), 200

@app.route('/')
def index():
    return app.send_static_file('index.html')

if __name__ == '__main__':
    app.run('127.0.0.1', port=5000)