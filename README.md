<div align="center">

<img src="fluxreel_logo.svg" alt="FluxReel Logo" width="100%">

# FluxReel 🎬
### Code Less. Render Faster.

**FluxReel** is a high-performance, programmable video engine designed for developers who find existing tools too complex. It combines a native Rust core with a simplified Pythonic scripting layer, offering the perfect balance between terminal automation and GUI visualization.

[Features](#-key-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [CLI Commands](#-cli-commands) • [Author](#-author)

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)
![License](https://img.shields.io/badge/license-MIT-orange)

</div>

---

## 🚀 Why FluxReel?

Existing programmatic video tools (like Motion Canvas or Manim) are powerful but often suffer from heavy web-based architectures, complex generator syntax, or slow rendering times.

**FluxReel** strips away the complexity:
* **Native Speed:** Built on a Rust rendering engine with Python bindings.
* **Dual Workflow:** Compile from the CLI *or* edit visually in the Studio GUI.
* **Simple Syntax:** No complex async/await or generator functions. Just linear, readable scripts.
* **Professional Tools:** Color grading, motion tracking, masking, professional audio processing, and more.
* **Rich Feature Set:** 30+ easing functions, transitions, effects, and comprehensive video editing tools.

## ✨ Key Features

### 🎞️ Flexible Rendering
* **Multi-Format:** Output to `.mp4`, `.mov` (ProRes), `.gif`, `.webp`, or `PNG Sequence`.
* **Resolutions:** Built-in presets for 4K, 1080p (YouTube), 720p, and 9:16 (TikTok/Reels).
* **High FPS:** Support for 120+ FPS for high-quality slow-motion editing.
* **Custom Resolutions:** Support for any custom width×height resolution.

### 🎛️ Architecture
* **Unified Layer System:** Text, Images, Videos, and Shapes are all `Nodes` with uniform properties.
* **Smart Transitions:** 12+ transition effects (fade, slide, zoom, wipe, blur, glitch, etc.).
* **Audio-Reactive:** Built-in beat detection and BPM calculation to drive animation properties automatically.
* **Keyframe Animation:** Advanced keyframe-based animation system with easing support.

### 🎨 Visual Effects
* **Blur & Glow:** Apply blur and glow effects to any node.
* **Shadows:** Drop shadows with customizable offset, blur, and color.
* **Color Adjustment:** Brightness, contrast, saturation, and hue controls.
* **Chroma Key:** Green screen / chroma key support.
* **Noise Effects:** Add grain and noise effects.
* **Blend Modes:** 20+ blend modes (Multiply, Screen, Overlay, Soft Light, etc.).

### 🎨 Professional Color Grading
* **Color Wheels:** Lift/Gamma/Gain three-point color correction.
* **Color Curves:** Luma and RGB channel curves for precise color control.
* **LUT Support:** Load and apply Look-Up Tables (.cube, .3dl files).
* **Temperature & Tint:** White balance controls.
* **Exposure & Contrast:** Global exposure and contrast adjustments.

### 🎭 Masking & Rotoscoping
* **Bezier Masks:** Custom shapes with bezier curve control.
* **Rectangle & Ellipse Masks:** Geometric masks with feathering.
* **Mask Properties:** Feather, expansion, inversion, and opacity controls.
* **Point-in-Mask Testing:** Check if points are inside masks.

### 🎯 Motion Tracking & Stabilization
* **Point Tracking:** Track single points across frames.
* **Motion Tracker:** Configurable search radius and pattern size.
* **Stabilizer:** Warp stabilizer with smoothing and auto-crop options.
* **Smooth Tracks:** Reduce jitter in tracking data.

### 🎵 Professional Audio Processing
* **Equalizer:** Multi-band parametric EQ with customizable bands.
* **Compressor:** Dynamic range compression with attack/release controls.
* **Reverb:** Room reverb simulation with size, damping, and wet/dry controls.
* **Limiter:** Peak limiting with lookahead to prevent clipping.
* **Audio Mixer:** Multi-track mixing with individual volume control.

### 📐 Geometry & Math
* **Shape Primitives:** Rectangle, Circle, and Polygon classes with collision detection.
* **Point Utilities:** Distance, angle calculation, and rotation functions.
* **Easing Functions:** 30+ easing functions (linear, quadratic, cubic, elastic, bounce, etc.).

### 🎵 Audio Processing
* **Audio Tracks:** Load and manipulate audio files.
* **Beat Detection:** Energy-based beat detection algorithm.
* **BPM Calculation:** Automatic BPM calculation from beat times.
* **Audio Mixer:** Multi-track audio mixing with volume control.
* **Waveform Analysis:** Extract waveform data and amplitude at specific times.

### ⏱️ Time & Speed Control
* **Speed Ramping:** Variable speed changes with keyframes and easing.
* **Time Remapping:** Control playback speed over time.
* **Frame Blending:** Smooth slow motion with frame interpolation.
* **Frame Conversion:** Convert between frames and seconds.
* **Time Formatting:** Format time as HH:MM:SS.mmm.
* **Time Parsing:** Parse time strings to seconds.

### 📡 Live Streaming
* **RTMP Streaming:** Stream directly to RTMP servers (YouTube, Twitch, etc.).
* **Multiple Protocols:** Support for RTMP, RTSP, SRT, WebRTC, and HLS.
* **Quality Presets:** Pre-configured quality settings (Low, Medium, High, Ultra).
* **Custom Quality:** Define custom bitrate, resolution, and FPS.
* **Auto-Reconnect:** Automatic reconnection on connection loss.
* **Stream Statistics:** Monitor frames sent, bitrate, and dropped frames.

### 🎥 Multi-Camera Editing
* **Multiple Angles:** Add and manage multiple camera angles.
* **Auto-Sync:** Sync cameras by timecode, audio, or in-point.
* **Angle Switching:** Quick cuts between camera angles.
* **Cut Management:** Add and manage cuts with transitions.
* **Preview Modes:** View active angle, all angles, or quad view.
* **Timecode Display:** Show timecode overlay during editing.

### 🤖 AI Asset Generation
* **Image Generation:** Generate images from text prompts (Stability AI, DALL-E, Midjourney).
* **Video Generation:** Create videos from text descriptions (RunwayML).
* **Audio Generation:** Text-to-speech and music generation (ElevenLabs).
* **Text Generation:** Generate scripts and subtitles using LLMs.
* **Batch Generation:** Generate multiple assets at once.
* **Image Upscaling:** Upscale generated images.
* **Video Extension:** Extend videos with AI.

## 📦 Installation

**FluxReel** is available as a single binary or a Python package.

### Building from Source

#### Prerequisites
- Rust (latest stable)
- Python 3.8+
- Cargo

#### Build Rust Components
```bash
cd rust
cargo build --release
```

#### Install Python Package
```bash
cd python
pip install -e .
```

#### Install CLI Tool
```bash
cd rust/fluxreel-cli
cargo install --path .
```

### via PIP (Python) - Coming Soon
```bash
pip install fluxreel
```

### via Cargo (Rust) - Coming Soon
```bash
cargo install fluxreel-cli
```

## ⚡ Quick Start

### Create a New Project

```bash
fluxreel new my_first_video
cd my_first_video
```

Available templates:
- `default` - Full-featured template with intro scene
- `minimal` - Minimal template with basic setup
- `vertical` - Template optimized for 9:16 format

```bash
fluxreel new my_video --template minimal
```

### Basic Example

Edit `main.flux` (or `main.py`) with your script:

```python
from fluxreel import Scene, Text, Image, Audio, setup

# Global Settings
setup(res="1080p", fps=60)

# Create intro scene
intro_scene = Scene("Intro")
bg = Image("assets/background.jpg")
title = Text("Hello FluxReel", size=100, color="#FFFFFF")

# Declarative Animation
title.align("center")
title.fade_in(duration=1.0)
title.scale(start=0.5, end=1.0, ease="elastic_out")

# Audio Sync
voice = Audio("assets/intro.mp3")
voice.play()

# Create main scene
main_scene = Scene("Main")
transition(effect="slide_left", duration=0.5)
```

### Advanced Example with Effects

```python
from fluxreel import (
    Scene, Text, Image, 
    GlowEffect, ShadowEffect, BlurEffect,
    Animation, AnimationSequence
)

setup(res="1080p", fps=60)

scene = Scene("Advanced")
title = Text("FluxReel", size=120, color="#FF6B6B")

# Add visual effects
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")

# Keyframe animation
anim = Animation("opacity", duration=2.0)
anim.add_keyframe(0.0, 0.0, "ease_in")
anim.add_keyframe(1.0, 1.0, "ease_out")
anim.infinite()  # Loop infinitely
```

### Professional Color Grading Example

```python
from fluxreel import ColorGrading, ColorCurves, LUT

# Color grading with wheels
grade = ColorGrading()
grade.set_lift(r=0.05, g=0.0, b=-0.05)  # Cool shadows
grade.set_gamma(r=0.0, g=0.03, b=0.0)  # Slight green in midtones
grade.set_gain(r=1.05, g=1.0, b=0.95)  # Warm highlights
grade.temperature = 10.0  # Warmer
grade.contrast = 15.0
grade.saturation = 110.0

# Color curves
curves = ColorCurves()
curves.add_point("luma", 0.5, 0.45)  # S-curve for contrast
curves.add_point("red", 0.3, 0.35)  # Lift reds in shadows

# Apply LUT
lut = LUT("cinematic_look.cube")
lut.intensity = 0.8
```

### Motion Tracking & Masking Example

```python
from fluxreel import MotionTracker, Stabilizer, Mask, RectangleMask

# Stabilize shaky footage
stabilizer = Stabilizer()
stabilizer.smoothing = 0.7
stabilizer.method = "all"
stabilized = stabilizer.stabilize("shaky_footage.mp4")

# Track a point
tracker = MotionTracker()
tracker.track_point(start_x=320, start_y=240, start_frame=0)
position = tracker.get_position_at_frame(100)

# Create mask
mask = Mask()
mask.add_point(100, 100)
mask.add_point(500, 100)
mask.add_point(500, 400)
mask.add_point(100, 400)
mask.feather = 20.0
```

### Professional Audio Processing Example

```python
from fluxreel import Equalizer, Compressor, Reverb, Limiter

# Multi-band EQ
eq = Equalizer()
eq.add_band(frequency=80.0, gain=3.0, q=1.0, band_type="low_shelf")  # Boost bass
eq.add_band(frequency=2000.0, gain=1.5, q=1.5, band_type="peak")  # Boost presence

# Compressor
compressor = Compressor()
compressor.threshold = -12.0
compressor.ratio = 4.0
compressor.attack = 3.0
compressor.release = 100.0
compressor.makeup_gain = 3.0

# Reverb
reverb = Reverb()
reverb.room_size = 0.6
reverb.wet_level = 0.25
reverb.dry_level = 0.75

# Limiter
limiter = Limiter()
limiter.ceiling = -0.1
```

### Speed Ramping Example

```python
from fluxreel import SpeedRamp, TimeRemap

# Variable speed
speed_ramp = SpeedRamp()
speed_ramp.frame_blending = True  # Smooth slow motion
speed_ramp.add_keyframe(time=0.0, speed=1.0, ease="linear")  # Normal
speed_ramp.add_keyframe(time=2.0, speed=0.5, ease="ease_in_out")  # Slow to 50%
speed_ramp.add_keyframe(time=4.0, speed=2.0, ease="ease_in_out")  # Speed up to 200%

# Time remapping
time_remap = TimeRemap()
time_remap.maintain_pitch = True
time_remap.set_speed(time=0.0, speed=1.0)
time_remap.set_speed(time=5.0, speed=0.25)  # Slow motion
```

### Live Streaming Example

```python
from fluxreel import StreamingConfig, RTMPStreamer, Streamer

# Create RTMP streamer
rtmp = RTMPStreamer(
    server_url="rtmp://a.rtmp.youtube.com/live2",
    stream_key="your-stream-key-here"
)

# Configure streaming
config = StreamingConfig(
    url="rtmp://a.rtmp.youtube.com/live2/your-stream-key",
    stream_key="your-stream-key-here"
)
config.set_protocol("rtmp")
config.set_quality("high")  # 1080p, 5Mbps
config.reconnect = True
config.reconnect_delay = 5

# Start streaming
streamer = Streamer(config)
streamer.start()

# Send frames (in actual implementation)
# streamer.send_frame(frame_data)

# Get statistics
stats = streamer.get_stats()
print(f"Frames sent: {stats[0]}, Connected: {stats[1]}")

# Stop streaming
streamer.stop()
```

### Multi-Camera Editing Example

```python
from fluxreel import MultiCamEditor, CameraAngle

# Create multi-cam editor
editor = MultiCamEditor()

# Add camera angles
editor.add_camera("Camera A", "footage/camera_a.mp4")
editor.add_camera("Camera B", "footage/camera_b.mp4")
editor.add_camera("Camera C", "footage/camera_c.mp4")
editor.add_camera("Wide Shot", "footage/wide.mp4")

# Auto-sync cameras by audio
editor.auto_sync()

# Add cuts between angles
editor.cut_to_angle(time=0.0, angle_name="Wide Shot")
editor.cut_to_angle(time=5.0, angle_name="Camera A")
editor.cut_to_angle(time=10.0, angle_name="Camera B")
editor.cut_to_angle(time=15.0, angle_name="Camera C")

# Preview all angles
angles = editor.sequence.preview_all_angles()
for angle in angles:
    print(angle)

# Render final multi-cam edit
editor.render("multicam_final.mp4")
```

### AI Asset Generation Example

```python
from fluxreel import ImageGenerator, VideoGenerator, AudioGenerator, TextGenerator, generate_asset

# Image generation
img_gen = ImageGenerator("stability", "your-api-key")
img_gen.width = 1920
img_gen.height = 1080
img_gen.generator.style = "cinematic"
image = img_gen.generate("A futuristic cityscape at sunset, cyberpunk style")

# Video generation
vid_gen = VideoGenerator("runway", "your-api-key")
vid_gen.duration = 5.0
video = vid_gen.generate("A drone flying through a mountain range")

# Audio generation (Text-to-Speech)
audio_gen = AudioGenerator("elevenlabs", "your-api-key")
audio_gen.voice = "professional"
audio_gen.language = "en"
speech = audio_gen.generate_speech("Welcome to FluxReel, the future of video editing.")

# Music generation
music = audio_gen.generate_music("upbeat electronic background music", duration=30.0)

# Text generation (Scripts)
text_gen = TextGenerator("openai", "your-api-key")
script = text_gen.generate_script("Create a 30-second product launch video script")

# Quick asset generation
asset = generate_asset("A beautiful sunset over the ocean", "image")
```

### Render Your Project

```bash
# Basic render
fluxreel render main.flux -o output.mp4

# With options
fluxreel render main.flux -o output.mp4 --resolution 4K --fps 120 --quality high
```

### Launch Studio GUI

```bash
fluxreel studio
# Or with a project
fluxreel studio my_first_video
```

## 💻 CLI Commands

### `fluxreel new <name> [--template <template>]`
Create a new FluxReel project.

```bash
fluxreel new my_video
fluxreel new my_video --template vertical
```

### `fluxreel render <input> [options]`
Render a script to video.

**Options:**
- `-o, --output <path>` - Output file path
- `-r, --resolution <res>` - Resolution override (4K, 1080p, 720p, 9:16)
- `-f, --fps <fps>` - FPS override
- `-q, --quality <quality>` - Quality preset (low, medium, high, ultra)

```bash
fluxreel render main.flux -o output.mp4 --resolution 4K --fps 60
```

### `fluxreel studio [project]`
Launch FluxReel Studio GUI.

```bash
fluxreel studio
fluxreel studio my_project
```

### `fluxreel copilot <prompt> [--output <file>]`
Generate code from natural language prompt.

```bash
fluxreel copilot "Create a scene with a title that fades in"
```

### `fluxreel extract-audio <input> [--output <path>]`
Extract audio from video file.

```bash
fluxreel extract-audio video.mp4 -o audio.mp3
```

### `fluxreel convert <input> <output> [--format <format>]`
Convert video format.

```bash
fluxreel convert input.mp4 output.mov --format mov
```

### `fluxreel analyze-audio <input> [--beats] [--bpm]`
Analyze audio file.

```bash
fluxreel analyze-audio track.mp3 --beats --bpm
```

### `fluxreel templates`
List available project templates.

### `fluxreel validate <input>`
Validate script syntax.

```bash
fluxreel validate main.flux
```

## 📚 Documentation

### Easing Functions

FluxReel supports 30+ easing functions for smooth animations:

**Basic:**
- `linear` - No easing

**Quadratic:**
- `ease_in`, `ease_out`, `ease_in_out`
- `quad_in`, `quad_out`, `quad_in_out`

**Cubic:**
- `cubic_in`, `cubic_out`, `cubic_in_out`

**Advanced:**
- `elastic_in`, `elastic_out`, `elastic_in_out`
- `bounce_in`, `bounce_out`, `bounce_in_out`
- `back_in`, `back_out`, `back_in_out`
- `sine_in`, `sine_out`, `sine_in_out`
- `expo_in`, `expo_out`, `expo_in_out`
- `circ_in`, `circ_out`, `circ_in_out`

### Transitions

Available transition effects:
- `fade` - Crossfade between scenes
- `slide_left`, `slide_right`, `slide_up`, `slide_down`
- `zoom_in`, `zoom_out`
- `wipe_left`, `wipe_right`, `wipe_up`, `wipe_down`
- `rotate`, `blur`, `glitch`, `pixelate`

### Node Methods

All nodes support these methods:
- `align(alignment)` - Align node (center, left, right, top, bottom)
- `move_to(x, y)` - Move to absolute position
- `move_by(dx, dy)` - Move by relative offset
- `rotate(angle)` - Set rotation angle
- `rotate_by(angle)` - Rotate by relative angle
- `set_opacity(opacity)` - Set opacity (0.0 to 1.0)
- `set_scale(scale_x, scale_y)` - Set scale
- `set_visible(visible)` - Show/hide node
- `fade_in(duration)` - Fade in animation
- `fade_out(duration)` - Fade out animation
- `scale(start, end, ease)` - Scale animation
- `slide(start_x, start_y, end_x, end_y, ease)` - Slide animation

### Visual Effects

```python
from fluxreel import BlurEffect, GlowEffect, ShadowEffect, ColorAdjustEffect

# Blur
blur = BlurEffect(radius=10.0)

# Glow
glow = GlowEffect(intensity=1.5, color="#FF0000", radius=20.0)

# Shadow
shadow = ShadowEffect(offset_x=5.0, offset_y=5.0, blur=10.0, color="#000000")

# Color adjustment
color_adj = ColorAdjustEffect()
color_adj.brightness = 0.2
color_adj.contrast = 1.2
color_adj.saturation = 1.5
color_adj.hue = 30.0
```

### Audio Processing

```python
from fluxreel import Audio, AudioMixer, detect_beats_energy, calculate_bpm

# Load audio
audio = Audio("track.mp3")

# Detect beats
beats = audio.detect_beats()
bpm = calculate_bpm(beats)

# Audio mixer
mixer = AudioMixer()
mixer.add_track(audio)
mixer.set_volume(0, 0.8)
mixer.master_volume = 0.9
```

### Geometry Utilities

```python
from fluxreel import Rectangle, Circle, Polygon, point_distance, rotate_point

# Create shapes
rect = Rectangle(0, 0, 100, 100)
circle = Circle(50, 50, 25)
polygon = Polygon([(0, 0), (100, 0), (50, 100)])

# Check collisions
if rect.contains(50, 50):
    print("Point is inside rectangle")

# Calculate distance
dist = point_distance(0, 0, 100, 100)

# Rotate point
x, y = rotate_point(10, 10, 0, 0, 45)  # Rotate 45 degrees
```

### Time Utilities

```python
from fluxreel import frames_to_seconds, seconds_to_frames, format_time, parse_time

# Convert frames to seconds
seconds = frames_to_seconds(1800, 60)  # 30 seconds at 60fps

# Convert seconds to frames
frames = seconds_to_frames(30.0, 60)  # 1800 frames

# Format time
time_str = format_time(125.5)  # "00:02:05.500"

# Parse time
seconds = parse_time("02:05.500")  # 125.5 seconds
```

## 🗺️ Roadmap

### ✅ Completed
- [x] Core Rendering Engine (Rust)
- [x] Python Bindings (PyO3)
- [x] CLI Tool with multiple commands
- [x] 30+ Easing Functions
- [x] Transition Effects
- [x] Visual Effects (Blur, Glow, Shadow, etc.)
- [x] Audio Processing (Beat Detection, BPM, Mixer)
- [x] Geometry Utilities
- [x] Animation System (Keyframes)
- [x] Time Utilities
- [x] **Professional Color Grading** (Color Wheels, Curves, LUTs)
- [x] **Masking & Rotoscoping** (Bezier, Rectangle, Ellipse)
- [x] **Motion Tracking & Stabilization**
- [x] **Professional Audio** (EQ, Compressor, Reverb, Limiter)
- [x] **Speed Ramping & Time Remapping**
- [x] **Blend Modes** (20+ blend modes)
- [x] **Live Streaming** (RTMP, RTSP, SRT, WebRTC, HLS)
- [x] **Multi-Camera Editing** (Auto-sync, angle switching, cut management)
- [x] **AI Asset Generation** (Images, Videos, Audio, Text)

### 🚧 In Progress
- [ ] **FluxReel Studio** (GUI Editor with Timeline)
- [ ] GPU Shader Support (GLSL)
- [ ] Actual Video Rendering Implementation
- [ ] Graph Editor for Keyframes
- [ ] Real-time Preview

### 📋 Planned
- [ ] Webcam/Screen Capture Support
- [ ] Advanced Particle System
- [ ] 3D Transformations
- [ ] Multi-threaded Rendering
- [ ] Proxy Editing
- [ ] Multi-Camera Editing
- [ ] Nested Sequences
- [ ] Color Scopes (Waveform, Vectorscope, Histogram)

## 🤝 Contributing

Contributions are welcome! Please read `CONTRIBUTING.md` for details on our code of conduct and the process for submitting pull requests.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/makalin/FluxReel.git
cd FluxReel

# Build Rust components
cd rust && cargo build

# Install Python package
cd ../python && pip install -e .

# Run tests
pytest tests/
cargo test
```

## 📖 Additional Resources

- [API Documentation](docs/API.md)
- [Features Reference](FEATURES.md)
- [Professional Features](PRO_FEATURES.md) - Comprehensive list of pro video editing features
- [Professional Implementation Guide](PRO_IMPLEMENTATION.md) - Code examples for professional features
- [Project Structure](PROJECT_STRUCTURE.md)
- [Quick Start Guide](QUICKSTART.md)

## ✍️ Author

**Mehmet T. AKALIN** *Digital Vision*

* **GitHub:** [makalin](https://github.com/makalin)
* **Website:** [dv.com.tr](https://dv.com.tr)
* **LinkedIn:** [Mehmet T. AKALIN](https://www.linkedin.com/in/makalin/)
* **X (Twitter):** [@makalin](https://x.com/makalin)

---

<div align="center">
Made with ❤️ in Rust & Python
</div>
