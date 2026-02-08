from fluxreel import Scene, Text, StreamingConfig, Streamer

# Setup
setup(res="1080p", fps=30)

# Create streaming configuration
config = StreamingConfig(
    url="rtmp://a.rtmp.youtube.com/live2/your-stream-key",
    stream_key="your-stream-key-here"
)
config.set_protocol("rtmp")
config.set_quality("high")  # 1080p, 5Mbps
config.reconnect = True

# Start streaming
streamer = Streamer(config)
streamer.start()

# Create live scene
scene = Scene("Live")
title = Text("Live on FluxReel", size=100, color="#FF0000")
title.align("center")

# Stream frames
# (In actual implementation, frames would be sent continuously)
# streamer.send_frame(frame_data)

