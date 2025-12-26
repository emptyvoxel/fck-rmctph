from pycaw.pycaw import AudioUtilities, IAudioEndpointVolume
from flask import Flask, jsonify, render_template, request
from sys import argv

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
    return jsonify({'volume': level}), 200 # OK

@app.route('/')
def index():
    level = int(volume.GetMasterVolumeLevelScalar() * 100)
    return render_template('index.html', value=level)

if __name__ == '__main__':
    app.run(host=argv[1], port=5000)