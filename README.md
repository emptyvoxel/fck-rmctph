# fck-rmctph

Basically, my TV remote decided to stop working. Since I only needed the volume buttons, I built this tiny <s>Flask</s> Rust server to control Windows volume over LAN instead.

## that-escalated version

Here's the thing: the HTML/JS front-end gets the job done! But something was bothering me: to change my PC volume, I had to unlock my phone, open my browser and move the slider. Couldn't I just use my phone's volume buttons? As it turns out: no! I can't. 

Even though the [W3C Recommendations](https://www.w3.org/TR/uievents-key/#keys-audio) provide codes for the volume buttons (KEYCODE_VOLUME_UP/DOWN), most browsers don't recognize them as keyboard events. [Why isn't it possible? It's just not](https://www.youtube.com/watch?v=coY2IA-oBvw).

If I wanted to do this, I would have to rewrite the whole front-end in Kotlin, so I could use those key codes. And let's be honest: this is just a stupid 20 minute code adventure, I'm not gonna download Android Studio and learn a new language...

So I downloaded Android Studio and opened its Kotlin tutorials.

It's just a front-end update. The Flask server works just fine! It's just a simple HTTP server that listens to a single endpoint. Nothing wrong with that.

But then, something struck me: say someone comes to my house and we're going to watch something on their computer. We plug the computer in my TV, but now it's their computer's volume that needs to be controlled from a phone. The person could clone my repo, install Python, run `pip install -r requirements.txt` and things would work fine. But that's quite clumsy, isn't it?

So, in the span of a single day, what was supposed to be a 20 minute code adventure became a nightmare. Now I'm rewriting the backend in Rust, and adding an alternative, Android frontend. So yeah: things escalated quite quickly.
