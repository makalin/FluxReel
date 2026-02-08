from fluxreel import Scene, Image, Video, Audio, ImageGenerator, VideoGenerator, AudioGenerator

# Setup
setup(res="1080p", fps=60)

# Generate AI assets
img_gen = ImageGenerator("stability", "your-api-key")
img_gen.width = 1920
img_gen.height = 1080

# Generate background image
bg_image = img_gen.generate("A futuristic workspace with neon lights, cyberpunk aesthetic")
bg = Image(bg_image)

# Generate video clip
vid_gen = VideoGenerator("runway", "your-api-key")
vid_gen.duration = 5.0
video_clip = vid_gen.generate("Abstract particles floating in space")
video = Video(video_clip)

# Generate voiceover
audio_gen = AudioGenerator("elevenlabs", "your-api-key")
audio_gen.voice = "professional"
voiceover = audio_gen.generate_speech("Welcome to the future of video creation with AI-powered tools.")
audio = Audio(voiceover)

# Create scene with AI-generated assets
scene = Scene("AI Generated")
scene.add_node(bg)
scene.add_node(video)
scene.add_node(audio)

