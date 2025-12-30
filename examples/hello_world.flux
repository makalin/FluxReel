from fluxreel import Scene, Text, Image, Audio

# Global Settings
setup(res="1080p", fps=60)

scene "Intro":
    # Assets
    bg = Image("assets/background.jpg")
    title = Text("Hello FluxReel", size=100, color="#FFFFFF")
    
    # Declarative Animation
    title.align("center")
    title.fade_in(duration=1.0)
    title.scale(start=0.5, end=1.0, ease="elastic_out")
    
    # Audio Sync
    voice = Audio("assets/intro.mp3")
    voice.play()
    
    # Wait for audio to finish automatically
    wait(voice.duration)

scene "Main":
    transition(effect="slide_left", duration=0.5)
    subtitle = Text("Welcome to FluxReel", size=60, color="#FF6B6B")
    subtitle.align("center")
    subtitle.fade_in(duration=0.8)

