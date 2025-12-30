from fluxreel import Scene, Text, Image

# Global Settings for TikTok/Reels format
setup(res="9:16", fps=60)

scene "VerticalIntro":
    bg = Image("assets/vertical_bg.jpg")
    title = Text("FluxReel", size=120, color="#FFFFFF")
    subtitle = Text("Code Less. Render Faster.", size=50, color="#CCCCCC")
    
    title.align("center")
    title.fade_in(duration=1.0)
    
    subtitle.align("center")
    subtitle.position.y = -0.2
    subtitle.fade_in(duration=1.2)

